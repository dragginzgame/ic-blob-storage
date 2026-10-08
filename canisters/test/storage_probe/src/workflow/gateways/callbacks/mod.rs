//! Local root-state observation through shared gateway authority, with no effects.
use crate::ops::gateways::callbacks;
use blob_test_protocol::storage::{
    Failure,
    gateways::{RootView, RootsInput},
};
use ic_blob_storage::policy::gateway::GatewayCallbackContext;
use ic_blob_storage::workflow::gateways::callbacks::GatewayCallbackError;
use ic_blob_storage::workflow::gateways::callbacks::observe_roots;
use ic_blob_storage_contracts::upload::binding::UploadContext;
pub(crate) fn roots(context: UploadContext, input: &RootsInput) -> Result<Vec<RootView>, Failure> {
    let (scope, batch) = callbacks::parse(input)?;
    let entries = callbacks::with_owners(|registry, uploads| {
        observe_roots(
            registry,
            uploads,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
            scope,
            &batch,
        )
    })
    .map_err(|error| match error {
        GatewayCallbackError::Access(error) => callbacks::access_failure(error),
        GatewayCallbackError::Registry(error) => callbacks::registry_failure(error),
        GatewayCallbackError::Uploads(error) => callbacks::upload_failure(error),
    })?;
    callbacks::present(entries)
}
