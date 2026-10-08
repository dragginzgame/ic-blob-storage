//! Actual local reads with authority rechecked after the inter-canister await.
use crate::model::content::ContentRequest;
use crate::ops;
use blob_test_protocol::journey::{
    JourneyFailure,
    readback::{JourneyReadChunk, ReadExecutionProfile},
};
use candid::Principal;
use ic_blob_storage::policy::{
    gateway::{GatewayCallbackContext, assess_gateway_callback},
    liveness::assess_reference_liveness,
    tenant::TenantAccessContext,
};
use ops::journey::readback::resources::Measurement;

pub(crate) fn arm_callback_trap(context: TenantAccessContext, root: &[u8]) -> bool {
    if !super::super::is_operator(context) {
        return false;
    }
    let Ok(root) = ic_blob_storage_contracts::identity::ProviderRootHash::try_from(root) else {
        return false;
    };
    ops::journey::readback::arm_callback_trap(root)
}

pub(crate) async fn read(
    context: TenantAccessContext,
    root: &[u8],
    index: u64,
) -> Result<JourneyReadChunk, JourneyFailure> {
    let mut measurement = Measurement::begin(context.actor);
    let result = measured_read(context, root, index, &mut measurement).await;
    measurement.complete(&result);
    result
}

pub(crate) fn resources(
    context: TenantAccessContext,
) -> Result<Option<ReadExecutionProfile>, JourneyFailure> {
    // These volatile counters cannot describe the retained inspection owner.
    if ops::archive::recovery::is_fenced() {
        return Err(JourneyFailure::Denied);
    }
    if !super::super::is_operator(context) {
        return Err(JourneyFailure::Denied);
    }
    Ok(ops::journey::readback::resources::latest())
}

async fn measured_read(
    context: TenantAccessContext,
    root: &[u8],
    index: u64,
    measurement: &mut Measurement,
) -> Result<JourneyReadChunk, JourneyFailure> {
    let request = super::owned_request(context, root)?;
    let gateway = ops::read(|state| state.registry.gateways().principals().first().copied())
        .ok_or(JourneyFailure::Denied)?;
    authorize(context, request, gateway)?;
    let token = ops::journey::readback::begin(request, index, gateway)?;
    measurement.sending();
    let response = ops::journey::readback::fetch(gateway, request, index).await;
    measurement.replied(&response);
    // Even a failed reply releases only its own slot. Invalidation does not allow
    // another response buffer while the old call is still outstanding.
    ops::journey::readback::finish(token)?;
    authorize(context, request, gateway)?;
    ops::journey::readback::verify(request, index, &response?, measurement)
}

fn authorize(
    context: TenantAccessContext,
    request: ContentRequest,
    gateway: Principal,
) -> Result<(), JourneyFailure> {
    ops::read(|state| {
        let journal = state
            .journey
            .catalog
            .confirmed()
            .get(request.upload.object.root)
            .ok_or(JourneyFailure::InvalidPhase)?;
        let live =
            assess_reference_liveness(journal.lifecycle(), request.upload.object.first, context)
                .map_err(|_| JourneyFailure::Denied)?;
        if !live {
            return Err(JourneyFailure::InvalidPhase);
        }
        assess_gateway_callback(
            request.upload.object.first.object(),
            &state.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: gateway,
            },
        )
        .map_err(|_| JourneyFailure::Denied)
    })
}
