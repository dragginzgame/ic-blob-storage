//! Operator scope conversion, exact cancellation and typed transport failures.
use super::{GatewayScope, GatewayStoreError, GatewaySyncError, StableGatewayRegistry};
use crate::{
    dto::{
        gateway::sync::{GatewaySyncCancellation, GatewaySyncFailure as Failure},
        operator::OperatorScope,
    },
    model::service::upload::UploadContext,
    ops::caffeine::{
        gateway::GatewayReplyError,
        query::{
            reply::{BoundGatewayReplyError, QueryReplyBindingError},
            transport::replicated::ReplicatedQueryError,
        },
    },
};
use ic_memory::ic_stable_structures::Memory;

/// Explicit operator refresh update; linking exports no endpoint.
pub const GATEWAY_SYNC_METHOD: &str = "blob_sync_gateways";
/// Exact local read-only cancellation update; linking exports no endpoint.
pub const GATEWAY_SYNC_CANCEL_METHOD: &str = "blob_cancel_gateway_sync";
pub(crate) fn scope<M: Memory>(
    store: &StableGatewayRegistry<M>,
    input: OperatorScope,
) -> Result<GatewayScope, Failure> {
    if input.namespace == 0 {
        return Err(Failure::Invalid);
    }
    let config = store.inspection_configuration();
    let expected = OperatorScope {
        service: config.bindings().service,
        namespace: config.bindings().namespace.get(),
        cashier: config.billing().cashier(),
        payment_account: config.bindings().payment_account,
    };
    if input != expected {
        return Err(Failure::Binding);
    }
    Ok(StableGatewayRegistry::<M>::scope(config))
}
pub(crate) fn cancel<M: Memory>(
    store: &mut StableGatewayRegistry<M>,
    context: UploadContext,
    input: GatewaySyncCancellation,
) -> Result<(), Failure> {
    if input.sequence == 0 {
        return Err(Failure::Invalid);
    }
    let scope = scope(store, input.scope)?;
    store
        .mutate(context, scope, |registry| {
            registry
                .cancel_observed_sync(input.sequence)
                .map_err(GatewayStoreError::Sync)
        })
        .map_err(store_failure)
}
pub(crate) fn store_failure(error: GatewayStoreError) -> Failure {
    match error {
        GatewayStoreError::Binding => Failure::Binding,
        GatewayStoreError::NotOperator => Failure::Denied,
        GatewayStoreError::Fenced => Failure::Fenced,
        GatewayStoreError::Sync(error) => sync_failure(error),
        _ => Failure::Internal,
    }
}
fn sync_failure(error: GatewaySyncError) -> Failure {
    match error {
        GatewaySyncError::WrongScope => Failure::Binding,
        GatewaySyncError::StaleSync => Failure::Conflict,
        GatewaySyncError::SyncInProgress => Failure::Busy,
        GatewaySyncError::SequenceExhausted => Failure::Exhausted,
        GatewaySyncError::InvalidList(_) => Failure::InvalidReply,
    }
}
pub(crate) fn reply_failure(error: super::reply::GatewaySyncReplyError) -> Failure {
    use super::reply::GatewaySyncReplyError;
    match error {
        GatewaySyncReplyError::Store(error) => store_failure(error),
        GatewaySyncReplyError::Request(_) => Failure::Invalid,
        GatewaySyncReplyError::Reply(BoundGatewayReplyError::Binding(
            QueryReplyBindingError::SourceMismatch | QueryReplyBindingError::MethodMismatch,
        )) => Failure::Binding,
        GatewaySyncReplyError::Reply(BoundGatewayReplyError::Reply(error)) => match error {
            GatewayReplyError::ReplyTooLarge => Failure::ReplyTooLarge,
            GatewayReplyError::InvalidReply => Failure::InvalidReply,
            GatewayReplyError::Sync(error) => sync_failure(error),
        },
    }
}
pub(crate) fn transport_failure(error: ReplicatedQueryError) -> Failure {
    match error {
        ReplicatedQueryError::Binding => Failure::Binding,
        ReplicatedQueryError::ReplyTooLarge => Failure::ReplyTooLarge,
        ReplicatedQueryError::NotEnqueued => Failure::NotEnqueued,
        ReplicatedQueryError::Rejected(code) => Failure::Rejected(code),
        _ => Failure::Invalid,
    }
}
