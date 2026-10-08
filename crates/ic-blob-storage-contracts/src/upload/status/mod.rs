//! Pure upload status contracts.
pub mod reply;
use crate::dto::reference::ReferenceFailure;
use crate::dto::reference::ReferenceUpload;
use crate::dto::upload::UploadStatusFailure;
use crate::reference as references;
use crate::upload::binding::UploadContext;
use crate::upload::binding::UploadRequest;
/// Canonical passive conversion; no state access or dispatch.
/// # Errors
/// Rejects malformed or differently bound input.
pub fn parse(
    context: UploadContext,
    upload: ReferenceUpload,
) -> Result<UploadRequest, UploadStatusFailure> {
    references::parse_upload(context, upload).map_err(boundary_failure)
}
/// Map structurally equivalent reference refusal to the status boundary.
#[must_use]
pub fn boundary_failure(error: ReferenceFailure) -> UploadStatusFailure {
    match error {
        ReferenceFailure::Invalid => UploadStatusFailure::Invalid,
        ReferenceFailure::Denied => UploadStatusFailure::Denied,
        ReferenceFailure::Binding => UploadStatusFailure::Binding,
        ReferenceFailure::Unknown => UploadStatusFailure::Unknown,
        ReferenceFailure::Conflict => UploadStatusFailure::Conflict,
        _ => UploadStatusFailure::Internal,
    }
}
