//! Exact tenant upload inspection through the shared handler.
use ic_blob_storage::dto::upload::admission::{
    UploadAdmissionFailure, UploadAdmissionRequest, UploadAdmissionResponse,
};
pub(crate) fn manifest(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<
    ic_blob_storage::dto::upload::manifest::UploadManifestResponse,
    ic_blob_storage::dto::upload::manifest::UploadManifestFailure,
> {
    crate::ops::read::download::with_uploads(|uploads| {
        ic_blob_storage::workflow::uploads::manifests::inspect(uploads, context, input)
    })
}
pub(crate) fn admission(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
    crate::ops::read::download::with_uploads(|uploads| {
        ic_blob_storage::workflow::uploads::admission::inspect(uploads, context, input)
    })
}
use ic_blob_storage::{
    dto::{
        reference::ReferenceUpload,
        upload::{UploadStatusFailure, UploadStatusResponse},
    },
    model::service::upload::UploadContext,
};
pub(crate) fn inspect(
    context: UploadContext,
    upload: ReferenceUpload,
) -> Result<UploadStatusResponse, UploadStatusFailure> {
    crate::ops::read::download::with_uploads(|uploads| {
        ic_blob_storage::workflow::uploads::inspect(uploads, context, upload)
    })
}
