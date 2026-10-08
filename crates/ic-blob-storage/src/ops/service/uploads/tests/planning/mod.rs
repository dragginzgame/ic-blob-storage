use super::*;
use crate::model::service::upload::content::ContentLookup;
use crate::model::service::upload::planning::AdmissionCapacityLookup;
use crate::ops::service::uploads::read::UploadRootObservation;
use ic_blob_storage_contracts::identity::HashParseError;
use ic_blob_storage_contracts::identity::batch::ProviderRootBatch;
use ic_blob_storage_contracts::identity::batch::RootBatchLimits;
use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_blob_storage_contracts::upload::history::UploadRootState;
use std::num::NonZeroUsize;

fn scope() -> AdmissionCapacityLookup {
    AdmissionCapacityLookup {
        tenant: p(4),
        namespace: NonZeroU128::MIN,
    }
}
fn content(input: UploadPermission) -> ContentLookup {
    ContentLookup {
        tenant: p(4),
        namespace: NonZeroU128::MIN,
        root: input.request.object.root,
    }
}
pub(super) fn parity(
    store: &StableUploads<VectorMemory>,
    heap: &UploadAdmissions,
    input: UploadPermission,
) {
    assert_eq!(
        store.admission_capacity(context(4), scope()).unwrap(),
        heap.admission_capacity(context(4), scope()).unwrap()
    );
    assert_eq!(
        store
            .reference_capacity(context(4), content(input))
            .unwrap(),
        heap.reference_capacity(context(4), content(input)).unwrap()
    );
}

#[test]
fn capacity_scope_suspension_and_fenced_inspection_do_not_reserve_or_refund_history() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    assert_eq!(
        store.admission_capacity(context(4), scope()),
        Err(UploadAdmissionError::Tenant(TenantError::NotEnrolled).into())
    );
    let active = enroll(&mut store);
    let initial = store.admission_capacity(context(4), scope()).unwrap();
    assert_eq!(initial.remaining_objects, 2);
    assert_eq!(initial.remaining_bytes, 20);
    let input = permission(1);
    store.admit(context(4), input, 1).unwrap();
    assert_eq!(
        store.reference_capacity(context(4), content(input)),
        Ok(None)
    );
    assert_eq!(
        store
            .admission_capacity(context(4), scope())
            .unwrap()
            .remaining_bytes,
        10
    );
    store.revoke(context(4), input.request).unwrap();
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(active),
                active: false,
            },
        )
        .unwrap();
    let view = store.admission_capacity(context(4), scope()).unwrap();
    assert!(!view.enrollment.active);
    assert_eq!(view.remaining_objects, initial.remaining_objects - 1);
    assert_eq!(
        view.remaining_manifest_chunks,
        initial.remaining_manifest_chunks - 1
    );
    assert_eq!(
        view.remaining_active_uploads,
        initial.remaining_active_uploads
    );
    assert_eq!(view.remaining_bytes, initial.remaining_bytes);
    for actor in [2, 5, 6] {
        assert_eq!(
            store.admission_capacity(context(actor), scope()),
            Err(UploadAdmissionError::NotProject.into())
        );
        assert_eq!(
            store.reference_capacity(context(actor), content(input)),
            Err(UploadAdmissionError::NotProject.into())
        );
    }
    assert_eq!(
        store.admission_capacity(
            UploadContext {
                service: p(9),
                ..context(4)
            },
            scope()
        ),
        Err(UploadAdmissionError::WrongService.into())
    );
    assert_eq!(
        store.admission_capacity(
            context(4),
            AdmissionCapacityLookup {
                namespace: NonZeroU128::new(2).unwrap(),
                ..scope()
            }
        ),
        Err(UploadAdmissionError::WrongNamespace.into())
    );
    assert_eq!(
        store.reference_capacity(
            context(6),
            ContentLookup {
                tenant: p(6),
                ..content(input)
            }
        ),
        Ok(None)
    );
    let before = store.usage().unwrap();
    drop(store);
    let mut restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(restored.admission_capacity(context(4), scope()), Ok(view));
    assert_eq!(restored.usage(), Ok(before));
    assert_eq!(
        restored.admit(context(4), permission(2), 1),
        Err(UploadStoreError::Fenced)
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered root observation and fenced restore journey"
)]
fn root_observations_preserve_order_uncertainty_cleanup_and_fenced_history() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let input = lifecycle::exposed(&mut store);
    store.admit(context(4), permission(2), 1).unwrap();
    store.revoke(context(4), input.request).unwrap();
    let batch = ProviderRootBatch::from_bytes(
        &[
            input.request.object.root.as_bytes().to_vec(),
            vec![],
            vec![9; 32],
            vec![2; 32],
            input.request.object.root.as_bytes().to_vec(),
        ],
        RootBatchLimits {
            max_entries: NonZeroUsize::new(5).unwrap(),
            max_bytes: NonZeroUsize::new(128).unwrap(),
        },
    )
    .unwrap();
    let observe = |store: &StableUploads<VectorMemory>| {
        store
            .observe_roots(context(2), NonZeroU128::MIN, &batch)
            .unwrap()
    };
    let first = observe(&store);
    assert_eq!(first[0], first[4]);
    assert_eq!(
        first[1],
        UploadRootObservation::Malformed(HashParseError::InvalidByteLength { actual: 0 })
    );
    assert_eq!(first[2], UploadRootObservation::Unknown);
    assert!(
        matches!(first[0], UploadRootObservation::Known(v) if v.request == input.request && v.state == UploadRootState::ExposurePossible)
    );
    assert!(
        matches!(first[3], UploadRootObservation::Known(v) if v.state == UploadRootState::Reserved)
    );
    store.revoke(context(4), permission(2).request).unwrap();
    store.confirm_upload(input.request).unwrap();
    for phase in [
        LifecyclePhase::Live,
        LifecyclePhase::DeletionPending,
        LifecyclePhase::ProviderDeleted,
        LifecyclePhase::Settled,
    ] {
        let views = observe(&store);
        assert!(
            matches!(views[0], UploadRootObservation::Known(v) if v.state == UploadRootState::Confirmed(phase))
        );
        assert!(
            matches!(views[3], UploadRootObservation::Known(v) if v.state == UploadRootState::Cancelled)
        );
        match phase {
            LifecyclePhase::Live => {
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
            }
            LifecyclePhase::DeletionPending => {
                store.confirm_provider_deleted(input.request).unwrap();
            }
            LifecyclePhase::ProviderDeleted => {
                store.confirm_billing_stopped(input.request).unwrap();
            }
            LifecyclePhase::Settled => {}
        }
    }
    let before = observe(&store);
    drop(store);
    let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(observe(&restored), before);
    let empty = ProviderRootBatch::from_bytes(
        &[],
        RootBatchLimits {
            max_entries: NonZeroUsize::MIN,
            max_bytes: NonZeroUsize::MIN,
        },
    )
    .unwrap();
    for batch in [&batch, &empty] {
        assert_eq!(
            restored.observe_roots(context(4), NonZeroU128::MIN, batch),
            Err(UploadAdmissionError::NotOperator.into())
        );
        assert_eq!(
            restored.observe_roots(
                UploadContext {
                    service: p(9),
                    ..context(2)
                },
                NonZeroU128::MIN,
                batch
            ),
            Err(UploadAdmissionError::WrongService.into())
        );
        assert_eq!(
            restored.observe_roots(context(2), NonZeroU128::new(2).unwrap(), batch),
            Err(UploadAdmissionError::WrongNamespace.into())
        );
    }
}
