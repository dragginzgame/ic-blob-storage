//! Immutable maintenance-request custody and guarded dispatch; no renewal or resubmission.

mod activate;
mod start;
pub(crate) mod status;
mod stop;
mod upgrade;

use super::{Context, read};
use crate::{files, overview::hex};
use ic_blob_storage::{
    local::Error,
    protocol::{Command, Installation, Outcome, RPC_LIMITS},
};
use ic_host_fs::{Directory, FilePolicy};
use ic_platform::{
    ingress::{Content, decode_envelope},
    native::RequestStatus,
};
use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    path::Path,
    process::Stdio,
    time::{SystemTime, UNIX_EPOCH},
};

const LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 16 * 1024,
    allocation: 128 * 1024,
    nodes: 2048,
    depth: 16,
};

#[derive(Clone, Copy)]
pub(super) enum Operation<'a> {
    Drain,
    Stop,
    Upgrade {
        wasm: &'a [u8],
        stop_record: &'a Path,
    },
    Start {
        wasm: &'a [u8],
        stop_record: &'a Path,
        upgrade_record: &'a Path,
    },
    Activate {
        wasm: &'a [u8],
        stop_record: &'a Path,
        upgrade_record: &'a Path,
        start_record: &'a Path,
    },
}

impl<'a> Operation<'a> {
    fn name(self) -> &'static str {
        match self {
            Self::Drain => "drain",
            Self::Stop => "stop",
            Self::Upgrade { .. } => "upgrade",
            Self::Start { .. } => "start",
            Self::Activate { .. } => "activate",
        }
    }

    fn recipient(self, context: &Context) -> Principal {
        match self {
            Self::Drain | Self::Activate { .. } => context.canister,
            Self::Stop | Self::Upgrade { .. } | Self::Start { .. } => {
                Principal::management_canister()
            }
        }
    }

    fn method(self) -> &'static str {
        match self {
            Self::Drain | Self::Activate { .. } => "blob_update",
            Self::Stop => "stop_canister",
            Self::Upgrade { .. } => "install_code",
            Self::Start { .. } => "start_canister",
        }
    }

    fn argument(
        self,
        context: &Context,
        attempt: u64,
        positional: bool,
    ) -> Result<Vec<u8>, String> {
        match self {
            Self::Drain => argument(&Command::Drain(attempt), positional),
            Self::Activate { .. } => argument(&Command::Activate(attempt), positional),
            Self::Stop | Self::Start { .. } => stop::argument(context),
            Self::Upgrade { wasm, .. } => upgrade::argument(context, wasm, positional),
        }
    }

    async fn admit(self, context: &Context, attempt: u64) -> Result<(), String> {
        if let Self::Activate { wasm, .. } = self {
            let original = self
                .predecessor(context, attempt)?
                .expect("start predecessor");
            // Observe the original start and recheck the currently drained host.
            // The service checks manager and exact attempt again when activation executes.
            start::observe(
                context,
                &original.id,
                attempt,
                request_status(context, &original).await?,
                wasm,
            )
            .await?;

            return Ok(());
        }
        if let Self::Upgrade { stop_record, .. } = self {
            return upgrade::admit(context, stop_record, attempt).await;
        }
        if let Self::Start {
            wasm,
            stop_record,
            upgrade_record,
        } = self
        {
            return start::admit(context, stop_record, upgrade_record, attempt, wasm).await;
        }
        let view = context.inspect().await?;

        match self {
            Self::Drain
                if view.maintenance.active
                    && view.maintenance.attempt.checked_add(1) == Some(attempt) =>
            {
                Ok(())
            }
            Self::Stop if !view.maintenance.active && view.maintenance.attempt == attempt => {
                stop::admit(context).await
            }
            _ => Err(format!(
                "current admission does not permit this {} attempt",
                self.name()
            )),
        }
    }

    fn marker(self, attempt: u64) -> String {
        match self {
            // Preserve the 0.100.3 claim namespace and intent format.
            Self::Drain => format!("attempt-{attempt}.claim"),
            Self::Stop => format!("attempt-{attempt}.stop.claim"),
            Self::Upgrade { .. } => format!("attempt-{attempt}.upgrade.claim"),
            Self::Start { .. } => format!("attempt-{attempt}.start.claim"),
            Self::Activate { .. } => format!("attempt-{attempt}.activate.claim"),
        }
    }

    fn packet_limit(self) -> usize {
        match self {
            Self::Upgrade { .. } => 2 * 1024 * 1024,
            _ => LIMITS.bytes,
        }
    }

    fn predecessor(self, context: &Context, attempt: u64) -> Result<Option<Original>, String> {
        if let Some((operation, record)) = self.parent() {
            let original = load(context, record, operation)?;
            if original.attempt != attempt {
                return Err("predecessor attempt mismatch".into());
            }

            return Ok(Some(original));
        }

        Ok(None)
    }

    fn parent(self) -> Option<(Self, &'a Path)> {
        match self {
            Self::Activate {
                wasm,
                stop_record,
                upgrade_record,
                start_record,
            } => Some((
                Self::Start {
                    wasm,
                    stop_record,
                    upgrade_record,
                },
                start_record,
            )),
            Self::Upgrade { stop_record, .. } => Some((Self::Stop, stop_record)),
            Self::Start {
                wasm,
                stop_record,
                upgrade_record,
            } => Some((Self::Upgrade { wasm, stop_record }, upgrade_record)),
            _ => None,
        }
    }

    fn admit_custody(
        self,
        context: &Context,
        attempt: u64,
        target: &Directory,
    ) -> Result<(), String> {
        // The closed operation set bounds this chain: activate -> start -> upgrade -> stop.
        if let Some((operation, record)) = self.parent() {
            let previous = load(context, record, operation)?;
            if previous.attempt != attempt
                || files::read_at(target, operation.marker(attempt).as_ref(), 32)?
                    != previous.id.to_vec()
            {
                return Err(format!(
                    "{} dispatch claim does not match predecessor",
                    operation.name()
                ));
            }
            operation.admit_custody(context, attempt, target)?;
        }

        Ok(())
    }
}

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

