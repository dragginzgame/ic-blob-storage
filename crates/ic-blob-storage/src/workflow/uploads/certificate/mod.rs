//! Shared Caffeine root-only issuance boundary. Linking exports no IC endpoint.
use crate::policy::upload::exposure::UploadExposureAssessment;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_contracts::dto::upload::certificate::UploadCertificateAssessmentResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

/// Assess installed-owner eligibility and evidence through an immutable borrow.
/// Resolves actual uploader authority before deriving the installation's facts;
/// a positive assessment neither reserves issuance nor grants mutation authority.
/// # Errors
/// Retains the existing local permission, evidence and recovery-fence refusals.
pub fn inspect_installation<M: Memory>(
    installation: &crate::ops::service::installation::ServiceInstallation<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<UploadCertificateAssessmentResponse, UploadExposureFailure> {
    let permission = resolve(&installation.stores().uploads, context, root, now)?;
    inspect(
        &installation.stores().uploads,
        context,
        root,
        installation.certificate_evidence(permission, now),
        now,
    )
}

/// Issue through the installed evidence owner in one synchronous replicated update.
/// The host returns the plain reply and propagates traps so exposure and reply
/// commit together. No await, deferred reply or query issuance is authorized.
/// Resolution precedes evidence acquisition; the final transition rechecks both.
/// These local facts do not qualify deployed provider billing or replay behavior.
/// # Errors
/// Retains the existing local permission, evidence and recovery-fence refusals.
pub fn issue_installation<M: Memory>(
    installation: &mut crate::ops::service::installation::ServiceInstallation<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<CaffeineUploadCertificateResponse, UploadCertificateFailure> {
    let permission = resolve(&installation.stores().uploads, context, root, now)
        .map_err(UploadCertificateFailure::Exposure)?;
    let evidence = installation.certificate_evidence(permission, now);
    issue(
        &mut installation.stores_mut().uploads,
        context,
        root,
        evidence,
        now,
    )
}

/// Local refusal. Adapters must reject/trap the ingress update, never encode this
/// error as a successful provider reply or replace it with a certificate response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UploadCertificateFailure {
    /// Local identity, permission, preparation, phase or recovery fence rejected issuance.
    Exposure(UploadExposureFailure),
    /// Independent host prerequisites are missing; no exposure was written.
    Blocked(UploadExposureAssessment),
}

pub(crate) mod boundary;
use boundary::{inspect, issue, resolve};
