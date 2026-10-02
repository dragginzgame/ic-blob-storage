//! Guarded intent-before-effect bookkeeping. No certificate or provider call is made.
use crate::{
    dto::upload::{
        admission::{UploadAdmissionRequest, UploadAdmissionResponse},
        exposure::UploadExposureFailure,
    },
    model::service::upload::UploadContext,
    ops::service::uploads::{StableUploads, exposure},
    policy::upload::exposure::{
        UploadExposureAssessment, UploadExposureHostEvidence, assess_exposure,
    },
};
use ic_memory::ic_stable_structures::Memory;
/// Result of this synchronous operation; never a reusable certificate-issuance permit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UploadExposureResult {
    /// Missing host facts; no state was written.
    Blocked(UploadExposureAssessment),
    /// Possible exposure is now retained. This does not establish stored bytes,
    /// actual certificate delivery or permission to repeat an uncertain effect.
    Exposed(Box<UploadAdmissionResponse>),
}
/// Inspect actual local eligibility and independently supplied current host facts.
/// The host must derive installed uploader trust, the restricted envelope, local
/// namespace, current-owner eligibility and durable commit. Do not deserialize evidence from ingress
/// or reuse an assessment after an await; commit re-reads local state independently.
/// # Errors
/// Refuses foreign/changed permission or evidence, expiry, phase, revocation or fence.
pub fn inspect_preparation<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<UploadExposureAssessment, UploadExposureFailure> {
    exposure::check(store, context, input, evidence, now)?;
    Ok(assess_exposure(evidence, now))
}
/// Recheck eligibility and host evidence, then record possible exposure exactly once.
/// All checks/writes are synchronous. The host must commit this intent before any
/// certificate/effect escapes and propagate stable traps for IC atomic rollback.
/// Failure/loss after a committed exposure retains uncertainty: inspect history,
/// never mint again based on timeout or missing acknowledgment. This result is no
/// authorization to issue later after an await, restart, revocation or stale restore.
/// # Errors
/// Refuses permission/evidence mismatch, expiry, repeated exposure, revocation or fence.
pub fn commit<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<UploadExposureResult, UploadExposureFailure> {
    let permission = exposure::check(store, context, input, evidence, now)?;
    let assessment = assess_exposure(evidence, now);
    if !assessment.blockers.is_empty() {
        return Ok(UploadExposureResult::Blocked(assessment));
    }
    exposure::commit(store, context, permission, now)?;
    Ok(UploadExposureResult::Exposed(Box::new(exposure::inspect(
        store, context, input,
    )?)))
}
/// Recover exact historical permission state as tenant or original uploader,
/// including after expiry, revocation and restore. Performs no policy override,
/// mutation, certificate issuance or renewal; absence never grants retry authority.
/// # Errors
/// Rejects wrong actor/service and missing or changed original permission.
pub fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadExposureFailure> {
    exposure::inspect(store, context, input)
}
