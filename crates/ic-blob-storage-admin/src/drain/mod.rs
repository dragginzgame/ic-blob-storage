//! Immutable original-request custody and one guarded dispatch; no renewal or resubmission.

use super::{Context, read};
use ic_blob_storage::{
    local::Error,
    protocol::{Command, Installation, Outcome, RPC_LIMITS},
};
use ic_host_fs::durable::{self, PublicationMode, WriteOptions};
use ic_platform::{
    Principal,
    agent::{CallResponse, EffectiveId, RequestId, RequestStatusResponse, to_request_id},
    ic_protocol::{EnvelopeContent, decode_call_content_strict},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    path::Path,
    process::Stdio,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 16 * 1024,
    allocation: 128 * 1024,
    nodes: 1024,
    depth: 16,
};

#[derive(Serialize, Deserialize)]
struct Intent {
    version: u8,
    url: String,
    canister: Principal,
    configuration: Installation,
    wasm_hash: [u8; 32],
    root_hash: [u8; 32],
    state_version: u32,
    attempt: u64,
    packet_hash: [u8; 32],
}

fn argument(attempt: u64) -> Result<Vec<u8>, String> {
    ic_wire::native::encode_request(
        ic_wire::native::method_id("blob_update"),
        &Command::Drain(attempt),
        RPC_LIMITS,
    )
    .map_err(|_| "cannot encode drain".into())
}

pub(super) async fn prepare(
    context: &Context,
    directory: &Path,
    attempt: u64,
) -> Result<String, String> {
    let view = context.inspect().await?;
    if !view.maintenance.active || view.maintenance.attempt.checked_add(1) != Some(attempt) {
        return Err("drain requires active admission and the exact next attempt".into());
    }
    let target = EffectiveId::Canister(
        context
            .canister
            .as_slice()
            .try_into()
            .map_err(|_| "invalid target")?,
    );
    let signed = context
        .agent
        .update(&target.as_principal(), "blob_update")
        .with_arg(argument(attempt)?)
        .expire_after(Duration::from_secs(120))
        .sign()
        .map_err(|_| "cannot sign drain")?;
    let intent = Intent {
        version: 1,
        url: context.url.clone(),
        canister: context.canister,
        configuration: context.configuration,
        wasm_hash: context.wasm_hash,
        root_hash: context.root_hash,
        state_version: context.state_version,
        attempt,
        packet_hash: Sha256::digest(&signed.signed_update).into(),
    };
    let bytes =
        ic_wire::native::encode(&intent, LIMITS).map_err(|_| "cannot encode drain intent")?;
    if signed.signed_update.len() > LIMITS.bytes {
        return Err("signed drain exceeds record limit".into());
    }
    // A partial directory remains claimed. No automatic cleanup or repair on any
    // failure, including a sync failure after publication. Parent custody is caller-owned.
    let _directory = durable::create_private_directory(directory).map_err(|error| {
        format!("cannot claim drain directory; preserve any existing claim: {error}")
    })?;
    let options = WriteOptions {
        mode: PublicationMode::CreateNew,
        permissions: 0o600,
    };
    durable::write_bytes(&directory.join("intent.wire"), options, &bytes)
        .map_err(|error| format!("drain intent publication failed; preserve claim: {error}"))?;
    durable::write_bytes(
        &directory.join("request.cbor"),
        options,
        &signed.signed_update,
    )
    .map_err(|error| format!("signed drain publication failed; preserve claim: {error}"))?;
    Ok(format!(
        "Prepared drain: request_id={} attempt={attempt}; NOT submitted. Preserve the original claim; no renewal or retry is authorized.",
        signed.request_id
    ))
}

struct Original {
    attempt: u64,
    packet: Vec<u8>,
    id: RequestId,
    expires_at: u64,
}

fn load(context: &Context, directory: &Path) -> Result<Original, String> {
    let intent: Intent =
        ic_wire::native::decode(&read(&directory.join("intent.wire"), LIMITS.bytes)?, LIMITS)
            .map_err(|_| "invalid drain intent")?;
    let packet = read(&directory.join("request.cbor"), LIMITS.bytes)?;
    let hash: [u8; 32] = Sha256::digest(&packet).into();
    if intent.version != 1
        || intent.url != context.url
        || intent.canister != context.canister
        || intent.configuration != context.configuration
        || intent.root_hash != context.root_hash
        || intent.wasm_hash != context.wasm_hash
        || intent.state_version != context.state_version
        || intent.attempt == 0
        || intent.packet_hash != hash
    {
        return Err("drain claim binding mismatch".into());
    }
    // Strictly inspect correspondence, not signature authenticity. The authenticated
    // read_state result below establishes the original request's observed outcome.
    let content = decode_call_content_strict(&packet, LIMITS.bytes)
        .map_err(|_| "invalid signed drain content")?;
    let EnvelopeContent::Call {
        sender,
        canister_id,
        method_name,
        arg,
        sender_info,
        ingress_expiry,
        ..
    } = &content
    else {
        return Err("drain claim is not a call".into());
    };
    if sender.as_slice() != context.configuration.manager.as_slice()
        || canister_id.as_slice() != context.canister.as_slice()
        || method_name != "blob_update"
        || *arg != argument(intent.attempt)?
        || sender_info.is_some()
    {
        return Err("signed drain fields do not match intent".into());
    }
    // Derive the request ID from original content; never regenerate a fresh signed call.
    // Expired requests remain inspectable until the network forgets their outcomes.
    let id = to_request_id(&content).map_err(|_| "cannot derive original request ID")?;
    Ok(Original {
        attempt: intent.attempt,
        packet,
        id,
        expires_at: *ingress_expiry,
    })
}

