//! Convert the shared owner's passive capacity observation under one state borrow.

use super::{STATE, conversion::failure};
use blob_test_protocol::admission::{
    Enrollment, Failure,
    planning::{AdmissionCapacity, AdmissionCapacityInput},
};
use ic_blob_storage::model::service::upload::{UploadContext, planning::AdmissionCapacityLookup};
use std::num::NonZeroU128;

pub(crate) fn capacity(
    context: UploadContext,
    input: AdmissionCapacityInput,
) -> Result<AdmissionCapacity, Failure> {
    if context.service != input.service {
        return Err(Failure::WrongService);
    }
    let scope = AdmissionCapacityLookup {
        tenant: input.tenant,
        namespace: NonZeroU128::new(input.namespace).ok_or(Failure::InvalidInput)?,
    };
    STATE.with_borrow(|state| {
        let view = state
            .as_ref()
            .expect("initialized probe")
            .owner
            .admission_capacity(context, scope)
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
