//! Original certificate-request custody and authenticated historical recovery.

pub(crate) mod custody;
pub(crate) mod transfer;

use crate::files;
use crate::overview::{Network, principal};
use clap::Args;
use ic_platform::{
    Principal,
    agent::{Agent, EffectiveId, RequestId, RequestStatusResponse, to_request_id},
    candid_codec::{
        arguments::encode_args,
        decoder::{Arity, Limits, decode_args},
    },
    certification::cbor::encode_certificate,
    ic_protocol::{EnvelopeContent, decode_call_content_strict},
};
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
    fn agent(&self) -> Result<Agent, String> {
        let agent = self.network.agent()?;
        if agent
            .get_principal()
            .map_err(|_| "cannot identify signer")?
            .as_slice()
            != self.uploader.as_slice()
        {
            return Err("certificate identity must match original uploader".into());
        }
        Ok(agent)
    }

    fn effective(&self) -> Result<EffectiveId, String> {
        Ok(EffectiveId::Canister(
            self.canister
                .as_slice()
                .try_into()
                .map_err(|_| "invalid issuer")?,
        ))
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
) -> Result<RequestId, String> {
    let content = decode_call_content_strict(packet, PACKET_LIMIT)
        .map_err(|_| "invalid retained ingress content")?;
    let EnvelopeContent::Call {
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
        encode_args(&(root.to_owned(),), 1024).map_err(|_| "cannot encode expected root")?;
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
    to_request_id(&content).map_err(|_| "cannot derive original request ID".into())
}

ic_platform::candid_record! {
    struct Reply {
        156956385 => method: String,
        278101296 => blob_hash: String,
    }
}

fn admit_reply(bytes: &[u8], root: &str) -> Result<(), String> {
    let (reply,): (Reply,) = decode_args(bytes, REPLY_LIMITS, Arity::Exact)
        .map_err(|_| "invalid certified upload reply")?;
    if reply.method != "upload" || reply.blob_hash != root {
        return Err("certified reply does not match upload and original root".into());
    }
    Ok(())
}

pub(crate) async fn run(input: Input) -> Result<String, String> {
    let agent = input.target.agent()?;
    let packet = crate::read(&input.request, PACKET_LIMIT)?;
    let id = original(
        &packet,
        input.target.uploader,
        input.target.canister,
        &input.target.root,
    )?;
    let target = input.target.effective()?;
    let (status, certificate) = agent.request_status_raw(&id, target).await
        .map_err(|_| "cannot authenticate original request status; preserve original request, do not reissue")?;
    let RequestStatusResponse::Replied(reply) = status else {
        return Err(
            "original request has no recoverable reply; preserve original request, do not reissue"
                .into(),
        );
    };
    admit_reply(&reply.arg, &input.target.root)?;
    let bytes = encode_certificate(&certificate, CERTIFICATE_LIMIT)
        .map_err(|_| "authenticated certificate exceeds output encoding budget")?;
    let (parent, name) = files::parent(&input.output)?;
    files::publish(&parent, name, &bytes).map_err(|error| {
        format!("certificate publication failed; preserve original request and any output: {error}")
    })?;
    Ok(format!(
        "Recovered certified historical reply: request={id} root={}\nCertificate saved; provider acceptance and current availability are not established.",
        input.target.root
    ))
}

#[cfg(test)]
mod tests;
