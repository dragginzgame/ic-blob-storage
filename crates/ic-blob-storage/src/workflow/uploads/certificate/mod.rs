//! Shared Caffeine root-only issuance boundary. Linking exports no IC endpoint.
use crate::{
    dto::upload::{
        certificate::CaffeineUploadCertificateResponse, exposure::UploadExposureFailure,
    },
    model::service::upload::{UploadContext, UploadPermission},
    ops::service::uploads::{StableUploads, certificate},
    policy::upload::exposure::{UploadExposureAssessment, UploadExposureHostEvidence},
    workflow::uploads::exposure::{self, UploadExposureResult},
};
use ic_memory::ic_stable_structures::Memory;

/// Reviewed Caffeine ingress update name. Its wire reply is a plain record, not Result.
pub const CAFFEINE_UPLOAD_CERTIFICATE_METHOD: &str = "_immutableObjectStorageCreateCertificate";

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

/// Re-resolve a root, recheck exact current host evidence and record exposure before
/// constructing Caffeine's successful reply payload. The host must call this
/// synchronously in a replicated ingress update, return the plain record directly,
/// and propagate traps so state and reply commit atomically. Do not await, save the
/// reply for later, wrap it in Result on the wire, or issue it through a query.
///
/// The IC supplies the response certificate; this function does not sign bytes or
/// send them to a gateway. The host owns ingress decoding limits and independent
/// provider/recovery qualification. Never accept host evidence from ingress.
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
