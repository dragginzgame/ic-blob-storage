//! Exact upload-permission binding and bounded replies.
pub mod reply;
use crate::dto::reference::ReferenceFailure;
use crate::dto::upload::admission::UploadAdmissionFailure;
use crate::dto::upload::admission::UploadAdmissionRequest;
use crate::reference;
use crate::upload::binding::UploadContext;
use crate::upload::binding::UploadPermission;
/// Canonical structural binding; the caller supplies authenticated context.
/// # Errors
/// Rejects malformed or differently bound identities without effects.
pub fn parse(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadPermission, UploadAdmissionFailure> {
    if input.upload.service != context.service {
        return Err(UploadAdmissionFailure::Binding);
    }
    if input.upload.tenant != context.actor {
        return Err(UploadAdmissionFailure::Denied);
    }
    parse_binding(context.service, input)
}
/// Canonical structural binding; the caller supplies authenticated context.
/// # Errors
/// Rejects malformed or differently bound identities without effects.
pub fn parse_binding(
    service: candid::Principal,
    input: UploadAdmissionRequest,
) -> Result<UploadPermission, UploadAdmissionFailure> {
    let request = reference::parse_upload_binding(service, input.upload).map_err(|e| match e {
        ReferenceFailure::Denied => UploadAdmissionFailure::Denied,
        ReferenceFailure::Binding => UploadAdmissionFailure::Binding,
        _ => UploadAdmissionFailure::Invalid,
    })?;
    if [
        candid::Principal::anonymous(),
        candid::Principal::management_canister(),
    ]
    .contains(&input.uploader)
    {
        return Err(UploadAdmissionFailure::Invalid);
    }
    Ok(UploadPermission {
        request,
        uploader: input.uploader,
        expires_at_ns: input.expires_at_ns,
    })
}
