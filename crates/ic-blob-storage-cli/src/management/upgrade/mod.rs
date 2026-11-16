//! One ordinary upgrade of a stopped host, chained to an acknowledged original stop.

use super::{Context, Operation, load, request_status, stop};
use ic_blob_storage::protocol::RPC_LIMITS;
use ic_platform_legacy::{
    agent::{RequestId, RequestStatusResponse},
    candid_codec::arguments::encode_args,
    management_canister_types::{CanisterInstallMode, CanisterStatusType, InstallCodeArgs},
};
use sha2::{Digest, Sha256};
use std::path::Path;

pub(super) fn argument(context: &Context, wasm: &[u8]) -> Result<Vec<u8>, String> {
    if wasm.len() > 1024 * 1024 || !wasm.starts_with(b"\0asm\x01\0\0\0") {
        return Err("upgrade requires an uncompressed Wasm module of at most 1 MiB".into());
    }
    let arg = ic_wire::encode(
        &(),
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_post_upgrade"),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "cannot encode fixed upgrade arguments")?;
    encode_args(
        &(InstallCodeArgs {
            // Omitted flags select ordinary upgrade: run pre_upgrade, replace heap.
            // There are deliberately no CLI overrides or fresh-install fallback.
            mode: CanisterInstallMode::Upgrade(None),
            canister_id: context.canister,
            wasm_module: wasm.to_vec(),
            arg,
            sender_canister_version: None,
        },),
        2 * 1024 * 1024,
    )
    .map_err(|_| "cannot encode bounded upgrade".into())
}

pub(super) async fn admit(context: &Context, record: &Path, attempt: u64) -> Result<(), String> {
    confirm_stop(context, record, attempt).await?;
    if stop::status(context, &context.wasm_hash).await? != CanisterStatusType::Stopped {
        return Err("upgrade requires the admitted host to remain stopped".into());
    }
    Ok(())
}

pub(super) async fn confirm_stop(
    context: &Context,
    record: &Path,
    attempt: u64,
) -> Result<(), String> {
    let original = load(context, record, Operation::Stop)?;
    if original.attempt != attempt {
        return Err("stop predecessor attempt mismatch".into());
    }
    match request_status(context, &original.id).await? {
        RequestStatusResponse::Replied(reply) => stop::reply(&reply.arg)?,
        _ => {
            return Err(
                "upgrade requires original stop acknowledgment; preserve uncertainty".into(),
            );
        }
    }
    Ok(())
}

pub(super) fn acknowledge(id: &RequestId, bytes: &[u8]) -> Result<String, String> {
    stop::reply(bytes).map_err(|_| "invalid original upgrade reply")?;
    Ok(format!(
        "Original upgrade acknowledged: request_id={id}; historical outcome only, no start or activation approval."
    ))
}

pub(super) async fn observe(
    context: &Context,
    id: &RequestId,
    original: RequestStatusResponse,
    wasm: &[u8],
) -> Result<String, String> {
    match original {
        RequestStatusResponse::Replied(reply) => {
            acknowledge(id, &reply.arg)?;
        }
        RequestStatusResponse::Rejected(_) => {
            return Err(
                "original upgrade rejected; preserve custody, no retry or reinstall".into(),
            );
        }
        _ => return Err(
            "original upgrade outcome unresolved; module hash cannot replace its acknowledgment"
                .into(),
        ),
    }
    let expected: [u8; 32] = Sha256::digest(wasm).into();
    if stop::status(context, &expected).await? != CanisterStatusType::Stopped {
        return Err("acknowledged upgrade target is no longer stopped".into());
    }
    Ok(format!(
        "Upgrade observed: request_id={id}; original acknowledgment and replacement Wasm matched, platform=Stopped; no start or activation approval."
    ))
}
