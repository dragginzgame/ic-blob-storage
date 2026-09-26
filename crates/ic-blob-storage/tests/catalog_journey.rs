//! Native multi-object composition, not provider execution or IC authentication.

use std::num::{NonZeroU128, NonZeroUsize};

use candid::Principal;
use ic_blob_storage::{
    model::{
        catalog::{
            BlobCatalog, CatalogError, CatalogInsertOutcome, CatalogLimits, CatalogReferenceKey,
            CatalogUsage, ConfirmedObject,
            pending::{PendingCursorError, PendingPageLimits},
        },
        gateway::{
            GatewayListLimits,
            membership::GatewayMembership,
            registry::{GatewayRegistry, GatewayScope, GatewaySyncError},
        },
        identity::{
            HashParseError, ProviderRootHash,
            batch::{ProviderRootBatch, RootBatchLimits},
        },
        lifecycle::{
            LifecycleChange, LifecycleError, LifecyclePhase, ReferenceId,
            binding::{ObjectBinding, ObjectBindingMismatch, ObjectIdentity, ReferenceKey},
            requests::{
                ReferenceOperation, ReferenceReceiptView, ReferenceRequest, ReferenceRequestError,
                ReferenceRequestId, ReferenceRequestOutcome,
            },
        },
    },
    policy::{
        catalog::{
            CatalogGatewayReadError, CatalogRootStatus, assess_gateway_pending,
            assess_gateway_roots, assess_tenant_usage,
            tenant::{CatalogTenantReadError, assess_tenant_receipt, assess_tenant_references},
        },
        gateway::{GatewayAccessError, GatewayCallbackContext},
        liveness::assess_reference_liveness,
        tenant::{TenantAccessContext, TenantAccessError, assess_tenant_access},
    },
};

fn p(id: u8) -> Principal {
    Principal::from_slice(&[id, 1])
}
fn n(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("positive identity")
}
fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive bound")
}
fn root(id: u8) -> ProviderRootHash {
    ProviderRootHash::try_from([id; 32].as_slice()).expect("root")
}
fn catalog() -> BlobCatalog {
    BlobCatalog::new(
        p(1),
        CatalogLimits {
            max_objects: bound(10),
            max_tenant_objects: bound(6),
            max_physical_bytes: n(1000),
            max_liability_bytes: n(1000),
            max_tenant_logical_bytes: n(500),
            max_references_per_object: bound(3),
            max_receipts_per_object: bound(8),
        },
    )
    .expect("catalog")
}
fn input(id: u8, tenant: u8, namespace: u128) -> ConfirmedObject {
    let object = ObjectBinding::new(
        p(1),
        p(tenant),
        ObjectIdentity {
            namespace: n(namespace),
            object: n(u128::from(id)),
            incarnation: n(1),
        },
    )
    .expect("binding");
    ConfirmedObject {
        root: root(id),
        bytes: 100,
        first: ReferenceKey::new(object, ReferenceId::new(n(1))),
    }
}
fn registry(namespace: u128) -> GatewayRegistry {
    let mut members = GatewayMembership::new(GatewayListLimits {
        max_entries: bound(2),
        max_unique: bound(2),
    });
    members.add(p(7)).expect("gateway");
    GatewayRegistry::new(
        GatewayScope::new(p(1), n(namespace), p(8)).expect("scope"),
        members,
    )
}
fn request(id: u128, operation: ReferenceOperation) -> ReferenceRequest {
    ReferenceRequest {
        id: ReferenceRequestId::new(n(id)),
        operation,
    }
}
fn release(catalog: &mut BlobCatalog, input: ConfirmedObject) {
    let caller = TenantAccessContext {
        service: p(1),
        actor: input.first.object().tenant(),
    };
    assess_tenant_access(
        catalog
            .get(input.root)
            .expect("entry")
            .lifecycle()
            .binding(),
        caller,
    )
    .expect("direct tenant");
    assert_eq!(
        catalog.apply_reference(
            input.root,
            caller.actor,
            request(1, ReferenceOperation::Release(input.first))
        ),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Ok(LifecycleChange::Changed)
        })
    );
}
fn page_limits() -> PendingPageLimits {
    PendingPageLimits {
        max_scan: bound(1),
        max_results: bound(1),
    }
}

