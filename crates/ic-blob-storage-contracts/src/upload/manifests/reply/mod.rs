//! Bounded manifest replies after independent authentication of the selected service.
use crate::dto::upload::admission::UploadAdmissionRequest;
use crate::dto::upload::manifest::UploadManifestDeclaration;
use crate::dto::upload::manifest::UploadManifestFailure;
use crate::dto::upload::manifest::UploadManifestInspection;
use crate::dto::upload::manifest::UploadManifestMutation;
use crate::dto::upload::manifest::UploadManifestRequest;
use crate::dto::upload::manifest::UploadManifestResponse;
use crate::identity::ProviderRootHash;
use crate::identity::caffeine::manifest::CaffeineChunkHash;
use crate::identity::caffeine::manifest::CaffeineChunkManifest;
use crate::identity::caffeine::manifest::CaffeineManifestLimits;
use crate::upload::admission;
use candid::CandidType;
use candid::Deserialize;
use candid::de::DecoderConfig;
use candid::decode_one_with_config;
use std::num::NonZeroUsize;
use thiserror::Error;
/// Encoded response and declaration budgets, separate from platform ingress/buffer limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadManifestReplyLimits {
    /// Complete encoded Candid reply bound, checked before decoding.
    pub max_reply_bytes: NonZeroUsize,
    /// Content length, leaf count and original metadata bounds before conversion.
    pub declaration: CaffeineManifestLimits,
}
/// Unusable response, never authority to repeat an uncertain preparation/effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadManifestReplyError {
    /// Encoded response or raw declaration exceeds its budget.
    #[error("manifest reply exceeds limit")]
    Limit,
    /// Malformed data, inconsistent declaration or impossible mutation result.
    #[error("invalid manifest reply")]
    Invalid,
    /// Returned permission or leaves differ from the original request.
    #[error("manifest reply binding mismatch")]
    Binding,
    /// Authenticated service refusal.
    #[error("manifest refused: {0:?}")]
    Remote(UploadManifestFailure),
}
fn decode<T: CandidType + for<'de> Deserialize<'de>>(
    input: UploadAdmissionRequest,
    bytes: &[u8],
    limits: UploadManifestReplyLimits,
) -> Result<T, UploadManifestReplyError> {
    admission::parse_binding(input.upload.service, input)
        .map_err(|_| UploadManifestReplyError::Invalid)?;
    if bytes.len() > limits.max_reply_bytes.get() {
        return Err(UploadManifestReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(2_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(limits.max_reply_bytes.get())
        .set_full_error_message(false);
    let result: Result<T, UploadManifestFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| UploadManifestReplyError::Invalid)?;
    result.map_err(UploadManifestReplyError::Remote)
}
/// Validate a saved declaration against the original permission before dispatch.
/// Matching metadata/leaves establish structure, never provider completion.
/// # Errors
/// Rejects oversized declarations, invalid original metadata and root/length mismatch.
pub fn validate_declaration(
    input: UploadAdmissionRequest,
    declaration: &UploadManifestDeclaration,
    limits: CaffeineManifestLimits,
) -> Result<(), UploadManifestReplyError> {
    super::bounds(declaration, limits).map_err(|_| UploadManifestReplyError::Limit)?;
    if input.upload.bytes > limits.max_content_bytes.get() {
        return Err(UploadManifestReplyError::Limit);
    }
    let headers = super::headers(declaration);
    crate::upload::metadata::validate_upload_metadata(
        &headers,
        input.upload.bytes,
        limits.max_headers.get(),
        limits.max_header_bytes.get(),
    )
    .map_err(|_| UploadManifestReplyError::Invalid)?;
    let chunks = declaration
        .chunks
        .iter()
        .map(|c| CaffeineChunkHash::try_from(c.as_slice()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| UploadManifestReplyError::Invalid)?;
    CaffeineChunkManifest::new(
        ProviderRootHash::try_from(input.upload.root.as_slice())
            .map_err(|_| UploadManifestReplyError::Invalid)?,
        input.upload.bytes,
        &chunks,
        &headers,
        limits,
    )
    .map_err(|_| UploadManifestReplyError::Invalid)?;
    Ok(())
}
fn bound(
    input: UploadAdmissionRequest,
    response: &UploadManifestResponse,
    limits: UploadManifestReplyLimits,
) -> Result<(), UploadManifestReplyError> {
    if response.permission != input {
        return Err(UploadManifestReplyError::Binding);
    }
    if let UploadManifestInspection::Prepared(declaration) = &response.manifest {
        validate_declaration(input, declaration, limits.declaration)?;
    }
    Ok(())
}
/// Recover an independently authenticated original declaration within explicit budgets.
/// Root consistency is not provider completion or verified file bytes.
/// # Errors
/// Refuses malformed, oversized, changed-permission or inconsistent declarations.
pub fn inspection(
    input: UploadAdmissionRequest,
    bytes: &[u8],
    limits: UploadManifestReplyLimits,
) -> Result<UploadManifestResponse, UploadManifestReplyError> {
    let response = decode(input, bytes, limits)?;
    bound(input, &response, limits)?;
    Ok(response)
}
/// Validate a preparation acknowledgment. Equivalent metadata retries return the
/// original spelling/order; prepared leaves and both declarations must match the root.
/// # Errors
/// Also rejects unprepared mutation responses or leaves unlike the submitted intent.
pub fn mutation(
    input: &UploadManifestRequest,
    bytes: &[u8],
    limits: UploadManifestReplyLimits,
) -> Result<UploadManifestMutation, UploadManifestReplyError> {
    let response: UploadManifestMutation = decode(input.permission, bytes, limits)?;
    bound(input.permission, &response.observation, limits)?;
    validate_declaration(input.permission, &input.declaration, limits.declaration)?;
    let UploadManifestInspection::Prepared(retained) = &response.observation.manifest else {
        return Err(UploadManifestReplyError::Invalid);
    };
    if retained.chunks != input.declaration.chunks {
        return Err(UploadManifestReplyError::Binding);
    }
    Ok(response)
}
