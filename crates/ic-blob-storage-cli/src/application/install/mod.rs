//! Fresh-install request bindings and offline readback; execution retains these exact bytes.

pub(super) mod execution;
#[cfg(test)]
mod tests;

use crate::{
    files, network,
    overview::{Network, hex, principal},
};
use clap::{Args, ValueEnum};
use ic_blob_storage::protocol::{Installation as BlobInstallation, RPC_LIMITS};
use ic_blob_storage_reference_app::protocol::Installation as AppInstallation;
use ic_platform::{
    ingress::{Content, decode_envelope},
    native::PreparedUpdate,
};
use ic_platform_protocol::management::{InstallCodeArgs, InstallMode};
use ic_platform_types::Principal;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const PACKET_BYTES: usize = 2 * 1024 * 1024;
const INTENT_LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 8192,
    allocation: 32_768,
    nodes: 1024,
    depth: 16,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
enum Role {
    Blob,
    App,
}

#[derive(Args)]
#[command(
    after_help = "Retain both init files, selected Wasm and the single shared custody root. Preparation and inspection are offline; inspection needs no --identity. Preparation permanently claims this network/target, including on partial failure. Changing paths must not renew it. Only dispatch-install sends the original request; observe-install never resubmits."
)]
pub(crate) struct Input {
    #[command(flatten)]
    network: Network,
    /// Which member of the paired installation this one request installs.
    #[arg(long, value_enum)]
    role: Role,
    /// Proposed reference-application canister.
    #[arg(long, value_parser = principal)]
    canister: Principal,
    /// Proposed standalone Blob canister.
    #[arg(long, value_parser = principal)]
    blob_canister: Principal,
    /// Expected signing controller; not inferred from a retained request.
    #[arg(long, value_parser = principal)]
    controller: Principal,
    /// Directory containing the original blob-init.wire and app-init.wire.
    #[arg(long)]
    inputs: PathBuf,
    /// Reviewed uncompressed Wasm for the selected role, at most 1 MiB.
    #[arg(long)]
    wasm: PathBuf,
    /// Single operator-owned custody root shared across preparation invocations.
    #[arg(long)]
    custody: PathBuf,
}

#[derive(Deserialize, Eq, PartialEq, Serialize)]
struct Binding {
    url: String,
    root_hash: [u8; 32],
    root_subnet: Principal,
    controller: Principal,
    app: Principal,
    blob: Principal,
    role: Role,
    wasm_hash: [u8; 32],
    blob_init_hash: [u8; 32],
    app_init_hash: [u8; 32],
}

#[derive(Deserialize, Serialize)]
struct Intent {
    version: u8,
    binding: Binding,
    packet_hash: [u8; 32],
}

struct Selection {
    binding: Binding,
    target: Principal,
    argument: Vec<u8>,
    directory: PathBuf,
    configuration: String,
}

fn decode<T: DeserializeOwned>(method: &str, bytes: &[u8]) -> Result<T, String> {
    ic_wire::decode(
        bytes,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "invalid method-bound installation input".into())
}

impl Input {
    fn selection(&self) -> Result<Selection, String> {
        let (root, root_subnet) = self.network.trust()?;
        // Admit the key without constructing a transport or loading an identity.
        ic_platform::certificate::PreparedRootKey::from_der(&root)
            .map_err(|_| "invalid trusted root key")?;
        let inputs = files::directory(&self.inputs, false)?;
        let blob_bytes = files::read_at(&inputs, "blob-init.wire".as_ref(), RPC_LIMITS.bytes)?;
        let app_bytes = files::read_at(&inputs, "app-init.wire".as_ref(), RPC_LIMITS.bytes)?;
        let blob: BlobInstallation = decode("blob_init", &blob_bytes)?;
        let app: AppInstallation = decode("app_init", &app_bytes)?;
        super::prepare::validate_roles(
            self.canister,
            self.blob_canister,
            blob.manager,
            &app.users,
        )?;
        if blob.application != self.canister
            || app.blob != self.blob_canister
            || blob.limits.uploads.get() > ic_blob_storage::local::persistence::MAX_RECORDS
            || blob.limits.references.get() > ic_blob_storage::local::persistence::MAX_RECORDS
        {
            return Err(
                "installation inputs do not match the selected pair or supported capacity".into(),
            );
        }
        let wasm = crate::read(&self.wasm, 1024 * 1024)?;
        if !wasm.starts_with(b"\0asm\x01\0\0\0") {
            return Err("install requires an uncompressed Wasm module".into());
        }
        let binding = Binding {
            url: self.network.url.to_string(),
            root_hash: Sha256::digest(&root).into(),
            root_subnet,
            controller: self.controller,
            app: self.canister,
            blob: self.blob_canister,
            role: self.role,
            wasm_hash: Sha256::digest(&wasm).into(),
            blob_init_hash: Sha256::digest(&blob_bytes).into(),
            app_init_hash: Sha256::digest(&app_bytes).into(),
        };
        let (target, arg) = match self.role {
            Role::Blob => (self.blob_canister, blob_bytes),
            Role::App => (self.canister, app_bytes),
        };
        let argument = InstallCodeArgs {
            mode: InstallMode::Install,
            canister_id: target,
            wasm_module: wasm,
            arg,
            sender_canister_version: None,
        }
        .encode(PACKET_BYTES)
        .map_err(|_| "cannot encode bounded fresh install")?;
        // Endpoint aliases, signer changes and role changes cannot acquire another
        // claim for the same network/target. Separate roots are not retry permission.
        let directory = self
            .custody
            .join(hex(&binding.root_hash))
            .join(target.to_text())
            .join("install");
        Ok(Selection {
            binding,
            target,
            argument,
            directory,
            configuration: format!(
                "Manager: {}\nUsers: {:?}\nBlob limits: {:?}",
                blob.manager,
                app.users.iter().map(Principal::to_text).collect::<Vec<_>>(),
                blob.limits
            ),
        })
    }
}