#[test]
fn gateway_pages_recheck_revocation_and_reject_foreign_scope_continuations() {
    let mut catalog = catalog();
    for value in [input(1, 2, 1), input(2, 3, 2), input(3, 3, 1)] {
        catalog.insert_confirmed(value).expect("confirmed input");
        release(&mut catalog, value);
    }
    let mut gateways = registry(1);
    let context = GatewayCallbackContext {
        service: p(1),
        actor: p(7),
    };
    let before = catalog.clone();
    let first = assess_gateway_pending(&catalog, &gateways, context, None, page_limits())
        .expect("first page");
    assert_eq!(first.entries[0].root, root(1));
    let cursor = first.next.expect("more entries");
    assert_eq!(
        assess_gateway_pending(&catalog, &registry(2), context, Some(cursor), page_limits()),
        Err(CatalogGatewayReadError::Cursor(PendingCursorError))
    );
    let pending = gateways.begin_sync().expect("sync begun before revocation");
    gateways.remove(p(7));
    assert_eq!(
        gateways.apply_sync(pending, gateways.scope(), &[p(7)]),
        Err(GatewaySyncError::StaleSync)
    );
    assert_eq!(
        assess_gateway_pending(&catalog, &gateways, context, Some(cursor), page_limits()),
        Err(CatalogGatewayReadError::Access(
            GatewayAccessError::NotGateway
        ))
    );
    gateways
        .add(p(7))
        .expect("new explicit operator authorization");
    let filtered =
        assess_gateway_pending(&catalog, &gateways, context, Some(cursor), page_limits())
            .expect("authorized again");
    assert!(filtered.entries.is_empty());
    assert_eq!(filtered.scanned, 1);
    let last = assess_gateway_pending(&catalog, &gateways, context, filtered.next, page_limits())
        .expect("next namespace match");
    assert_eq!(last.entries[0].root, root(3));
    assert_eq!(last.entries[0].object, input(3, 3, 1).first.object());
    assert!(last.next.is_none());
    assert_eq!(catalog, before);
}

#[test]
fn gateway_root_observations_preserve_order_and_do_not_default_unknowns_to_dead() {
    let mut catalog = catalog();
    for value in [
        input(1, 2, 1),
        input(2, 2, 1),
        input(3, 3, 2),
        input(4, 3, 1),
        input(5, 2, 1),
    ] {
        catalog.insert_confirmed(value).expect("insert");
    }
    for value in [input(2, 2, 1), input(4, 3, 1), input(5, 2, 1)] {
        release(&mut catalog, value);
    }
    for value in [input(4, 3, 1), input(5, 2, 1)] {
        catalog
            .confirm_provider_deleted(value.root, value.first.object())
            .expect("supplied deletion evidence");
    }
    catalog
        .confirm_billing_stopped(root(5), input(5, 2, 1).first.object())
        .expect("supplied settlement evidence");
    let raw = vec![
        vec![2; 32],
        vec![],
        vec![1; 32],
        vec![9; 32],
        vec![3; 32],
        vec![2; 32],
        vec![4; 32],
        vec![5; 32],
    ];
    let batch = ProviderRootBatch::from_bytes(
        &raw,
        RootBatchLimits {
            max_entries: bound(8),
            max_bytes: bound(224),
        },
    )
    .expect("bounded batch");
    let gateways = registry(1);
    let caller = GatewayCallbackContext {
        service: p(1),
        actor: p(7),
    };
    let before = catalog.clone();
    assert_eq!(
        assess_gateway_roots(&catalog, &gateways, caller, &batch),
        Ok(vec![
            CatalogRootStatus::Known(LifecyclePhase::DeletionPending),
            CatalogRootStatus::Malformed(HashParseError::InvalidByteLength { actual: 0 }),
            CatalogRootStatus::Known(LifecyclePhase::Live),
            CatalogRootStatus::Unknown,
            CatalogRootStatus::WrongNamespace,
            CatalogRootStatus::Known(LifecyclePhase::DeletionPending),
            CatalogRootStatus::Known(LifecyclePhase::ProviderDeleted),
            CatalogRootStatus::Known(LifecyclePhase::Settled),
        ])
    );
    let empty = ProviderRootBatch::from_bytes(
        &[],
        RootBatchLimits {
            max_entries: bound(1),
            max_bytes: bound(1),
        },
    )
    .expect("empty");
    assert_eq!(
        assess_gateway_roots(&catalog, &gateways, caller, &empty),
        Ok(vec![])
    );
    assert_eq!(catalog, before);
}

