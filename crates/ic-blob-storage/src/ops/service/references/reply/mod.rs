//! Bounded decoding after independent authentication of the selected service.
use crate::{
    dto::reference::{
        ReferenceCommand, ReferenceFailure, ReferenceMutationResponse, ReferenceReceiptLookup,
    },
    model::service::upload::UploadContext,
};
use candid::{CandidType, Deserialize, de::DecoderConfig, decode_one_with_config};
use std::num::NonZeroUsize;
use thiserror::Error;
/// Reference response failure, separate from a stored transition failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ReferenceReplyError {
    /// Returned exact operation differs from the requested one.
    #[error("reference receipt binding mismatch")]
    Binding,
    /// Complete encoded response exceeds the configured application limit.
    #[error("reference receipt exceeds limit")]
    Limit,
    /// Malformed request or Candid response.
    #[error("invalid reference receipt")]
    Invalid,
    /// Authenticated service refused the request.
    #[error("service refused reference request: {0:?}")]
    Remote(ReferenceFailure),
}
pub(crate) fn check_request(request: ReferenceCommand) -> Result<(), ReferenceReplyError> {
    super::parse(
        UploadContext {
            service: request.upload.service,
            actor: request.upload.tenant,
        },
        request,
    )
    .map(|_| ())
    .map_err(|_| ReferenceReplyError::Invalid)
}
/// Decode a reply from an independently authenticated service. Absence, original
/// success, original failure and lookup refusal remain distinct. Neither absence
/// nor historical success supplies mutation, retry or current liveness authority.
/// # Errors
/// Rejects oversized/malformed responses, changed original arguments and refusals.
pub fn decode(
    request: ReferenceCommand,
    bytes: &[u8],
    max_reply_bytes: NonZeroUsize,
) -> Result<ReferenceReceiptLookup, ReferenceReplyError> {
    check_request(request)?;
    let response = bounded(bytes, max_reply_bytes)?;
    if matches!(response, ReferenceReceiptLookup::Found(r) if r.request != request) {
        return Err(ReferenceReplyError::Binding);
    }
    Ok(response)
}

/// Decode the exact mutation result after independent service authentication.
/// A decoding/size failure can follow a committed mutation: preserve intent and
/// reconcile its receipt. A stored inner failure is not a successful retain/release.
/// # Errors
/// Rejects invalid input, oversized/malformed replies, changed intent or service refusal.
pub fn decode_mutation(
    request: ReferenceCommand,
    bytes: &[u8],
    max_reply_bytes: NonZeroUsize,
) -> Result<ReferenceMutationResponse, ReferenceReplyError> {
    check_request(request)?;
    let response: ReferenceMutationResponse = bounded(bytes, max_reply_bytes)?;
    if response.receipt.request != request {
        return Err(ReferenceReplyError::Binding);
    }
    Ok(response)
}

fn bounded<T: CandidType + for<'de> Deserialize<'de>>(
    bytes: &[u8],
    max_reply_bytes: NonZeroUsize,
) -> Result<T, ReferenceReplyError> {
    if bytes.len() > max_reply_bytes.get() {
        return Err(ReferenceReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(1_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(max_reply_bytes.get())
        .set_full_error_message(false);
    let response: Result<T, ReferenceFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| ReferenceReplyError::Invalid)?;
    response.map_err(ReferenceReplyError::Remote)
}
#[cfg(test)]
mod tests;
