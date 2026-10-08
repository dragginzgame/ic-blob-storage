//! Passive exact-reference recovery inspection, including suspended and restored owners.
use crate::ops::service::references::status;
use crate::ops::service::uploads::StableUploads;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

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
    status::inspect(uploads, context, request)
}
