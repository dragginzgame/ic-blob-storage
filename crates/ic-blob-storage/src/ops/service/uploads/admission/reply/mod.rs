//! Bounded replies validated against the complete original permission.
use crate::{
    dto::upload::admission::{
        UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
        UploadAdmissionResponse, UploadRevocationResponse,
    },
    model::service::upload::UploadContext,
};
use candid::{CandidType, Deserialize, de::DecoderConfig, decode_one_with_config};
use std::num::NonZeroUsize;
use thiserror::Error;
/// Reply failure. Even an unusable admission response can follow a committed reservation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadAdmissionReplyError {
    /// Encoded response exceeds its budget.
    #[error("admission reply exceeds limit")]
    Limit,
    /// Malformed request or response.
    #[error("invalid admission reply")]
    Invalid,
    /// Returned permission differs from the saved intent.
    #[error("admission reply binding mismatch")]
    Binding,
    /// Authenticated service refusal.
    #[error("admission refused: {0:?}")]
    Remote(UploadAdmissionFailure),
}
pub(crate) fn check(input: UploadAdmissionRequest) -> Result<(), UploadAdmissionReplyError> {
    super::parse(
        UploadContext {
            service: input.upload.service,
            actor: input.upload.tenant,
        },
        input,
    )
    .map(|_| ())
    .map_err(|_| UploadAdmissionReplyError::Invalid)
}
fn decode<T: CandidType + for<'de> Deserialize<'de>>(
    input: UploadAdmissionRequest,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<T, UploadAdmissionReplyError> {
    check(input)?;
    if bytes.len() > max.get() {
        return Err(UploadAdmissionReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(1_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(max.get())
        .set_full_error_message(false);
    let result: Result<T, UploadAdmissionFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| UploadAdmissionReplyError::Invalid)?;
    result.map_err(UploadAdmissionReplyError::Remote)
}
fn bound(
    input: UploadAdmissionRequest,
    observed: UploadAdmissionResponse,
) -> Result<UploadAdmissionResponse, UploadAdmissionReplyError> {
    if observed.permission != input {
        return Err(UploadAdmissionReplyError::Binding);
    }
    Ok(observed)
}
/// Decode an independently authenticated exact permission observation.
/// # Errors
/// Rejects oversize, malformed, foreign or refused replies before disclosure.
pub fn inspection(
    input: UploadAdmissionRequest,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<UploadAdmissionResponse, UploadAdmissionReplyError> {
    bound(input, decode(input, bytes, max)?)
}
/// Decode a reservation acknowledgment; historical replay does not renew permission.
/// # Errors
/// Rejects oversize, malformed, foreign or refused replies before disclosure.
pub fn mutation(
    input: UploadAdmissionRequest,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<UploadAdmissionMutation, UploadAdmissionReplyError> {
    let result: UploadAdmissionMutation = decode(input, bytes, max)?;
    bound(input, result.admission)?;
    Ok(result)
}
#[cfg(test)]
mod tests;

/// Decode independently authenticated withdrawal evidence bound to the full permission.
/// # Errors
/// Rejects oversized, malformed, foreign, refused or non-revoked replies.
pub fn revocation(
    input: UploadAdmissionRequest,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<UploadRevocationResponse, UploadAdmissionReplyError> {
    let result: UploadRevocationResponse = decode(input, bytes, max)?;
    bound(input, result.admission)?;
    if !result.admission.revoked {
        return Err(UploadAdmissionReplyError::Invalid);
    }
    Ok(result)
}
