//! Shared conversion over the exact maintained reference lookup.
use crate::ops::service::uploads::StableUploads;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;
use ic_blob_storage_contracts::reference::status::parse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

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
