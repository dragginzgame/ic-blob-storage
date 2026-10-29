//! Historical exposure inspection, with mutation owned by installed certificate issuance.
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::exposure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::Memory;

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
    let view = exposure::permission(store, context, input, false)?;
    crate::ops::service::uploads::admission::present(input, &view)
        .map_err(UploadExposureFailure::Permission)
}
