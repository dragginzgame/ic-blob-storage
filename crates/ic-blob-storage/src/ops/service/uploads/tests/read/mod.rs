use super::*;
use crate::{
    model::{
        catalog::admission::read::{UploadPageLimits, UploadRootState},
        lifecycle::{
            LifecyclePhase,
            requests::{ReferenceOperation, ReferenceRequest, ReferenceRequestId},
        },
        service::upload::content::ContentLookup,
    },
    ops::service::uploads::read::{UploadScanFilter, UploadScanScope},
};
use std::num::NonZeroUsize;
fn scope() -> UploadScanScope {
    UploadScanScope::Tenant {
        tenant: p(4),
        namespace: NonZeroU128::MIN,
    }
}
fn limits() -> UploadPageLimits {
    UploadPageLimits {
        max_scan: NonZeroUsize::MIN,
        max_results: NonZeroUsize::MIN,
    }
}
fn content(input: UploadPermission) -> ContentLookup {
    ContentLookup {
        tenant: p(4),
        namespace: NonZeroU128::MIN,
        root: input.request.object.root,
    }
}
fn prepared(store: &mut StableUploads<VectorMemory>, id: u128) -> UploadPermission {
    enroll(store);
    let built = built();
    let mut input = permission(1);
    input.request.id = UploadRequestId::new(NonZeroU128::new(id).unwrap());
    input.request.object.root = built.hashes().provider_root;
    store.admit(context(4), input, 1).unwrap();
    store
        .prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: built.manifest().chunks(),
                headers: &HEADERS,
            },
            2,
        )
        .unwrap();
    input
}
#[test]
fn indexed_lookup_preserves_independent_request_identity_and_tenant_isolation() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let input = prepared(&mut store, u128::MAX);
    let found = store
        .lookup_content(context(4), content(input))
        .unwrap()
        .unwrap();
    assert_eq!(found.request, input.request);
    assert_ne!(
        found.request.id.get(),
        found.request.object.first.object().identity().object
    );
    assert_eq!(
        store.lookup_content(context(5), content(input)),
        Err(UploadAdmissionError::NotProject.into())
    );
    assert_eq!(
        store.lookup_content(
            context(6),
            ContentLookup {
                tenant: p(6),
                ..content(input)
            }
        ),
        Ok(None)
    );
    assert_eq!(
        store.lookup_content(context(4), content(permission(2))),
        Ok(None)
    );
    let before = store.usage().unwrap();
    drop(store);
    let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(
        restored.lookup_content(context(4), content(input)),
        Ok(Some(found))
    );
    assert_eq!(restored.usage(), Ok(before));
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered exact-reference, original-header and fenced-restore journey"
)]
fn descriptors_require_the_exact_live_reference_and_preserve_first_headers() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let input = prepared(&mut store, 1);
    let first = input.request.object.first;
    assert_eq!(
        store.retained_content_descriptor(context(4), input.request.object.root, first),
        Ok(None)
    );
    let built = built();
    let reversed = [HEADERS[1], HEADERS[0]];
    store
        .prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: built.manifest().chunks(),
                headers: &reversed,
            },
            2,
        )
        .unwrap();
    store.expose(context(5), input.request, 3).unwrap();
    store.confirm_upload(input.request).unwrap();
    let second = ReferenceKey::new(
        first.object(),
        ReferenceId::new(NonZeroU128::new(2).unwrap()),
    );
    for (id, operation) in [
        (1, ReferenceOperation::Retain(second)),
        (2, ReferenceOperation::Release(first)),
    ] {
        store
            .apply_reference(
                context(4),
                input.request,
                ReferenceRequest {
                    id: ReferenceRequestId::new(NonZeroU128::new(id).unwrap()),
                    operation,
                },
            )
            .unwrap();
    }
    assert_eq!(
        store.retained_content_descriptor(context(4), input.request.object.root, first),
        Ok(None)
    );
    let descriptor = store
        .retained_content_descriptor(context(4), input.request.object.root, second)
        .unwrap()
        .unwrap();
    assert_eq!(
        descriptor
            .descriptor
            .headers
            .iter()
            .map(|h| (h.name.as_str(), h.value.as_str()))
            .collect::<Vec<_>>(),
        HEADERS
            .iter()
            .map(|h| (h.name, h.value))
            .collect::<Vec<_>>()
    );
    let different = ReferenceKey::new(
        ObjectBinding::new(
            p(1),
            p(4),
            ObjectIdentity {
                incarnation: NonZeroU128::new(2).unwrap(),
                ..first.object().identity()
            },
        )
        .unwrap(),
        second.reference(),
    );
    assert_eq!(
        store.retained_content_descriptor(context(4), input.request.object.root, different),
        Ok(None)
    );
    let active = store.tenant(context(4), p(4)).unwrap();
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: active,
                active: false,
            },
        )
        .unwrap();
    let totals = store.usage().unwrap();
    drop(store);
    let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(
        restored.retained_content_descriptor(context(4), input.request.object.root, second),
        Ok(Some(descriptor))
    );
    assert_eq!(
        restored.retained_content_descriptor(context(4), input.request.object.root, first),
        Ok(None)
    );
    assert_eq!(restored.usage(), Ok(totals));
}
#[test]
fn filtered_pages_advance_and_new_sweeps_find_phase_changes_behind_the_cursor() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    let input = prepared(&mut store, 1);
    store.expose(context(5), input.request, 3).unwrap();
    store.confirm_upload(input.request).unwrap();
    let mut last = permission(2);
    last.request.id = UploadRequestId::new(NonZeroU128::MAX);
    store.admit(context(4), last, 1).unwrap();
    store.revoke(context(4), last.request).unwrap();
    let filter = UploadScanFilter::DeletionPending;
    let first = store
        .scan(context(4), scope(), filter, None, limits())
        .unwrap();
    assert_eq!(first.entries, []);
    assert_eq!(first.scanned, 1);
    let cursor = first.next.unwrap();
    let before = store.usage().unwrap();
    assert_eq!(
        store.scan(
            context(4),
            scope(),
            UploadScanFilter::All,
            Some(cursor),
            limits()
        ),
        Err(UploadStoreError::CursorScope)
    );
    assert_eq!(
        store.scan(
            context(4),
            scope(),
            filter,
            Some(crate::ops::service::uploads::read::UploadScanCursor {
                after_tenant: p(6),
                ..cursor
            }),
            limits()
        ),
        Err(UploadStoreError::CursorScope)
    );
    assert_eq!(store.usage(), Ok(before));
    store
        .apply_reference(
            context(4),
            input.request,
            ReferenceRequest {
                id: ReferenceRequestId::new(NonZeroU128::MIN),
                operation: ReferenceOperation::Release(input.request.object.first),
            },
        )
        .unwrap();
    let end = store
        .scan(context(4), scope(), filter, Some(cursor), limits())
        .unwrap();
    assert_eq!(end.entries, []);
    assert_eq!(end.scanned, 1);
    assert!(end.next.is_none());
    let restarted = store
        .scan(context(4), scope(), filter, None, limits())
        .unwrap();
    assert_eq!(restarted.entries[0].request, input.request);
    assert_eq!(
        restarted.entries[0].state,
        UploadRootState::Confirmed(LifecyclePhase::DeletionPending)
    );
    let service = UploadScanScope::Service {
        namespace: NonZeroU128::MIN,
    };
    assert_eq!(
        store.scan(context(4), service, filter, None, limits()),
        Err(UploadAdmissionError::NotOperator.into())
    );
    assert_eq!(
        store
            .scan(context(2), service, filter, None, limits())
            .unwrap()
            .entries,
        restarted.entries
    );
}
#[test]
fn tenant_ranges_do_not_consume_other_tenants_scan_budget() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    enroll(&mut store);
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(6),
                expected: None,
                active: true,
            },
        )
        .unwrap();
    store.admit(context(4), permission(1), 1).unwrap();
    let mut foreign = permission(2);
    foreign.request.object.first = ReferenceKey::new(
        ObjectBinding::new(p(1), p(6), foreign.request.object.first.object().identity()).unwrap(),
        foreign.request.object.first.reference(),
    );
    store.admit(context(6), foreign, 1).unwrap();
    let capacity = store
        .admission_capacity(
            context(4),
            crate::model::service::upload::planning::AdmissionCapacityLookup {
                tenant: p(4),
                namespace: NonZeroU128::MIN,
            },
        )
        .unwrap();
    // The other tenant consumes shared capacity without exposing its identity.
    assert_eq!(capacity.remaining_objects, 0);
    assert_eq!(capacity.remaining_active_uploads, 0);
    assert_eq!(capacity.remaining_bytes, 0);
    let page = store
        .scan(context(4), scope(), UploadScanFilter::All, None, limits())
        .unwrap();
    assert_eq!(page.entries.len(), 1);
    assert_eq!(page.scanned, 1);
    assert!(page.next.is_none());
    assert_eq!(page.entries[0].request, permission(1).request);
}
#[test]
fn missing_changed_or_orphaned_root_request_index_rejects_reopen_without_repair() {
    for damage in 0..3 {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        let input = prepared(&mut store, u128::MAX);
        match damage {
            0 => {
                store
                    .root_requests
                    .remove(input.request.object.root.as_bytes());
            }
            1 => {
                store
                    .root_requests
                    .insert(*input.request.object.root.as_bytes(), 1);
            }
            _ => {
                store.root_requests.insert([7; 32], 4);
            }
        }
        drop(store);
        let before = m.root_requests.borrow().clone();
        assert!(matches!(
            StableUploads::open(clone_memory(&m), config()),
            Err(UploadStoreError::InvalidRecord)
        ));
        assert_eq!(*m.root_requests.borrow(), before);
    }
}

