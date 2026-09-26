use super::*;
use crate::model::lifecycle::{
    ReferenceId,
    binding::ObjectIdentity,
    requests::{ReferenceOperation, ReferenceRequestId},
};
use pending::{PendingCursorError, PendingPageLimits};

fn p(id: u8) -> Principal {
    Principal::from_slice(&[id, 1])
}
fn n(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("positive value")
}
fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("bound")
}
fn root(id: u8) -> ProviderRootHash {
    ProviderRootHash::try_from([id; 32].as_slice()).expect("root")
}
fn limits() -> CatalogLimits {
    CatalogLimits {
        max_objects: bound(8),
        max_tenant_objects: bound(6),
        max_physical_bytes: n(1000),
        max_liability_bytes: n(1000),
        max_tenant_logical_bytes: n(1000),
        max_references_per_object: bound(3),
        max_receipts_per_object: bound(8),
    }
}
fn input(id: u8, tenant: u8, namespace: u128, bytes: u64) -> ConfirmedObject {
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
        bytes,
        first: ReferenceKey::new(object, ReferenceId::new(n(1))),
    }
}
fn request(id: u128, operation: ReferenceOperation) -> ReferenceRequest {
    ReferenceRequest {
        id: ReferenceRequestId::new(n(id)),
        operation,
    }
}
fn release(catalog: &mut BlobCatalog, input: ConfirmedObject) {
    assert_eq!(
        catalog.apply_reference(
            input.root,
            input.first.object().tenant(),
            request(1, ReferenceOperation::Release(input.first))
        ),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Ok(LifecycleChange::Changed)
        })
    );
}
fn deleted(catalog: &mut BlobCatalog, input: ConfirmedObject) {
    catalog
        .confirm_provider_deleted(input.root, input.first.object())
        .expect("deletion evidence");
}
fn settled(catalog: &mut BlobCatalog, input: ConfirmedObject) {
    deleted(catalog, input);
    catalog
        .confirm_billing_stopped(input.root, input.first.object())
        .expect("billing evidence");
}

#[test]
fn capacity_rejections_are_atomic_and_do_not_reserve_a_root() {
    for (limits, resource) in [
        (
            CatalogLimits {
                max_objects: bound(1),
                ..limits()
            },
            CatalogCapacity::Objects,
        ),
        (
            CatalogLimits {
                max_tenant_objects: bound(1),
                ..limits()
            },
            CatalogCapacity::TenantObjects,
        ),
        (
            CatalogLimits {
                max_physical_bytes: n(100),
                ..limits()
            },
            CatalogCapacity::PhysicalBytes,
        ),
        (
            CatalogLimits {
                max_liability_bytes: n(100),
                ..limits()
            },
            CatalogCapacity::LiabilityBytes,
        ),
        (
            CatalogLimits {
                max_tenant_logical_bytes: n(100),
                ..limits()
            },
            CatalogCapacity::TenantLogicalBytes,
        ),
    ] {
        let mut catalog = BlobCatalog::new(p(1), limits).expect("catalog");
        let first = input(1, 2, 1, 100);
        catalog.insert_confirmed(first).expect("exact byte bound");
        // Same tenant, other namespace: tenant capacity cannot be bypassed by namespace.
        let second = input(2, 2, 2, 1);
        let before = catalog.clone();
        assert_eq!(
            catalog.insert_confirmed(second),
            Err(CatalogError::Capacity(resource))
        );
        assert_eq!(catalog, before);
        assert_eq!(
            catalog.insert_confirmed(first),
            Ok(CatalogInsertOutcome::Existing)
        );
        assert_eq!(catalog, before);
        release(&mut catalog, first);
        settled(&mut catalog, first);
        if matches!(
            resource,
            CatalogCapacity::Objects | CatalogCapacity::TenantObjects
        ) {
            assert_eq!(
                catalog.insert_confirmed(second),
                Err(CatalogError::Capacity(resource))
            );
        } else {
            // Reuse the rejected root with a different object/tenant. No rejected claim remains.
            let changed = ConfirmedObject {
                first: input(3, 3, 1, 1).first,
                ..second
            };
            assert_eq!(
                catalog.insert_confirmed(changed),
                Ok(CatalogInsertOutcome::Inserted)
            );
        }
    }
}

