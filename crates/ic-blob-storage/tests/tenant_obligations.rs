//! Current local obligation views; supplied confirmations are not provider evidence.

use std::num::{NonZeroU128, NonZeroUsize};

use candid::Principal;
use ic_blob_storage::{
    model::{
        catalog::{
            BlobCatalog, CatalogError, CatalogInsertOutcome, CatalogLimits, ConfirmedObject,
            admission::{
                UploadCatalog, UploadLimits, UploadObject, UploadRequest, UploadRequestId,
            },
            tenant::{TenantObjectCursorError, TenantObjectPageLimits},
        },
        identity::{ContentDigest, ProviderRootHash},
        lifecycle::{
            LifecycleChange, LifecyclePhase, ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
            requests::{
                ReferenceOperation, ReferenceRequest, ReferenceRequestId, ReferenceRequestOutcome,
            },
            roots::RootClaimError,
        },
    },
    policy::{
        catalog::tenant::{TenantObjectReadError, assess_tenant_unsettled_objects},
        tenant::{TenantAccessContext, TenantAccessError},
    },
};

fn p(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}
fn n(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("positive")
}
fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive")
}
fn limits() -> CatalogLimits {
    CatalogLimits {
        max_objects: bound(30),
        max_tenant_objects: bound(20),
        max_physical_bytes: n(10000),
        max_liability_bytes: n(10000),
        max_tenant_logical_bytes: n(10000),
        max_references_per_object: bound(2),
        max_receipts_per_object: bound(4),
    }
}
fn catalog() -> BlobCatalog {
    BlobCatalog::new(p(1), limits()).expect("catalog")
}
fn context(tenant: u8) -> TenantAccessContext {
    TenantAccessContext {
        service: p(1),
        actor: p(tenant),
    }
}
fn budget(scan: usize, results: usize) -> TenantObjectPageLimits {
    TenantObjectPageLimits {
        max_scan: bound(scan),
        max_results: bound(results),
    }
}
fn object(root: u8, tenant: u8, namespace: u128, bytes: u64) -> ConfirmedObject {
    ConfirmedObject {
        root: ProviderRootHash::try_from([root; 32].as_slice()).expect("root"),
        bytes,
        first: ReferenceKey::new(
            ObjectBinding::new(
                p(1),
                p(tenant),
                ObjectIdentity {
                    namespace: n(namespace),
                    object: n(u128::from(root) + 1),
                    incarnation: n(1),
                },
            )
            .expect("binding"),
            ReferenceId::new(n(1)),
        ),
    }
}
fn release(catalog: &mut BlobCatalog, object: ConfirmedObject) {
    assert_eq!(
        catalog.apply_reference(
            object.root,
            object.first.object().tenant(),
            ReferenceRequest {
                id: ReferenceRequestId::new(n(1)),
                operation: ReferenceOperation::Release(object.first),
            }
        ),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Ok(LifecycleChange::Changed)
        })
    );
}
fn deleted(catalog: &mut BlobCatalog, object: ConfirmedObject) {
    catalog
        .confirm_provider_deleted(object.root, object.first.object())
        .expect("supplied deletion fact");
}
fn settled(catalog: &mut BlobCatalog, object: ConfirmedObject) {
    catalog
        .confirm_billing_stopped(object.root, object.first.object())
        .expect("supplied billing fact");
}

