//! Original phase paths, reconstructed from existing intent and control records.
//! These paths grant no effect or completion authority; each phase owner validates
//! its original claims. Missing history refuses instead of allocating a replacement.
use super::{
    Failure, PreparedBatch, SessionIntentRecord,
    control::{Frame, UnattemptedPhaseRecord},
};
use crate::native::{artifacts::Run, exact_wire, publish_map};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub(super) struct Sources {
    pub files: Vec<FileSources>,
}

// One file owns its three optional original paths; it grants no replay authority.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct FileSources {
    pub setup: Option<PathBuf>,
    pub transfer: Option<PathBuf>,
}

/// Passive original-path provenance. Dispatch claims stay with phase owners;
/// this record preserves unused sources across a status-only recovery run.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SessionSourcesRecord {
    format: String,
    source_session: PathBuf,
    setup: Vec<Option<PathBuf>>,
    transfers: Vec<Option<PathBuf>>,
}
impl SessionSourcesRecord {
    const FORMAT: &'static str = "ic-blob-storage/publication-session-sources:trusted-registration";
    const MAX_BYTES: u64 = 8 * 1024 * 1024;
}

pub(super) fn directory(path: &Path) -> Result<PathBuf, Failure> {
    let metadata = std::fs::symlink_metadata(path).map_err(|_| Failure::Binding)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(Failure::Binding);
    }
    path.canonicalize().map_err(|_| Failure::File)
}

pub(super) fn packet(path: &Path, maximum: u64) -> Result<Vec<u8>, Failure> {
    // Preserve missing/inaccessible original-evidence classification.
    std::fs::symlink_metadata(path).map_err(|_| Failure::Binding)?;
    crate::native::read_no_follow(path, maximum)
}

fn pin(slot: &mut Option<PathBuf>, path: &Path) -> Result<(), Failure> {
    let original = directory(path)?;
    if slot.as_ref().is_some_and(|p| *p != original) {
        return Err(Failure::Binding);
    }
    *slot = Some(original);
    Ok(())
}

fn unattempted(action: &Path, claim: &Path, index: usize, files: usize) -> Result<bool, Failure> {
    match std::fs::symlink_metadata(claim) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let record: UnattemptedPhaseRecord =
                serde_json::from_slice(&packet(&action.join("unattempted.json"), 8192)?)
                    .map_err(|_| Failure::Binding)?;
            if record.format != UnattemptedPhaseRecord::FORMAT
                || record.index != index
                || record.next_index == index
                || record.next_index > files
            {
                return Err(Failure::Binding);
            }
            Ok(true)
        }
        Err(_) => Err(Failure::File),
        Ok(_) => Ok(false),
    }
}

fn history_binding(
    source: &Path,
    expected: &SessionIntentRecord,
    batch: &PreparedBatch,
) -> Result<(u64, Option<PathBuf>), Failure> {
    let mut original: SessionIntentRecord =
        serde_json::from_slice(&packet(&source.join("intent.json"), 16384)?)
            .map_err(|_| Failure::Binding)?;
    if original.max_steps == 0
        || super::budget::step_limit(batch.files.len())
            .is_none_or(|limit| original.max_steps > limit)
        || super::budget::query_limit(original.max_steps, batch.files.len())
            != Some(original.max_service_queries)
        || !(1..=3600).contains(&original.timeout_seconds)
    {
        return Err(Failure::Binding);
    }
    let max_steps = original.max_steps;
    let source_session = original.source_session.clone();
    // Control budgets may change on restart; immutable bindings may not.
    original.max_steps = expected.max_steps;
    original.max_service_queries = expected.max_service_queries;
    original.timeout_seconds = expected.timeout_seconds;
    original.source_session.clone_from(&expected.source_session);
    if original != *expected
        || packet(&source.join("inventory.json"), batch.inventory.len() as u64)? != batch.inventory
        || packet(
            &source.join("installation.wire"),
            batch.installation.bytes.len() as u64,
        )? != batch.installation.bytes
    {
        return Err(Failure::Binding);
    }
    let configuration = directory(&source.join("configuration"))?;
    let host = publish_map::decode_configuration(
        batch,
        &packet(
            &configuration.join("query-0000-reply.wire"),
            exact_wire::INSTALLATION_BYTES as u64,
        )?,
    )?;
    if host.fenced {
        return Err(Failure::Denied);
    }
    Ok((max_steps, source_session))
}

fn steps(source: &Path, max_steps: u64, files: usize) -> Result<BTreeMap<u64, PathBuf>, Failure> {
    let mut steps = BTreeMap::new();
    for (count, entry) in std::fs::read_dir(source)
        .map_err(|_| Failure::File)?
        .enumerate()
    {
        if count as u64 >= max_steps + files as u64 + 16 {
            return Err(Failure::ReplyLimit);
        }
        let entry = entry.map_err(|_| Failure::File)?;
        let name = entry.file_name();
        let name = name.to_str().ok_or(Failure::Binding)?;
        if let Some(number) = name.strip_prefix("step-") {
            let step = number.parse::<u64>().map_err(|_| Failure::Binding)?;
            if step >= max_steps || name != format!("step-{step:04}") {
                return Err(Failure::Binding);
            }
            steps.insert(step, directory(&entry.path())?);
        }
    }
    Ok(steps)
}

