//! Bounded interpretation of a descriptor from an independently authenticated service.
use crate::download::scope::CaffeineDownloadScope;
use crate::dto::download::DownloadFailure;
use crate::dto::download::DownloadRequest;
use crate::dto::download::DownloadResponse;
use crate::identity::caffeine::CaffeineHeader;
use crate::upload::binding::UploadContext;
use crate::upload::metadata::validate_upload_metadata;
use std::num::NonZeroU64;
use std::num::NonZeroUsize;
use thiserror::Error;
#[cfg(test)]
mod tests;
// One validation implementation for owned async results and input-backed native views.
trait MetadataText {
    fn as_str(&self) -> &str;
}
impl MetadataText for String {
    fn as_str(&self) -> &str {
        String::as_str(self)
    }
}
impl<const N: usize> MetadataText for crate::wire::TextRef<'_, N> {
    fn as_str(&self) -> &str {
        crate::wire::TextRef::as_str(self)
    }
}

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
    /// Malformed Wire, invalid identity or inconsistent metadata.
    #[error("invalid descriptor reply")]
    Invalid,
    /// Typed refusal by the authenticated service.
    #[error("service refused descriptor: {0:?}")]
    Remote(DownloadFailure),
}
/// Validate passive input before an independently authenticated adapter call.
/// # Errors
/// Rejects malformed or differently bound input; grants no dispatch authority.
pub fn check_request(
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
    decode_as(request, scope, bytes, limits)
}
/// Decode metadata as input-backed views, without copying its project or headers.
/// The caller retains the authenticated frame for the complete view lifetime.
/// # Errors
/// Preserves the owned decoder's bounds, binding checks and typed refusals.
pub fn decode_borrowed<'de>(
    request: DownloadRequest,
    scope: &CaffeineDownloadScope,
    bytes: &'de [u8],
    limits: DownloadReplyLimits,
) -> Result<
    DownloadResponse<crate::wire::TextRef<'de, { crate::wire::MAX_MESSAGE_BYTES }>>,
    DownloadReplyError,
> {
    decode_as(request, scope, bytes, limits)
}
fn decode_as<'de, T: MetadataText + serde::Deserialize<'de>>(
    request: DownloadRequest,
    scope: &CaffeineDownloadScope,
    bytes: &'de [u8],
    limits: DownloadReplyLimits,
) -> Result<DownloadResponse<T>, DownloadReplyError> {
    check_request(request, scope)?;
    if bytes.len() > limits.max_reply_bytes.get() {
        return Err(DownloadReplyError::Limit);
    }
    let response: Result<DownloadResponse<T>, DownloadFailure> = crate::wire::decode_response(
        "blob_download_descriptor",
        bytes,
        limits.max_reply_bytes.get(),
    )
    .map_err(|_| DownloadReplyError::Invalid)?;
    let response = response.map_err(DownloadReplyError::Remote)?;
    if response.request != request
        || response.owner != scope.owner()
        || response.project.as_str() != scope.project()
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
            name: h.name.as_str(),
            value: h.value.as_str(),
        })
        .collect();
    validate_upload_metadata(
        &headers,
        response.bytes,
        limits.max_headers.get(),
        limits.max_header_bytes.get(),
    )
    .map_err(|error| match error {
        crate::upload::metadata::UploadMetadataError::HeaderCount
        | crate::upload::metadata::UploadMetadataError::HeaderBytes => DownloadReplyError::Limit,
        _ => DownloadReplyError::Invalid,
    })?;
    Ok(response)
}
