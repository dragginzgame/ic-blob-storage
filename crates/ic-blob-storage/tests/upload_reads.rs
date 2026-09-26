//! Native admission/read composition. Context and provider facts are substitutes.

use candid::Principal;
use ic_blob_storage::{
    model::{
        catalog::{
            CatalogLimits,
            admission::{
                UploadCatalog, UploadLimits, UploadObject, UploadPhase, UploadRequest,
                UploadRequestId, UploadUsage,
                read::{UploadCursorError, UploadPageLimits, UploadRootState},
            },
        },
        gateway::{
            GatewayListLimits,
            membership::GatewayMembership,
            registry::{GatewayRegistry, GatewayScope},
        },
        identity::{
            ContentDigest, HashParseError, ProviderRootHash,
            batch::{ProviderRootBatch, RootBatchLimits},
        },
        lifecycle::{
            LifecyclePhase, ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
            requests::{ReferenceOperation, ReferenceRequest, ReferenceRequestId},
        },
    },
    policy::{
        catalog::upload::{
            UploadReadError, UploadRootStatus, assess_gateway_upload_roots,
            assess_tenant_active_uploads, assess_tenant_upload_usage,
        },
        gateway::{GatewayAccessError, GatewayCallbackContext},
        tenant::{TenantAccessContext, TenantAccessError},
    },
};
use std::num::{NonZeroU128, NonZeroUsize};

fn n(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("nonzero")
}
fn b(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("nonzero")
}
fn p(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}
fn root(value: u8) -> ProviderRootHash {
    ProviderRootHash::try_from([value; 32].as_slice()).expect("root")
}
fn owner(service: u8) -> UploadCatalog {
    UploadCatalog::new(
        p(service),
        CatalogLimits {
            max_objects: b(16),
            max_tenant_objects: b(12),
            max_physical_bytes: n(10000),
            max_liability_bytes: n(10000),
            max_tenant_logical_bytes: n(10000),
            max_references_per_object: b(1),
            max_receipts_per_object: b(1),
        },
        UploadLimits {
            max_active: b(16),
            max_tenant_active: b(12),
        },
    )
    .expect("owner")
}
fn request(id: u8, tenant: u8, namespace: u128) -> UploadRequest {
    UploadRequest {
        id: UploadRequestId::new(n(u128::from(id))),
        object: UploadObject {
            root: root(id),
            bytes: 100,
            first: ReferenceKey::new(
                ObjectBinding::new(
                    p(1),
                    p(tenant),
                    ObjectIdentity {
                        namespace: n(namespace),
                        object: n(u128::from(id)),
                        incarnation: n(1),
                    },
                )
                .expect("binding"),
                ReferenceId::new(n(1)),
            ),
        },
        content: ContentDigest::compute(&[id]),
    }
}
fn context(tenant: u8) -> TenantAccessContext {
    TenantAccessContext {
        service: p(1),
        actor: p(tenant),
    }
}
fn limits(scan: usize, results: usize) -> UploadPageLimits {
    UploadPageLimits {
        max_scan: b(scan),
        max_results: b(results),
    }
}
fn reserve(owner: &mut UploadCatalog, request: UploadRequest) {
    owner
        .reserve(request.object.first.object().tenant(), request)
        .expect("reserve");
}
fn confirm(owner: &mut UploadCatalog, request: UploadRequest) {
    owner
        .mark_exposure_possible(request.object.first.object().tenant(), request)
        .expect("exposure");
    owner.confirm_upload(request).expect("completion fact");
}
fn registry(service: u8, namespace: u128) -> GatewayRegistry {
    let mut members = GatewayMembership::new(GatewayListLimits {
        max_entries: b(2),
        max_unique: b(2),
    });
    members.add(p(7)).expect("gateway");
    GatewayRegistry::new(
        GatewayScope::new(p(service), n(namespace), p(8)).expect("scope"),
        members,
    )
}
fn batch(inputs: &[Vec<u8>]) -> ProviderRootBatch {
    ProviderRootBatch::from_bytes(
        inputs,
        RootBatchLimits {
            max_entries: b(16),
            max_bytes: b(512),
        },
    )
    .expect("bounded batch")
}
fn gateway() -> GatewayCallbackContext {
    GatewayCallbackContext {
        service: p(1),
        actor: p(7),
    }
}

