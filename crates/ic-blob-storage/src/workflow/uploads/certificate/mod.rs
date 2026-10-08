//! Shared Caffeine root-only issuance boundary. Linking exports no IC endpoint.
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::certificate;
use crate::policy::upload::exposure::UploadExposureAssessment;
use crate::policy::upload::exposure::UploadExposureHostEvidence;
use crate::workflow::uploads::exposure;
use crate::workflow::uploads::exposure::UploadExposureResult;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_contracts::dto::upload::certificate::UploadCertificateAssessmentResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_memory::ic_stable_structures::Memory;

/// Local refusal. Adapters must reject/trap the ingress update, never encode this
/// error as a successful provider reply or replace it with a certificate response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UploadCertificateFailure {
    /// Local identity, permission, preparation, phase or recovery fence rejected issuance.
    Exposure(UploadExposureFailure),
    /// Independent host prerequisites are missing; no exposure was written.
    Blocked(UploadExposureAssessment),
}

/// Resolve the retained permission as the actual uploader before acquiring host
/// evidence. Uses indexed bounded reads; no tree rebuilding or file bytes.
/// This does not reserve issuance, supply host facts or authorize a later reply.
/// # Errors
/// Rejects malformed/foreign/unknown roots and current local ineligibility.
pub fn resolve<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<UploadPermission, UploadExposureFailure> {
    certificate::resolve(store, context, root, now)
}

/// Inspect the same local eligibility and independent evidence used at issuance.
/// The root is a locator, not tenant authority. Host evidence must be obtained
/// from installed trust/current-owner facts and bound to the complete original permission.
/// No state is changed, even when every prerequisite is established.
/// # Errors
/// Refuses wrong actors/roots, stale local permissions, changed evidence or fences.
pub fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    root: &str,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<UploadCertificateAssessmentResponse, UploadExposureFailure> {
    let permission = resolve(store, context, root, now)?;
    let assessment = exposure::inspect_preparation(
        store,
        context,
        certificate::input(permission),
        evidence,
        now,
    )?;
    Ok(certificate::assessment(permission, now, assessment))
}

/// Re-resolve a root, recheck exact current host evidence and record exposure before
/// constructing Caffeine's successful reply payload. The host must call this
/// synchronously in a replicated ingress update, return the plain record directly,
/// and propagate traps so state and reply commit atomically. Do not await, save the
/// reply for later, wrap it in Result on the wire, or issue it through a query.
///
/// The IC supplies the response certificate; this function does not sign bytes or
/// send them to a gateway. The host owns ingress decoding limits and independent
/// trusted-uploader contract. Never accept host evidence from ingress or claim
/// that local prerequisites establish provider charge or replay enforcement.
/// A lost committed reply must be inspected, not reissued. A local deadline cannot
/// recall an escaped certificate or establish the provider's replay rules.
/// # Errors
/// Refuses invalid/local-ineligible roots, mismatched evidence or missing host facts.
pub fn issue<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    root: &str,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<CaffeineUploadCertificateResponse, UploadCertificateFailure> {
    let permission =
        resolve(store, context, root, now).map_err(UploadCertificateFailure::Exposure)?;
    match exposure::commit(
        store,
        context,
        certificate::input(permission),
        evidence,
        now,
    )
    .map_err(UploadCertificateFailure::Exposure)?
    {
        UploadExposureResult::Blocked(assessment) => {
            Err(UploadCertificateFailure::Blocked(assessment))
        }
        UploadExposureResult::Exposed(_) => Ok(certificate::response(permission)),
    }
}
