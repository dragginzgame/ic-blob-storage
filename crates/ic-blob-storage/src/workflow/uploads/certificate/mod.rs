//! Installed-owner certificate issuance; linking exports no endpoint.
use crate::ops::service::{installation::ServiceInstallation, uploads::certificate};
use ic_blob_storage_contracts::{
    dto::upload::{
        certificate::{CaffeineUploadCertificateResponse, UploadCertificateAssessmentResponse},
        exposure::UploadExposureFailure,
    },
    upload::binding::UploadContext,
};
use ic_memory::Memory;

/// Inspect actual installed state. A successful observation grants no later authority.
/// # Errors
/// Refuses wrong actors/roots, expiry, revocation, phase or recovery fences.
pub fn inspect_installation<M: Memory>(
    installation: &ServiceInstallation<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<UploadCertificateAssessmentResponse, UploadExposureFailure> {
    certificate::inspect(&installation.stores().uploads, context, root, now)
}
/// Commit exposure synchronously before returning the plain provider response.
/// The host must propagate traps and return in this update so writes/reply are atomic.
/// # Errors
/// Refuses wrong actors/roots, expiry, revocation, phase or recovery fences.
pub fn issue_installation<M: Memory>(
    installation: &mut ServiceInstallation<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<CaffeineUploadCertificateResponse, UploadExposureFailure> {
    certificate::issue(&mut installation.stores_mut().uploads, context, root, now)
}