#[test]
fn zero_byte_history_cannot_bypass_the_tenant_object_limit() {
    let mut catalog = BlobCatalog::new(
        p(1),
        CatalogLimits {
            max_objects: bound(3),
            max_tenant_objects: bound(2),
            ..limits()
        },
    )
    .expect("catalog");
    let first = input(1, 2, 1, 0);
    let second = input(2, 2, 2, 0);
    for object in [first, second] {
        catalog.insert_confirmed(object).expect("tenant slot");
        release(&mut catalog, object);
        settled(&mut catalog, object);
    }
    let before = catalog.clone();
    assert_eq!(catalog.tenant_usage(p(2)).logical_bytes, 0);
    assert_eq!(catalog.tenant_usage(p(2)).objects, 2);
    assert_eq!(
        catalog.insert_confirmed(input(3, 2, 3, 0)),
        Err(CatalogError::Capacity(CatalogCapacity::TenantObjects))
    );
    assert_eq!(catalog, before);
    assert_eq!(
        catalog.insert_confirmed(first),
        Ok(CatalogInsertOutcome::Existing)
    );
    assert_eq!(catalog, before);
    // Another tenant can still use the shared slot; no failed root claim leaked.
    catalog
        .insert_confirmed(input(3, 3, 3, 0))
        .expect("other tenant's slot");
    assert_eq!(catalog.usage().objects, 3);
}

#[test]
fn logical_release_and_physical_deletion_do_not_erase_billing_liabilities() {
    let mut catalog = BlobCatalog::new(
        p(1),
        CatalogLimits {
            max_physical_bytes: n(100),
            max_liability_bytes: n(100),
            ..limits()
        },
    )
    .expect("catalog");
    let first = input(1, 2, 1, 100);
    let second = input(2, 3, 2, 100);
    catalog.insert_confirmed(first).expect("initial object");
    release(&mut catalog, first);
    assert_eq!(catalog.usage().logical_bytes, 0);
    assert_eq!(catalog.usage().pending_deletions, 1);
    assert_eq!(
        catalog.insert_confirmed(second),
        Err(CatalogError::Capacity(CatalogCapacity::PhysicalBytes))
    );
    deleted(&mut catalog, first);
    assert_eq!(catalog.usage().physical_bytes, 0);
    assert_eq!(catalog.usage().liability_bytes, 100);
    assert_eq!(catalog.usage().unsettled_objects, 1);
    assert_eq!(
        catalog.insert_confirmed(second),
        Err(CatalogError::Capacity(CatalogCapacity::LiabilityBytes))
    );
    settled(&mut catalog, first);
    assert_eq!(catalog.usage().unsettled_objects, 0);
    assert_eq!(catalog.usage().objects, 1);
    catalog
        .insert_confirmed(second)
        .expect("freed byte capacity");
    assert_eq!(catalog.usage().objects, 2);
    assert_eq!(catalog.tenant_usage(p(2)).reference_slots, 1);
    assert_eq!(catalog.tenant_usage(p(2)).receipt_slots, 1);
    assert_eq!(catalog.tenant_usage(p(3)).logical_bytes, 100);
    assert_eq!(catalog.tenant_usage(p(9)), CatalogUsage::default());
}

