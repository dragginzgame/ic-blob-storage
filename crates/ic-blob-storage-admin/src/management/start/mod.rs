//! Start only an acknowledged upgrade, then verify the reopened host remains drained.

use super::{Context, Operation, load, request_status, stop, upgrade};
use ic_blob_storage::protocol::ManagementView;
use ic_platform::{
    agent::{RequestId, RequestStatusResponse},
    management_canister_types::CanisterStatusType,
};
use sha2::{Digest, Sha256};
use std::path::Path;

pub(super) async fn admit(
    context: &Context,
    stop_record: &Path,
    upgrade_record: &Path,
    attempt: u64,
    wasm: &[u8],
) -> Result<(), String> {
    let original = load(
        context,
        upgrade_record,
        Operation::Upgrade { wasm, stop_record },
    )?;
    if original.attempt != attempt {
        return Err("upgrade predecessor attempt mismatch".into());
    }
    // Recheck both original outcomes: a stopped observation or matching module
    // hash alone cannot close an outstanding stop or upgrade request.
    upgrade::confirm_stop(context, stop_record, attempt).await?;
    upgrade::observe(
        context,
        &original.id,
        request_status(context, &original.id).await?,
        wasm,
    )
    .await?;
    Ok(())
}

pub(super) async fn acknowledge(
    context: &Context,
    id: &RequestId,
    attempt: u64,
    bytes: &[u8],
    wasm: &[u8],
) -> Result<String, String> {
    stop::reply(bytes).map_err(|_| "invalid original start reply")?;
    let view = inspect(context, wasm).await?;
    if view.maintenance.attempt != attempt || view.maintenance.active {
        return Err(
            "reopened host does not match the current drained attempt; no activation approval"
                .into(),
        );
    }
    Ok(format!(
        "Start observed: request_id={id} attempt={attempt} active=false; replacement artifact, configuration and reopened host matched; no activation performed."
    ))
}

pub(super) async fn inspect(context: &Context, wasm: &[u8]) -> Result<ManagementView, String> {
    let expected: [u8; 32] = Sha256::digest(wasm).into();
    if stop::status(context, &expected).await? != CanisterStatusType::Running {
        return Err("reopened target is not running; preserve custody".into());
    }
    context.inspect_artifact(&expected).await
}

pub(super) async fn observe(
    context: &Context,
    id: &RequestId,
    attempt: u64,
    original: RequestStatusResponse,
    wasm: &[u8],
) -> Result<String, String> {
    match original {
        RequestStatusResponse::Replied(reply) => {
            acknowledge(context, id, attempt, &reply.arg, wasm).await
        }
        RequestStatusResponse::Rejected(_) => {
            Err("original start rejected; preserve custody, no resubmission".into())
        }
        _ => Err(
            "original start outcome unresolved; running state cannot replace its acknowledgment"
                .into(),
        ),
    }
}
