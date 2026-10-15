//! Admission headroom is an observation, never a capacity reservation.
use crate::dto::tenant::TenantScope;
use crate::dto::upload::capacity::{UploadCapacityFailure, UploadCapacityResponse};
use std::num::NonZeroUsize;
use thiserror::Error;

/// Unusable capacity reply after independent service authentication.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadCapacityReplyError {
    /// Encoded response exceeds the caller's complete-input ceiling.
    #[error("capacity reply exceeds limit")]
    Limit,
    /// Malformed Candid, zero limits/generation or contradictory byte dimensions.
    #[error("invalid capacity reply")]
    Invalid,
    /// Echoed service, namespace or tenant differs from the request.
    #[error("capacity reply binding mismatch")]
    Binding,
    /// Authenticated service refusal, never an implicit empty capacity.
    #[error("service refused capacity inspection: {0:?}")]
    Remote(UploadCapacityFailure),
}

/// Admit one bounded reply and its exact scope while preserving suspension/fences.
/// The caller owns service authentication and planning over this staleable view.
/// # Errors
/// Refuses oversized/malformed replies, changed scope and inconsistent headroom.
pub fn decode(
    scope: TenantScope,
    bytes: &[u8],
    max_reply_bytes: NonZeroUsize,
) -> Result<UploadCapacityResponse, UploadCapacityReplyError> {
    use UploadCapacityReplyError as E;
    if bytes.len() > max_reply_bytes.get() {
        return Err(E::Limit);
    }
    let limits = crate::decoding::limits(100_000, 1000, 64, max_reply_bytes.get());
    let response: Result<UploadCapacityResponse, UploadCapacityFailure> =
        candid::decode_one_with_config(bytes, &limits).map_err(|_| E::Invalid)?;
    let response = response.map_err(E::Remote)?;
    if response.scope != scope {
        return Err(E::Binding);
    }
    if response.enrollment.generation == 0
        || response.max_object_bytes == 0
        || response.max_headers == 0
        || response.max_header_bytes == 0
        || response.remaining_bytes
            != response
                .remaining_logical_bytes
                .min(response.remaining_physical_bytes)
                .min(response.remaining_liability_bytes)
    {
        return Err(E::Invalid);
    }
    Ok(response)
}
#[cfg(test)]
mod tests;
