//! Exact tenant upload inspection through the shared handler.
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::UploadStatusFailure;
use ic_blob_storage_contracts::dto::upload::UploadStatusResponse;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;

pub(crate) fn manifest(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<
    ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse,
    ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure,
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
pub(crate) fn inspect(
    context: UploadContext,
    upload: ReferenceUpload,
) -> Result<UploadStatusResponse, UploadStatusFailure> {
    crate::ops::read::download::with_uploads(|uploads| {
        ic_blob_storage::workflow::uploads::inspect(uploads, context, upload)
    })
}
