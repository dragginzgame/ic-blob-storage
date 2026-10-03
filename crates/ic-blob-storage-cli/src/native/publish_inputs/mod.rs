//! Freeze a bounded, caller-allocated inventory without service/provider effects.
mod capacity;
mod paths;
#[cfg(test)]
mod tests;

use super::{
    Failure,
    artifacts::{FailureRecord, Run},
    candidate_candid, read,
    upload_inputs::{BINDING_BYTES, InputFiles, MANIFEST_BYTES, PreparedInput, digest},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{num::NonZeroU64, path::Path};

const INVENTORY_BYTES: u64 = 2 * 1024 * 1024;
const METADATA_BYTES: usize = 16 * 1024 * 1024;
pub(super) const MAX_FILES: usize = 4096;
pub(super) const MAX_FILE_BYTES: u64 = 1024 * 1024 * 1024;
pub(super) const MAX_TOTAL_BYTES: u64 = 1024 * 1024 * 1024 * 1024;
pub(super) const MAX_TIMEOUT_SECONDS: u64 = 3600;

/// One complete frozen batch; caller-supplied identities remain proposals.
pub(super) struct PreparedBatch {
    pub files: Vec<FrozenFile>,
    pub inventory: Vec<u8>,
    pub installation: Vec<u8>,
}
/// Keep the verified input and its original expected raw digest inseparable.
pub(super) struct FrozenFile {
    pub input: PreparedInput,
    pub body_sha256: String,
}
impl PreparedBatch {
    pub fn open_frozen(
        directory: &Path,
        maximum: NonZeroU64,
        total: NonZeroU64,
    ) -> Result<Self, Failure> {
        let root = directory.canonicalize().map_err(|_| Failure::File)?;
        let inventory_bytes = read(&paths::resolve(&root, "inventory.json")?, INVENTORY_BYTES)?;
        let installation = read(
            &paths::resolve(&root, "installation.candid")?,
            candidate_candid::MAX_BYTES as u64,
        )?;
        let summary: Value = serde_json::from_slice(&read(
            &paths::resolve(&root, "summary.json")?,
            8 * 1024 * 1024,
        )?)
        .map_err(|_| Failure::Arguments)?;
        if summary["schema"] != 1
            || summary["observation"] != "local_publish_inputs"
            || summary["all_bodies_verified"] != true
            || summary["inventory_sha256"] != digest(&inventory_bytes)
            || summary["installation_sha256"] != digest(&installation)
        {
            return Err(Failure::Binding);
        }
        let mut inventory: Inventory =
            serde_json::from_slice(&inventory_bytes).map_err(|_| Failure::Arguments)?;
        if inventory.schema != 1 || inventory.files.is_empty() || inventory.files.len() > MAX_FILES
        {
            return Err(Failure::Arguments);
        }
        for (index, entry) in inventory.files.iter_mut().enumerate() {
            let prefix = format!("file-{index:04}");
            entry.binding = format!("{prefix}/binding.json");
            entry.manifest = format!("{prefix}/manifest.json");
            entry.body = format!("{prefix}/body.bin");
        }
        let (inputs, _) = preflight(&inventory, &root, &installation, maximum, total)?;
        for (index, (input, entry)) in inputs.iter().zip(&inventory.files).enumerate() {
            input.verify_body(&entry.body_sha256)?;
            let load = |name: &str, maximum| {
                read(
                    &paths::resolve(&root, &format!("file-{index:04}/{name}"))?,
                    maximum,
                )
            };
            input.check_requests(
                &load("permission.candid", 4096)?,
                &load("manifest.candid", 65536)?,
                &load("certificate-binding.json", 65536)?,
                &load("installation.candid", candidate_candid::MAX_BYTES as u64)?,
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
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    binding: String,
    binding_sha256: String,
    manifest: String,
    manifest_sha256: String,
    body: String,
    body_sha256: String,
}

use super::parsing::positive;
fn checked_hash(value: &str) -> Result<(), Failure> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(Failure::Arguments);
    }
    Ok(())
}
fn metadata(root: &Path, name: &str, hash: &str, maximum: u64) -> Result<Vec<u8>, Failure> {
    checked_hash(hash)?;
    let bytes = read(&paths::resolve(root, name)?, maximum)?;
    if digest(&bytes) != hash {
        return Err(Failure::Binding);
    }
    Ok(bytes)
}

pub(super) fn run(args: &[String]) -> Result<Value, Failure> {
    let mut flags = super::parsing::flags(&args[1..])?;
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    let inventory_path = Path::new(take("--inventory")?);
    let root = Path::new(take("--root")?)
        .canonicalize()
        .map_err(|_| Failure::File)?;
    let installation_path = Path::new(take("--installation")?);
    let maximum: NonZeroU64 = positive(take("--max-bytes")?)?;
    let total_maximum: NonZeroU64 = positive(take("--max-total-bytes")?)?;
    let directory = Path::new(take("--run-dir")?);
    if !flags.is_empty() || maximum.get() > MAX_FILE_BYTES || total_maximum.get() > MAX_TOTAL_BYTES
    {
        return Err(Failure::Arguments);
    }
    let inventory_bytes = read(inventory_path, INVENTORY_BYTES)?;
    let inventory: Inventory =
        serde_json::from_slice(&inventory_bytes).map_err(|_| Failure::Arguments)?;
    if inventory.schema != 1 || inventory.files.is_empty() || inventory.files.len() > MAX_FILES {
        return Err(Failure::Arguments);
    }
    let installation = read(installation_path, candidate_candid::MAX_BYTES as u64)?;
    let (prepared, capacity) = preflight(&inventory, &root, &installation, maximum, total_maximum)?;
    let run = Run::create(directory)?;
    run.bytes("inventory.json", &inventory_bytes)?;
    run.bytes("installation.candid", &installation)?;
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
    let result = json!({"schema":1, "observation":"local_publish_inputs",
        "inventory_sha256":digest(&inventory_bytes), "installation_sha256":digest(&installation),
        "capacity":capacity.summary(), "files":results, "all_bodies_verified":true,
        "authenticated":false, "identities_allocated":false, "installed_state_observed":false,
        "remaining_capacity_observed":false, "namespace_provisioned":false,
        "service_dispatched":false, "provider_dispatched":false});
    run.json("summary.json", &result)?;
    Ok(result)
}

fn preflight(
    inventory: &Inventory,
    root: &Path,
    installation: &[u8],
    maximum: NonZeroU64,
    total_maximum: NonZeroU64,
) -> Result<(Vec<PreparedInput>, capacity::Capacity), Failure> {
    let mut capacity = capacity::Capacity::new(total_maximum);
    let mut prepared = Vec::with_capacity(inventory.files.len());
    let mut metadata_bytes = 0_usize;
    for entry in &inventory.files {
        checked_hash(&entry.body_sha256)?;
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
        let input = PreparedInput::load(
            InputFiles {
                binding,
                manifest,
                installation: installation.to_vec(),
                body: paths::resolve(root, &entry.body)?,
            },
            maximum,
        )?;
        input.check_body()?;
        capacity.add(&input)?;
        prepared.push(input);
    }
    Ok((prepared, capacity))
}
