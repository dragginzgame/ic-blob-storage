//! One permanent claim per network/issuer/root in caller-owned trusted custody.

use super::{METHOD, PACKET_LIMIT, Target, original};
use crate::{files, network::Client, overview::hex};
use clap::Args;
use ic_host_fs::Directory;
use ic_platform::{
    ingress::{Content, decode_envelope},
    native::{Dispatch as NativeDispatch, PreparedUpdate, RequestStatus},
};
use ic_platform_protocol::caffeine as c;
use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

const LIMITS: usize = 8192;

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
    pub(super) fn directory(&self, agent: &Client) -> PathBuf {
        self.custody
            .join(hex(&Sha256::digest(&agent.root)))
            .join(self.target.canister.to_text())
            .join(&self.target.root[7..])
    }
}

pub(crate) fn prepare(input: &Input) -> Result<String, String> {
    let agent = input.target.client()?;
    let signed = agent.prepare(
        input.target.canister,
        input.target.canister,
        METHOD,
        &c::encode_root(&input.target.root, 1024).map_err(|_| "cannot encode root")?,
    )?;
    if signed.envelope_bytes().len() > PACKET_LIMIT {
        return Err("signed request exceeds custody budget".into());
    }
    let intent = Intent {
        version: 3,
        url: input.target.network.url.to_string(),
        canister: input.target.canister,
        uploader: input.target.uploader,
        root: input.target.root.clone(),
        root_key_hash: Sha256::digest(&agent.root).into(),
        packet_hash: Sha256::digest(signed.envelope_bytes()).into(),
    };
    let bytes = ic_blob_storage::storage_codec::encode(&intent, LIMITS)
        .map_err(|_| "cannot encode certificate intent")?;
    let directory = input.directory(&agent);
    let parent = files::directory(directory.parent().ok_or("missing custody parent")?, true)
        .map_err(|_| "cannot admit certificate custody")?;
    // Any existing claim, including a partial preparation, permanently refuses.
    let claim = parent.create_directory(directory.file_name().ok_or("missing root name")?, 0o700, 0o077)
        .map_err(|_| "certificate root already claimed or custody unavailable; preserve it, do not reprepare")?;
    files::publish(&claim, "intent.wire".as_ref(), &bytes)?;
    files::publish(&claim, "request.cbor".as_ref(), signed.envelope_bytes())?;

    Ok(format!(
        "Prepared original request={} without submission.\nRetain request at {:?}; expires in 120 seconds, never renew it.",
        hex(signed.request_id()),
        directory.join("request.cbor")
    ))
}

pub(super) fn load(
    input: &Input,
    agent: &Client,
) -> Result<(Directory, Vec<u8>, [u8; 32]), String> {
    let target = &input.target;
    let directory = input.directory(agent);
    let held = files::directory(&directory, false)?;
    let intent: Intent = ic_blob_storage::storage_codec::decode(
        &files::read_at(&held, "intent.wire".as_ref(), LIMITS)?,
        LIMITS,
        3,
    )
    .map_err(|_| "invalid retained certificate intent")?;
    let packet = files::read_at(&held, "request.cbor".as_ref(), PACKET_LIMIT)?;
    if intent.version != 3
        || intent.url != target.network.url.as_str()
        || intent.canister != target.canister
        || intent.uploader != target.uploader
        || intent.root != target.root
        || intent.root_key_hash != <[u8; 32]>::from(Sha256::digest(&agent.root))
        || intent.packet_hash != <[u8; 32]>::from(Sha256::digest(&packet))
    {
        return Err("certificate custody does not match original bindings and packet".into());
    }
    let id = original(&packet, target.uploader, target.canister, &target.root)?;

    Ok((held, packet, id))
}

pub(crate) async fn dispatch(input: Dispatch) -> Result<String, String> {
    let agent = input.input.target.client()?;
    let target = &input.input.target;
    let (held, packet, id) = load(&input.input, &agent)?;
    let Content::Call { ingress_expiry, .. } = decode_envelope(&packet, PACKET_LIMIT)
        .map_err(|_| "invalid retained request")?
        .value
        .content
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
    files::publish(&held, "dispatch.claim".as_ref(), &id)?;
    // One HTTP attempt with exact retained bytes; any uncertainty keeps the claim.
    let prepared = PreparedUpdate::from_bytes(&packet, target.canister, PACKET_LIMIT)
        .map_err(|_| "invalid original signed request")?;
    let result = agent.transport.dispatch(&prepared, crate::network::policy(false)?).await
        .map_err(|_| "certificate dispatch outcome uncertain; preserve custody and recover the original request, never resubmit")?;
    match result {
        NativeDispatch::UncertifiedRejection { .. } => {
            return Err(
                "uncertified dispatch rejection; preserve custody and recover the original request"
                    .into(),
            );
        }
        NativeDispatch::Observed(observed) => match observed.status() {
            RequestStatus::Replied(bytes) => super::admit_reply(bytes, &target.root)?,
            RequestStatus::Rejected { .. } => {
                return Err("original request rejected; preserve custody, no resubmission".into());
            }
            RequestStatus::Done | RequestStatus::Unavailable => {
                return Err(
                    "original outcome unavailable; preserve custody, no resubmission".into(),
                );
            }
            RequestStatus::Unknown | RequestStatus::Received | RequestStatus::Processing => {}
        },
        NativeDispatch::Accepted => {}
    }

    Ok(format!(
        "Original request={} dispatched; preserve custody and use blob certificate recover.\nNo provider acceptance or availability is established.",
        hex(&id)
    ))
}
