//! Bounded independently authenticated observations; no dispatch or retry authority.
use crate::dto::tenant::TenantEnrollment;
use crate::dto::tenant::TenantEnrollmentResponse;
use crate::dto::tenant::TenantFailure;
use crate::dto::tenant::TenantScope;
use crate::dto::tenant::TenantUpdateRequest;
use crate::tenant::next_enrollment;
use candid::Principal;
use candid::de::DecoderConfig;
use candid::decode_one_with_config;
use std::num::NonZeroUsize;
use thiserror::Error;

/// Rejected reply; an unusable acknowledgment can follow a committed update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum TenantReplyError {
    /// Encoded response exceeds the caller's byte budget.
    #[error("tenant reply exceeds limit")]
    Limit,
    /// Malformed encoding, invalid scope or impossible enrollment transition.
    #[error("invalid tenant request or reply")]
    Invalid,
    /// Response service, namespace or tenant differs from the selected scope.
    #[error("tenant reply binding mismatch")]
    Binding,
    /// Authenticated service refusal; not authority to retry an uncertain command.
    #[error("tenant request refused: {0:?}")]
    Remote(TenantFailure),
}

/// Validate passive input before an independently authenticated adapter call.
/// # Errors
/// Rejects malformed or differently bound input; grants no dispatch authority.
pub fn check_scope(scope: TenantScope) -> Result<(), TenantReplyError> {
    if scope.namespace == 0
        || [scope.service, scope.tenant]
            .into_iter()
            .any(|p| p == Principal::anonymous() || p == Principal::management_canister())
    {
        return Err(TenantReplyError::Invalid);
    }
    Ok(())
}
fn expected(input: TenantUpdateRequest) -> Result<TenantEnrollment, TenantReplyError> {
    check_scope(input.scope)?;
    let command = super::parse(input).map_err(|_| TenantReplyError::Invalid)?;
    // Reuse the same transition as the service. This predicts only a success
    // acknowledgment; actual authority, capacity and current state remain remote.
    let next =
        next_enrollment(command, command.expected, true).map_err(|_| TenantReplyError::Invalid)?;
    Ok(TenantEnrollment {
        generation: next.generation.get(),
        active: next.active,
    })
}
/// Validate an exact command before retaining intent; sends nothing and grants no authority.
/// # Errors
/// Rejects invalid scope, zero expected generation or exhausted reactivation.
pub fn validate_update(input: TenantUpdateRequest) -> Result<(), TenantReplyError> {
    expected(input).map(|_| ())
}

/// Decode a scoped current observation, including absent or fenced enrollment.
/// The caller must independently authenticate the selected service and bound
/// transport buffering. This is not a historical operation receipt.
/// # Errors
/// Rejects invalid input, oversized/malformed/foreign replies or service refusal.
pub fn inspection(
    scope: TenantScope,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<TenantEnrollmentResponse, TenantReplyError> {
    check_scope(scope)?;
    if bytes.len() > max.get() {
        return Err(TenantReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(max.get())
        .set_full_error_message(false);
    let result: Result<TenantEnrollmentResponse, TenantFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| TenantReplyError::Invalid)?;
    let observed = result.map_err(TenantReplyError::Remote)?;
    if observed.scope != scope {
        return Err(TenantReplyError::Binding);
    }
    if observed.enrollment.is_some_and(|v| v.generation == 0) {
        return Err(TenantReplyError::Invalid);
    }
    Ok(observed)
}

/// Validate one update acknowledgment against the exact saved precondition/action.
/// An error never proves that the service did not commit; inspect current state.
/// # Errors
/// Rejects invalid, foreign, absent, fenced or impossible successful responses.
pub fn mutation(
    input: TenantUpdateRequest,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<TenantEnrollmentResponse, TenantReplyError> {
    let next = expected(input)?;
    let observed = inspection(input.scope, bytes, max)?;
    if observed.fenced || observed.enrollment != Some(next) {
        return Err(TenantReplyError::Invalid);
    }
    Ok(observed)
}

#[cfg(test)]
mod tests;
