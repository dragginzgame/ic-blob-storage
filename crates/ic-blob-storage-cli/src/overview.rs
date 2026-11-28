//! Read-only remote observations; no management or provider-effect authority.

use crate::network::Client;
use clap::Args;
use ic_blob_storage::{
    local::Error,
    protocol::{Inspection, Observation, RPC_LIMITS},
};
use ic_platform_types::Principal;
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
    /// Independently trusted root subnet; required with a custom root key.
    #[arg(long, requires = "root_key")]
    root_subnet: Option<Principal>,
}

impl Network {
    pub(crate) fn trust(&self) -> Result<(Vec<u8>, Principal), String> {
        if self.url.scheme() == "http" && self.root_key.is_none() {
            return Err("loopback inspection requires an independently trusted --root-key".into());
        }
        let trust = match &self.root_key {
            Some(path) => (
                crate::read(path, 1024)?,
                self.root_subnet
                    .ok_or("custom root requires --root-subnet")?,
            ),
            // Public trust data from ic-agent 0.49.3 and its NNS subnet constant.
            None => (
                include_bytes!("mainnet-root.der").to_vec(),
                Principal::from_text(
                    "tdb26-jop6k-aogll-7ltgs-eruif-6kk7m-qpktf-gdiqx-mxtrf-vb5e6-eqe",
                )
                .unwrap(),
            ),
        };

        Ok(trust)
    }

    pub(crate) fn client(&self) -> Result<Client, String> {
        let (key, subnet) = self.trust()?;
        let pem = self
            .identity
            .as_ref()
            .map(|p| crate::read(p, 16 * 1024))
            .transpose()?;

        Client::new(self.url.as_str(), key, subnet, pem.as_deref())
    }
}

pub(crate) fn is_special_principal(value: Principal) -> bool {
    value == Principal::anonymous() || value == Principal::management_canister()
}

pub(crate) fn principal(value: &str) -> Result<Principal, String> {
    let value = Principal::from_text(value).map_err(|_| "invalid principal")?;
    if is_special_principal(value) {
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
    let request = ic_blob_storage::protocol::frame::encode(
        &Inspection::Overview(input.principal),
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_query"),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "cannot encode overview")?;
    let target = input.canister;
    let reply = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        input
            .network
            .client()?
            .query(target, "blob_query", &request),
    )
    .await
    .map_err(|_| "Blob query timed out; storage is unknown")?
    .map_err(|error| format!("Blob query failed: {:?}", error))?;
    let result: Result<Observation, Error> = ic_blob_storage::protocol::frame::decode(
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
    if view.uploader != input.principal {
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
