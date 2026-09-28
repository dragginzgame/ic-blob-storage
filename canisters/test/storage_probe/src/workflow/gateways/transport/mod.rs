//! Shared async handler against a labelled local source.
use crate::ops::gateways::{
    sync_failure,
    transport::{FixtureRegistry, LocalQuery, attempt},
};
use blob_test_protocol::storage::{Failure, gateways::TransportInput};
use ic_blob_storage::{
    model::service::upload::UploadContext,
    workflow::gateways::transport::{GatewayQueryError, query_sync},
};

pub(crate) async fn run(context: UploadContext, input: TransportInput) -> Result<(), Failure> {
    let retained = attempt(context, input)?;
    query_sync(
        &FixtureRegistry {
            callback_fault: input.callback_fault,
        },
        context,
        &retained,
        &LocalQuery,
        super::limits(),
    )
    .await
    .map_err(|error| match error {
        GatewayQueryError::Registry(error) => sync_failure(error),
        GatewayQueryError::Transport(error) => error,
    })
}
