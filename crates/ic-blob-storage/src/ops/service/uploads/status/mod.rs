//! Read-only exact upload status conversion and authenticated IC client.
pub mod client;
pub mod reply;
use crate::{
    dto::{
        reference::{ReferenceFailure, ReferenceUpload},
        upload::{UploadState, UploadStatusFailure, UploadStatusResponse},
    },
    model::{
        catalog::admission::{UploadPhase, UploadRequest},
        service::upload::UploadContext,
    },
    ops::service::{references, uploads::UploadStoreError},
};
/// Canonical query, also called through replicated execution. Linking exports nothing.
pub const UPLOAD_STATUS_METHOD: &str = "blob_upload_status";
pub(crate) fn parse(
    context: UploadContext,
    upload: ReferenceUpload,
) -> Result<UploadRequest, UploadStatusFailure> {
    references::parse_upload(context, upload).map_err(boundary_failure)
}
fn boundary_failure(error: ReferenceFailure) -> UploadStatusFailure {
    match error {
        ReferenceFailure::Invalid => UploadStatusFailure::Invalid,
        ReferenceFailure::Denied => UploadStatusFailure::Denied,
        ReferenceFailure::Binding => UploadStatusFailure::Binding,
        ReferenceFailure::Unknown => UploadStatusFailure::Unknown,
        ReferenceFailure::Conflict => UploadStatusFailure::Conflict,
        _ => UploadStatusFailure::Internal,
    }
}
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
            UploadPhase::ExposurePossible => UploadState::ExposurePossible,
            UploadPhase::Confirmed => UploadState::Confirmed,
            UploadPhase::Cancelled => UploadState::Cancelled,
        },
    }
}
