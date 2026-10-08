//! Shared exact reference mutation and receipt inspection; no provider dispatch.
pub mod capacity;
pub mod status;
use crate::ops::service::references;
use crate::ops::service::uploads::StableUploads;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::ReferenceReceiptLookup;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;
/// Inspect an exact original result for the authenticated tenant. The adapter
/// supplies actual caller/service and bounds ingress decoding. Inspection remains
/// available under suspension and restore fences, and after settlement. `Absent`
/// proves only absence in this owner; it never authorizes retry or publication.
/// # Errors
/// Rejects invalid/foreign bindings, changed arguments, unconfirmed uploads and
/// inconsistent state. Recorded transition failures remain inside the response.
pub fn receipt<M: Memory>(
    uploads: &StableUploads<M>,
    context: UploadContext,
    request: ReferenceCommand,
) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
    let (upload, operation) = ic_blob_storage_contracts::reference::parse(context, request)?;
    let view = uploads
        .reference_receipt(context, upload, operation)
        .map_err(references::failure)?;
    match view {
        Some(view) => references::present(request, view).map(ReferenceReceiptLookup::Found),
        None => Ok(ReferenceReceiptLookup::Absent),
    }
}

/// Apply the authenticated tenant's exact retain/release intent synchronously.
/// Adapters supply actual context and propagate storage traps for IC rollback.
/// The caller must persist intent before dispatch and keep it through uncertain
/// replies. Exact retries return the original result, including failures, without
/// reviving references. Fresh retains need active enrollment; releases/replays
/// preserve cleanup under suspension. Restore fences reject every mutation.
/// Success is historical evidence, not a publication lease or provider deletion.
/// # Errors
/// Rejects invalid/foreign/conflicting intent, unavailable receipt capacity,
/// inactive fresh retains, unconfirmed uploads and restored owners.
/// # Panics
/// Stable-write traps must roll back the complete IC message.
pub fn apply<M: Memory>(
    uploads: &mut StableUploads<M>,
    context: UploadContext,
    request: ReferenceCommand,
) -> Result<ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse, ReferenceFailure>
{
    let (upload, operation) = ic_blob_storage_contracts::reference::parse(context, request)?;
    let outcome = uploads
        .apply_reference(context, upload, operation)
        .map_err(references::failure)?;
    references::mutation(request, &outcome)
}
