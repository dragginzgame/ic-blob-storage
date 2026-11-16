//! Read-only remote observations; no management or provider-effect authority.

use clap::Args;
use ic_blob_storage::{
    local::Error,
    protocol::{Inspection, Observation, RPC_LIMITS},
};
use ic_platform_legacy::{
    Principal,
    agent::{Agent, BasicIdentity},
};
use std::{fmt::Write, path::PathBuf};
use url::Url;

#[derive(Args)]
pub(crate) struct Network {
    /// IC API endpoint (HTTPS, or loopback HTTP with --root-key).
    #[arg(long, value_parser = crate::endpoint)]
    pub(crate) url: Url,
    /// Optional Ed25519 PKCS#8 PEM. Omission uses anonymous query access.
    #[arg(long)]
    identity: Option<PathBuf>,
    /// Independently trusted BLS DER key. Omission uses the built-in IC mainnet key.
    #[arg(long)]
    root_key: Option<PathBuf>,
}

impl Network {
    pub(crate) fn agent(&self) -> Result<Agent, String> {
        if self.url.scheme() == "http" && self.root_key.is_none() {
            return Err("loopback inspection requires an independently trusted --root-key".into());
        }
        let mut builder = Agent::builder()
            .with_url(self.url.as_str())
            .with_verify_query_signatures(true)
            .with_max_response_body_size(2 * 1024 * 1024)
            .with_max_tcp_error_retries(0);
        if let Some(path) = &self.identity {
            let signer = BasicIdentity::from_pem(crate::read(path, 16 * 1024)?)
                .map_err(|_| "invalid Ed25519 identity PEM")?;
            builder = builder.with_identity(signer);
        }
        let agent = builder
            .build()
            .map_err(|_| "cannot configure IC transport")?;
        if let Some(path) = &self.root_key {
            agent.set_root_key(crate::read(path, 1024)?);
        }
        Ok(agent)
    }
}

pub(crate) fn principal(value: &str) -> Result<Principal, String> {
    let value = Principal::from_text(value).map_err(|_| "invalid principal")?;
    if value == Principal::anonymous() || value == Principal::management_canister() {
        return Err("expected a non-special principal".into());
    }
    Ok(value)
}

#[derive(Args)]
pub(crate) struct Input {
    #[command(flatten)]
    network: Network,
    /// Blob installation to inspect. The signer must be its application authority.
    #[arg(long, value_parser = principal)]
    canister: Principal,
    /// Bound uploader to select; does not confer inspection rights.
    #[arg(long, value_parser = principal)]
    principal: Principal,
}

pub(crate) async fn run(input: Input) -> Result<String, String> {
    let request = ic_wire::encode(
        &Inspection::Overview(ic_platform_types::Principal::from_slice(
            input.principal.as_slice(),
        )),
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_query"),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "cannot encode overview")?;
    let target = input
        .canister
        .as_slice()
        .try_into()
        .map_err(|_| "invalid canister")?;
    let reply = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        input
            .network
            .agent()?
            .query(&target, "blob_query")
            .with_arg(request)
            .call(),
    )
    .await
    .map_err(|_| "Blob query timed out; storage is unknown")?
    .map_err(|error| format!("Blob query failed: {:?}", error.to_string()))?;
    let result: Result<Observation, Error> = ic_wire::decode(
        &reply,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_query"),
            direction: ic_wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "invalid Blob overview reply (host may not support this command)")?;
    let Observation::Overview(view) =
        result.map_err(|error| format!("Blob overview refused: {error:?}"))?
    else {
        return Err("unexpected Blob observation".into());
    };
    if view.uploader.as_slice() != input.principal.as_slice() {
        return Err("overview principal mismatch".into());
    }
    let mut output = format!(
        "Blob installation: {}\nUploader: {}\nSource: {}\nLocal history only; provider storage not observed.\n\nDistinct associated roots: {}\nMatching uploads: {}\nApplication reference names on these roots: {}\nDeclared bytes: reserved={} live={} unresolved={}\nShared roots overlap other uploader views; do not add these counters.\n",
        input.canister,
        view.uploader,
        input.network.url,
        view.objects.len(),
        view.usage.uploads,
        view.usage.references,
        view.usage.reserved_bytes,
        view.usage.live_bytes,
        view.usage.unresolved_bytes
    );
    for entry in view.objects {
        let object = entry.object;
        let original = object.original_upload.request;
        writeln!(output, "\nRoot: {}\n  Declared bytes: {} | retention: {:?} | live references: {}\n  Original upload: {} | original uploader: {}",
            hex(&original.root), original.bytes, object.retention, object.live_references,
            original.id, original.uploader).unwrap();
        for upload in entry.uploads {
            writeln!(
                output,
                "  Upload {}: {:?} | first reference: {}",
                upload.request.id, upload.state, upload.request.first_reference
            )
            .unwrap();
        }
        for (reference, state) in entry.references {
            writeln!(output, "  Application reference {reference}: {state:?}").unwrap();
        }
    }
    output.push_str(
        "\nQuery observation, not consensus-certified evidence or a provider availability claim.",
    );
    Ok(output)
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut text, byte| {
            write!(text, "{byte:02x}").unwrap();
            text
        })
}
