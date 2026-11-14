//! IC stop semantics: controller admission and status observation while Blob cannot answer.

use super::Context;
use ic_platform::{
    Principal,
    agent::{RequestId, RequestStatusResponse},
    candid_codec::{
        arguments::encode_args,
        decoder::{Arity, Limits, decode_args, decode_one},
    },
    management_canister_types::{CanisterIdRecord, CanisterStatusResult, CanisterStatusType},
};

const LIMITS: Limits = Limits {
    bytes: 64 * 1024,
    header: 8 * 1024,
    types: 128,
    work: 512 * 1024,
    allocation: 256 * 1024,
};

pub(super) fn argument(context: &Context) -> Result<Vec<u8>, String> {
    encode_args(
        &(CanisterIdRecord {
            canister_id: context.canister,
        },),
        1024,
    )
    .map_err(|_| "cannot encode canister management target".into())
}

pub(super) async fn status(
    context: &Context,
    wasm_hash: &[u8; 32],
) -> Result<CanisterStatusType, String> {
    let target = context
        .canister
        .as_slice()
        .try_into()
        .map_err(|_| "invalid target")?;
    let management = Principal::management_canister()
        .as_slice()
        .try_into()
        .map_err(|_| "invalid management recipient")?;
    // Management status is a replicated call, routed to the target's subnet. It
    // remains usable while the Blob canister is stopping or stopped.
    let bytes = context
        .agent
        .update(&management, "canister_status")
        .with_effective_canister_id(target)
        .with_arg(argument(context)?)
        .call_and_wait()
        .await
        .map_err(|_| "platform status authentication failed")?;
    let result: CanisterStatusResult =
        decode_one(&bytes, LIMITS).map_err(|_| "invalid platform status reply")?;
    if result.module_hash.as_deref() != Some(wasm_hash.as_slice())
        || !result
            .settings
            .controllers
            .contains(&context.configuration.manager)
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