#[test]
fn gateway_catalog_reads_reject_unauthorized_context_even_for_empty_batches() {
    let catalog = catalog();
    let mut gateways = registry(1);
    let caller = GatewayCallbackContext {
        service: p(1),
        actor: p(7),
    };
    let batch = ProviderRootBatch::from_bytes(
        &[],
        RootBatchLimits {
            max_entries: bound(1),
            max_bytes: bound(1),
        },
    )
    .expect("empty batch");
    let before = catalog.clone();
    for actor in [
        Principal::anonymous(),
        Principal::management_canister(),
        p(1),
        p(2),
        p(8),
    ] {
        let context = GatewayCallbackContext { actor, ..caller };
        assert_eq!(
            assess_gateway_roots(&catalog, &gateways, context, &batch),
            Err(GatewayAccessError::NotGateway)
        );
        assert_eq!(
            assess_gateway_pending(&catalog, &gateways, context, None, page_limits()),
            Err(CatalogGatewayReadError::Access(
                GatewayAccessError::NotGateway
            ))
        );
    }
    assert_eq!(
        assess_gateway_roots(
            &catalog,
            &gateways,
            GatewayCallbackContext {
                service: p(9),
                ..caller
            },
            &batch
        ),
        Err(GatewayAccessError::WrongService)
    );
    let foreign = GatewayRegistry::new(
        GatewayScope::new(p(9), n(1), p(8)).expect("foreign scope"),
        gateways.gateways().clone(),
    );
    assert_eq!(
        assess_gateway_roots(&catalog, &foreign, caller, &batch),
        Err(GatewayAccessError::WrongService)
    );
    gateways.remove(p(7));
    assert_eq!(
        assess_gateway_roots(&catalog, &gateways, caller, &batch),
        Err(GatewayAccessError::NotGateway)
    );
    assert_eq!(catalog, before);
}

#[test]
fn tenant_usage_views_do_not_expose_another_tenants_totals() {
    let mut catalog = catalog();
    let first = input(1, 2, 1);
    let second = input(2, 3, 1);
    let third = input(3, 2, 2);
    for value in [first, second, third] {
        catalog.insert_confirmed(value).expect("insert");
    }
    let tenant = TenantAccessContext {
        service: p(1),
        actor: p(2),
    };
    assert_eq!(
        assess_tenant_usage(&catalog, tenant)
            .expect("own usage")
            .logical_bytes,
        200
    );
    assert_eq!(
        assess_tenant_usage(
            &catalog,
            TenantAccessContext {
                actor: p(3),
                ..tenant
            }
        )
        .expect("other tenant's own usage")
        .logical_bytes,
        100
    );
    assert_eq!(
        assess_tenant_usage(
            &catalog,
            TenantAccessContext {
                actor: p(1),
                ..tenant
            }
        ),
        Ok(CatalogUsage::default())
    );
    for actor in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            assess_tenant_usage(&catalog, TenantAccessContext { actor, ..tenant }),
            Err(TenantAccessError::NotTenant)
        );
    }
    assert_eq!(
        assess_tenant_usage(
            &catalog,
            TenantAccessContext {
                service: p(9),
                ..tenant
            }
        ),
        Err(TenantAccessError::WrongService)
    );
}

