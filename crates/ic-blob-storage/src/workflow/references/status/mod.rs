//! Passive exact-reference recovery inspection, including suspended and restored owners.
use crate::ops::service::references::failure;
use crate::ops::service::uploads::StableUploads;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;
use ic_blob_storage_contracts::reference::status::parse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::Memory;

/// Inspect current local liveness under one synchronous owner borrow.
/// The adapter authenticates actual service/caller and bounds ingress separately.
/// This grants no serving, allocation, retry, or operational recovery authority.
/// # Errors
/// Rejects malformed/foreign bindings, missing/changed uploads, unconfirmed content
/// and inconsistent retained state. Refusal is distinct from a non-live reference.
pub fn inspect<M: Memory>(
    uploads: &StableUploads<M>,
    context: UploadContext,
    request: ReferenceStatusRequest,
) -> Result<ReferenceStatusResponse, ReferenceFailure> {
    let (upload, reference) = parse(context, request)?;
    let live = uploads
        .reference_is_live(context, upload, reference)
        .map_err(failure)?;
    Ok(ReferenceStatusResponse {
        request,
        live,
        fenced: uploads.is_fenced(),
    })
}
