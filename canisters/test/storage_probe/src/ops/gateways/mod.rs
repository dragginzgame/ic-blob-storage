//! Passive conversion and explicit local-only gateway journal controls.
pub(crate) mod callbacks;
pub(crate) mod transport;

pub(crate) fn replicated_failure(
    error: ic_blob_storage::ops::caffeine::query::transport::replicated::ReplicatedQueryError,
) -> blob_test_protocol::storage::Failure {
    use ic_blob_storage::ops::caffeine::query::transport::replicated::ReplicatedQueryError as E;
    match error {
        E::Binding => Failure::Binding,
        E::Execution => Failure::Phase,
        E::ReplyTooLarge => Failure::Capacity,
        E::NotEnqueued | E::Rejected(_) => Failure::Transport,
        E::Method | E::InvalidPrincipal | E::TimeoutOutOfRange => Failure::Invalid,
    }
}
use super::{STATE, TRAP_WRITE, UploadContext};
use blob_test_protocol::storage::{
    Failure, WriteFault,
    gateways::{Scope, View},
};
use ic_blob_storage::{
    model::gateway::{
        GatewayListError,
        registry::{GatewayScope, GatewaySyncError},
    },
    ops::service::gateways::GatewayStoreError,
};
use std::num::NonZeroU128;
pub(crate) fn scope(input: Scope) -> Result<GatewayScope, Failure> {
    GatewayScope::new(
        input.service,
        NonZeroU128::new(input.namespace).ok_or(Failure::Invalid)?,
        input.cashier,
    )
    .map_err(|_| Failure::Invalid)
}
pub(crate) fn failure(error: GatewayStoreError) -> Failure {
    match error {
        GatewayStoreError::NotOperator => Failure::Denied,
        GatewayStoreError::Binding | GatewayStoreError::Sync(GatewaySyncError::WrongScope) => {
            Failure::Binding
        }
        GatewayStoreError::Fenced => Failure::Fenced,
        GatewayStoreError::Sync(GatewaySyncError::StaleSync) => Failure::Conflict,
        GatewayStoreError::Sync(GatewaySyncError::SyncInProgress) => Failure::Phase,
        GatewayStoreError::List(
            GatewayListError::TooManyEntries { .. } | GatewayListError::TooManyUnique { .. },
        )
        | GatewayStoreError::Sync(
            GatewaySyncError::SequenceExhausted
            | GatewaySyncError::InvalidList(
                GatewayListError::TooManyEntries { .. } | GatewayListError::TooManyUnique { .. },
            ),
        ) => Failure::Capacity,
        _ => Failure::Invalid,
    }
}
pub(super) fn initialize(
    memory: super::ProbeMemory,
    config: &ic_blob_storage::model::service::configuration::ServiceConfiguration,
    restored: bool,
) -> ic_blob_storage::ops::service::gateways::StableGatewayRegistry<super::ProbeMemory> {
    use ic_blob_storage::ops::service::gateways::StableGatewayRegistry;
    if restored {
        StableGatewayRegistry::open(memory, *config)
    } else {
        StableGatewayRegistry::install(memory, *config)
    }
    .unwrap()
}
pub(crate) fn inspect(context: UploadContext, input: Scope) -> Result<View, Failure> {
    let scope = scope(input)?;
    let value = STATE
        .with_borrow(|state| state.as_ref().unwrap().gateways.inspect(context, scope))
        .map_err(failure)?;
    Ok(View {
        members: value.principals,
        last_sequence: value.sync.last_sequence,
        pending_sequence: value.sync.pending_sequence,
        fenced: value.fenced,
    })
}

pub(crate) fn sync_failure(
    error: ic_blob_storage::ops::service::gateways::reply::GatewaySyncReplyError,
) -> Failure {
    use ic_blob_storage::ops::{
        caffeine::{gateway::GatewayReplyError, query::reply::BoundGatewayReplyError},
        service::gateways::reply::GatewaySyncReplyError,
    };
    match error {
        GatewaySyncReplyError::Store(error) => failure(error),
        GatewaySyncReplyError::Reply(BoundGatewayReplyError::Binding(_)) => Failure::Binding,
        GatewaySyncReplyError::Reply(BoundGatewayReplyError::Reply(GatewayReplyError::Sync(
            error,
        ))) => failure(GatewayStoreError::Sync(error)),
        GatewaySyncReplyError::Reply(BoundGatewayReplyError::Reply(
            GatewayReplyError::ReplyTooLarge,
        )) => Failure::Capacity,
        _ => Failure::Invalid,
    }
}
// Synchronous state access only; orchestration stays in workflow.
pub(crate) fn with_registry<R>(
    fault: bool,
    change: impl FnOnce(
        &mut ic_blob_storage::ops::service::gateways::StableGatewayRegistry<super::ProbeMemory>,
        &mut Vec<ic_blob_storage::ops::service::gateways::reply::GatewaySyncRequest>,
    ) -> R,
) -> R {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().unwrap();
        TRAP_WRITE.set(fault.then_some(WriteFault::Gateways));
        let result = change(&mut state.gateways, &mut state.gateway_attempts);
        TRAP_WRITE.set(None);
        result
    })
}