#[test]
fn deleted_and_zero_byte_objects_remain_visible_until_billing_stops() {
    let mut catalog = catalog();
    let inputs = [
        object(1, 2, 1, 10),
        object(2, 2, 2, 20),
        object(3, 2, 1, 30),
        object(4, 2, 2, 0),
        object(5, 2, 1, 40),
        object(6, 3, 1, 100),
    ];
    for input in inputs {
        catalog.insert_confirmed(input).expect("insert");
    }
    for input in &inputs[1..5] {
        release(&mut catalog, *input);
    }
    for input in &inputs[2..5] {
        deleted(&mut catalog, *input);
    }
    settled(&mut catalog, inputs[4]);
    let before = catalog.clone();
    let page =
        assess_tenant_unsettled_objects(&catalog, context(2), None, budget(10, 10)).expect("page");
    assert_eq!(
        page.entries
            .iter()
            .map(|v| (
                v.root.as_bytes()[0],
                v.phase,
                v.logical_bytes,
                v.physical_bytes,
                v.liability_bytes
            ))
            .collect::<Vec<_>>(),
        vec![
            (1, LifecyclePhase::Live, 10, 10, 10),
            (2, LifecyclePhase::DeletionPending, 0, 20, 20),
            (3, LifecyclePhase::ProviderDeleted, 0, 0, 30),
            (4, LifecyclePhase::ProviderDeleted, 0, 0, 0),
        ]
    );
    for (view, input) in page.entries.iter().zip(inputs) {
        assert_eq!(view.object, input.first.object());
    }
    assert_eq!(page.scanned, 5);
    assert_eq!(page.next, None);
    let usage = catalog.tenant_usage(p(2));
    assert_eq!(usage.objects, 5);
    assert_eq!(usage.unsettled_objects, page.entries.len());
    assert_eq!(
        usage.liability_bytes,
        page.entries
            .iter()
            .map(|v| u128::from(v.liability_bytes))
            .sum::<u128>()
    );
    assert_eq!(catalog, before);
    settled(&mut catalog, inputs[3]);
    let page = assess_tenant_unsettled_objects(&catalog, context(2), None, budget(10, 10))
        .expect("after settlement");
    assert!(!page.entries.iter().any(|v| v.root == inputs[3].root));
    assert_eq!(catalog.tenant_usage(p(2)).objects, 5);
}

#[test]
fn independent_budgets_progress_through_sparse_history_without_foreign_rows() {
    let mut catalog = catalog();
    for root in 0..12 {
        let input = object(root, if root % 2 == 0 { 2 } else { 3 }, 1, 0);
        catalog.insert_confirmed(input).expect("insert");
        if root % 4 == 0 {
            release(&mut catalog, input);
            deleted(&mut catalog, input);
            settled(&mut catalog, input);
        }
    }
    let before = catalog.clone();
    for scan in 1..=7 {
        for results in 1..=7 {
            let mut cursor = None;
            let mut observed = Vec::new();
            let mut scanned = 0;
            loop {
                let page = assess_tenant_unsettled_objects(
                    &catalog,
                    context(2),
                    cursor,
                    budget(scan, results),
                )
                .expect("page");
                assert!(page.scanned <= scan);
                assert!(page.entries.len() <= results);
                assert!(page.scanned > 0);
                scanned += page.scanned;
                assert!(scanned <= 6, "cursor must make forward progress");
                observed.extend(page.entries.iter().map(|v| v.root.as_bytes()[0]));
                let Some(next) = page.next else {
                    break;
                };
                assert_ne!(Some(next), cursor);
                cursor = Some(next);
            }
            assert_eq!(observed, [2, 6, 10]);
            assert_eq!(scanned, 6);
        }
    }
    let empty = assess_tenant_unsettled_objects(&catalog, context(9), None, budget(1, 1))
        .expect("unrelated tenant");
    assert!(empty.entries.is_empty());
    assert_eq!(empty.scanned, 0);
    assert_eq!(empty.next, None);
    assert_eq!(catalog, before);
}

#[test]
fn execution_context_and_cursor_scope_are_rechecked_on_every_page() {
    let mut catalog = catalog();
    for root in [1, 2] {
        catalog
            .insert_confirmed(object(root, 2, 1, 10))
            .expect("insert");
    }
    let cursor = assess_tenant_unsettled_objects(&catalog, context(2), None, budget(1, 1))
        .expect("first")
        .next;
    assert!(cursor.is_some());
    let before = catalog.clone();
    assert_eq!(
        assess_tenant_unsettled_objects(&catalog, context(3), cursor, budget(1, 1)),
        Err(TenantObjectReadError::Cursor(TenantObjectCursorError))
    );
    for actor in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            assess_tenant_unsettled_objects(
                &catalog,
                TenantAccessContext {
                    actor,
                    ..context(2)
                },
                cursor,
                budget(1, 1)
            ),
            Err(TenantObjectReadError::Access(TenantAccessError::NotTenant))
        );
    }
    assert_eq!(
        assess_tenant_unsettled_objects(
            &catalog,
            TenantAccessContext {
                service: p(9),
                ..context(2)
            },
            cursor,
            budget(1, 1)
        ),
        Err(TenantObjectReadError::Access(
            TenantAccessError::WrongService
        ))
    );
    let other_service = BlobCatalog::new(p(9), limits()).expect("another service");
    assert_eq!(
        assess_tenant_unsettled_objects(
            &other_service,
            TenantAccessContext {
                service: p(9),
                ..context(2)
            },
            cursor,
            budget(1, 1)
        ),
        Err(TenantObjectReadError::Cursor(TenantObjectCursorError))
    );
    assert_eq!(catalog, before);
}