impl Sources {
    pub fn record(&self, run: &Run, source: &Path) -> Result<(), Failure> {
        let record = SessionSourcesRecord {
            format: SessionSourcesRecord::FORMAT.into(),
            source_session: directory(source)?,
            setup: self.files.iter().map(|file| file.setup.clone()).collect(),
            transfers: self
                .files
                .iter()
                .map(|file| file.transfer.clone())
                .collect(),
        };
        let bytes = serde_json::to_vec_pretty(&record).map_err(|_| Failure::File)?;
        if bytes.len() as u64 > SessionSourcesRecord::MAX_BYTES {
            return Err(Failure::ReplyLimit);
        }
        run.bytes("source-session.json", &bytes)
    }

    fn seed(source: &Path, parent: Option<&Path>, files: usize) -> Result<Self, Failure> {
        let path = source.join("source-session.json");
        match std::fs::symlink_metadata(&path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && parent.is_none() => {
                return Ok(Self {
                    files: vec![FileSources::default(); files],
                });
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Err(Failure::Binding);
            }
            Err(_) => return Err(Failure::File),
            Ok(_) => {}
        }
        let mut record: SessionSourcesRecord =
            serde_json::from_slice(&packet(&path, SessionSourcesRecord::MAX_BYTES)?)
                .map_err(|_| Failure::Binding)?;
        if record.format != SessionSourcesRecord::FORMAT
            || Some(record.source_session.as_path()) != parent
            || !record.source_session.is_absolute()
            || record.setup.len() != files
            || record.transfers.len() != files
        {
            return Err(Failure::Binding);
        }
        // Do not follow the source-session pointer: each surviving path is direct.
        // Phase owners still validate exact original contents before executing.
        for path in record
            .setup
            .iter_mut()
            .chain(&mut record.transfers)
            .flatten()
        {
            if !path.is_absolute() {
                return Err(Failure::Binding);
            }
            *path = directory(path)?;
        }
        Ok(Self {
            files: record
                .setup
                .into_iter()
                .zip(record.transfers)
                .map(|(setup, transfer)| FileSources { setup, transfer })
                .collect(),
        })
    }
    pub fn load(
        source: Option<&Path>,
        expected: &SessionIntentRecord,
        batch: &PreparedBatch,
    ) -> Result<Self, Failure> {
        let Some(source) = source else {
            return Ok(Self {
                files: vec![FileSources::default(); batch.files.len()],
            });
        };
        let source = directory(source)?;
        let (max_steps, parent) = history_binding(&source, expected, batch)?;
        let mut sources = Self::seed(&source, parent.as_deref(), batch.files.len())?;
        for (expected_step, (step, action)) in steps(&source, max_steps, batch.files.len())?
            .into_iter()
            .enumerate()
        {
            if step != expected_step as u64 {
                return Err(Failure::Binding);
            }
            let frame: Frame = serde_json::from_slice(&packet(&action.join("request.json"), 8192)?)
                .map_err(|_| Failure::Binding)?;
            match frame {
                Frame::Prepare { index, source_run } => {
                    let slot = &mut sources.files.get_mut(index).ok_or(Failure::Binding)?.setup;
                    if source_run.is_none()
                        && slot.is_none()
                        && unattempted(&action, &action.join("setup"), index, batch.files.len())?
                    {
                        continue;
                    }
                    let path = source_run
                        .or_else(|| slot.clone())
                        .unwrap_or_else(|| action.join("setup"));
                    pin(slot, &path)?;
                }
                Frame::Transfer {
                    index,
                    source_transfer,
                } => {
                    let slot = &mut sources
                        .files
                        .get_mut(index)
                        .ok_or(Failure::Binding)?
                        .transfer;
                    if source_transfer.is_none()
                        && slot.is_none()
                        && unattempted(
                            &action,
                            &action.join("transfer.json"),
                            index,
                            batch.files.len(),
                        )?
                    {
                        continue;
                    }
                    let path = source_transfer.or_else(|| slot.clone()).unwrap_or(action);
                    super::transfer::original(&path, index)?;
                    pin(slot, &path)?;
                }
                Frame::Status { index } if index >= batch.files.len() => {
                    return Err(Failure::Binding);
                }
                Frame::Status { .. } | Frame::Map {} => {}
            }
        }
        Ok(sources)
    }

    /// Resolve omitted recovery paths to surviving originals, and refuse attempts
    /// to substitute another path once this session has selected its source.
    pub fn resolve(&mut self, mut frame: Frame) -> Result<Frame, Failure> {
        let (supplied, slot) = match &mut frame {
            Frame::Prepare { index, source_run } => (
                source_run,
                &mut self.files.get_mut(*index).ok_or(Failure::Arguments)?.setup,
            ),
            Frame::Transfer {
                index,
                source_transfer,
            } => (
                source_transfer,
                &mut self
                    .files
                    .get_mut(*index)
                    .ok_or(Failure::Arguments)?
                    .transfer,
            ),
            Frame::Status { .. } | Frame::Map {} => return Ok(frame),
        };
        if let Some(supplied) = supplied.as_ref() {
            pin(slot, supplied)?;
        }
        supplied.clone_from(slot);
        Ok(frame)
    }
}

#[cfg(test)]
mod tests;
