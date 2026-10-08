//! Bounded local operation recovery, without provider calls or retry authority.
use crate::model::catalog::admission::read::UploadPageLimits;
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::history;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryFailure;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryPage;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryRequest;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

/// Inspect retained identities as their tenant or the configured service operator.
/// The host supplies actual execution context and trusted scan/result bounds.
/// Suspended/restored inspection does not grant mutation or provider authority.
/// Pages observe current state: resweep for changes behind a cursor.
/// # Errors
/// Rejects scope/actor, malformed or mismatched cursors and inconsistent storage.
pub fn inspect<M: Memory>(
    uploads: &StableUploads<M>,
    context: UploadContext,
    input: UploadHistoryRequest,
    limits: UploadPageLimits,
) -> Result<UploadHistoryPage, UploadHistoryFailure> {
    history::inspect(uploads, context, input, limits)
}