#[test]
fn tenant_pages_bound_work_skip_terminal_history_and_do_not_scan_other_tenants() {
    let mut owner = owner(1);
    let requests = [
        request(1, 2, 1),
        request(2, 3, 1),
        request(3, 2, 2),
        request(4, 2, 1),
        request(5, 2, 1),
    ];
    for request in requests {
        reserve(&mut owner, request);
    }
    owner.cancel(p(2), requests[0]).expect("cancel");
    owner
        .mark_exposure_possible(p(2), requests[2])
        .expect("uncertain");
    confirm(&mut owner, requests[3]);
    let before = owner.usage();
    for (scan, results) in [(1, 1), (2, 1), (1, 3), (16, 16)] {
        let mut cursor = None;
        let mut seen = Vec::new();
        let mut scanned = 0;
        loop {
            let page =
                assess_tenant_active_uploads(&owner, context(2), cursor, limits(scan, results))
                    .expect("page");
            assert!(page.scanned <= scan);
            assert!(page.entries.len() <= results);
            assert_eq!(
                assess_tenant_active_uploads(&owner, context(2), cursor, limits(scan, results)),
                Ok(page.clone())
            );
            scanned += page.scanned;
            seen.extend(
                page.entries
                    .into_iter()
                    .map(|entry| (entry.request, entry.phase)),
            );
            cursor = page.next;
            if cursor.is_none() {
                break;
            }
        }
        assert_eq!(
            seen,
            vec![
                (requests[2], UploadPhase::ExposurePossible),
                (requests[4], UploadPhase::Reserved)
            ]
        );
        assert_eq!(scanned, 4); // This tenant's history only; terminal entries still cost work.
    }
    let empty = assess_tenant_active_uploads(&owner, context(2), None, limits(1, 1))
        .expect("empty filtered page");
    assert!(empty.entries.is_empty());
    assert!(empty.next.is_some());
    let other = assess_tenant_active_uploads(&owner, context(3), None, limits(16, 16))
        .expect("other tenant");
    assert_eq!(other.scanned, 1);
    assert_eq!(other.entries[0].request, requests[1]);
    assert_eq!(owner.usage(), before);
}

#[test]
fn pages_observe_intervening_transitions_and_new_lower_ids_require_a_fresh_sweep() {
    let mut owner = owner(1);
    let requests = [request(2, 2, 1), request(4, 2, 1), request(6, 2, 2)];
    for request in requests {
        reserve(&mut owner, request);
    }
    let first =
        assess_tenant_active_uploads(&owner, context(2), None, limits(1, 1)).expect("first");
    assert_eq!(first.entries[0].request, requests[0]);
    owner
        .cancel(p(2), requests[1])
        .expect("intervening cancellation");
    confirm(&mut owner, requests[2]);
    let inserted = request(1, 2, 1);
    reserve(&mut owner, inserted);
    let resumed = assess_tenant_active_uploads(&owner, context(2), first.next, limits(16, 16))
        .expect("resume");
    assert!(resumed.entries.is_empty());
    assert!(resumed.next.is_none());
    assert_eq!(resumed.scanned, 2);
    let restarted =
        assess_tenant_active_uploads(&owner, context(2), None, limits(16, 16)).expect("restart");
    assert_eq!(
        restarted
            .entries
            .iter()
            .map(|entry| entry.request)
            .collect::<Vec<_>>(),
        vec![inserted, requests[0]]
    );
    assert_eq!(owner.usage().active_reservations, 2);
}

