//! Explicit offline asset inventory; no discovery, service reservation or upload.
//!
//! Relative paths resolve from the inventory file's directory. The caller must
//! control that source tree throughout the run; path checks are not a sandbox
//! against concurrent replacement. Every source is checked and hashed, including
//! duplicates. Budgets therefore apply before deduplication. Distinct-root totals
//! describe this inventory alone, not available service capacity or paid storage.

use super::{
    Declaration, MAX_HEADER_BYTES, MAX_HEADERS, PreparationError, PreparationLimits,
    PreparedUpload, prepare,
};
use ic_blob_storage::model::identity::{
    caffeine::{CAFFEINE_CHUNK_BYTES, CaffeineHeader},
    verification::ContentVerifier,
};
use ic_blob_storage::model::service::upload::manifest::validate_upload_metadata;
use ic_host_artifacts::artifact::{ArtifactError, read_reader};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{self, Read},
    num::{NonZeroU64, NonZeroUsize},
    path::{Component, Path, PathBuf},
};
use thiserror::Error;

const INVENTORY_BYTES: usize = 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Inventory {
    limits: InventoryLimits,
    files: Vec<InventoryFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InventoryLimits {
    files: NonZeroUsize,
    file_bytes: NonZeroU64,
    file_chunks: NonZeroUsize,
    total_bytes: NonZeroU64,
    total_chunks: NonZeroU64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InventoryFile {
    asset: String,
    source: PathBuf,
    declaration: Declaration,
}

#[derive(Debug, Serialize)]
pub(super) struct InventoryReport {
    totals: InventoryTotals,
    assets: Vec<PreparedAsset>,
    blobs: Vec<PreparedUpload>,
}

#[derive(Debug, Serialize)]
struct InventoryTotals {
    assets: usize,
    source_bytes: u64,
    source_chunks: u64,
    distinct_blobs: usize,
    distinct_bytes: u64,
    distinct_chunks: u64,
}

#[derive(Debug, Serialize)]
struct PreparedAsset {
    asset: String,
    source: PathBuf,
    root: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum InventoryLimit {
    Files,
    FileBytes,
    FileChunks,
    TotalBytes,
    TotalChunks,
}

#[derive(Debug, Error)]
pub(super) enum InventoryError {
    #[error("inventory exceeds its encoded byte limit")]
    EncodedLimit,
    #[error("invalid inventory JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("inventory I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("inventory allocation failed: {0}")]
    Allocation(std::collections::TryReserveError),
    #[error("inventory budget exceeded: {0:?}")]
    Limit(InventoryLimit),
    #[error("inventory requires at least one asset")]
    Empty,
    #[error("asset IDs must be unique, nonempty, control-free and at most 256 UTF-8 bytes")]
    Asset,
    #[error("sources must be relative normal paths to regular files without symlinks")]
    Source,
    #[error("source size differs from its declaration")]
    SourceLength,
    #[error("asset {asset} at {path:?}: {reason}")]
    SourceFile {
        asset: String,
        path: PathBuf,
        reason: Box<InventoryError>,
    },
    #[error("asset {asset} could not be prepared: {error}")]
    Preparation {
        asset: String,
        error: PreparationError,
    },
}

fn add(total: u64, bytes: u64, maximum: u64, limit: InventoryLimit) -> Result<u64, InventoryError> {
    total
        .checked_add(bytes)
        .filter(|sum| *sum <= maximum)
        .ok_or(InventoryError::Limit(limit))
}

fn preflight(inventory: &Inventory) -> Result<InventoryTotals, InventoryError> {
    if inventory.files.is_empty() {
        return Err(InventoryError::Empty);
    }
    let limits = &inventory.limits;
    if inventory.files.len() > limits.files.get() {
        return Err(InventoryError::Limit(InventoryLimit::Files));
    }
    let mut names = BTreeSet::new();
    let mut bytes = 0;
    let mut chunks = 0;
    for file in &inventory.files {
        if file.asset.is_empty()
            || file.asset.len() > 256
            || file.asset.chars().any(char::is_control)
            || !names.insert(&file.asset)
        {
            return Err(InventoryError::Asset);
        }
        if file.source.as_os_str().is_empty()
            || !file
                .source
                .components()
                .all(|c| matches!(c, Component::Normal(_)))
        {
            return Err(InventoryError::Source);
        }
        let size = file.declaration.bytes;
        if size == 0 || size > limits.file_bytes.get() || size > ContentVerifier::MAX_BYTES {
            return Err(InventoryError::Limit(InventoryLimit::FileBytes));
        }
        let leaves = size.div_ceil(CAFFEINE_CHUNK_BYTES as u64);
        if usize::try_from(leaves)
            .ok()
            .is_none_or(|n| n > limits.file_chunks.get())
        {
            return Err(InventoryError::Limit(InventoryLimit::FileChunks));
        }
        bytes = add(
            bytes,
            size,
            limits.total_bytes.get(),
            InventoryLimit::TotalBytes,
        )?;
        chunks = add(
            chunks,
            leaves,
            limits.total_chunks.get(),
            InventoryLimit::TotalChunks,
        )?;
        let headers: Vec<_> = file
            .declaration
            .headers
            .iter()
            .map(|h| CaffeineHeader {
                name: &h.name,
                value: &h.value,
            })
            .collect();
        validate_upload_metadata(&headers, size, MAX_HEADERS, MAX_HEADER_BYTES).map_err(
            |error| InventoryError::Preparation {
                asset: file.asset.clone(),
                error: error.into(),
            },
        )?;
    }
    Ok(InventoryTotals {
        assets: inventory.files.len(),
        source_bytes: bytes,
        source_chunks: chunks,
        distinct_blobs: 0,
        distinct_bytes: 0,
        distinct_chunks: 0,
    })
}

fn source(base: &Path, relative: &Path, bytes: u64) -> Result<File, InventoryError> {
    let mut path = base.to_path_buf();
    for component in relative.components() {
        path.push(component);
        if std::fs::symlink_metadata(&path)?.file_type().is_symlink() {
            return Err(InventoryError::Source);
        }
    }
    let metadata = std::fs::metadata(&path)?;
    if !metadata.is_file() {
        return Err(InventoryError::Source);
    }
    if metadata.len() != bytes {
        return Err(InventoryError::SourceLength);
    }
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(InventoryError::Source);
    }
    if metadata.len() != bytes {
        return Err(InventoryError::SourceLength);
    }
    Ok(file)
}

fn prepare_inventory(inventory: Inventory, base: &Path) -> Result<InventoryReport, InventoryError> {
    prepare_with(inventory, base, prepare)
}

pub(super) fn prepare_with(
    inventory: Inventory,
    base: &Path,
    mut prepare_body: impl FnMut(
        File,
        Declaration,
        PreparationLimits,
    ) -> Result<PreparedUpload, PreparationError>,
) -> Result<InventoryReport, InventoryError> {
    // Validate the whole declaration and work budget before opening any body.
    let mut totals = preflight(&inventory)?;
    let mut assets = Vec::with_capacity(inventory.files.len());
    let mut blobs = BTreeMap::new();
    for entry in inventory.files {
        let body = source(base, &entry.source, entry.declaration.bytes).map_err(|reason| {
            InventoryError::SourceFile {
                asset: entry.asset.clone(),
                path: entry.source.clone(),
                reason: Box::new(reason),
            }
        })?;
        let prepared = prepare_body(
            body,
            entry.declaration,
            PreparationLimits {
                bytes: inventory.limits.file_bytes,
                chunks: inventory.limits.file_chunks,
            },
        )
        .map_err(|error| InventoryError::Preparation {
            asset: entry.asset.clone(),
            error,
        })?;
        let root = prepared.claim.root.clone();
        assets.push(PreparedAsset {
            asset: entry.asset,
            source: entry.source,
            root: root.clone(),
        });
        // Preserve the first descriptor's original header order for each root.
        blobs.entry(root).or_insert(prepared);
    }
    let blobs: Vec<PreparedUpload> = blobs.into_values().collect();
    totals.distinct_blobs = blobs.len();
    // Distinct totals cannot exceed the preflight sums over all source entries.
    totals.distinct_bytes = blobs.iter().map(|blob| blob.claim.bytes).sum();
    totals.distinct_chunks = blobs
        .iter()
        .map(|blob| blob.chunk_hashes.len() as u64)
        .sum();
    Ok(InventoryReport {
        totals,
        assets,
        blobs,
    })
}

pub(super) fn run(path: &Path) -> Result<InventoryReport, InventoryError> {
    let (inventory, base) = load(path)?;
    prepare_inventory(inventory, &base)
}

pub(super) fn load(path: &Path) -> Result<(Inventory, PathBuf), InventoryError> {
    let inventory = load_reader(File::open(path)?)?;
    let base = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
        .canonicalize()?;
    Ok((inventory, base))
}

fn load_reader(reader: impl Read) -> Result<Inventory, InventoryError> {
    let encoded = read_reader(reader, INVENTORY_BYTES).map_err(|error| match error {
        ArtifactError::LimitExceeded { .. } => InventoryError::EncodedLimit,
        ArtifactError::Io(error) => InventoryError::Io(error),
        ArtifactError::Allocation(error) => InventoryError::Allocation(error),
        // File identity and digest errors are not produced by stream collection.
        error => InventoryError::Io(io::Error::other(error)),
    })?;
    Ok(serde_json::from_slice(&encoded)?)
}

#[cfg(test)]
mod tests;
