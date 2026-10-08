//! Exact uploader exposure context and historical tenant/uploader inspection.
use super::{StableUploads, UploadStoreError, admission};
use crate::model::service::upload::UploadAdmissionError;
use crate::policy::upload::exposure::UploadExposureHostEvidence;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_memory::ic_stable_structures::Memory;

/// Present every independent policy blocker at the Candid boundary. Conversion
/// grants no issuance authority and does not acquire or authenticate host evidence.
#[must_use]
pub fn blockers(
    assessment: crate::policy::upload::exposure::UploadExposureAssessment,
) -> Vec<ic_blob_storage_contracts::dto::upload::exposure::UploadExposureBlocker> {
    use crate::policy::upload::exposure::UploadExposureBlocker as P;
    use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureBlocker as D;
    assessment
        .blockers
        .into_iter()
        .map(|blocker| match blocker {
            P::StaleObservation => D::StaleObservation,
            P::NamespaceBinding => D::NamespaceBinding,
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
    let permission =
        ic_blob_storage_contracts::upload::admission::parse_binding(context.service, input)
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
