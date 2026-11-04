//! Bounded decoding and application of Cashier gateway-list replies.
//!
//! Selected schema: `storage_gateway_list_v1 : () -> (vec principal) query` in
//! `docs/evidence/caffeine-cashier.did`, SHA-256
//! `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
//! No transport, caller authentication, persistence or automatic sync occurs here.

use crate::ops::caffeine::decoding::CaffeineReplyLimits;

use ic_platform::Principal;
use thiserror::Error;

use crate::model::gateway::registry::{
    GatewayRegistry, GatewayScope, GatewaySyncError, GatewaySyncToken,
};

/// Decode and apply a reply to the exact pending sync, without partial mutation.
///
/// The caller must establish the response's Cashier and supply trusted transport
/// scope; the payload cannot assert its own authority. Scope and token checks
/// precede decoding. Byte/work/type bounds constrain decoding; registry raw and
/// distinct entry limits validate the decoded list before replacement. They do
/// not bound transport buffering or prevent decoding allocation in advance.
///
/// On failure the pending attempt remains available for explicit cancellation or
/// a valid response. Success consumes it. Neither result authorizes paid effects.
/// # Errors
/// Rejects wrong/stale correlation, oversized, malformed or over-budget Candid,
/// and invalid membership candidates, preserving all registry state on failure.
pub fn apply_gateway_sync_reply(
    registry: &mut GatewayRegistry,
    token: GatewaySyncToken,
    response_scope: GatewayScope,
    bytes: &[u8],
    limits: CaffeineReplyLimits,
) -> Result<(), GatewayReplyError> {
    registry.check_sync(token, response_scope)?;
    let principals: Vec<ic_platform::principal::ExternalPrincipal> = limits.decode(
        bytes,
        GatewayReplyError::ReplyTooLarge,
        GatewayReplyError::InvalidReply,
    )?;
    let principals = principals
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Principal>>();
    registry.apply_sync(token, response_scope, &principals)?;
    Ok(())
}

/// A gateway-list reply rejected without changing membership or pending state.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GatewayReplyError {
    /// Byte limit exceeded before decoding.
    #[error("gateway reply exceeds byte limit")]
    ReplyTooLarge,
    /// Malformed, incompatible or above configured decoder work/type bounds.
    #[error("invalid or over-budget gateway reply")]
    InvalidReply,
    /// Registry correlation or membership validation failed.
    #[error(transparent)]
    Sync(#[from] GatewaySyncError),
}

#[cfg(test)]
mod tests;
