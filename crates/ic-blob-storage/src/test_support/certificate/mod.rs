//! Test-only access to the same stored-state certificate gate used by installation.
use crate::ops::service::uploads::{StableUploads, certificate};
use ic_blob_storage_contracts::{
    dto::upload::{
        certificate::{CaffeineUploadCertificateResponse, UploadCertificateAssessmentResponse},
        exposure::UploadExposureFailure,
    },
    upload::binding::UploadContext,
};
use ic_memory::Memory;
/// Inspect actual stored eligibility without substituting host facts.
/// # Errors
/// Preserves authority, phase and restore refusals.
pub fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<UploadCertificateAssessmentResponse, UploadExposureFailure> {
    certificate::inspect(store, context, root, now)
}
/// Issue through the production gate; write traps propagate for rollback fixtures.
/// # Errors
/// Preserves authority, phase and restore refusals.
pub fn issue<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<CaffeineUploadCertificateResponse, UploadExposureFailure> {
    certificate::issue(store, context, root, now)
}
