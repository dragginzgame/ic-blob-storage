//! Read-only exact upload status conversion.
use crate::model::catalog::admission::UploadPhase;
use crate::ops::service::references;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::UploadStatusFailure;
use ic_blob_storage_contracts::dto::upload::UploadStatusResponse;
use ic_blob_storage_contracts::upload::status::boundary_failure;

pub(crate) fn failure(error: UploadStoreError) -> UploadStatusFailure {
    boundary_failure(references::failure(error))
}
pub(crate) fn present(
    upload: ReferenceUpload,
    phase: UploadPhase,
    revoked: bool,
) -> UploadStatusResponse {
    UploadStatusResponse {
        upload,
        revoked,
        state: match phase {
            UploadPhase::Reserved => UploadState::Reserved,
            UploadPhase::Registered => UploadState::Registered,
            UploadPhase::Cancelled => UploadState::Cancelled,
        },
    }
}
