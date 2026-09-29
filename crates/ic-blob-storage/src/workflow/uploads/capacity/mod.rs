//! Tenant capacity inspection using maintained accounting, never reservation authority.
use crate::{
    dto::{
        tenant::TenantScope,
        upload::capacity::{UploadCapacityFailure, UploadCapacityResponse},
    },
    model::service::upload::UploadContext,
    ops::service::uploads::capacity::{self, UploadCapacitySource},
};

/// Inspect one tenant under actual caller/service context. Hosts bound ingress and
/// keep one synchronous owner borrow. Positive headroom can change immediately;
/// it establishes no identity freshness, provider funds or safe upload/retry authority.
/// Suspension and restore preserve inspection and are reported independently.
/// # Errors
/// Rejects invalid inputs, foreign scope/caller, missing enrollment or inconsistent state.
pub fn inspect<S: UploadCapacitySource>(
    source: &S,
    context: UploadContext,
    scope: TenantScope,
) -> Result<UploadCapacityResponse, UploadCapacityFailure> {
    capacity::inspect(source, context, scope)
}
