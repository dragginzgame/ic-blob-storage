//! Exact root/scope correlation preserves absence, uncertain history and restore fences.
use crate::dto::upload::discovery::{
    UploadDiscoveryFailure, UploadDiscoveryRequest, UploadDiscoveryResponse,
};
use std::num::NonZeroUsize;
use thiserror::Error;

/// Unusable discovery reply after independent service authentication.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadDiscoveryReplyError {
    /// Encoded response exceeds the caller's complete-input ceiling.
    #[error("discovery reply exceeds limit")]
    Limit,
    /// Malformed or over-budget Candid.
    #[error("invalid discovery reply")]
    Invalid,
    /// Echoed request or original content scope/root differs from the lookup.
    #[error("discovery reply binding mismatch")]
    Binding,
    /// Original operation/object/incarnation/reference or content length is zero.
    #[error("invalid discovery content identity")]
    Content,
    /// Authenticated service refusal, never an absent root.
    #[error("service refused content discovery: {0:?}")]
    Remote(UploadDiscoveryFailure),
}

/// Admit an exact bounded lookup reply without granting allocation or retry authority.
/// Absence, every retained lifecycle state and the recovery fence remain distinct.
/// The caller independently authenticates the service and checks expected file bytes.
/// # Errors
/// Refuses oversized/malformed replies, changed bindings and invalid content identities.
pub fn decode(
    request: UploadDiscoveryRequest,
    bytes: &[u8],
    max_reply_bytes: NonZeroUsize,
) -> Result<UploadDiscoveryResponse, UploadDiscoveryReplyError> {
    use UploadDiscoveryReplyError as E;
    if bytes.len() > max_reply_bytes.get() {
        return Err(E::Limit);
    }
    let limits = crate::decoding::limits(100_000, 1000, 64, Some(max_reply_bytes.get()));
    let response: Result<UploadDiscoveryResponse, UploadDiscoveryFailure> =
        candid::decode_one_with_config(bytes, &limits).map_err(|_| E::Invalid)?;
    let response = response.map_err(E::Remote)?;
    if response.request != request {
        return Err(E::Binding);
    }
    if let Some(content) = response.content {
        let original = content.request;
        if original.service != request.scope.service
            || original.tenant != request.scope.tenant
            || original.namespace != request.scope.namespace
            || original.root != request.root
        {
            return Err(E::Binding);
        }
        if original.upload == 0
            || original.object == 0
            || original.incarnation == 0
            || original.first_reference == 0
            || original.bytes == 0
        {
            return Err(E::Content);
        }
    }
    Ok(response)
}
#[cfg(test)]
mod tests;
