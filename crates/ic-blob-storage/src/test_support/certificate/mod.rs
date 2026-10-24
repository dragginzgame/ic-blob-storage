//! Evidence substitution for certificate rejection and IC rollback fixtures.
use crate::ops::service::uploads::{StableUploads, certificate::resolve};
use crate::policy::upload::exposure::UploadExposureHostEvidence;
use crate::workflow::uploads::certificate::{UploadCertificateFailure, boundary};
use ic_blob_storage_contracts::{
    dto::upload::{
        certificate::{CaffeineUploadCertificateResponse, UploadCertificateAssessmentResponse},
        exposure::UploadExposureFailure,
    },
    upload::binding::UploadContext,
};
use ic_memory::Memory;

/// Assess a root using independently supplied fixture evidence.
/// # Errors
/// Preserves root, authority, evidence and retained-state refusals.
pub fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    root: &str,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<UploadCertificateAssessmentResponse, UploadExposureFailure> {
    let permission = resolve(store, context, root, now)?;
    boundary::inspect(store, context, permission, evidence, now)
}
/// Issue using fixture evidence, retaining the production exposure transition.
/// # Errors
/// Preserves root, authority, evidence and retained-state refusals.
pub fn issue<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    root: &str,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<CaffeineUploadCertificateResponse, UploadCertificateFailure> {
    let permission =
        resolve(store, context, root, now).map_err(UploadCertificateFailure::Exposure)?;
    boundary::issue(store, context, permission, evidence, now)
}
