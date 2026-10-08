//! Operator scope conversion, exact cancellation and typed transport failures.
use super::{GatewayScope, GatewayStoreError, GatewaySyncError, StableGatewayRegistry};
use crate::ops::caffeine::gateway::GatewayReplyError;
use crate::ops::caffeine::query::reply::BoundGatewayReplyError;
use crate::ops::caffeine::query::reply::QueryReplyBindingError;
use crate::ops::caffeine::query::transport::replicated::ReplicatedQueryError;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncCancellation;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure as Failure;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

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
