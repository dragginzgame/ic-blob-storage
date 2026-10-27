//! Direct exposure access for isolated interruption, resource and rollback fixtures.
use crate::ops::service::uploads::{StableUploads, UploadStoreError};
use crate::policy::upload::exposure::{UploadExposureAssessment, UploadExposureHostEvidence};
pub use crate::workflow::uploads::exposure::boundary::UploadExposureResult;
pub use crate::workflow::uploads::exposure::inspect;
use ic_blob_storage_contracts::dto::upload::{
    admission::UploadAdmissionRequest, exposure::UploadExposureFailure,
};
use ic_blob_storage_contracts::upload::binding::{UploadContext, UploadRequest};
use ic_memory::Memory;

/// Inspect fixture boundary data with the production eligibility checks.
/// # Errors
/// Preserves permission, evidence and restoration refusals.
pub fn inspect_preparation<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<UploadExposureAssessment, UploadExposureFailure> {
    let permission =
        crate::ops::service::uploads::exposure::permission(store, context, input, true)?.permission;
    crate::workflow::uploads::exposure::boundary::inspect_preparation(
        store, context, permission, evidence, now,
    )
}

/// Submit fixture data through the synchronous production exposure transition.
/// # Errors
/// Preserves permission, evidence and restoration refusals.
pub fn commit<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<UploadExposureResult, UploadExposureFailure> {
    let permission =
        crate::ops::service::uploads::exposure::permission(store, context, input, true)?.permission;
    crate::workflow::uploads::exposure::boundary::commit(store, context, permission, evidence, now)
}

/// Record exposure bookkeeping without minting a certificate, for resource fixtures.
/// No provider or host-evidence claim is established by this test-only access.
/// # Errors
/// Preserves the owner's authority, phase, manifest, clock and recovery refusals.
/// # Panics
/// Stable-write traps must propagate for IC rollback.
pub fn record<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    request: UploadRequest,
    now: u64,
) -> Result<(), UploadStoreError> {
    store.expose(context, request, now)
}
