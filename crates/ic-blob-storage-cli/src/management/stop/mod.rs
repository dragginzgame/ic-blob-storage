//! IC stop semantics: controller admission and status observation while Blob cannot answer.

use ic_platform::native::RequestStatus;
use ic_platform_protocol::management_snapshots::Status as CanisterStatusType;

use super::{
    Context,
    status::{self, LIMITS},
};

pub(super) fn argument(context: &Context) -> Result<Vec<u8>, String> {
    status::argument(context.canister)
}

pub(super) async fn status(
    context: &Context,
    wasm_hash: &[u8; 32],
) -> Result<CanisterStatusType, String> {
    let result = status::read(&context.client, context.canister).await?;
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
    ic_platform_protocol::caffeine::decode_empty(bytes, LIMITS)
        .map_err(|_| "invalid original stop reply".into())
}

pub(super) fn acknowledge(id: &[u8; 32], bytes: &[u8]) -> Result<String, String> {
    reply(bytes)?;
    Ok(format!(
        "Original stop acknowledged: request_id={display_id}; historical outcome only. Observe current platform status before any further management.",
        display_id = crate::overview::hex(id)
    ))
}

pub(super) async fn observe(
    context: &Context,
    id: &[u8; 32],
    original: RequestStatus,
) -> Result<String, String> {
    let outcome = match original {
        RequestStatus::Replied(value) => {
            reply(&value)?;
            "acknowledged"
        }
        RequestStatus::Rejected { .. } => "rejected",
        RequestStatus::Unknown | RequestStatus::Unavailable | RequestStatus::Done => "unavailable",
        RequestStatus::Received | RequestStatus::Processing => "pending",
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
        "Stop observed: request_id={display_id} original={outcome} platform=Stopped; current observation only, not continuity, upgrade or activation approval.",
        display_id = crate::overview::hex(id)
    ))
}