#[test]
fn scan_and_result_limits_are_independent_and_terminal_history_stays_bounded() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    enroll(&mut store);
    let first = permission(1);
    let mut last = permission(2);
    last.request.id = UploadRequestId::new(NonZeroU128::MAX);
    for input in [first, last] {
        store.admit(context(4), input, 1).unwrap();
    }
    let limits = UploadPageLimits {
        max_scan: NonZeroUsize::new(2).unwrap(),
        max_results: NonZeroUsize::MIN,
    };
    let page = store
        .scan(context(4), scope(), UploadScanFilter::All, None, limits)
        .unwrap();
    assert_eq!(page.scanned, 1);
    assert_eq!(page.entries.len(), 1);
    let tail = store
        .scan(
            context(4),
            scope(),
            UploadScanFilter::All,
            page.next,
            limits,
        )
        .unwrap();
    assert_eq!(tail.entries[0].request, last.request);
    assert!(tail.next.is_none());
    store.revoke(context(4), first.request).unwrap();
    let active = store
        .scan(context(4), scope(), UploadScanFilter::Active, None, limits)
        .unwrap();
    assert_eq!(active.scanned, 2);
    assert_eq!(active.entries.len(), 1);
    assert_eq!(active.entries[0].request, last.request);
    store.revoke(context(4), last.request).unwrap();
    let terminal = store
        .scan(
            context(4),
            scope(),
            UploadScanFilter::Outstanding,
            None,
            limits,
        )
        .unwrap();
    assert_eq!(terminal.scanned, 2);
    assert_eq!(terminal.entries, []);
    assert!(terminal.next.is_none());
}
#[test]
fn cleanup_views_keep_billing_obligations_after_physical_deletion() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    let input = prepared(&mut store, 1);
    store.expose(context(5), input.request, 3).unwrap();
    store.confirm_upload(input.request).unwrap();
    store
        .apply_reference(
            context(4),
            input.request,
            ReferenceRequest {
                id: ReferenceRequestId::new(NonZeroU128::MIN),
                operation: ReferenceOperation::Release(input.request.object.first),
            },
        )
        .unwrap();
    store.confirm_provider_deleted(input.request).unwrap();
    let outstanding = store
        .scan(
            context(4),
            scope(),
            UploadScanFilter::Outstanding,
            None,
            limits(),
        )
        .unwrap();
    assert_eq!(
        outstanding.entries[0].state,
        UploadRootState::Confirmed(LifecyclePhase::ProviderDeleted)
    );
    assert_eq!(
        store
            .scan(
                context(4),
                scope(),
                UploadScanFilter::DeletionPending,
                None,
                limits()
            )
            .unwrap()
            .entries,
        []
    );
    store.confirm_billing_stopped(input.request).unwrap();
    assert_eq!(
        store
            .scan(
                context(4),
                scope(),
                UploadScanFilter::Outstanding,
                None,
                limits()
            )
            .unwrap()
            .entries,
        []
    );
    assert_eq!(
        store
            .content_descriptor(context(4), content(input))
            .unwrap()
            .unwrap()
            .content
            .state,
        UploadRootState::Confirmed(LifecyclePhase::Settled)
    );
    assert_eq!(
        store.retained_content_descriptor(
            context(4),
            input.request.object.root,
            input.request.object.first
        ),
        Ok(None)
    );
}
