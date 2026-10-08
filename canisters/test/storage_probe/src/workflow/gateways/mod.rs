//! Labelled local observations exercise the shared durable decoder workflow.
pub(crate) mod callbacks;
pub(crate) mod replicated;
pub(crate) mod transport;
use crate::ops::gateways::{failure, scope, sync_failure, with_registry};
use blob_test_protocol::storage::{
    Failure,
    gateways::{Action, Command, Outcome},
};
use ic_blob_storage::model::gateway::membership::GatewayAddOutcome;
use ic_blob_storage::ops::caffeine::gateway::GatewayReplyLimits;
use ic_blob_storage::workflow::gateways::begin_sync;
use ic_blob_storage::workflow::gateways::complete_sync;
use ic_blob_storage_contracts::upload::binding::UploadContext;

pub(crate) fn apply(context: UploadContext, input: Command) -> Result<Outcome, Failure> {
    let installed = scope(input.scope)?;
    with_registry(input.fault, |store, attempts| {
        if store.inspect(context, installed).map_err(failure)?.fenced {
            return Err(Failure::Fenced);
        }
        match input.action {
            Action::Add(principal) => store
                .add(context, installed, principal)
                .map(|v| Outcome::Changed(v == GatewayAddOutcome::Added))
                .map_err(failure),
            Action::Begin => {
                if attempts.len() == 16 {
                    return Err(Failure::Capacity);
                }
                let attempt = begin_sync(store, context, installed).map_err(sync_failure)?;
                let sequence = attempt.sequence();
                attempts.push(attempt);
                Ok(Outcome::Begun(sequence))
            }
            Action::Apply {
                token,
                source,
                reply,
            } => {
                let attempt = attempts
                    .iter()
                    .find(|attempt| attempt.sequence() == token)
                    .ok_or(Failure::Unknown)?;
                complete_sync(store, context, attempt, scope(source)?, &reply, limits())
                    .map(|()| Outcome::Applied)
                    .map_err(sync_failure)
            }
        }
    })
}

pub(crate) fn revoke(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::gateway::GatewayRevocationRequest,
    fault: bool,
) -> Result<
    ic_blob_storage_contracts::dto::gateway::GatewayRevocationResponse,
    ic_blob_storage_contracts::dto::gateway::GatewayRevocationFailure,
> {
    with_registry(fault, |store, _| {
        ic_blob_storage::workflow::gateways::revocation::revoke(store, context, input)
    })
}

pub(super) fn limits() -> GatewayReplyLimits {
    GatewayReplyLimits {
        // Leave room for the command inside the 4 KiB ingress bound.
        max_bytes: 2048.try_into().unwrap(),
        decoding_quota: 100_000.try_into().unwrap(),
        skipping_quota: 1000.try_into().unwrap(),
        max_type_entries: 32.try_into().unwrap(),
    }
}

pub(crate) async fn refresh(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::operator::OperatorScope,
) -> Result<
    ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncResponse,
    ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure,
> {
    ic_blob_storage::workflow::gateways::sync::refresh(
        &crate::ops::gateways::transport::FixtureRegistry {
            callback_fault: false,
        },
        context,
        input,
        30.try_into().unwrap(),
        limits(),
    )
    .await
}
pub(crate) fn cancel_observed(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncCancellation,
    fault: bool,
) -> Result<(), ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure> {
    with_registry(fault, |store, _| {
        ic_blob_storage::workflow::gateways::sync::cancel(store, context, input)
    })
}
