//! Labelled local observations exercise the shared durable decoder workflow.
pub(crate) mod callbacks;
pub(crate) mod replicated;
pub(crate) mod transport;
use crate::ops::gateways::{failure, scope, sync_failure, with_registry};
use blob_test_protocol::storage::{
    Failure,
    gateways::{Action, Command, Outcome},
};
use ic_blob_storage::{
    model::{gateway::membership::GatewayAddOutcome, service::upload::UploadContext},
    ops::caffeine::gateway::GatewayReplyLimits,
    workflow::gateways::{begin_sync, cancel_sync, complete_sync},
};

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
            Action::Remove(principal) => store
                .remove(context, installed, principal)
                .map(Outcome::Changed)
                .map_err(failure),
            Action::Begin => {
                if attempts.len() == 16 {
                    return Err(Failure::Capacity);
                }
                let attempt = begin_sync(store, context, installed).map_err(sync_failure)?;
                attempts.push(attempt);
                Ok(Outcome::Begun(attempts.len() as u64))
            }
            Action::Apply {
                token,
                source,
                reply,
            } => {
                let attempt = token
                    .checked_sub(1)
                    .and_then(|v| usize::try_from(v).ok())
                    .and_then(|i| attempts.get(i))
                    .ok_or(Failure::Unknown)?;
                complete_sync(store, context, attempt, scope(source)?, &reply, limits())
                    .map(|()| Outcome::Applied)
                    .map_err(sync_failure)
            }
            Action::Cancel(token) => {
                let attempt = token
                    .checked_sub(1)
                    .and_then(|v| usize::try_from(v).ok())
                    .and_then(|i| attempts.get(i))
                    .ok_or(Failure::Unknown)?;
                cancel_sync(store, context, attempt)
                    .map(|()| Outcome::Applied)
                    .map_err(sync_failure)
            }
        }
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
