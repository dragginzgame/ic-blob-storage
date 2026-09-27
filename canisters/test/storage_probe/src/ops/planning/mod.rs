//! Passive capacity and operator reconciliation conversion under one owner borrow.
use super::{STATE, conversion::failure, read};
use blob_test_protocol::{
    admission::{
        ContentLookup, Enrollment,
        planning::{AdmissionCapacity, AdmissionCapacityInput},
        release::ReferenceCapacity,
    },
    storage::{
        Failure,
        read::{RootBatchInput, RootObservation},
    },
};
use ic_blob_storage::{
    model::{
        identity::{
            HashParseError,
            batch::{ProviderRootBatch, RootBatchLimits},
        },
        service::upload::{UploadContext, planning::AdmissionCapacityLookup},
    },
    ops::service::uploads::read::UploadRootObservation,
};
use std::num::{NonZeroU128, NonZeroUsize};

pub(crate) fn admission(
    execution: UploadContext,
    input: AdmissionCapacityInput,
) -> Result<AdmissionCapacity, Failure> {
    if execution.service != input.service {
        return Err(Failure::Binding);
    }
    let scope = AdmissionCapacityLookup {
        tenant: input.tenant,
        namespace: NonZeroU128::new(input.namespace).ok_or(Failure::Invalid)?,
    };
    STATE.with_borrow(|state| {
        let view = state
            .as_ref()
            .unwrap()
            .uploads
            .admission_capacity(execution, scope)
            .map_err(failure)?;
        Ok(AdmissionCapacity {
            scope: input,
            enrollment: Enrollment {
                generation: view.enrollment.generation.get(),
                active: view.enrollment.active,
            },
            max_object_bytes: view.max_object_bytes,
            max_headers: view.max_headers as u64,
            max_header_bytes: view.max_header_bytes as u64,
            remaining_objects: view.remaining_objects as u64,
            remaining_active_uploads: view.remaining_active_uploads as u64,
            remaining_manifest_chunks: view.remaining_manifest_chunks,
            remaining_bytes: view.remaining_bytes,
        })
    })
}
pub(crate) fn reference(
    execution: UploadContext,
    input: ContentLookup,
) -> Result<Option<ReferenceCapacity>, Failure> {
    let scope = read::lookup(execution, input)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .reference_capacity(execution, scope)
        })
        .map(|v| {
            v.map(|v| ReferenceCapacity {
                reference_slots: v.reference_slots as u64,
                unreserved_receipts: v.unreserved_receipts as u64,
                release_reserved_receipts: v.release_reserved_receipts as u64,
                fresh_retains: v.fresh_retains as u64,
            })
        })
        .map_err(failure)
}
pub(crate) fn roots(
    execution: UploadContext,
    input: &RootBatchInput,
) -> Result<Vec<RootObservation>, Failure> {
    if execution.service != input.service {
        return Err(Failure::Binding);
    }
    let namespace = NonZeroU128::new(input.namespace).ok_or(Failure::Invalid)?;
    // Fixed fixture processing bounds supplement the endpoint's bounded decoder.
    let batch = ProviderRootBatch::from_bytes(
        &input.roots,
        RootBatchLimits {
            max_entries: NonZeroUsize::new(8).unwrap(),
            max_bytes: NonZeroUsize::new(256).unwrap(),
        },
    )
    .map_err(|_| Failure::Capacity)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .observe_roots(execution, namespace, &batch)
        })
        .map(|entries| {
            entries
                .into_iter()
                .map(|entry| match entry {
                    UploadRootObservation::Known(view) => {
                        RootObservation::Known(read::observation(view))
                    }
                    UploadRootObservation::Unknown => RootObservation::Unknown,
                    UploadRootObservation::Malformed(HashParseError::InvalidByteLength {
                        actual,
                    }) => RootObservation::Malformed {
                        bytes: actual as u64,
                    },
                    // This binary parser produces only length errors.
                    UploadRootObservation::Malformed(_) => unreachable!("binary root parser"),
                })
                .collect()
        })
        .map_err(failure)
}
