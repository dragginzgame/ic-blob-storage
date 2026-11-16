//! Original certificate-request custody and authenticated historical recovery.

pub(crate) mod custody;
pub(crate) mod transfer;

use crate::files;
use crate::network::Client;
use crate::overview::{Network, principal};
use clap::Args;
use ic_platform::ingress::{Content, decode_envelope};
use ic_platform::native::{PreparedUpdate, RequestStatus};
use ic_platform_protocol::{caffeine as c, candid::Limits};
use ic_platform_types::Principal;
use std::path::PathBuf;

const METHOD: &str = "_immutableObjectStorageCreateCertificate";
const PACKET_LIMIT: usize = 8192;
const CERTIFICATE_LIMIT: usize = 128 * 1024;
const REPLY_LIMITS: Limits = Limits {
    bytes: 1024,
    header: 512,
    types: 8,
    work: 65_536,
    allocation: 65_536,
    vector: 1024,
    skipping: 65_536,
};

#[derive(Args)]
pub(crate) struct Input {
    #[command(flatten)]
    target: Target,
    /// Already retained signed ingress CBOR. Never modified or submitted.
    #[arg(long)]
    request: PathBuf,
    /// New certificate CBOR file under trusted parent custody; never overwritten.
    #[arg(long)]
    output: PathBuf,
}

#[derive(Args)]
pub(crate) struct Target {
    #[command(flatten)]
    network: Network,
    /// Original issuer, independently checked against the retained request.
    #[arg(long, value_parser = principal)]
    canister: Principal,
    /// Original uploader. --identity must sign as this principal.
    #[arg(long, value_parser = principal, requires = "identity")]
    uploader: Principal,
    /// Trusted original content root: sha256: followed by 64 lowercase hex digits.
    #[arg(long, value_parser = root)]
    root: String,
}

impl Target {
    fn client(&self) -> Result<Client, String> {
        let client = self.network.client()?;
        if client.principal() != self.uploader {
            return Err("certificate identity must match original uploader".into());
        }
        Ok(client)
    }
}

fn root(value: &str) -> Result<String, String> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err("expected canonical sha256 root".into());
    };
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("expected 64 lowercase hexadecimal root digits".into());
    }
    Ok(value.to_owned())
}

fn original(
    packet: &[u8],
    uploader: Principal,
    canister: Principal,
    root: &str,
) -> Result<[u8; 32], String> {
    let content = decode_envelope(packet, PACKET_LIMIT)
        .map_err(|_| "invalid retained ingress content")?
        .value
        .content;
    let Content::Call {
        sender,
        canister_id,
        method_name,
        arg,
        sender_info,
        ..
    } = &content
    else {
        return Err("retained ingress is not a call".into());
    };
    let expected =
        c::encode_root(&root.to_owned(), 1024).map_err(|_| "cannot encode expected root")?;
    if sender.as_slice() != uploader.as_slice()
        || canister_id.as_slice() != canister.as_slice()
        || method_name != METHOD
        || *arg != expected
        || sender_info.is_some()
    {
        return Err("retained request does not match uploader, issuer, method and root".into());
    }
    // Content inspection is not signature verification. Certified request_status
    // establishes the outcome. Expired original ingress remains inspectable.
    content
        .request_id()
        .map_err(|_| "cannot derive original request ID".into())
}

fn admit_reply(bytes: &[u8], root: &str) -> Result<(), String> {
    let reply =
        c::decode_certificate(bytes, REPLY_LIMITS).map_err(|_| "invalid certified upload reply")?;
    if reply.method != "upload" || reply.blob_hash != root {
        return Err("certified reply does not match upload and original root".into());
    }
    Ok(())
}

pub(crate) async fn run(input: Input) -> Result<String, String> {
    let agent = input.target.client()?;
    let packet = crate::read(&input.request, PACKET_LIMIT)?;
    let id = original(
        &packet,
        input.target.uploader,
        input.target.canister,
        &input.target.root,
    )?;
    let original = PreparedUpdate::from_bytes(&packet, input.target.canister, PACKET_LIMIT)
        .map_err(|_| "invalid retained signed request")?;
    let observation = agent.observe(&original).await?;
    let RequestStatus::Replied(reply) = observation.status() else {
        return Err("original request has no certified reply; never resubmit".into());
    };
    admit_reply(reply, &input.target.root)?;
    let bytes = observation.certificate_bytes();
    if bytes.len() > CERTIFICATE_LIMIT {
        return Err("certificate exceeds custody budget".into());
    }
    let (parent, name) = files::parent(&input.output)?;
    files::publish(&parent, name, bytes).map_err(|error| {
        format!("certificate publication failed; preserve original request and any output: {error}")
    })?;
    Ok(format!(
        "Recovered certified historical reply: request={} root={}\nCertificate saved; provider acceptance and current availability are not established.",
        crate::overview::hex(&id),
        input.target.root
    ))
}

#[cfg(test)]
mod tests;