fn argument(command: &Command, positional: bool) -> Result<Vec<u8>, String> {
    if positional {
        return ic_blob_storage::protocol::frame::encode_retained(
            command,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id("blob_update"),
                direction: ic_wire::Direction::Request,
            },
            RPC_LIMITS,
        )
        .map_err(|_| "cannot encode original positional admission command".into());
    }

    ic_blob_storage::protocol::frame::encode(
        command,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_update"),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "cannot encode admission command".into())
}

pub(super) async fn prepare(
    context: &Context,
    directory: &Path,
    attempt: u64,
    operation: Operation<'_>,
) -> Result<String, String> {
    operation.admit(context, attempt).await?;
    let recipient = operation.recipient(context);
    let signed = context.client.prepare(
        recipient,
        context.canister,
        operation.method(),
        &operation.argument(context, attempt, false)?,
    )?;
    let intent = Intent {
        version: 2,
        url: context.url.clone(),
        canister: context.canister,
        configuration: context.configuration,
        wasm_hash: context.wasm_hash,
        root_hash: context.root_hash,
        state_version: context.state_version,
        attempt,
        packet_hash: Sha256::digest(signed.envelope_bytes()).into(),
    };
    let bytes =
        ic_blob_storage::protocol::frame::encode(&intent, ic_wire::Encoding::Payload, LIMITS)
            .map_err(|_| "cannot encode management intent")?;
    if signed.envelope_bytes().len() > operation.packet_limit() {
        return Err("signed management request exceeds record limit".into());
    }
    // A partial directory remains claimed. No automatic cleanup or repair on any
    // failure, including a sync failure after publication. Parent custody is caller-owned.
    let held = files::claim(directory).map_err(|error| {
        format!("cannot claim management directory; preserve any existing claim: {error}")
    })?;
    files::publish(&held, "intent.wire".as_ref(), &bytes).map_err(|error| {
        format!("management intent publication failed; preserve claim: {error}")
    })?;
    if let Some(previous) = operation.predecessor(context, attempt)? {
        files::publish(&held, "predecessor.id".as_ref(), &previous.id)
            .map_err(|_| "predecessor publication failed; preserve claim")?;
    }
    files::publish(&held, "request.cbor".as_ref(), signed.envelope_bytes()).map_err(|error| {
        format!("signed management request publication failed; preserve claim: {error}")
    })?;

    Ok(format!(
        "Prepared {}: request_id={} attempt={attempt}; NOT submitted. Preserve the original claim; no renewal or retry is authorized.",
        operation.name(),
        hex(signed.request_id())
    ))
}

struct Original {
    positional: bool,
    attempt: u64,
    packet: Vec<u8>,
    id: [u8; 32],
    expires_at: u64,
}

