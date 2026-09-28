//! Bounded interpretation of a descriptor from an independently authenticated service.
use crate::{
    dto::download::{DownloadFailure, DownloadRequest, DownloadResponse},
    model::{
        identity::caffeine::CaffeineHeader,
        service::{
            read::download::CaffeineDownloadScope,
            upload::{UploadContext, manifest::validate_upload_metadata},
        },
    },
};
use candid::{de::DecoderConfig, decode_one_with_config};
use std::num::{NonZeroU64, NonZeroUsize};
use thiserror::Error;
#[cfg(test)]
mod tests;
/// Trusted application decode/metadata budgets, independent of platform buffering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DownloadReplyLimits {
    /// Maximum complete encoded reply before application decoding.
    pub max_reply_bytes: NonZeroUsize,
    /// Maximum declared body; no body is allocated or read here.
    pub max_content_bytes: NonZeroU64,
    /// Maximum original metadata entries.
    pub max_headers: NonZeroUsize,
    /// Maximum metadata text/framing bytes under the shared admission validator.
    pub max_header_bytes: NonZeroUsize,
}
/// Failed response authentication binding, decoding, bounds or service refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum DownloadReplyError {
    /// Original request or returned service/reference/project differs.
    #[error("descriptor binding mismatch")]
    Binding,
    /// Encoded bytes, declared content or metadata exceed trusted limits.
    #[error("descriptor exceeds limit")]
    Limit,
    /// Malformed Candid, invalid identity or inconsistent metadata.
    #[error("invalid descriptor reply")]
    Invalid,
    /// Typed refusal by the authenticated service.
    #[error("service refused descriptor: {0:?}")]
    Remote(DownloadFailure),
}
pub(crate) fn check_request(
    request: DownloadRequest,
    scope: &CaffeineDownloadScope,
) -> Result<(), DownloadReplyError> {
    super::parse(
        UploadContext {
            service: request.service,
            actor: request.tenant,
        },
        request,
    )
    .map_err(|_| DownloadReplyError::Invalid)?;
    if request.service != scope.owner() || request.namespace != scope.namespace().get() {
        return Err(DownloadReplyError::Binding);
    }
    Ok(())
}
/// Validate only after the host authenticates the selected service and its reply.
/// Decoding alone does not supply that provenance. Returned metadata is a snapshot,
/// not a publication lease; this operation never selects an HTTP URL or fetches bytes.
/// # Errors
/// Rejects oversized/malformed replies, changed bindings, service failures and
/// metadata inconsistent with the maintained upload declaration contract.
pub fn decode(
    request: DownloadRequest,
    scope: &CaffeineDownloadScope,
    bytes: &[u8],
    limits: DownloadReplyLimits,
) -> Result<DownloadResponse, DownloadReplyError> {
    check_request(request, scope)?;
    if bytes.len() > limits.max_reply_bytes.get() {
        return Err(DownloadReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(1_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(limits.max_reply_bytes.get())
        .set_full_error_message(false);
    let response: Result<DownloadResponse, DownloadFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| DownloadReplyError::Invalid)?;
    let response = response.map_err(DownloadReplyError::Remote)?;
    if response.request != request
        || response.owner != scope.owner()
        || response.project != scope.project()
    {
        return Err(DownloadReplyError::Binding);
    }
    if response.bytes == 0
        || response.bytes > limits.max_content_bytes.get()
        || response.headers.len() > limits.max_headers.get()
    {
        return Err(DownloadReplyError::Limit);
    }
    let headers: Vec<_> = response
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect();
    validate_upload_metadata(
        &headers,
        response.bytes,
        limits.max_headers.get(),
        limits.max_header_bytes.get(),
    )
    .map_err(|error| match error {
        crate::model::service::upload::manifest::UploadMetadataError::HeaderCount
        | crate::model::service::upload::manifest::UploadMetadataError::HeaderBytes => {
            DownloadReplyError::Limit
        }
        _ => DownloadReplyError::Invalid,
    })?;
    Ok(response)
}