#[test]
fn reference_receipts_remain_bound_across_objects_and_settlement() {
    let mut catalog = catalog();
    let first = input(1, 2, 1);
    let second = input(2, 3, 1);
    let third = input(3, 2, 2);
    for value in [first, second, third] {
        catalog.insert_confirmed(value).expect("insert");
    }
    let tenant = TenantAccessContext {
        service: p(1),
        actor: p(2),
    };
    let retained_key = ReferenceKey::new(first.first.object(), ReferenceId::new(n(2)));
    let retain = request(2, ReferenceOperation::Retain(retained_key));
    assess_tenant_access(first.first.object(), tenant).expect("direct tenant");
    catalog
        .apply_reference(first.root, tenant.actor, retain)
        .expect("retain");
    let before = catalog.clone();
    assert_eq!(
        catalog.apply_reference(second.root, tenant.actor, retain),
        Err(CatalogError::Request(
            ReferenceRequestError::BindingMismatch(ObjectBindingMismatch::Tenant)
        ))
    );
    assert_eq!(catalog, before);
    release(&mut catalog, first);
    assert!(
        !assess_reference_liveness(
            catalog.get(first.root).expect("entry").lifecycle(),
            first.first,
            tenant
        )
        .expect("authorized read")
    );
    assert_eq!(catalog.usage().logical_bytes, 300);
    catalog
        .apply_reference(
            first.root,
            tenant.actor,
            request(3, ReferenceOperation::Release(retained_key)),
        )
        .expect("last release");
    assert_eq!(catalog.usage().logical_bytes, 200);
    assert_eq!(catalog.usage().physical_bytes, 300);
    let before = catalog.clone();
    assert_eq!(
        catalog.confirm_provider_deleted(first.root, second.first.object()),
        Err(CatalogError::Lifecycle(
            ic_blob_storage::model::lifecycle::LifecycleError::BindingMismatch(
                ObjectBindingMismatch::Tenant
            )
        ))
    );
    assert_eq!(catalog, before);
    catalog
        .confirm_provider_deleted(first.root, first.first.object())
        .expect("deletion evidence");
    assert_eq!(catalog.usage().physical_bytes, 200);
    assert_eq!(catalog.usage().liability_bytes, 300);
    catalog
        .confirm_billing_stopped(first.root, first.first.object())
        .expect("settlement evidence");
    let before = catalog.clone();
    assert_eq!(
        catalog.apply_reference(first.root, tenant.actor, retain),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        })
    );
    assert_eq!(
        catalog.insert_confirmed(first),
        Ok(CatalogInsertOutcome::Existing)
    );
    assert_eq!(catalog, before);
    assert_eq!(catalog.usage().objects, 3);
    assert_eq!(catalog.usage().active_references, 2);
    assert_eq!(catalog.usage().reference_slots, 4);
    assert_eq!(catalog.usage().receipt_slots, 3);
    assert_eq!(catalog.usage().liability_bytes, 200);
    assert_eq!(
        assess_tenant_usage(&catalog, tenant)
            .expect("own usage")
            .logical_bytes,
        100
    );
}

fn catalog_key(root: ProviderRootHash, reference: ReferenceKey) -> CatalogReferenceKey {
    CatalogReferenceKey { root, reference }
}