#[test]
fn cursor_scope_and_current_context_are_checked_even_for_empty_reads() {
    let mut owner = owner(1);
    for id in [1, 2] {
        reserve(&mut owner, request(id, 2, 1));
    }
    let cursor = assess_tenant_active_uploads(&owner, context(2), None, limits(1, 1))
        .expect("page")
        .next
        .expect("continuation");
    let before = owner.usage();
    assert_eq!(
        assess_tenant_active_uploads(&owner, context(3), Some(cursor), limits(1, 1)),
        Err(UploadReadError::Cursor(UploadCursorError))
    );
    let other = self::owner(9);
    assert_eq!(
        other.active_uploads(p(2), Some(cursor), limits(1, 1)),
        Err(UploadCursorError)
    );
    for (context, error) in [
        (
            TenantAccessContext {
                service: p(9),
                actor: p(2),
            },
            TenantAccessError::WrongService,
        ),
        (
            TenantAccessContext {
                service: p(1),
                actor: Principal::anonymous(),
            },
            TenantAccessError::NotTenant,
        ),
        (
            TenantAccessContext {
                service: p(1),
                actor: Principal::management_canister(),
            },
            TenantAccessError::NotTenant,
        ),
    ] {
        assert_eq!(
            assess_tenant_active_uploads(&owner, context, Some(cursor), limits(1, 1)),
            Err(UploadReadError::Access(error))
        );
        assert_eq!(assess_tenant_upload_usage(&owner, context), Err(error));
    }
    // Service principal/controller-like caller has no access to the tenant's rows.
    let unrelated = assess_tenant_active_uploads(&owner, context(1), None, limits(1, 1))
        .expect("own empty history");
    assert!(unrelated.entries.is_empty());
    assert_eq!(unrelated.scanned, 0);
    assert_eq!(
        assess_tenant_upload_usage(&owner, context(1)),
        Ok(UploadUsage::default())
    );
    assert_eq!(owner.usage(), before);
}

#[test]
fn maximum_request_id_and_identical_ids_in_different_tenants_have_exact_boundaries() {
    let mut owner = owner(1);
    let max = UploadRequest {
        id: UploadRequestId::new(NonZeroU128::MAX),
        ..request(1, 2, 1)
    };
    let foreign = UploadRequest {
        id: max.id,
        ..request(2, 3, 1)
    };
    for request in [max, foreign] {
        reserve(&mut owner, request);
    }
    let page =
        assess_tenant_active_uploads(&owner, context(2), None, limits(1, 1)).expect("max ID");
    assert_eq!(page.entries[0].request, max);
    assert_eq!(page.scanned, 1);
    assert!(page.next.is_none());
    owner.cancel(p(2), max).expect("cancel max ID");
    let page = assess_tenant_active_uploads(&owner, context(2), None, limits(1, 1))
        .expect("terminal max ID");
    assert!(page.entries.is_empty());
    assert!(page.next.is_none());
    assert_eq!(page.scanned, 1);
}

#[test]
fn tenant_usage_includes_reservations_and_retains_separate_provider_obligations() {
    let mut owner = owner(1);
    let a = request(1, 2, 1);
    let c = request(3, 2, 2);
    for request in [a, request(2, 3, 1), c] {
        reserve(&mut owner, request);
    }
    assert_eq!(
        assess_tenant_upload_usage(&owner, context(2))
            .expect("usage")
            .reserved_bytes,
        200
    );
    confirm(&mut owner, a);
    let usage = assess_tenant_upload_usage(&owner, context(2)).expect("transferred");
    assert_eq!(usage.reserved_bytes, 100);
    assert_eq!(
        (
            usage.logical_bytes,
            usage.physical_bytes,
            usage.liability_bytes
        ),
        (200, 200, 200)
    );
    owner
        .apply_reference(
            a.object.root,
            p(2),
            ReferenceRequest {
                id: ReferenceRequestId::new(n(1)),
                operation: ReferenceOperation::Release(a.object.first),
            },
        )
        .expect("release");
    owner
        .confirm_provider_deleted(a.object.root, a.object.first.object())
        .expect("deletion fact");
    let usage = assess_tenant_upload_usage(&owner, context(2)).expect("separate liability");
    assert_eq!(
        (
            usage.logical_bytes,
            usage.physical_bytes,
            usage.liability_bytes
        ),
        (100, 100, 200)
    );
    owner
        .confirm_billing_stopped(a.object.root, a.object.first.object())
        .expect("billing fact");
    owner.cancel(p(2), c).expect("cancel reservation");
    assert_eq!(
        assess_tenant_upload_usage(&owner, context(2)),
        Ok(UploadUsage {
            operations: 2,
            ..UploadUsage::default()
        })
    );
    assert_eq!(
        assess_tenant_upload_usage(&owner, context(3))
            .expect("other tenant untouched")
            .reserved_bytes,
        100
    );
}

