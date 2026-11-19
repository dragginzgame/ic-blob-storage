//! Offline proof authentication and permanent custody for one provider transfer.

use super::{CERTIFICATE_LIMIT, admit_reply, custody, root};
use crate::files;
use clap::Args;
use ic_platform::native::{PreparedUpdate, RequestStatus};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

#[derive(Args)]
#[group(id = "transfer")]
pub(crate) struct Input {
    #[command(flatten)]
    original: custody::Input,
    /// Retained certificate for this original request, not a plain registration reply.
    #[arg(long)]
    certificate: PathBuf,
    /// Exact certificate digest from the client's frozen transfer plan (sha256:...).
    #[arg(long, value_parser = root)]
    certificate_sha256: String,
    /// Selected provider origin; no path, redirect or implicit account discovery.
    #[arg(long, value_parser = gateway)]
    gateway: String,
    /// Explicit provider project binding, independently admitted by the application.
    #[arg(long, value_parser = selector)]
    project: String,
    /// Explicit provider bucket binding, independently admitted by the application.
    #[arg(long, value_parser = selector)]
    bucket: String,
}

fn gateway(value: &str) -> Result<String, String> {
    let url = crate::endpoint(value)?;
    if url.path() != "/" {
        return Err("provider gateway must be an origin without a path".into());
    }

    Ok(url.origin().ascii_serialization())
}

fn selector(value: &str) -> Result<String, String> {
    if value.is_empty() || value.len() > 128 || !value.bytes().all(|b| b.is_ascii_graphic()) {
        return Err("provider selector must contain 1..128 printable ASCII bytes".into());
    }

    Ok(value.to_owned())
}

#[derive(Serialize)]
struct Claim<'a> {
    version: u8,
    original_request: &'a [u8],
    gateway: &'a str,
    project: &'a str,
    bucket: &'a str,
    certificate_sha256: [u8; 32],
}

pub(crate) fn claim(input: &Input) -> Result<String, String> {
    let target = &input.original.target;
    let agent = target.client()?;
    let (held, packet, id) = custody::load(&input.original, &agent)?;
    if files::read_at(&held, "dispatch.claim".as_ref(), 32)? != id.to_vec() {
        return Err("original dispatch custody is incomplete; preserve it".into());
    }
    let bytes = crate::read(&input.certificate, CERTIFICATE_LIMIT)?;
    let hash: [u8; 32] = Sha256::digest(&bytes).into();
    if format!("sha256:{}", crate::overview::hex(&hash)) != input.certificate_sha256 {
        return Err("certificate differs from the client's transfer plan".into());
    }
    let original = PreparedUpdate::from_bytes(&packet, target.canister, super::PACKET_LIMIT)
        .map_err(|_| "invalid retained signed request")?;
    let observed = agent
        .transport
        .authenticate_request(&original, &bytes, crate::network::policy(true)?)
        .map_err(|_| "retained certificate authentication failed")?;
    let RequestStatus::Replied(reply) = observed.status() else {
        return Err("certificate has no reply".into());
    };
    admit_reply(reply, &target.root)?;
    let intent = ic_wire::encode(
        &Claim {
            version: 1,
            original_request: id.as_ref(),
            gateway: &input.gateway,
            project: &input.project,
            bucket: &input.bucket,
            certificate_sha256: hash,
        },
        ic_wire::Encoding::Payload,
        ic_wire::Limits {
            bytes: 4096,
            allocation: 32_768,
            nodes: 1024,
            depth: 16,
        },
    )
    .map_err(|_| "cannot encode bounded transfer claim")?;
    // The directory is the irreversible claim. A partial publication still bars
    // another certificate, gateway or process from creating a second attempt.
    let claim = held
        .create_directory("transfer".as_ref(), 0o700, 0o077)
        .map_err(|_| "transfer already claimed or custody unavailable; preserve it, never retry")?;
    for (name, data) in [
        ("intent.wire", intent.as_slice()),
        ("certificate.cbor", bytes.as_slice()),
    ] {
        files::publish(&claim, name.as_ref(), data).map_err(|error| {
            format!("transfer custody publication failed; preserve claim: {error}")
        })?;
    }

    Ok(format!(
        "Transfer claim retained for original request={}; no network call made.\nThis authenticates historical proof, not current application permission or provider acceptance.",
        crate::overview::hex(&id)
    ))
}
