//! Boundary conversion around the authoritative durable revocation transition.
use super::{GatewayStoreError, StableGatewayRegistry};
use crate::{
    dto::gateway::{GatewayRevocationFailure, GatewayRevocationRequest, GatewayRevocationResponse},
    model::service::upload::UploadContext,
};
use candid::Principal;
use ic_memory::ic_stable_structures::Memory;

/// Canonical explicit operator update. Linking exports no endpoint.
pub const GATEWAY_REVOCATION_METHOD: &str = "blob_revoke_gateway";
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