#[test]
fn gateway_reads_pending_cancelled_and_confirmed_roots_without_leaking_other_namespaces() {
    let mut owner = owner(1);
    let a = request(1, 2, 1);
    let uncertain = request(2, 2, 1);
    let cancelled = request(3, 2, 1);
    let foreign = request(4, 3, 2);
    for request in [a, uncertain, cancelled, foreign] {
        reserve(&mut owner, request);
    }
    owner
        .mark_exposure_possible(p(2), uncertain)
        .expect("exposure");
    owner.cancel(p(2), cancelled).expect("cancel");
    let batch = batch(&[
        vec![2; 32],
        vec![4; 32],
        vec![],
        vec![1; 32],
        vec![2; 32],
        vec![3; 32],
        vec![9; 32],
    ]);
    let registry = registry(1, 1);
    let before = owner.usage();
    assert_eq!(
        assess_gateway_upload_roots(&owner, &registry, gateway(), &batch),
        Ok(vec![
            UploadRootStatus::Known(UploadRootState::ExposurePossible),
            UploadRootStatus::WrongNamespace,
            UploadRootStatus::Malformed(HashParseError::InvalidByteLength { actual: 0 }),
            UploadRootStatus::Known(UploadRootState::Reserved),
            UploadRootStatus::Known(UploadRootState::ExposurePossible),
            UploadRootStatus::Known(UploadRootState::Cancelled),
            UploadRootStatus::Unknown,
        ])
    );
    assert_eq!(owner.usage(), before);
    assert_eq!(
        owner.phase(p(2), uncertain),
        Ok(UploadPhase::ExposurePossible)
    );
    confirm(&mut owner, a);
    assert_eq!(
        owner.root_view(a.object.root).expect("confirmed").state,
        UploadRootState::Confirmed(LifecyclePhase::Live)
    );
    owner
        .apply_reference(
            a.object.root,
            p(2),
            ReferenceRequest {
                id: ReferenceRequestId::new(n(1)),
                operation: ReferenceOperation::Release(a.object.first),
            },
        )
        .expect("release");
    for phase in [
        LifecyclePhase::DeletionPending,
        LifecyclePhase::ProviderDeleted,
        LifecyclePhase::Settled,
    ] {
        assert_eq!(
            assess_gateway_upload_roots(&owner, &registry, gateway(), &self::batch(&[vec![1; 32]])),
            Ok(vec![UploadRootStatus::Known(UploadRootState::Confirmed(
                phase
            ))])
        );
        match phase {
            LifecyclePhase::DeletionPending => {
                owner
                    .confirm_provider_deleted(a.object.root, a.object.first.object())
                    .expect("delete");
            }
            LifecyclePhase::ProviderDeleted => {
                owner
                    .confirm_billing_stopped(a.object.root, a.object.first.object())
                    .expect("billing");
            }
            _ => {}
        }
    }
}

#[test]
fn gateway_membership_is_rechecked_before_all_roots_including_empty_batches() {
    let mut owner = owner(1);
    reserve(&mut owner, request(1, 2, 1));
    let mut registry = registry(1, 1);
    let foreign = self::registry(9, 1);
    for batch in [batch(&[]), batch(&[vec![1; 32], vec![]])] {
        assert!(assess_gateway_upload_roots(&owner, &registry, gateway(), &batch).is_ok());
        assert_eq!(
            assess_gateway_upload_roots(&owner, &foreign, gateway(), &batch),
            Err(GatewayAccessError::WrongService)
        );
        assert_eq!(
            assess_gateway_upload_roots(
                &owner,
                &registry,
                GatewayCallbackContext {
                    service: p(9),
                    ..gateway()
                },
                &batch
            ),
            Err(GatewayAccessError::WrongService)
        );
        for actor in [p(1), p(2), Principal::anonymous()] {
            assert_eq!(
                assess_gateway_upload_roots(
                    &owner,
                    &registry,
                    GatewayCallbackContext { actor, ..gateway() },
                    &batch
                ),
                Err(GatewayAccessError::NotGateway)
            );
        }
    }
    registry.remove(p(7));
    for batch in [batch(&[]), batch(&[vec![1; 32]])] {
        assert_eq!(
            assess_gateway_upload_roots(&owner, &registry, gateway(), &batch),
            Err(GatewayAccessError::NotGateway)
        );
    }
}