fn load(context: &Context, directory: &Path, operation: Operation<'_>) -> Result<Original, String> {
    let held = files::directory(directory, false)?;
    let intent: Intent = ic_blob_storage::protocol::frame::decode_record(
        &files::read_at(&held, "intent.wire".as_ref(), LIMITS.bytes)?,
        LIMITS,
    )
    .map_err(|_| "invalid management intent")?;
    let packet = files::read_at(&held, "request.cbor".as_ref(), operation.packet_limit())?;
    let hash: [u8; 32] = Sha256::digest(&packet).into();
    if !matches!(intent.version, 1 | 2)
        || intent.url != context.url
        || intent.canister != context.canister
        || intent.configuration != context.configuration
        || intent.root_hash != context.root_hash
        || intent.wasm_hash != context.wasm_hash
        || intent.state_version != context.state_version
        || intent.attempt == 0
        || intent.packet_hash != hash
    {
        return Err("management claim binding mismatch".into());
    }
    // Strictly inspect correspondence, not signature authenticity. The authenticated
    // read_state result below establishes the original request's observed outcome.
    if let Some(previous) = operation.predecessor(context, intent.attempt)?
        && files::read_at(&held, "predecessor.id".as_ref(), 32)? != previous.id.to_vec()
    {
        return Err("predecessor identity mismatch".into());
    }
    let content = decode_envelope(&packet, operation.packet_limit())
        .map_err(|_| "invalid signed management content")?
        .value
        .content;
    let Content::Call {
        sender,
        canister_id,
        method_name,
        arg,
        sender_info,
        ingress_expiry,
        ..
    } = &content
    else {
        return Err("management claim is not a call".into());
    };
    if *sender != context.configuration.manager
        || *canister_id != operation.recipient(context)
        || method_name != operation.method()
        || *arg != operation.argument(context, intent.attempt, intent.version == 1)?
        || sender_info.is_some()
    {
        return Err("signed management fields do not match intent".into());
    }
    // Derive the request ID from original content; never regenerate a fresh signed call.
    // Expired requests remain inspectable until the network forgets their outcomes.
    let id = content
        .request_id()
        .map_err(|_| "cannot derive original request ID")?;

    Ok(Original {
        positional: intent.version == 1,
        attempt: intent.attempt,
        packet,
        id,
        expires_at: *ingress_expiry,
    })
}

pub(super) async fn observe(
    context: &Context,
    directory: &Path,
    operation: Operation<'_>,
) -> Result<String, String> {
    let original = load(context, directory, operation)?;
    let status = request_status(context, &original).await?;
    if let Operation::Activate { wasm, .. } = operation {
        return activate::observe(
            context,
            &original.id,
            original.attempt,
            status,
            wasm,
            original.positional,
        )
        .await;
    }
    if let Operation::Start { wasm, .. } = operation {
        return start::observe(context, &original.id, original.attempt, status, wasm).await;
    }
    if let Operation::Upgrade { wasm, .. } = operation {
        return upgrade::observe(context, &original.id, status, wasm).await;
    }
    if matches!(operation, Operation::Stop) {
        return stop::observe(context, &original.id, status).await;
    }

    match status {
        RequestStatus::Replied(reply) => acknowledge(
            &original.id,
            original.attempt,
            &reply,
            false,
            original.positional,
        ),
        RequestStatus::Rejected { .. } => {
            Err("original drain was rejected; no retry authorized".into())
        }
        RequestStatus::Unknown | RequestStatus::Unavailable | RequestStatus::Done => {
            Err("original drain outcome unavailable; keep unresolved, no resubmission".into())
        }
        RequestStatus::Received | RequestStatus::Processing => {
            Err("original drain remains pending; no resubmission".into())
        }
    }
}

async fn request_status(context: &Context, original: &Original) -> Result<RequestStatus, String> {
    let prepared = ic_platform::native::PreparedUpdate::from_bytes(
        &original.packet,
        context.canister,
        crate::network::MESSAGE_BYTES,
    )
    .map_err(|_| "invalid original signed request")?;

    Ok(context.client.observe(&prepared).await?.status().clone())
}

fn acknowledge(
    id: &[u8; 32],
    attempt: u64,
    bytes: &[u8],
    active: bool,
    positional: bool,
) -> Result<String, String> {
    let operation = if active { "activation" } else { "drain" };
    let result: Result<Outcome, Error> = if positional {
        ic_blob_storage::protocol::frame::decode_retained(
            bytes,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id("blob_update"),
                direction: ic_wire::Direction::Response,
            },
            RPC_LIMITS,
        )
        .map_err(|_| ())
    } else {
        ic_blob_storage::protocol::frame::decode(
            bytes,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id("blob_update"),
                direction: ic_wire::Direction::Response,
            },
            RPC_LIMITS,
        )
        .map_err(|_| ())
    }
    .map_err(|()| format!("invalid original {operation} reply"))?;

    match result {
        Ok(Outcome::Maintenance(state)) if state.attempt == attempt && state.active == active => {
            Ok(format!(
                "Original {operation} acknowledged: request_id={display_id} attempt={}; historical outcome only, not current admission or activation approval.",
                attempt,
                display_id = hex(id)
            ))
        }
        _ => Err(format!(
            "original {operation} did not acknowledge the requested admission state"
        )),
    }
}

