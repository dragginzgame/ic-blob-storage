//! Freeze a bounded, caller-allocated inventory without service/provider effects.
mod capacity;
mod paths;
use super::parsing::positive;
use super::upload_inputs::installation::Installation;
use super::{
    Failure,
    artifacts::{FailureRecord, Run},
    exact_wire, read,
    upload_inputs::{BINDING_BYTES, InputFiles, MANIFEST_BYTES, PreparedInput, digest},
};
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;
use ic_host_artifacts::artifact::Sha256Digest;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{num::NonZeroU64, path::Path, rc::Rc};

#[cfg(test)]
mod tests;

const INVENTORY_BYTES: u64 = 2 * 1024 * 1024;
const METADATA_BYTES: usize = 16 * 1024 * 1024;
pub(super) const MAX_FILES: usize = 4096;
pub(super) const MAX_FILE_BYTES: u64 = 1024 * 1024 * 1024;
pub(super) const MAX_TOTAL_BYTES: u64 = 1024 * 1024 * 1024 * 1024;
pub(super) const MAX_TIMEOUT_SECONDS: u64 = 3600;

/// Recovery binding shared by the summary writer and frozen-input reader.
/// Other summary fields are presentation, not permission to skip verification.
#[derive(Deserialize, Serialize)]
struct FrozenBatchRecord {
    schema: u8,
    observation: String,
    inventory_sha256: String,
    installation_sha256: String,
    all_bodies_verified: bool,
}

/// One complete frozen batch; caller-supplied identities remain proposals.
pub(super) struct PreparedBatch {
    pub files: Vec<FrozenFile>,
    pub inventory: Vec<u8>,
    pub installation: Rc<Installation>,
}
/// Keep the verified input and its original expected raw digest inseparable.
pub(super) struct FrozenFile {
    pub input: PreparedInput,
    pub body_sha256: Sha256Digest,
}
impl PreparedBatch {
    /// Borrow the candidate admitted from the exact retained installation packet.
    /// This passive candidate does not authenticate installed service state.
    pub fn installation(&self) -> &ServiceInstallationInput {
        &self.installation.input
    }

    pub fn open_frozen(
        directory: &Path,
        maximum: NonZeroU64,
        total: NonZeroU64,
    ) -> Result<Self, Failure> {
        let root = paths::Root::new(directory)?;
        let inventory_bytes = root.read("inventory.json", INVENTORY_BYTES)?;
        let installation = root.read("installation.wire", exact_wire::INSTALLATION_BYTES as u64)?;
        let summary: Value = serde_json::from_slice(&root.read("summary.json", 8 * 1024 * 1024)?)
            .map_err(|_| Failure::Arguments)?;
        let summary: FrozenBatchRecord =
            serde_json::from_value(summary).map_err(|_| Failure::Binding)?;
        if summary.schema != 1
            || summary.observation != "local_publish_inputs"
            || !summary.all_bodies_verified
            || summary.inventory_sha256 != digest(&inventory_bytes)
            || summary.installation_sha256 != digest(&installation)
        {
            return Err(Failure::Binding);
        }
        let installation = Installation::decode(installation, maximum)?;
        let mut inventory = Inventory::decode(&inventory_bytes)?;
        for (index, entry) in inventory.files.iter_mut().enumerate() {
            let prefix = format!("file-{index:04}");
            entry.binding = format!("{prefix}/binding.json");
            entry.manifest = format!("{prefix}/manifest.json");
            entry.body = format!("{prefix}/body.bin");
        }
        let (inputs, _) = preflight(&inventory, &root, &installation, total)?;
        for (index, (input, entry)) in inputs.iter().zip(&inventory.files).enumerate() {
            input.verify_body(&entry.body_sha256)?;
            let load = |name: &str, maximum| root.read(&format!("file-{index:04}/{name}"), maximum);
            input.check_requests(
                &load("permission.wire", 4096)?,
                &load("manifest.wire", 65536)?,
                &load("certificate-binding.json", 65536)?,
                &load("installation.wire", exact_wire::INSTALLATION_BYTES as u64)?,
            )?;
        }
        Ok(Self {
            files: inputs
                .into_iter()
                .zip(inventory.files)
                .map(|(input, entry)| FrozenFile {
                    input,
                    body_sha256: entry.body_sha256,
                })
                .collect(),
            inventory: inventory_bytes,
            installation,
        })
    }
}