#[test]
fn consumer_reads_preserve_original_receipts_and_current_reference_status_separately() {
    let mut catalog = catalog();
    let first = input(1, 2, 1);
    let second = input(2, 2, 2);
    let foreign = input(3, 3, 1);
    for value in [first, second, foreign] {
        catalog.insert_confirmed(value).expect("confirmed object");
    }
    let caller = TenantAccessContext {
        service: p(1),
        actor: p(2),
    };
    let retained = ReferenceKey::new(first.first.object(), ReferenceId::new(n(2)));
    let unknown = ReferenceKey::new(first.first.object(), ReferenceId::new(n(3)));
    let failed = request(2, ReferenceOperation::Release(retained));
    let retain = request(3, ReferenceOperation::Retain(retained));
    assert_eq!(
        assess_tenant_receipt(&catalog, first.root, failed, caller),
        Ok(None)
    );
    assert_eq!(
        catalog.apply_reference(first.root, caller.actor, failed),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Err(LifecycleError::UnknownReference)
        })
    );
    catalog
        .apply_reference(first.root, caller.actor, retain)
        .expect("retain");
    release(&mut catalog, first);
    let keys = [
        catalog_key(second.root, second.first),
        catalog_key(first.root, first.first),
        catalog_key(first.root, retained),
        catalog_key(first.root, unknown),
        catalog_key(second.root, second.first),
    ];
    let before = catalog.clone();
    assert_eq!(
        assess_tenant_references(&catalog, &keys, caller, bound(keys.len())),
        Ok(vec![true, false, true, false, true])
    );
    assert_eq!(
        assess_tenant_references(&catalog, &[], caller, bound(1)),
        Ok(vec![])
    );
    assert_eq!(
        assess_tenant_receipt(&catalog, first.root, failed, caller),
        Ok(Some(ReferenceReceiptView {
            result: Err(LifecycleError::UnknownReference)
        }))
    );
    assert_eq!(catalog, before);

    catalog
        .apply_reference(
            first.root,
            caller.actor,
            request(4, ReferenceOperation::Release(retained)),
        )
        .expect("final release");
    catalog
        .confirm_provider_deleted(first.root, first.first.object())
        .expect("deletion evidence");
    catalog
        .confirm_billing_stopped(first.root, first.first.object())
        .expect("billing evidence");
    let before = catalog.clone();
    assert_eq!(
        assess_tenant_references(&catalog, &keys, caller, bound(keys.len())),
        Ok(vec![true, false, false, false, true])
    );
    assert_eq!(
        assess_tenant_receipt(&catalog, first.root, retain, caller),
        Ok(Some(ReferenceReceiptView {
            result: Ok(LifecycleChange::Changed)
        }))
    );
    assert_eq!(
        assess_tenant_receipt(&catalog, first.root, failed, caller),
        Ok(Some(ReferenceReceiptView {
            result: Err(LifecycleError::UnknownReference)
        }))
    );
    assert_eq!(
        assess_tenant_receipt(
            &catalog,
            first.root,
            request(5, ReferenceOperation::Retain(unknown)),
            caller
        ),
        Ok(None)
    );
    assert_eq!(catalog, before);
    assert_eq!(catalog.usage().physical_bytes, 200);
    assert_eq!(catalog.usage().liability_bytes, 200);
    assert_eq!(catalog.usage().receipt_slots, 4);
}

#[test]
fn consumer_lookup_checks_context_and_bounds_without_disclosing_foreign_objects() {
    let mut catalog = catalog();
    let first = input(1, 2, 1);
    let foreign = input(2, 3, 1);
    for value in [first, foreign] {
        catalog.insert_confirmed(value).expect("confirmed object");
    }
    release(&mut catalog, foreign);
    let caller = TenantAccessContext {
        service: p(1),
        actor: p(2),
    };
    let key = CatalogReferenceKey {
        root: first.root,
        reference: first.first,
    };
    let before = catalog.clone();
    let malformed = CatalogReferenceKey {
        reference: foreign.first,
        ..key
    };
    for hidden_root in [foreign.root, root(9)] {
        // Claiming the caller's own binding cannot authorize someone else's root.
        let hidden = CatalogReferenceKey {
            root: hidden_root,
            ..key
        };
        assert_eq!(
            assess_tenant_references(&catalog, &[hidden; 2], caller, bound(1)),
            Err(CatalogTenantReadError::TooManyEntries {
                actual: 2,
                maximum: 1
            })
        );
        for keys in [[key, hidden], [malformed, hidden], [hidden, malformed]] {
            assert_eq!(
                assess_tenant_references(&catalog, &keys, caller, bound(2)),
                Err(CatalogTenantReadError::Unavailable)
            );
        }
        for reference in [first.first, foreign.first] {
            for id in [1, 99] {
                assert_eq!(
                    assess_tenant_receipt(
                        &catalog,
                        hidden_root,
                        request(id, ReferenceOperation::Release(reference)),
                        caller
                    ),
                    Err(CatalogTenantReadError::Unavailable)
                );
            }
        }
    }
    for actor in [p(1), p(7), p(8)] {
        let context = TenantAccessContext { actor, ..caller };
        assert_eq!(
            assess_tenant_references(&catalog, &[key], context, bound(1)),
            Err(CatalogTenantReadError::Unavailable)
        );
        assert_eq!(
            assess_tenant_receipt(
                &catalog,
                first.root,
                request(1, ReferenceOperation::Release(first.first)),
                context
            ),
            Err(CatalogTenantReadError::Unavailable)
        );
    }
    assert_eq!(catalog, before);
}