#[test]
fn duplicate_heavy_mixed_batches_share_one_scan_and_match_single_root_observations() {
    let mut owner = owner(1);
    for id in 1..=12 {
        reserve(&mut owner, request(id, 2, 1));
    }
    owner
        .cancel(p(2), request(12, 2, 1))
        .expect("retained terminal history");
    owner
        .mark_exposure_possible(p(2), request(3, 2, 1))
        .expect("uncertain");
    confirm(&mut owner, request(5, 2, 1));
    let mut inputs = vec![
        vec![12; 32],
        vec![5; 32],
        vec![3; 32],
        vec![9; 32],
        vec![99; 32],
        vec![],
        vec![0; 31],
    ];
    inputs.extend((0..128).map(|_| vec![12; 32]));
    let batch = ProviderRootBatch::from_bytes(
        &inputs,
        RootBatchLimits {
            max_entries: b(inputs.len()),
            max_bytes: b(inputs.len() * 32),
        },
    )
    .expect("bounded duplicate-heavy batch");
    let before = owner.usage();
    let observed = owner.root_views(&batch);
    // The last requested history row is visited once, regardless of duplicates.
    assert_eq!(observed.scanned_operations, before.operations);
    let expected: Vec<_> = batch
        .entries()
        .iter()
        .map(|entry| match entry {
            Ok(root) => Ok(owner.root_view(*root)),
            Err(error) => Err(*error),
        })
        .collect();
    assert_eq!(observed.entries, expected);
    assert_eq!(owner.usage(), before);
    assert_eq!(
        observed.entries[0].expect("valid").expect("known").state,
        UploadRootState::Cancelled
    );
    assert_eq!(
        observed.entries[1].expect("valid").expect("known").state,
        UploadRootState::Confirmed(LifecyclePhase::Live)
    );
    assert_eq!(
        observed.entries[2].expect("valid").expect("known").state,
        UploadRootState::ExposurePossible
    );
    assert_eq!(observed.entries[4], Ok(None));
    assert_eq!(
        observed.entries[5],
        Err(HashParseError::InvalidByteLength { actual: 0 })
    );
    // Temporary results do not survive a read: later confirmation is observed.
    confirm(&mut owner, request(9, 2, 1));
    let updated = owner.root_views(&batch);
    assert_eq!(
        updated.entries[3].expect("valid").expect("known").state,
        UploadRootState::Confirmed(LifecyclePhase::Live)
    );
    assert_eq!(updated.scanned_operations, before.operations);
}

#[test]
fn shared_scan_stops_at_last_needed_operation_and_is_skipped_when_unnecessary() {
    let mut owner = owner(1);
    for id in 1..=12 {
        reserve(&mut owner, request(id, 2, 1));
    }
    let single = owner.root_views(&batch(&[vec![3; 32]]));
    let duplicates = owner.root_views(&batch(&vec![vec![3; 32]; 16]));
    assert_eq!(single.scanned_operations, 3);
    assert_eq!(duplicates.scanned_operations, single.scanned_operations);
    assert_eq!(duplicates.entries, vec![single.entries[0]; 16]);
    confirm(&mut owner, request(3, 2, 1));
    for inputs in [
        vec![],
        vec![vec![], vec![0; 31]],
        vec![vec![99; 32]],
        vec![vec![3; 32]; 16],
    ] {
        let view = owner.root_views(&batch(&inputs));
        assert_eq!(view.scanned_operations, 0);
        assert_eq!(view.entries.len(), inputs.len());
    }
    let fresh = owner.root_views(&batch(&[vec![3; 32]]));
    assert_eq!(
        fresh.entries[0].expect("valid").expect("confirmed").state,
        UploadRootState::Confirmed(LifecyclePhase::Live)
    );
}
