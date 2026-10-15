//! Internal evidence-taking certificate implementation.
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::certificate;
pub(crate) use crate::ops::service::uploads::certificate::resolve;
use crate::policy::upload::exposure::UploadExposureHostEvidence;
use crate::workflow::uploads::exposure;
use crate::workflow::uploads::exposure::UploadExposureResult;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_contracts::dto::upload::certificate::UploadCertificateAssessmentResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

use super::UploadCertificateFailure;
/// Inspect the same local eligibility and independent evidence used at issuance.
/// The root is a locator, not tenant authority. Host evidence must be obtained
/// from installed namespace/current-owner facts and bound to the complete original permission.
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
/// namespace/current-owner evidence. The exact retained project permission owns
/// uploader authority. Never accept host evidence from ingress or claim
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
