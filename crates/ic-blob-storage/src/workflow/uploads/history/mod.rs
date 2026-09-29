//! Bounded local operation recovery, without provider calls or retry authority.
use crate::{
    dto::upload::history::{UploadHistoryFailure, UploadHistoryPage, UploadHistoryRequest},
    model::{catalog::admission::read::UploadPageLimits, service::upload::UploadContext},
    ops::service::uploads::{StableUploads, history},
};
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
