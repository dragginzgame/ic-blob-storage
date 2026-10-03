//! Passive browser selection; phase/effect claims remain with existing owners.
use super::{Failure, Input, PreparedBatch};
use crate::native::read;
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
#[cfg(test)]
mod tests;

/// Immutable launch inputs proposed by the parent, checked by the launcher
/// against the actual SDK signer and assets before Chromium opens.
#[derive(Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct BrowserSelectionRecord {
    format: String,
    pub(super) session: PathBuf,
    profile: PathBuf,
    project: String,
    bucket: String,
    asset_port: u16,
    signer_sha256: String,
    host_sha256: String,
    worker_sha256: String,
    database: String,
    max_slots: usize,
}

fn location(path: &Path) -> Result<PathBuf, Failure> {
    if !path.is_absolute()
        || path
            .components()
            .any(|part| matches!(part, Component::CurDir | Component::ParentDir))
    {
        return Err(Failure::Binding);
    }
    let parent = super::sources::directory(path.parent().ok_or(Failure::Binding)?)?;
    let canonical = parent.join(path.file_name().ok_or(Failure::Binding)?);
    if canonical != path {
        return Err(Failure::Binding);
    }
    Ok(canonical)
}

pub(super) fn selection(
    input: &Input,
    batch: &PreparedBatch,
) -> Result<Option<BrowserSelectionRecord>, Failure> {
    let Some(path) = &input.browser_selection else {
        return Ok(None);
    };
    if std::fs::symlink_metadata(path)
        .map_err(|_| Failure::File)?
        .file_type()
        .is_symlink()
    {
        return Err(Failure::Binding);
    }
    let record: BrowserSelectionRecord =
        serde_json::from_slice(&read(path, 16384)?).map_err(|_| Failure::Binding)?;
    let hash = |value: &str| {
        value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    };
    if record.format != "ic-blob-storage/browser-selection"
        || record.asset_port == 0
        || !hash(&record.signer_sha256)
        || !hash(&record.host_sha256)
        || !hash(&record.worker_sha256)
        || record.database.is_empty()
        || record.database.encode_utf16().count() > 128
        || batch.files.iter().any(|file| {
            file.input.project() != record.project || file.input.bucket() != record.bucket
        })
        || !(1..=1_000_000).contains(&record.max_slots)
        || location(&record.profile)? == location(&record.session)?
    {
        return Err(Failure::Binding);
    }
    if input.source_session.is_none() {
        let fresh_profile = matches!(std::fs::symlink_metadata(&record.profile),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound);
        if record.session != location(&input.prepare.directory)? || !fresh_profile {
            return Err(Failure::Binding);
        }
    } else {
        super::sources::directory(&record.profile)?;
        super::sources::directory(&record.session)?;
    }
    Ok(Some(record))
}