#[test]
fn registration_identity_is_immutable_even_after_settlement() {
    let mut catalog = BlobCatalog::new(p(1), limits()).expect("catalog");
    let original = input(1, 2, 1, 100);
    catalog.insert_confirmed(original).expect("insert");
    for (candidate, error) in [
        (
            ConfirmedObject {
                bytes: 99,
                ..original
            },
            CatalogError::RegistrationConflict,
        ),
        (
            ConfirmedObject {
                first: ReferenceKey::new(original.first.object(), ReferenceId::new(n(2))),
                ..original
            },
            CatalogError::RegistrationConflict,
        ),
        (
            ConfirmedObject {
                first: input(1, 3, 1, 100).first,
                ..original
            },
            CatalogError::Root(RootClaimError::RootAlreadyClaimed),
        ),
        (
            ConfirmedObject {
                root: root(2),
                ..original
            },
            CatalogError::Root(RootClaimError::ObjectAlreadyClaimed),
        ),
        (
            ConfirmedObject {
                first: ReferenceKey::new(
                    ObjectBinding::new(p(9), p(2), original.first.object().identity())
                        .expect("foreign service"),
                    original.first.reference(),
                ),
                ..original
            },
            CatalogError::Root(RootClaimError::WrongService),
        ),
    ] {
        let before = catalog.clone();
        assert_eq!(catalog.insert_confirmed(candidate), Err(error));
        assert_eq!(catalog, before);
    }
    let before = catalog.clone();
    assert_eq!(
        catalog.confirm_provider_deleted(original.root, original.first.object()),
        Err(CatalogError::Lifecycle(
            LifecycleError::LiveReferencesRemain
        ))
    );
    assert_eq!(
        catalog.confirm_billing_stopped(original.root, original.first.object()),
        Err(CatalogError::Lifecycle(
            LifecycleError::DeletionNotConfirmed
        ))
    );
    assert_eq!(catalog, before);
    release(&mut catalog, original);
    settled(&mut catalog, original);
    let before = catalog.clone();
    assert_eq!(
        catalog.insert_confirmed(original),
        Ok(CatalogInsertOutcome::Existing)
    );
    assert_eq!(catalog, before);
    assert_eq!(
        catalog
            .get(original.root)
            .expect("retained entry")
            .lifecycle()
            .phase(),
        LifecyclePhase::Settled
    );
    assert_eq!(
        catalog.claims.resolve(original.root),
        Ok(original.first.object())
    );
}

#[test]
fn receipt_exhaustion_preserves_release_and_exact_replay_at_catalog_capacity() {
    let mut catalog = BlobCatalog::new(
        p(1),
        CatalogLimits {
            max_objects: bound(1),
            max_receipts_per_object: bound(3),
            ..limits()
        },
    )
    .expect("catalog");
    let first = input(1, 2, 1, 100);
    catalog.insert_confirmed(first).expect("insert");
    let unknown = ReferenceKey::new(first.first.object(), ReferenceId::new(n(2)));
    let failed = request(7, ReferenceOperation::Release(unknown));
    assert_eq!(
        catalog.apply_reference(first.root, p(2), failed),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Err(LifecycleError::UnknownReference)
        })
    );
    let noop = request(8, ReferenceOperation::Retain(first.first));
    catalog
        .apply_reference(first.root, p(2), noop)
        .expect("idempotent retain receipt");
    let before = catalog.clone();
    assert_eq!(
        catalog.apply_reference(
            first.root,
            p(2),
            request(9, ReferenceOperation::Retain(unknown))
        ),
        Err(CatalogError::Request(
            ReferenceRequestError::ReceiptLimitReached
        ))
    );
    assert_eq!(
        catalog.apply_reference(first.root, p(3), noop),
        Err(CatalogError::Request(
            ReferenceRequestError::RequestConflict
        ))
    );
    assert_eq!(
        catalog.apply_reference(root(9), p(2), noop),
        Err(CatalogError::UnknownRoot)
    );
    assert_eq!(catalog, before);
    release(&mut catalog, first);
    settled(&mut catalog, first);
    let before = catalog.clone();
    assert_eq!(
        catalog.apply_reference(first.root, p(2), failed),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Err(LifecycleError::UnknownReference)
        })
    );
    assert_eq!(
        catalog.apply_reference(first.root, p(2), noop),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Unchanged)
        })
    );
    assert_eq!(catalog, before);
    assert_eq!(catalog.usage().receipt_slots, 3);
    assert_eq!(catalog.usage().reference_slots, 1);
    assert_eq!(catalog.usage().active_references, 0);
}