/// File names and hashes are frozen input, never authority or allocated identities.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Inventory {
    schema: u8,
    files: Vec<Entry>,
}
impl Inventory {
    fn decode(bytes: &[u8]) -> Result<Self, Failure> {
        let inventory: Self = serde_json::from_slice(bytes).map_err(|_| Failure::Arguments)?;
        if inventory.schema != 1 || inventory.files.is_empty() || inventory.files.len() > MAX_FILES
        {
            return Err(Failure::Arguments);
        }
        Ok(inventory)
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    binding: String,
    #[serde(deserialize_with = "raw_digest")]
    binding_sha256: Sha256Digest,
    manifest: String,
    #[serde(deserialize_with = "raw_digest")]
    manifest_sha256: Sha256Digest,
    body: String,
    #[serde(deserialize_with = "raw_digest")]
    body_sha256: Sha256Digest,
}

fn raw_digest<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Sha256Digest, D::Error> {
    String::deserialize(decoder)?
        .parse()
        .map_err(serde::de::Error::custom)
}
fn metadata(
    root: &paths::Root,
    name: &str,
    hash: &Sha256Digest,
    maximum: u64,
) -> Result<Vec<u8>, Failure> {
    let bytes = root.read(name, maximum)?;
    if Sha256Digest::compute(&bytes) != *hash {
        return Err(Failure::Binding);
    }
    Ok(bytes)
}

pub(super) fn run(flags: &clap::ArgMatches) -> Result<Value, Failure> {
    let take = |name| {
        flags
            .get_one::<String>(name)
            .map(String::as_str)
            .ok_or(Failure::Arguments)
    };
    let inventory_path = Path::new(take("--inventory")?);
    let root = paths::Root::new(Path::new(take("--root")?))?;
    let installation_path = Path::new(take("--installation")?);
    let maximum: NonZeroU64 = positive(take("--max-bytes")?)?;
    let total_maximum: NonZeroU64 = positive(take("--max-total-bytes")?)?;
    let directory = Path::new(take("--run-dir")?);
    if maximum.get() > MAX_FILE_BYTES || total_maximum.get() > MAX_TOTAL_BYTES {
        return Err(Failure::Arguments);
    }
    let inventory_bytes = read(inventory_path, INVENTORY_BYTES)?;
    let inventory = Inventory::decode(&inventory_bytes)?;
    let installation = Installation::decode(
        read(installation_path, exact_wire::INSTALLATION_BYTES as u64)?,
        maximum,
    )?;
    let (prepared, capacity) = preflight(&inventory, &root, &installation, total_maximum)?;
    let run = Run::create(directory)?;
    run.bytes("inventory.json", &inventory_bytes)?;
    run.bytes("installation.wire", &installation.bytes)?;
    run.json("plan.json", &capacity.summary())?;
    let mut results = Vec::with_capacity(prepared.len());
    for (index, (input, entry)) in prepared.into_iter().zip(&inventory.files).enumerate() {
        let name = format!("file-{index:04}");
        let result = match input.publish(&directory.join(&name), Some(&entry.body_sha256)) {
            Ok(result) => result,
            Err(error) => {
                run.json(
                    "failure.json",
                    &json!({"file":name, "completed_files":results.len(),
                    "failure":FailureRecord {error:error.code()}}),
                )?;
                return Err(error);
            }
        };
        results.push(json!({"directory":name, "inputs":result}));
    }
    let mut result = serde_json::to_value(FrozenBatchRecord {
        schema: 1,
        observation: "local_publish_inputs".into(),
        inventory_sha256: digest(&inventory_bytes),
        installation_sha256: digest(&installation.bytes),
        all_bodies_verified: true,
    })
    .map_err(|_| Failure::File)?;
    result["capacity"] = capacity.summary();
    result["files"] = results.into();
    for field in [
        "authenticated",
        "identities_allocated",
        "installed_state_observed",
        "remaining_capacity_observed",
        "namespace_provisioned",
        "service_dispatched",
        "provider_dispatched",
    ] {
        result[field] = false.into();
    }
    run.json("summary.json", &result)?;
    Ok(result)
}

fn preflight(
    inventory: &Inventory,
    root: &paths::Root,
    installation: &Rc<Installation>,
    total_maximum: NonZeroU64,
) -> Result<(Vec<PreparedInput>, capacity::Capacity), Failure> {
    let mut capacity = capacity::Capacity::new(total_maximum);
    let mut prepared = Vec::with_capacity(inventory.files.len());
    let mut metadata_bytes = 0_usize;
    for entry in &inventory.files {
        let binding = metadata(root, &entry.binding, &entry.binding_sha256, BINDING_BYTES)?;
        let manifest = metadata(
            root,
            &entry.manifest,
            &entry.manifest_sha256,
            MANIFEST_BYTES,
        )?;
        metadata_bytes = metadata_bytes
            .checked_add(binding.len() + manifest.len())
            .ok_or(Failure::ReplyLimit)?;
        if metadata_bytes > METADATA_BYTES {
            return Err(Failure::ReplyLimit);
        }
        let input = PreparedInput::load(InputFiles {
            binding,
            manifest,
            installation: Rc::clone(installation),
            body: root.source(&entry.body)?,
        })?;
        input.check_body()?;
        capacity.add(&input)?;
        prepared.push(input);
    }
    Ok((prepared, capacity))
}
