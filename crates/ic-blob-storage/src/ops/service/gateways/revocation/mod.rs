//! Boundary conversion around the authoritative durable revocation transition.
use super::{GatewayStoreError, StableGatewayRegistry};
use candid::Principal;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationFailure;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationRequest;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

pub(crate) fn revoke<M: Memory>(
    store: &mut StableGatewayRegistry<M>,
    context: UploadContext,
    input: GatewayRevocationRequest,
) -> Result<GatewayRevocationResponse, GatewayRevocationFailure> {
    if input.scope.namespace == 0
        || [Principal::anonymous(), Principal::management_canister()].contains(&input.gateway)
    {
        return Err(GatewayRevocationFailure::Invalid);
    }
    let config = store.inspection_configuration();
    let installed = StableGatewayRegistry::<M>::scope(config);
    if input.scope.service != installed.service()
        || input.scope.namespace != installed.namespace().get()
        || input.scope.cashier != installed.cashier()
        || input.scope.payment_account != config.bindings().payment_account
    {
        return Err(GatewayRevocationFailure::Binding);
    }
    let removed = store
        .remove(context, installed, input.gateway)
        .map_err(|error| match error {
            GatewayStoreError::Binding => GatewayRevocationFailure::Binding,
            GatewayStoreError::NotOperator => GatewayRevocationFailure::Denied,
            GatewayStoreError::Fenced => GatewayRevocationFailure::Fenced,
            _ => GatewayRevocationFailure::Internal,
        })?;
    Ok(GatewayRevocationResponse {
        request: input,
        removed,
    })
}