pub(super) async fn observe(context: &Context, directory: &Path) -> Result<String, String> {
    let original = load(context, directory)?;
    let target = EffectiveId::Canister(
        context
            .canister
            .as_slice()
            .try_into()
            .map_err(|_| "invalid target")?,
    );
    let (status, _) = context
        .agent
        .request_status_raw(&original.id, target)
        .await
        .map_err(|_| "original drain status authentication failed")?;
    match status {
        RequestStatusResponse::Replied(reply) => {
            acknowledge(&original.id, original.attempt, &reply.arg)
        }
        RequestStatusResponse::Rejected(_) => {
            Err("original drain was rejected; no retry authorized".into())
        }
        RequestStatusResponse::Unknown | RequestStatusResponse::Done => {
            Err("original drain outcome unavailable; keep unresolved, no resubmission".into())
        }
        RequestStatusResponse::Received | RequestStatusResponse::Processing => {
            Err("original drain remains pending; no resubmission".into())
        }
    }
}

fn acknowledge(id: &RequestId, attempt: u64, bytes: &[u8]) -> Result<String, String> {
    let result: Result<Outcome, Error> = ic_wire::native::decode_response(
        ic_wire::native::method_id("blob_update"),
        bytes,
        RPC_LIMITS,
    )
    .map_err(|_| "invalid original drain reply")?;
    match result {
        Ok(Outcome::Maintenance(state)) if state.attempt == attempt && !state.active => {
            Ok(format!(
                "Original drain acknowledged: request_id={id} attempt={}; historical outcome only, not current admission or activation approval.",
                attempt
            ))
        }
        _ => Err("original drain did not acknowledge the requested closed attempt".into()),
    }
}

pub(super) async fn admit_tooling(checkout: &Path) -> Result<(), String> {
    let snapshot = read(&checkout.join(".toko-tooling.snapshot"), 64 * 1024)?;
    if snapshot
        != include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../.toko-tooling.snapshot"
        ))
    {
        return Err("checkout tooling snapshot differs from this binary".into());
    }
    // The checkout, its Makefile and inherited execution environment are trusted
    // operator inputs. This is shared admission, not a sandbox for arbitrary code.
    let status = tokio::process::Command::new("make")
        .current_dir(checkout)
        .args(["--no-print-directory", "toko-tooling-check"])
        .env_remove("MAKEFLAGS")
        .env_remove("MFLAGS")
        .env_remove("MAKEFILES")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .status()
        .await
        .map_err(|_| "cannot run tooling admission")?;
    if !status.success() {
        return Err("tooling admission refused dispatch".into());
    }
    Ok(())
}

fn unexpired(expires_at: u64) -> Result<(), String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "invalid local clock")?
        .as_nanos();
    if u128::from(expires_at) <= now {
        return Err("original drain request expired; no renewal authorized".into());
    }
    Ok(())
}

pub(super) async fn dispatch(
    context: &Context,
    directory: &Path,
    custody: &Path,
) -> Result<String, String> {
    let original = load(context, directory)?;
    unexpired(original.expires_at)?;
    let network: String = context
        .root_hash
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let target_directory = custody.join(network).join(context.canister.to_text());
    durable::prepare_private_parents(&target_directory)
        .map_err(|_| "cannot admit target custody")?;
    let file = durable::open_private_regular_lock_file(&target_directory.join("manager.lock"))
        .map_err(|_| "cannot open management lock")?;
    let _lock = durable::ExclusiveFileLock::try_acquire(file)
        .map_err(|_| "target management is already locked")?;
    let marker = target_directory.join(format!("attempt-{}.claim", original.attempt));
    // Presence of any entry (even partial or corrupt) conservatively bars dispatch.
    match std::fs::symlink_metadata(&marker) {
        Ok(_) => {
            return Err(
                "dispatch already claimed; observe the original request, never resubmit".into(),
            );
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err("cannot inspect dispatch claim".into()),
    }
    let view = context.inspect().await?;
    if !view.maintenance.active || view.maintenance.attempt.checked_add(1) != Some(original.attempt)
    {
        return Err("current admission does not permit this drain attempt".into());
    }
    unexpired(original.expires_at)?;
    // This is an irreversible local exposure boundary, not proof of transmission.
    // A crash or sync failure may strand a safe unsent request; never delete/retry it.
    durable::write_bytes(
        &marker,
        WriteOptions {
            mode: PublicationMode::CreateNew,
            permissions: 0o600,
        },
        &original.id.to_vec(),
    )
    .map_err(|error| {
        format!("dispatch claim failed; preserve custody and do not resubmit: {error}")
    })?;
    let target = EffectiveId::Canister(
        context
            .canister
            .as_slice()
            .try_into()
            .map_err(|_| "invalid target")?,
    );
    match context.agent.update_signed(target, original.packet).await {
        Ok(CallResponse::Response(bytes)) => acknowledge(&original.id, original.attempt, &bytes),
        Ok(CallResponse::Poll(_)) => {
            Err("original drain pending; observe it, do not resubmit".into())
        }
        Err(_) => Err(
            "drain dispatch outcome uncertain; preserve custody and observe the original request"
                .into(),
        ),
    }
}
