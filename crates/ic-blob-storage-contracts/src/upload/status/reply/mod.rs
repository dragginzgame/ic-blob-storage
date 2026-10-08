//! Bounded status decoding after independent service authentication.
use crate::dto::reference::ReferenceUpload;
use crate::dto::upload::UploadStatusFailure;
use crate::dto::upload::UploadStatusResponse;
use crate::upload::binding::UploadContext;
use candid::de::DecoderConfig;
use candid::decode_one_with_config;
use std::num::NonZeroUsize;
use thiserror::Error;
/// Reply refusal, never authority to repeat an uncertain upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadStatusReplyError {
    /// Complete encoded response exceeds the caller's budget.
    #[error("upload status exceeds limit")]
    Limit,
    /// Invalid request or malformed response.
    #[error("invalid upload status")]
    Invalid,
    /// Returned original upload differs.
    #[error("upload status binding mismatch")]
    Binding,
    /// Authenticated service refusal.
    #[error("upload status refused: {0:?}")]
    Remote(UploadStatusFailure),
}
/// Validate passive input before an independently authenticated adapter call.
/// # Errors
/// Rejects malformed or differently bound input; grants no dispatch authority.
pub fn check(upload: ReferenceUpload) -> Result<(), UploadStatusReplyError> {
    super::parse(
        UploadContext {
            service: upload.service,
            actor: upload.tenant,
        },
        upload,
    )
    .map(|_| ())
    .map_err(|_| UploadStatusReplyError::Invalid)
}
/// Decode an independently authenticated service reply with the complete original
/// upload binding. Confirmation is historical; copies are not publication leases.
/// # Errors
/// Rejects oversized/malformed data, changed original arguments and lookup refusals.
pub fn decode(
    upload: ReferenceUpload,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<UploadStatusResponse, UploadStatusReplyError> {
    check(upload)?;
    if bytes.len() > max.get() {
        return Err(UploadStatusReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(1_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(max.get())
        .set_full_error_message(false);
    let response: Result<UploadStatusResponse, UploadStatusFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| UploadStatusReplyError::Invalid)?;
    let response = response.map_err(UploadStatusReplyError::Remote)?;
    if response.upload != upload {
        return Err(UploadStatusReplyError::Binding);
    }
    Ok(response)
}
#[cfg(test)]
mod tests;
