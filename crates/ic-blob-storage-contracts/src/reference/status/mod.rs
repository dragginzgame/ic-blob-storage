//! Passive current-reference lookup binding.
use crate::binding::ReferenceId;
use crate::binding::ReferenceKey;
use crate::dto::reference::ReferenceFailure;
use crate::dto::reference::status::ReferenceStatusRequest;
use crate::upload::binding::UploadContext;
use crate::upload::binding::UploadRequest;
use std::num::NonZeroU128;
/// Canonical structural binding; the caller supplies authenticated context.
/// # Errors
/// Rejects malformed or differently bound identities without effects.
pub fn parse(
    context: UploadContext,
    request: ReferenceStatusRequest,
) -> Result<(UploadRequest, ReferenceKey), ReferenceFailure> {
    let upload = super::parse_upload(context, request.upload)?;
    let reference = NonZeroU128::new(request.reference).ok_or(ReferenceFailure::Invalid)?;
    Ok((
        upload,
        ReferenceKey::new(upload.object.first.object(), ReferenceId::new(reference)),
    ))
}