#[test]
fn zero_byte_liabilities_and_totals_above_u64_remain_visible() {
    let wide = n(u128::MAX);
    let mut catalog = BlobCatalog::new(
        p(1),
        CatalogLimits {
            max_physical_bytes: wide,
            max_liability_bytes: wide,
            max_tenant_logical_bytes: wide,
            ..limits()
        },
    )
    .expect("catalog");
    for id in [1, 2] {
        catalog
            .insert_confirmed(input(id, 2, 1, u64::MAX))
            .expect("wide byte accounting");
    }
    let expected = 2 * u128::from(u64::MAX);
    assert_eq!(catalog.usage().physical_bytes, expected);
    assert_eq!(catalog.tenant_usage(p(2)).logical_bytes, expected);
    let zero = input(3, 3, 2, 0);
    catalog
        .insert_confirmed(zero)
        .expect("zero bytes still occupy an object slot");
    release(&mut catalog, zero);
    deleted(&mut catalog, zero);
    assert_eq!(catalog.tenant_usage(p(3)).liability_bytes, 0);
    assert_eq!(catalog.tenant_usage(p(3)).unsettled_objects, 1);
    assert_eq!(catalog.usage().unsettled_objects, 3);
    settled(&mut catalog, zero);
    assert_eq!(catalog.usage().unsettled_objects, 2);
    assert_eq!(catalog.usage().objects, 3);
}

#[test]
fn pages_advance_through_sparse_results_and_reject_cross_scope_cursors() {
    let mut catalog = BlobCatalog::new(p(1), limits()).expect("catalog");
    let inputs = [
        input(1, 2, 1, 1),
        input(2, 2, 2, 1),
        input(3, 2, 1, 1),
        input(4, 3, 1, 1),
        input(5, 2, 1, 1),
    ];
    for input in inputs {
        catalog.insert_confirmed(input).expect("insert");
    }
    for input in [inputs[1], inputs[2], inputs[3], inputs[4]] {
        release(&mut catalog, input);
    }
    deleted(&mut catalog, inputs[4]);
    let before = catalog.clone();
    let limits = PendingPageLimits {
        max_scan: bound(1),
        max_results: bound(1),
    };
    let first = catalog
        .pending_deletions(n(1), None, limits)
        .expect("first page");
    assert!(first.entries.is_empty());
    assert_eq!(first.scanned, 1);
    let cursor = first.next.expect("empty page must advance");
    assert_eq!(
        catalog.pending_deletions(n(2), Some(cursor), limits),
        Err(PendingCursorError)
    );
    let foreign = BlobCatalog::new(p(9), super::tests::limits()).expect("foreign service");
    assert_eq!(
        foreign.pending_deletions(n(1), Some(cursor), limits),
        Err(PendingCursorError)
    );
    let mut next = Some(cursor);
    let mut roots = Vec::new();
    while let Some(cursor) = next {
        let page = catalog
            .pending_deletions(n(1), Some(cursor), limits)
            .expect("continuation");
        assert!(page.scanned <= 1);
        roots.extend(page.entries.into_iter().map(|entry| entry.root));
        next = page.next;
    }
    assert_eq!(roots, vec![root(3), root(4)]);
    let results_limited = catalog
        .pending_deletions(
            n(1),
            None,
            PendingPageLimits {
                max_scan: bound(8),
                max_results: bound(1),
            },
        )
        .expect("result bound");
    assert_eq!(results_limited.scanned, 3);
    assert_eq!(results_limited.entries[0].root, root(3));
    assert!(results_limited.next.is_some());
    assert_eq!(catalog, before);
    // Pending transitions behind a cursor require another sweep, not a snapshot claim.
    release(&mut catalog, inputs[0]);
    let resumed = catalog
        .pending_deletions(
            n(1),
            Some(cursor),
            PendingPageLimits {
                max_scan: bound(8),
                max_results: bound(8),
            },
        )
        .expect("resume");
    assert_eq!(
        resumed
            .entries
            .iter()
            .map(|entry| entry.root)
            .collect::<Vec<_>>(),
        vec![root(3), root(4)]
    );
    let restarted = catalog
        .pending_deletions(
            n(1),
            None,
            PendingPageLimits {
                max_scan: bound(8),
                max_results: bound(8),
            },
        )
        .expect("new sweep");
    assert_eq!(
        restarted
            .entries
            .iter()
            .map(|entry| entry.root)
            .collect::<Vec<_>>(),
        vec![root(1), root(3), root(4)]
    );
}
