//! Shared passive reference capacity, including reserved cleanup receipts.
use crate::{
    dto::reference::capacity::{
        ReferenceCapacityFailure, ReferenceCapacityRequest, ReferenceCapacityResponse,
    },
    model::service::upload::UploadContext,
    ops::service::references::capacity::{self, ReferenceCapacitySource},
};
/// Inspect tenant-scoped reference headroom using authenticated service/caller context.
/// Hosts bound ingress and retain one synchronous owner borrow. Suspension and restore
/// preserve inspection; positive counts do not authorize retain, retry or publication.
/// Unknown/foreign/unconfirmed content shares the same absence response.
/// # Errors
/// Rejects invalid/foreign scope, unrelated callers or inconsistent retained counters.
pub fn inspect<S: ReferenceCapacitySource>(
    source: &S,
    context: UploadContext,
    request: ReferenceCapacityRequest,
) -> Result<ReferenceCapacityResponse, ReferenceCapacityFailure> {
    capacity::inspect(source, context, request)
}
