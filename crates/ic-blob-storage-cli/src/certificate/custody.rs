//! One permanent claim per network/issuer/root in caller-owned trusted custody.

use super::{METHOD, PACKET_LIMIT, Target, original};
use crate::overview::hex;
use clap::Args;
use ic_host_fs::durable::{self, PublicationMode, WriteOptions};
use ic_platform::{
    Principal,
    agent::Agent,
    candid_codec::arguments::encode_args,
    ic_protocol::{EnvelopeContent, decode_call_content_strict},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 8192,
    allocation: 32_768,
    nodes: 1024,
    depth: 16,
};
const OPTIONS: WriteOptions = WriteOptions {
    mode: PublicationMode::CreateNew,
    permissions: 0o600,
};

#[derive(Args)]
pub(crate) struct Input {
    #[command(flatten)]
    pub(super) target: Target,
    /// Single trusted custody root shared by all uploaders for this installation.
    #[arg(long)]
    custody: PathBuf,
}

#[derive(Args)]
pub(crate) struct Dispatch {
    #[command(flatten)]
    input: Input,
    /// Trusted Blob checkout with this binary's admitted tooling snapshot.
    #[arg(long)]
    checkout: PathBuf,
}

#[derive(Deserialize, Serialize)]
struct Intent {
    version: u8,
    url: String,
    canister: Principal,
    uploader: Principal,
    root: String,
    root_key_hash: [u8; 32],
    packet_hash: [u8; 32],
}

impl Input {
    pub(super) fn directory(&self, agent: &Agent) -> PathBuf {
        self.custody
            .join(hex(&Sha256::digest(agent.read_root_key())))
            .join(self.target.canister.to_text())
            .join(&self.target.root[7..])
    }
}

fn publish(path: &Path, bytes: &[u8]) -> Result<(), String> {
    durable::write_bytes(path, OPTIONS, bytes)
        .map_err(|error| format!("publication failed; preserve certificate custody: {error}"))
}

pub(crate) fn prepare(input: &Input) -> Result<String, String> {
    let agent = input.target.agent()?;
    let signed = agent
        .update(&input.target.effective()?.as_principal(), METHOD)
        .with_arg(
            encode_args(&(input.target.root.clone(),), 1024).map_err(|_| "cannot encode root")?,
        )
        .expire_after(Duration::from_secs(120))
        .sign()
        .map_err(|_| "cannot sign original certificate request")?;
    if signed.signed_update.len() > PACKET_LIMIT {
        return Err("signed request exceeds custody budget".into());
    }
    let intent = Intent {
        version: 1,
        url: input.target.network.url.to_string(),
        canister: input.target.canister,
        uploader: input.target.uploader,
        root: input.target.root.clone(),
        root_key_hash: Sha256::digest(agent.read_root_key()).into(),
        packet_hash: Sha256::digest(&signed.signed_update).into(),
    };
    let bytes = ic_wire::encode(&intent, ic_wire::Encoding::Payload, LIMITS)
        .map_err(|_| "cannot encode certificate intent")?;
    let directory = input.directory(&agent);
    durable::prepare_private_parents(directory.parent().ok_or("missing custody parent")?)
        .map_err(|_| "cannot admit certificate custody")?;
    // Any existing claim, including a partial preparation, permanently refuses.
    let _claim = durable::create_private_directory(&directory)
        .map_err(|_| "certificate root already claimed or custody unavailable; preserve it, do not reprepare")?;
    publish(&directory.join("intent.wire"), &bytes)?;
    publish(&directory.join("request.cbor"), &signed.signed_update)?;
    Ok(format!(
        "Prepared original request={} without submission.\nRetain request at {:?}; expires in 120 seconds, never renew it.",
        signed.request_id,
        directory.join("request.cbor")
    ))
}

pub(super) fn load(
    input: &Input,
    agent: &Agent,
) -> Result<(Vec<u8>, ic_platform::agent::RequestId), String> {
    let target = &input.target;
    let directory = input.directory(agent);
    let intent: Intent = ic_wire::decode(
        &crate::read(&directory.join("intent.wire"), LIMITS.bytes)?,
        ic_wire::Encoding::Payload,
        LIMITS,
    )
    .map_err(|_| "invalid retained certificate intent")?;
    let packet = crate::read(&directory.join("request.cbor"), PACKET_LIMIT)?;
    if intent.version != 1
        || intent.url != target.network.url.as_str()
        || intent.canister != target.canister
        || intent.uploader != target.uploader
        || intent.root != target.root
        || intent.root_key_hash != <[u8; 32]>::from(Sha256::digest(agent.read_root_key()))
        || intent.packet_hash != <[u8; 32]>::from(Sha256::digest(&packet))
    {
        return Err("certificate custody does not match original bindings and packet".into());
    }
    let id = original(&packet, target.uploader, target.canister, &target.root)?;
    Ok((packet, id))
}

pub(crate) async fn dispatch(input: Dispatch) -> Result<String, String> {
    let agent = input.input.target.agent()?;
    let target = &input.input.target;
    let directory = input.input.directory(&agent);
    let (packet, id) = load(&input.input, &agent)?;
    let EnvelopeContent::Call { ingress_expiry, .. } =
        decode_call_content_strict(&packet, PACKET_LIMIT)
            .map_err(|_| "invalid retained request")?
    else {
        return Err("retained request is not a call".into());
    };
    crate::management::admit_tooling(&input.checkout).await?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "invalid local clock")?
        .as_nanos();
    if u128::from(ingress_expiry) <= now {
        return Err("original request expired; preserve custody, never renew or reprepare".into());
    }
    // CreateNew is the cross-process arbitration point. Even an empty/partial
    // marker bars another invocation; no outcome can remove this claim.
    publish(&directory.join("dispatch.claim"), &id.to_vec())?;
    // The Agent may internally retry HTTP 429/503 with identical signed bytes.
    // This is one application dispatch, not a promise of one HTTP attempt.
    agent.update_signed(target.effective()?, packet).await
        .map_err(|_| "certificate dispatch outcome uncertain; preserve custody and recover the original request, never resubmit")?;
    Ok(format!(
        "Original request={id} dispatched; preserve custody and use recover-certificate.\nNo provider acceptance or availability is established."
    ))
}
