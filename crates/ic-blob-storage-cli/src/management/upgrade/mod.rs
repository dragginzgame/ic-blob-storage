//! One native-memory-preserving upgrade of a stopped host, chained to an acknowledged original stop.

use ic_platform::native::RequestStatus;
use ic_platform_protocol::management_snapshots::Status as CanisterStatusType;

use super::{Context, Operation, load, request_status, stop};
use ic_blob_storage::protocol::RPC_LIMITS;
use ic_platform_protocol::management::{
    InstallCodeArgs, InstallMode, MemoryPersistence, UpgradeFlags,
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
    InstallCodeArgs {
        // Keep native OP memory and require the source close hook.
        mode: InstallMode::Upgrade(Some(UpgradeFlags {
            skip_pre_upgrade: None,
            wasm_memory_persistence: Some(MemoryPersistence::Keep),
        })),
        canister_id: context.canister,
        wasm_module: wasm.to_vec(),
        arg,
        sender_canister_version: None,
    }
    .encode(2 * 1024 * 1024)
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
    match request_status(context, &original).await? {
        RequestStatus::Replied(reply) => stop::reply(&reply)?,
        _ => {
            return Err(
                "upgrade requires original stop acknowledgment; preserve uncertainty".into(),
            );
        }
    }
    Ok(())
}

pub(super) fn acknowledge(id: &[u8; 32], bytes: &[u8]) -> Result<String, String> {
    stop::reply(bytes).map_err(|_| "invalid original upgrade reply")?;
    Ok(format!(
        "Original upgrade acknowledged: request_id={display_id}; historical outcome only, no start or activation approval.",
        display_id = crate::overview::hex(id)
    ))
}

pub(super) async fn observe(
    context: &Context,
    id: &[u8; 32],
    original: RequestStatus,
    wasm: &[u8],
) -> Result<String, String> {
    match original {
        RequestStatus::Replied(reply) => {
            acknowledge(id, &reply)?;
        }
        RequestStatus::Rejected { .. } => {
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
        "Upgrade observed: request_id={display_id}; original acknowledgment and replacement Wasm matched, platform=Stopped; no start or activation approval.",
        display_id = crate::overview::hex(id)
    ))
}