#[test]
fn consumer_lookup_rejects_every_binding_dimension_even_for_unrecorded_requests() {
    let mut catalog = catalog();
    let first = input(1, 2, 1);
    catalog.insert_confirmed(first).expect("confirmed object");
    release(&mut catalog, first);
    let object = first.first.object();
    let identity = object.identity();
    let caller = TenantAccessContext {
        service: p(1),
        actor: p(2),
    };
    let variants = [
        (
            ObjectBinding::new(p(9), p(2), identity).expect("service"),
            ObjectBindingMismatch::Service,
        ),
        (
            ObjectBinding::new(p(1), p(9), identity).expect("tenant"),
            ObjectBindingMismatch::Tenant,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    namespace: n(2),
                    ..identity
                },
            )
            .expect("namespace"),
            ObjectBindingMismatch::Namespace,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    object: n(2),
                    ..identity
                },
            )
            .expect("object"),
            ObjectBindingMismatch::Object,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    incarnation: n(2),
                    ..identity
                },
            )
            .expect("incarnation"),
            ObjectBindingMismatch::Incarnation,
        ),
    ];
    let before = catalog.clone();
    for (binding, error) in variants {
        let reference = ReferenceKey::new(binding, first.first.reference());
        assert_eq!(
            assess_tenant_references(
                &catalog,
                &[
                    catalog_key(first.root, first.first),
                    catalog_key(first.root, reference),
                ],
                caller,
                bound(2)
            ),
            Err(CatalogTenantReadError::Binding(error))
        );
        for id in [1, 99] {
            assert_eq!(
                assess_tenant_receipt(
                    &catalog,
                    first.root,
                    request(id, ReferenceOperation::Release(reference)),
                    caller
                ),
                Err(CatalogTenantReadError::Request(
                    ReferenceRequestError::BindingMismatch(error)
                ))
            );
        }
        assert_eq!(catalog, before);
    }
    assert_eq!(
        assess_tenant_receipt(
            &catalog,
            first.root,
            request(1, ReferenceOperation::Retain(first.first)),
            caller
        ),
        Err(CatalogTenantReadError::Request(
            ReferenceRequestError::RequestConflict
        ))
    );
    assert_eq!(catalog, before);
}

#[test]
fn consumer_lookup_checks_execution_context_before_empty_or_oversized_input() {
    let catalog = catalog();
    let first = input(1, 2, 1);
    let caller = TenantAccessContext {
        service: p(1),
        actor: p(2),
    };
    let key = catalog_key(first.root, first.first);
    let before = catalog.clone();
    for actor in [Principal::anonymous(), Principal::management_canister()] {
        let context = TenantAccessContext { actor, ..caller };
        for keys in [&[][..], &[key, key][..]] {
            assert_eq!(
                assess_tenant_references(&catalog, keys, context, bound(1)),
                Err(CatalogTenantReadError::Access(TenantAccessError::NotTenant))
            );
        }
        assert_eq!(
            assess_tenant_receipt(
                &catalog,
                root(9),
                request(1, ReferenceOperation::Release(first.first)),
                context
            ),
            Err(CatalogTenantReadError::Access(TenantAccessError::NotTenant))
        );
    }
    let wrong_service = TenantAccessContext {
        service: p(9),
        ..caller
    };
    assert_eq!(
        assess_tenant_references(&catalog, &[], wrong_service, bound(1)),
        Err(CatalogTenantReadError::Access(
            TenantAccessError::WrongService
        ))
    );
    assert_eq!(
        assess_tenant_receipt(
            &catalog,
            first.root,
            request(1, ReferenceOperation::Release(first.first)),
            wrong_service
        ),
        Err(CatalogTenantReadError::Access(
            TenantAccessError::WrongService
        ))
    );
    assert_eq!(catalog, before);
}
