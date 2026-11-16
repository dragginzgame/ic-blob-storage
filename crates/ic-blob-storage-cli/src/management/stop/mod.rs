//! IC stop semantics: controller admission and status observation while Blob cannot answer.

use super::{
    Context,
    status::{self, LIMITS},
};
use ic_platform_legacy::{
    agent::{RequestId, RequestStatusResponse},
    candid_codec::decoder::{Arity, decode_args},
    management_canister_types::CanisterStatusType,
};

pub(super) fn argument(context: &Context) -> Result<Vec<u8>, String> {
    status::argument(context.canister)
}

pub(super) async fn status(
    context: &Context,
    wasm_hash: &[u8; 32],
) -> Result<CanisterStatusType, String> {
    let result = status::read(&context.agent, context.canister).await?;
    if result.module_hash.as_deref() != Some(wasm_hash.as_slice())
        || !result
            .settings
            .controllers
            .iter()
            .any(|controller| controller.as_slice() == context.configuration.manager.as_slice())
    {
        return Err("platform artifact or manager/controller mismatch".into());
    }
    Ok(result.status)
}

pub(super) async fn admit(context: &Context) -> Result<(), String> {
    if status(context, &context.wasm_hash).await? != CanisterStatusType::Running {
        return Err("stop preparation/dispatch requires a running, drained host".into());
    }
    Ok(())
}

pub(super) fn reply(bytes: &[u8]) -> Result<(), String> {
    decode_args::<()>(bytes, LIMITS, Arity::Exact).map_err(|_| "invalid original stop reply".into())
}

pub(super) fn acknowledge(id: &RequestId, bytes: &[u8]) -> Result<String, String> {
    reply(bytes)?;
    Ok(format!(
        "Original stop acknowledged: request_id={id}; historical outcome only. Observe current platform status before any further management."
    ))
}

pub(super) async fn observe(
    context: &Context,
    id: &RequestId,
    original: RequestStatusResponse,
) -> Result<String, String> {
    let outcome = match original {
        RequestStatusResponse::Replied(value) => {
            reply(&value.arg)?;
            "acknowledged"
        }
        RequestStatusResponse::Rejected(_) => "rejected",
        RequestStatusResponse::Unknown | RequestStatusResponse::Done => "unavailable",
        RequestStatusResponse::Received | RequestStatusResponse::Processing => "pending",
    };
    // Stop rejection/timeout need not undo stopping. Report original outcome and
    // current platform state separately; never start to make an error disappear.
    let current = status(context, &context.wasm_hash).await?;
    if current != CanisterStatusType::Stopped {
        return Err(format!(
            "original stop {outcome}; platform={current:?}; preserve custody, no resubmission or restart"
        ));
    }
    Ok(format!(
        "Stop observed: request_id={id} original={outcome} platform=Stopped; current observation only, not continuity, upgrade or activation approval."
    ))
}
