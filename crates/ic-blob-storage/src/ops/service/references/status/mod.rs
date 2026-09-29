//! Shared conversion over the exact maintained reference lookup.
use crate::{
    dto::reference::{
        ReferenceFailure,
        status::{ReferenceStatusRequest, ReferenceStatusResponse},
    },
    model::{
        catalog::admission::UploadRequest,
        lifecycle::{ReferenceId, binding::ReferenceKey},
        service::upload::UploadContext,
    },
    ops::service::uploads::StableUploads,
};
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU128;

/// Canonical passive reference query; linking exports no endpoint.
pub const REFERENCE_STATUS_METHOD: &str = "blob_reference_status";

pub(crate) fn parse(
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
pub(crate) fn inspect<M: Memory>(
    uploads: &StableUploads<M>,
    context: UploadContext,
    request: ReferenceStatusRequest,
) -> Result<ReferenceStatusResponse, ReferenceFailure> {
    let (upload, reference) = parse(context, request)?;
    let live = uploads
        .reference_is_live(context, upload, reference)
        .map_err(super::failure)?;
    Ok(ReferenceStatusResponse {
        request,
        live,
        fenced: uploads.is_fenced(),
    })
}