pub(super) async fn admit_tooling(checkout: &Path) -> Result<(), String> {
    let snapshot = read(&checkout.join(".toko-tooling.snapshot"), 64 * 1024)?;
    if hex(&Sha256::digest(&snapshot))
        != include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tooling.sha256")).trim()
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
        return Err("original request expired; no renewal authorized".into());
    }

    Ok(())
}

pub(super) async fn dispatch(
    context: &Context,
    directory: &Path,
    custody: &Path,
    operation: Operation<'_>,
) -> Result<String, String> {
    let original = load(context, directory, operation)?;
    unexpired(original.expires_at)?;
    let network = hex(&context.root_hash);
    let target_directory = custody.join(network).join(context.canister.to_text());
    let held =
        files::directory(&target_directory, true).map_err(|_| "cannot admit target custody")?;
    let _lock = held
        .lock(
            "manager.lock".as_ref(),
            0o600,
            FilePolicy {
                forbidden: 0o077,
                single_link: true,
            },
        )
        .map_err(|_| "target management is already locked or lock custody invalid")?;
    let marker = operation.marker(original.attempt);
    // Presence of any entry (even partial or corrupt) conservatively bars dispatch.
    if held
        .open_file(
            marker.as_ref(),
            ic_host_fs::Access::Read,
            FilePolicy {
                forbidden: 0,
                single_link: false,
            },
        )
        .map_err(|_| "cannot inspect dispatch claim")?
        .is_some()
    {
        return Err(
            "dispatch already claimed; observe the original request, never resubmit".into(),
        );
    }
    if original.positional {
        return Err(
            "positional management claim is observation-only after the Wire cutover".into(),
        );
    }
    operation.admit_custody(context, original.attempt, &held)?;
    operation.admit(context, original.attempt).await?;
    unexpired(original.expires_at)?;
    // This is an irreversible local exposure boundary, not proof of transmission.
    // A crash or sync failure may strand a safe unsent request; never delete/retry it.
    files::publish(&held, marker.as_ref(), &original.id).map_err(|error| {
        format!("dispatch claim failed; preserve custody and do not resubmit: {error}")
    })?;
    let prepared = ic_platform::native::PreparedUpdate::from_bytes(
        &original.packet,
        context.canister,
        crate::network::MESSAGE_BYTES,
    )
    .map_err(|_| "invalid original signed request")?;
    let result = context
        .client
        .transport
        .dispatch(&prepared, crate::network::policy(false)?)
        .await;

    match result {
        Ok(ic_platform::native::Dispatch::Observed(observed))
            if matches!(observed.status(), RequestStatus::Replied(_)) =>
        {
            let RequestStatus::Replied(bytes) = observed.status() else {
                unreachable!()
            };
            match operation {
                Operation::Drain => acknowledge(&original.id, original.attempt, bytes, false, original.positional),
                Operation::Stop => stop::acknowledge(&original.id, bytes),
                Operation::Upgrade { .. } => upgrade::acknowledge(&original.id, bytes),
                Operation::Start { wasm, .. } => {
                    start::acknowledge(context, &original.id, original.attempt, bytes, wasm).await
                }
                Operation::Activate { wasm, .. } => {
                    activate::acknowledge(context, &original.id, original.attempt, bytes, wasm, false)
                        .await
                }
            }
        }
        Ok(ic_platform::native::Dispatch::UncertifiedRejection { .. }) => Err("uncertified dispatch rejection; preserve custody and observe the original request".into()),
        Ok(ic_platform::native::Dispatch::Observed(observed)) if matches!(observed.status(), RequestStatus::Rejected { .. } | RequestStatus::Done | RequestStatus::Unavailable) => Err("original request rejected or outcome unavailable; preserve custody, no resubmission".into()),
        Ok(_) => Err("original request pending; observe it, do not resubmit".into()),
        Err(_) => Err(
            "dispatch outcome uncertain; preserve custody and observe the original request".into(),
        ),
    }
}
