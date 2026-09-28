//! Durable read-only gateway sync lifecycle using the canonical Cashier decoder.
//!
//! Transport remains host-owned: authenticate the source, bound buffering and
//! preserve original scope/operator context across the query. No automatic
//! transport, retries, update fallback or provider qualification occurs here.
pub mod callbacks;
pub mod transport;
use crate::{
    model::{gateway::registry::GatewayScope, service::upload::UploadContext},
    ops::{
        caffeine::{
            gateway::GatewayReplyLimits,
            query::{CashierQuery, CashierQueryRequest},
        },
        service::gateways::{
            StableGatewayRegistry,
            reply::{GatewaySyncReplyError, GatewaySyncRequest},
        },
    },
};
use ic_memory::ic_stable_structures::Memory;

/// Authenticate, construct the canonical query, then persist its pending identity.
/// Returning a request authorizes no paid effect. A host must not dispatch before
/// this synchronous transaction commits; lost output leaves a pending sync.
/// # Errors
/// Rejects authority, request construction, fencing or overlapping/exhausted sync.
/// # Panics
/// Stable write failures must propagate for IC rollback.
pub fn begin_sync<M: Memory>(
    store: &mut StableGatewayRegistry<M>,
    context: UploadContext,
    scope: GatewayScope,
) -> Result<GatewaySyncRequest, GatewaySyncReplyError> {
    store.inspect(context, scope)?;
    let request = CashierQueryRequest::new(scope.cashier(), CashierQuery::StorageGateways)?;
    let token = store.begin_sync(context, scope)?;
    Ok(GatewaySyncRequest {
        scope,
        token,
        request,
    })
}

/// Apply a trusted transport observation to its exact original pending query.
/// Failure preserves the attempt for explicit cancellation or a valid response;
/// success consumes it. This does not grant callback or read-session authority.
/// # Errors
/// Rejects wrong authority/fence, correlation and invalid or over-budget replies.
/// # Panics
/// Stable write failures must propagate for IC rollback.
pub fn complete_sync<M: Memory>(
    store: &mut StableGatewayRegistry<M>,
    context: UploadContext,
    attempt: &GatewaySyncRequest,
    response_scope: GatewayScope,
    bytes: &[u8],
    limits: GatewayReplyLimits,
) -> Result<(), GatewaySyncReplyError> {
    store.apply_reply(context, attempt, response_scope, bytes, limits)
}

/// Abandon only this exact read-only attempt; no paid-effect retry is implied.
/// # Errors
/// Rejects authority, fence or a stale attempt without changing history.
/// # Panics
/// Stable write failures must propagate for IC rollback.
pub fn cancel_sync<M: Memory>(
    store: &mut StableGatewayRegistry<M>,
    context: UploadContext,
    attempt: &GatewaySyncRequest,
) -> Result<(), GatewaySyncReplyError> {
    Ok(store.cancel_sync(context, attempt.scope, attempt.token)?)
}