impl Selection {
    fn report(&self, signed: &PreparedUpdate) -> Result<String, String> {
        Ok(format!(
            "Offline signed installation\nRole: {:?}\nTarget: {}\nApplication: {}\nBlob: {}\nController: {}\nEndpoint: {:?}\nRoot subnet: {}\nRoot key SHA-256: {}\nMode: install\nWasm SHA-256: {}\nBlob init SHA-256: {}\nApp init SHA-256: {}\n{}\nRequest ID: {}\nExpiry (Unix nanoseconds): {}\nExpired: {}\nRecord: {:?}\n\nExact signature and input bindings checked. This command submits nothing and does not establish prior execution or installed state. Root, endpoint and routing are local bindings, not signed ingress fields. Expiry never renews; preserve the original claim. No target freshness, controller authority, artifact suitability or provider acceptance is established.",
            self.binding.role,
            self.target,
            self.binding.app,
            self.binding.blob,
            self.binding.controller,
            self.binding.url,
            self.binding.root_subnet,
            hex(&self.binding.root_hash),
            hex(&self.binding.wasm_hash),
            hex(&self.binding.blob_init_hash),
            hex(&self.binding.app_init_hash),
            self.configuration,
            hex(signed.request_id()),
            signed.expiry(),
            signed.expiry() <= network::now()?,
            self.directory,
        ))
    }
}

pub(super) fn prepare(input: &Input) -> Result<String, String> {
    let selected = input.selection()?;
    let client = input.network.client()?;
    // Recheck trust if a root file changed between reads, before signing/publishing.
    if client.principal() != input.controller
        || <[u8; 32]>::from(Sha256::digest(&client.root)) != selected.binding.root_hash
    {
        return Err("signer or trusted root differs from the selected installation".into());
    }
    let signed = client.prepare(
        Principal::management_canister(),
        selected.target,
        "install_code",
        &selected.argument,
    )?;
    let intent = Intent {
        version: 1,
        packet_hash: Sha256::digest(signed.envelope_bytes()).into(),
        binding: selected.binding,
    };
    let bytes = ic_wire::encode(&intent, ic_wire::Encoding::Payload, INTENT_LIMITS)
        .map_err(|_| "cannot encode installation intent")?;
    let selected = Selection {
        binding: intent.binding,
        ..selected
    };
    let report = selected.report(&signed)?;
    files::directory(
        selected
            .directory
            .parent()
            .ok_or("missing custody parent")?,
        true,
    )?;
    // Claim before publication; any partial failure remains occupied forever.
    let held = files::claim(&selected.directory)?;
    files::publish(&held, "intent.wire".as_ref(), &bytes)?;
    files::publish(&held, "request.cbor".as_ref(), signed.envelope_bytes())?;
    Ok(report)
}

pub(super) fn inspect(input: &Input) -> Result<String, String> {
    let selected = input.selection()?;
    let (_, signed) = load(input, &selected)?;
    selected.report(&signed)
}

fn load(
    input: &Input,
    selected: &Selection,
) -> Result<(ic_host_fs::Directory, PreparedUpdate), String> {
    let held = files::directory(&selected.directory, false)?;
    let intent: Intent = ic_wire::decode(
        &files::read_at(&held, "intent.wire".as_ref(), INTENT_LIMITS.bytes)?,
        ic_wire::Encoding::Payload,
        INTENT_LIMITS,
    )
    .map_err(|_| "invalid retained installation intent")?;
    let packet = files::read_at(&held, "request.cbor".as_ref(), PACKET_BYTES)?;
    if intent.version != 1
        || intent.binding != selected.binding
        || intent.packet_hash != <[u8; 32]>::from(Sha256::digest(&packet))
    {
        return Err("installation custody does not match the selected inputs".into());
    }
    // Platform verifies the signature over the exact original bytes, including
    // after expiry. Matching only a saved digest is not signature verification.
    let signed = PreparedUpdate::from_bytes(&packet, selected.target, PACKET_BYTES)
        .map_err(|_| "invalid original installation signature or envelope")?;
    let Content::Call {
        method_name,
        arg,
        nonce,
        ..
    } = decode_envelope(&packet, PACKET_BYTES)
        .map_err(|_| "invalid original installation content")?
        .value
        .content
    else {
        return Err("installation request is not a call".into());
    };
    if signed.sender() != input.controller
        || signed.recipient() != Principal::management_canister()
        || signed.expiry() == 0
        || method_name != "install_code"
        || arg != selected.argument
        || nonce.is_some()
    {
        return Err("signed request differs from the exact fresh installation".into());
    }
    Ok((held, signed))
}