#[test]
fn continuation_observes_settlement_and_new_lower_roots_need_a_new_sweep() {
    let mut catalog = catalog();
    let first = object(2, 2, 1, 10);
    let last = object(255, 2, 1, 0);
    for input in [first, last] {
        catalog.insert_confirmed(input).expect("insert");
    }
    let cursor = assess_tenant_unsettled_objects(&catalog, context(2), None, budget(1, 1))
        .expect("first")
        .next;
    release(&mut catalog, last);
    deleted(&mut catalog, last);
    settled(&mut catalog, last);
    catalog
        .insert_confirmed(object(0, 2, 2, 20))
        .expect("new earlier root");
    let tail =
        assess_tenant_unsettled_objects(&catalog, context(2), cursor, budget(1, 1)).expect("tail");
    assert!(tail.entries.is_empty());
    assert_eq!((tail.scanned, tail.next), (1, None));
    let fresh = assess_tenant_unsettled_objects(&catalog, context(2), None, budget(10, 10))
        .expect("new sweep");
    assert_eq!(
        fresh
            .entries
            .iter()
            .map(|v| v.root.as_bytes()[0])
            .collect::<Vec<_>>(),
        [0, 2]
    );
    assert_eq!(fresh.scanned, 3);
}

#[test]
fn confirmed_index_tracks_exact_replays_rejections_and_upload_transfer() {
    let mut catalog = catalog();
    let first = object(1, 2, 1, 10);
    catalog.insert_confirmed(first).expect("first");
    let before = catalog.clone();
    assert_eq!(
        catalog.insert_confirmed(first),
        Ok(CatalogInsertOutcome::Existing)
    );
    assert_eq!(
        catalog.insert_confirmed(ConfirmedObject { bytes: 11, ..first }),
        Err(CatalogError::RegistrationConflict)
    );
    assert_eq!(
        catalog.insert_confirmed(object(1, 3, 1, 10)),
        Err(CatalogError::Root(RootClaimError::RootAlreadyClaimed))
    );
    assert_eq!(catalog, before);
    let own =
        assess_tenant_unsettled_objects(&catalog, context(2), None, budget(10, 10)).expect("own");
    assert_eq!(own.entries.len(), 1);
    assert_eq!(own.scanned, 1);
    assert_eq!(catalog.tenant_usage(p(3)).objects, 0);

    let mut uploads = UploadCatalog::new(
        p(1),
        limits(),
        UploadLimits {
            max_active: bound(2),
            max_tenant_active: bound(2),
        },
    )
    .expect("uploads");
    let request = UploadRequest {
        id: UploadRequestId::new(n(1)),
        object: UploadObject {
            root: first.root,
            bytes: first.bytes,
            first: first.first,
        },
        content: ContentDigest::compute(b"fixture"),
    };
    uploads.reserve(p(2), request).expect("reserve");
    uploads
        .mark_exposure_possible(p(2), request)
        .expect("exposure");
    let pending =
        assess_tenant_unsettled_objects(uploads.confirmed(), context(2), None, budget(1, 1))
            .expect("confirmed only");
    assert_eq!(pending.scanned, 0);
    assert!(pending.entries.is_empty());
    assert_eq!(uploads.tenant_usage(p(2)).reserved_bytes, 10);
    uploads.confirm_upload(request).expect("trusted completion");
    assert_eq!(
        uploads.confirm_upload(request),
        Ok(LifecycleChange::Unchanged)
    );
    let confirmed =
        assess_tenant_unsettled_objects(uploads.confirmed(), context(2), None, budget(1, 1))
            .expect("transferred");
    assert_eq!(confirmed.entries, own.entries);
    assert_eq!(confirmed.scanned, 1);
    assert_eq!(uploads.tenant_usage(p(2)).reserved_bytes, 0);
    assert_eq!(uploads.tenant_usage(p(2)).liability_bytes, 10);
}
