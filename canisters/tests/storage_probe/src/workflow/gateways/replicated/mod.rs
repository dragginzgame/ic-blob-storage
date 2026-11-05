//! The production transport primitive, tested against a local query-only source.
use crate::ops::gateways::{
    replicated_failure, sync_failure,
    transport::{FixtureRegistry, attempt},
};
use blob_test_protocol::storage::{Failure, gateways::ReplicatedInput};
use ic_blob_storage::ops::caffeine::query::transport::replicated::ReplicatedGatewayQuery;
use ic_blob_storage::workflow::gateways::transport::GatewayQueryError;
use ic_blob_storage::workflow::gateways::transport::query_sync;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use std::num::{NonZeroU32, NonZeroUsize};
pub(crate) async fn run(context: UploadContext, input: ReplicatedInput) -> Result<(), Failure> {
    let retained = attempt(context, input.attempt)?;
    // Explicit bindings are adversarial fixture controls, not production ingress configuration.
    let transport =
        ReplicatedGatewayQuery::new(input.service, input.cashier, NonZeroU32::new(30).unwrap())
            .map_err(replicated_failure)?;
    let mut limits = super::limits();
    limits.max_bytes = NonZeroUsize::new(input.max_reply_bytes as usize).ok_or(Failure::Invalid)?;
    query_sync(
        &FixtureRegistry {
            callback_fault: input.attempt.callback_fault,
        },
        context,
        &retained,
        &transport,
        limits,
    )
    .await
    .map_err(|error| match error {
        GatewayQueryError::Registry(error) => sync_failure(error),
        GatewayQueryError::Transport(error) => replicated_failure(error),
    })
}
