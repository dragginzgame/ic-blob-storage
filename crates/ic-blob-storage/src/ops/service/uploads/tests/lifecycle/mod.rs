use super::*;
use crate::model::lifecycle::{
    LifecycleError, LifecyclePhase, ReferenceState,
    requests::{
        ReferenceOperation, ReferenceRequest, ReferenceRequestError, ReferenceRequestId,
        ReferenceRequestOutcome,
    },
};
fn command(upload: UploadRequest, id: u128, reference: u128, retain: bool) -> ReferenceRequest {
    let key = ReferenceKey::new(
        upload.object.first.object(),
        ReferenceId::new(NonZeroU128::new(reference).unwrap()),
    );
    ReferenceRequest {
        id: ReferenceRequestId::new(NonZeroU128::new(id).unwrap()),
        operation: if retain {
            ReferenceOperation::Retain(key)
        } else {
            ReferenceOperation::Release(key)
        },
    }
}
pub(super) fn exposed(store: &mut StableUploads<VectorMemory>) -> UploadPermission {
    enroll(store);
    let built = built();
    let mut input = permission(1);
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
    store.expose(context(5), input.request, 3).unwrap();
    input
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered heap/stable comparison through settlement and fenced restoration"
)]
fn confirmed_reference_and_settlement_journey_matches_heap_accounting() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let input = exposed(&mut store);
    let mut heap = UploadAdmissions::new(config());
    heap.update_tenant(
        context(2),
        TenantUpdate {
            tenant: p(4),
            expected: None,
            active: true,
        },
    )
    .unwrap();
    heap.admit(context(4), input, 1).unwrap();
    let built = built();
    heap.prepare_manifest(
        context(5),
        input.request,
        UploadManifest {
            chunks: built.manifest().chunks(),
            headers: &HEADERS,
        },
        2,
    )
    .unwrap();
    heap.expose(context(5), input.request, 3).unwrap();
    // Withdrawal after exposure cannot discard verified provider completion.
    store.revoke(context(4), input.request).unwrap();
    heap.revoke(context(4), input.request).unwrap();
    assert_eq!(
        store.confirm_upload(input.request).unwrap(),
        heap.confirm_upload(input.request).unwrap()
    );
    assert_eq!(store.usage().unwrap(), heap.catalog().usage());
    planning::parity(&store, &heap, input);
    let retained = command(input.request, u128::MAX, 2, true);
    for request in [
        retained,
        retained,
        command(input.request, 2, 1, false),
        command(input.request, 3, 2, false),
    ] {
        assert_eq!(
            store
                .apply_reference(context(4), input.request, request)
                .unwrap(),
            heap.apply_reference(context(4), input.request.object.root, request)
                .unwrap()
        );
        assert_eq!(store.usage().unwrap(), heap.catalog().usage());
        planning::parity(&store, &heap, input);
    }
    assert_eq!(store.usage().unwrap().logical_bytes, 0);
    assert_eq!(store.usage().unwrap().physical_bytes, 10);
    assert_eq!(
        store.confirmed(context(4), input.request).unwrap().phase,
        LifecyclePhase::DeletionPending
    );
    store.confirm_provider_deleted(input.request).unwrap();
    heap.confirm_provider_deleted(
        input.request.object.root,
        input.request.object.first.object(),
    )
    .unwrap();
    assert_eq!(store.usage().unwrap(), heap.catalog().usage());
    planning::parity(&store, &heap, input);
    assert_eq!(store.usage().unwrap().liability_bytes, 10);
    store.confirm_billing_stopped(input.request).unwrap();
    heap.confirm_billing_stopped(
        input.request.object.root,
        input.request.object.first.object(),
    )
    .unwrap();
    assert_eq!(store.usage().unwrap(), heap.catalog().usage());
    planning::parity(&store, &heap, input);
    let before = store.usage().unwrap();
    assert_eq!(
        store.confirm_upload(input.request),
        Ok(LifecycleChange::Unchanged)
    );
    assert_eq!(store.usage(), Ok(before));
    assert_eq!(
        store.apply_reference(context(4), input.request, retained),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        })
    );
    assert!(
        !store
            .reference_is_live(context(4), input.request, retained.operation.key())
            .unwrap()
    );
    drop(store);
    let mut restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(restored.usage(), Ok(before));
    assert_eq!(
        restored
            .reference_receipt(context(4), input.request, retained)
            .unwrap()
            .unwrap()
            .result,
        Ok(LifecycleChange::Changed)
    );
    assert_eq!(
        restored.confirmed(context(4), input.request).unwrap().phase,
        LifecyclePhase::Settled
    );
    assert_eq!(
        restored.apply_reference(context(4), input.request, retained),
        Err(UploadStoreError::Fenced)
    );
    assert_eq!(
        restored.confirm_upload(input.request),
        Err(UploadStoreError::Fenced)
    );
    assert_eq!(
        restored.confirm_provider_deleted(input.request),
        Err(UploadStoreError::Fenced)
    );
    assert_eq!(
        restored.confirm_billing_stopped(input.request),
        Err(UploadStoreError::Fenced)
    );
}
#[test]
fn receipt_headroom_preserves_cleanup_and_suspension_allows_replay_and_release() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    let input = exposed(&mut store);
    store.confirm_upload(input.request).unwrap();
    let retained = command(input.request, 1, 2, true);
    store
        .apply_reference(context(4), input.request, retained)
        .unwrap();
    let before = store.confirmed(context(4), input.request).unwrap();
    assert_eq!(
        store.apply_reference(
            context(4),
            input.request,
            command(input.request, 2, 9, false)
        ),
        Err(UploadStoreError::Reference(
            ReferenceRequestError::ReceiptLimitReached
        ))
    );
    assert_eq!(store.confirmed(context(4), input.request), Ok(before));
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
    assert!(matches!(
        store.apply_reference(context(4), input.request, retained),
        Ok(ReferenceRequestOutcome::Replayed { .. })
    ));
    assert_eq!(
        store.apply_reference(
            context(4),
            input.request,
            command(input.request, 4, 3, true)
        ),
        Err(UploadStoreError::Admission(UploadAdmissionError::Tenant(
            TenantError::Suspended
        )))
    );
    for (id, reference) in [(2, 1), (3, 2)] {
        store
            .apply_reference(
                context(4),
                input.request,
                command(input.request, id, reference, false),
            )
            .unwrap();
    }
    assert_eq!(
        store
            .confirmed(context(4), input.request)
            .unwrap()
            .receipt_slots,
        3
    );
    assert_eq!(store.usage().unwrap().physical_bytes, 10);
    assert_eq!(
        store.reference_receipt(context(5), input.request, retained),
        Err(UploadStoreError::Admission(
            UploadAdmissionError::NotProject
        ))
    );
    assert_eq!(
        store.reference_receipt(
            context(4),
            input.request,
            command(input.request, 1, 1, true)
        ),
        Err(UploadStoreError::Reference(
            ReferenceRequestError::RequestConflict
        ))
    );
}
#[test]
fn recorded_failure_survives_later_success_without_reinterpretation() {
    use std::num::NonZeroUsize;
    let base = config();
    let mut limits = base.limits();
    limits.catalog.max_receipts_per_object = NonZeroUsize::new(4).unwrap();
    let config = ServiceConfiguration::new(base.bindings(), limits, base.billing()).unwrap();
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config).unwrap();
    let input = exposed(&mut store);
    store.confirm_upload(input.request).unwrap();
    let failure = command(input.request, 1, 2, false);
    assert_eq!(
        store.apply_reference(context(4), input.request, failure),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Err(LifecycleError::UnknownReference)
        })
    );
    let retained = command(input.request, 2, 2, true);
    store
        .apply_reference(context(4), input.request, retained)
        .unwrap();
    assert_eq!(
        store.apply_reference(context(4), input.request, failure),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Err(LifecycleError::UnknownReference)
        })
    );
    assert!(
        store
            .reference_is_live(context(4), input.request, retained.operation.key())
            .unwrap()
    );
    drop(store);
    let restored = StableUploads::open(clone_memory(&m), config).unwrap();
    assert_eq!(
        restored
            .reference_receipt(context(4), input.request, failure)
            .unwrap()
            .unwrap()
            .result,
        Err(LifecycleError::UnknownReference)
    );
}
#[test]
fn every_confirmed_phase_reopens_with_separate_byte_obligations() {
    for stage in 0..4 {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        let input = exposed(&mut store);
        store.confirm_upload(input.request).unwrap();
        if stage > 0 {
            store
                .apply_reference(
                    context(4),
                    input.request,
                    command(input.request, 1, 1, false),
                )
                .unwrap();
        }
        if stage > 1 {
            store.confirm_provider_deleted(input.request).unwrap();
        }
        if stage > 2 {
            store.confirm_billing_stopped(input.request).unwrap();
        }
        let before = store.usage().unwrap();
        let phase = store.confirmed(context(4), input.request).unwrap();
        drop(store);
        let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
        assert_eq!(restored.usage(), Ok(before));
        assert_eq!(restored.confirmed(context(4), input.request), Ok(phase));
    }
}
#[test]
fn missing_or_orphaned_reference_receipt_and_lifecycle_rows_reject_restore() {
    for damage in 0..5 {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        let input = exposed(&mut store);
        store.confirm_upload(input.request).unwrap();
        store
            .apply_reference(
                context(4),
                input.request,
                command(input.request, 1, 2, true),
            )
            .unwrap();
        match damage {
            0 => {
                store.references.remove(&(key(input.request), 1));
            }
            1 => {
                store.receipts.remove(&(key(input.request), 1));
            }
            2 => {
                store.confirmed.remove(&key(input.request));
            }
            3 => {
                store.references.insert(
                    ((p(4), 99), 1),
                    ReferenceRecord::new(ReferenceState::Active),
                );
            }
            _ => {
                store.usage.insert(p(4), UploadUsageRecord::empty());
            }
        }
        drop(store);
        let before = m.references.borrow().clone();
        assert!(matches!(
            StableUploads::open(clone_memory(&m), config()),
            Err(UploadStoreError::InvalidRecord)
        ));
        assert_eq!(*m.references.borrow(), before);
    }
}
#[test]
fn premature_or_changed_confirmation_never_releases_capacity() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    enroll(&mut store);
    let input = permission(1);
    store.admit(context(4), input, 1).unwrap();
    let before = store.usage().unwrap();
    assert_eq!(
        store.confirm_upload(input.request),
        Err(UploadError::InvalidPhase(UploadPhase::Reserved).into())
    );
    let mut changed = input.request;
    changed.object.bytes = 9;
    assert_eq!(
        store.confirm_upload(changed),
        Err(UploadAdmissionError::PermissionConflict.into())
    );
    assert_eq!(store.usage(), Ok(before));
    store.revoke(context(4), input.request).unwrap();
    assert_eq!(
        store.confirm_upload(input.request),
        Err(UploadError::InvalidPhase(UploadPhase::Cancelled).into())
    );
}
