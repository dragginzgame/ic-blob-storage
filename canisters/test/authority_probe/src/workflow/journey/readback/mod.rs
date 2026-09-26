//! Actual local reads with authority rechecked after the inter-canister await.
use crate::ops;
use blob_test_protocol::journey::{JourneyFailure, readback::JourneyReadChunk};
use candid::Principal;
use ic_blob_storage::{
    model::catalog::admission::UploadRequest,
    policy::{
        gateway::{GatewayCallbackContext, assess_gateway_callback},
        liveness::assess_reference_liveness,
        tenant::TenantAccessContext,
    },
};

pub(crate) fn arm_callback_trap(context: TenantAccessContext, root: &[u8]) -> bool {
    if !super::super::is_operator(context) {
        return false;
    }
    let Ok(root) = ic_blob_storage::model::identity::ProviderRootHash::try_from(root) else {
        return false;
    };
    ops::journey::readback::arm_callback_trap(root)
}

pub(crate) async fn read(
    context: TenantAccessContext,
    root: &[u8],
    index: u64,
) -> Result<JourneyReadChunk, JourneyFailure> {
    let request = super::owned_request(context, root)?;
    let gateway = ops::read(|state| state.registry.gateways().principals().first().copied())
        .ok_or(JourneyFailure::Denied)?;
    authorize(context, request, gateway)?;
    let token = ops::journey::readback::begin(request, index, gateway)?;
    let response = ops::journey::readback::fetch(gateway, request, index).await;
    // Even a failed reply releases only its own slot. Invalidation does not allow
    // another response buffer while the old call is still outstanding.
    ops::journey::readback::finish(token)?;
    authorize(context, request, gateway)?;
    ops::journey::readback::verify(request, index, &response?)
}

fn authorize(
    context: TenantAccessContext,
    request: UploadRequest,
    gateway: Principal,
) -> Result<(), JourneyFailure> {
    ops::read(|state| {
        let journal = state
            .journey
            .catalog
            .confirmed()
            .get(request.object.root)
            .ok_or(JourneyFailure::InvalidPhase)?;
        let live = assess_reference_liveness(journal.lifecycle(), request.object.first, context)
            .map_err(|_| JourneyFailure::Denied)?;
        if !live {
            return Err(JourneyFailure::InvalidPhase);
        }
        assess_gateway_callback(
            request.object.first.object(),
            &state.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: gateway,
            },
        )
        .map_err(|_| JourneyFailure::Denied)
    })
}
