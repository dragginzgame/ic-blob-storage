//! Explicit bounded refresh with durable pending identity and exact cancellation.
use crate::ops::caffeine::gateway::GatewayReplyLimits;
use crate::ops::caffeine::query::transport::replicated::ReplicatedGatewayQuery;
use crate::ops::service::gateways::StableGatewayRegistry;
use crate::ops::service::gateways::access::GatewayRegistryAccess;
use crate::ops::service::gateways::sync;
use crate::workflow::gateways::transport::GatewayQueryError;
use crate::workflow::gateways::transport::query_sync;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncCancellation;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncResponse;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU32;

/// Persist a pending sync then make one replicated call to the installed Cashier.
/// The host supplies actual operator/service, bounded limits and timeout. No store
/// borrow crosses the await. The original identity is rechecked before applying
/// membership; later revocation/cancellation wins over a delayed response.
///
/// No attached cycles, retries, timers or fallback transport. IC fees still apply.
/// Failed transport/decoding leaves the exact pending sync visible in local status;
/// cancel it explicitly before choosing another refresh. Success does not qualify
/// deployed Caffeine semantics, callback authority, provider credit or recovery.
/// # Errors
/// Rejects configuration/authority/fence/overlap, transport and invalidated replies.
/// # Panics
/// Host stable-write traps must propagate for IC transaction rollback.
pub async fn refresh<H: GatewayRegistryAccess>(
    host: &H,
    context: UploadContext,
    input: OperatorScope,
    timeout: NonZeroU32,
    limits: GatewayReplyLimits,
) -> Result<GatewaySyncResponse, GatewaySyncFailure> {
    let transport = ReplicatedGatewayQuery::new(context.service, input.cashier, timeout)
        .map_err(sync::transport_failure)?;
    let (attempt, sequence) = host.with_gateway_registry(|store| {
        let scope = sync::scope(store, input)?;
        let attempt = super::begin_sync(store, context, scope).map_err(sync::reply_failure)?;
        let sequence = attempt.sequence();
        Ok::<_, GatewaySyncFailure>((attempt, sequence))
    })?;
    query_sync(host, context, &attempt, &transport, limits)
        .await
        .map_err(|error| match error {
            GatewayQueryError::Registry(error) => sync::reply_failure(error),
            GatewayQueryError::Transport(error) => sync::transport_failure(error),
        })?;
    Ok(GatewaySyncResponse {
        scope: input,
        sequence,
    })
}
/// Cancel only the observed pending read-only identity; no token comes from ingress.
/// Retains allocated sequence history, membership and read generations. A late
/// callback cannot apply afterward. Does not cancel payments or release read slots.
/// # Errors
/// Rejects malformed identity, scope/authority, restore fence or stale sequence.
/// # Panics
/// Host stable-write traps must propagate for IC rollback.
pub fn cancel<M: Memory>(
    store: &mut StableGatewayRegistry<M>,
    context: UploadContext,
    input: GatewaySyncCancellation,
) -> Result<(), GatewaySyncFailure> {
    sync::cancel(store, context, input)
}
