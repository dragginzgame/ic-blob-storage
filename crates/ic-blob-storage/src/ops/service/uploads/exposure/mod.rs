//! Exact uploader exposure context and historical tenant/uploader inspection.
use super::{StableUploads, UploadStoreError, admission};
use crate::{
    dto::upload::{
        admission::{UploadAdmissionFailure, UploadAdmissionRequest, UploadAdmissionResponse},
        exposure::UploadExposureFailure,
    },
    model::service::{
        tenant::TenantError,
        upload::{UploadAdmissionError, UploadContext, UploadPermission},
    },
    policy::upload::exposure::UploadExposureHostEvidence,
};
use ic_memory::ic_stable_structures::Memory;

/// Present every independent policy blocker at the Candid boundary. Conversion
/// grants no issuance authority and does not acquire or authenticate host evidence.
#[must_use]
pub fn blockers(
    assessment: crate::policy::upload::exposure::UploadExposureAssessment,
) -> Vec<crate::dto::upload::exposure::UploadExposureBlocker> {
    use crate::{
        dto::upload::exposure::UploadExposureBlocker as D,
        policy::upload::exposure::UploadExposureBlocker as P,
    };
    assessment
        .blockers
        .into_iter()
        .map(|blocker| match blocker {
            P::StaleObservation => D::StaleObservation,
            P::TrialBounds => D::TrialBounds,
            P::NamespaceBinding => D::NamespaceBinding,
            P::TrustedUploader => D::TrustedUploader,
            P::CurrentOwner => D::CurrentOwner,
            P::Durability => D::Durability,
        })
        .collect()
}
fn permission<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
    exposing: bool,
) -> Result<UploadPermission, UploadExposureFailure> {
    let uploader = context.actor == input.uploader;
    let tenant_observer = !exposing && context.actor == input.upload.tenant;
    if !uploader && !tenant_observer {
        return Err(UploadExposureFailure::Permission(
            UploadAdmissionFailure::Denied,
        ));
    }
    let permission = admission::parse_binding(context.service, input)
        .map_err(UploadExposureFailure::Permission)?;
    let retained = store.lookup(context, permission.request).map_err(failure)?;
    if retained.permission != permission {
        return Err(UploadExposureFailure::Permission(
            UploadAdmissionFailure::Conflict,
        ));
    }
    Ok(permission)
}
pub(crate) fn check<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
    evidence: UploadExposureHostEvidence,
    now: u64,
) -> Result<UploadPermission, UploadExposureFailure> {
    let permission = permission(store, context, input, true)?;
    if evidence.permission != permission {
        return Err(UploadExposureFailure::EvidenceBinding);
    }
    store
        .exposure_record(context, permission.request, now)
        .map_err(failure)?;
    Ok(permission)
}
pub(crate) fn commit<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    permission: UploadPermission,
    now: u64,
) -> Result<(), UploadExposureFailure> {
    store
        .expose(context, permission.request, now)
        .map_err(failure)
}
pub(crate) fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadExposureFailure> {
    let permission = permission(store, context, input, false)?;
    let view = store.lookup(context, permission.request).map_err(failure)?;
    admission::present(input, &view).map_err(UploadExposureFailure::Permission)
}
pub(super) fn failure(error: UploadStoreError) -> UploadExposureFailure {
    use UploadAdmissionError as A;
    match error {
        UploadStoreError::Admission(A::NotUploader | A::NotObserver) => {
            UploadExposureFailure::Permission(UploadAdmissionFailure::Denied)
        }
        UploadStoreError::Admission(A::ManifestNotPrepared) => UploadExposureFailure::Unprepared,
        UploadStoreError::Admission(A::Revoked) => UploadExposureFailure::Revoked,
        UploadStoreError::Admission(A::NotReserved(_) | A::ClockReversed) => {
            UploadExposureFailure::Phase
        }
        UploadStoreError::Admission(A::Tenant(TenantError::StalePermission)) => {
            UploadExposureFailure::Permission(UploadAdmissionFailure::Inactive)
        }
        other => UploadExposureFailure::Permission(admission::failure(other)),
    }
}
