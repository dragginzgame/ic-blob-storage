//! Retain handoff before returning it; repeated phases request observation only.
use super::{Failure, Input, Options, PreparedBatch, Run, Value, json, publish_prepare, sources};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct BrowserHandoffRecord {
    format: String,
    session: PathBuf,
    file_index: usize,
    source_run: PathBuf,
    source_transfer: Option<PathBuf>,
    transfer: Value,
}
const FORMAT: &str = "ic-blob-storage/browser-handoff";
const MAX_BYTES: u64 = 512 * 1024;

pub(super) fn original(path: &Path, index: usize) -> Result<BrowserHandoffRecord, Failure> {
    let record: BrowserHandoffRecord = serde_json::from_slice(&sources::packet(
        &sources::directory(path)?.join("transfer.json"),
        MAX_BYTES,
    )?)
    .map_err(|_| Failure::Binding)?;
    if record.format != FORMAT || record.file_index != index || record.source_transfer.is_some() {
        return Err(Failure::Binding);
    }
    Ok(record)
}

pub(super) struct Context<'a> {
    pub options: &'a Options,
    pub input: &'a Input,
    pub batch: &'a PreparedBatch,
    pub browser: &'a super::browser::BrowserSelectionRecord,
}
impl Context<'_> {
    pub fn run(
        &self,
        index: usize,
        action: &Run,
        directory: &Path,
        setup: Option<&Path>,
        source: Option<&Path>,
    ) -> Result<Value, Failure> {
        let Self {
            options,
            input,
            batch,
            browser,
        } = self;
        let setup = sources::directory(setup.ok_or(Failure::Unprepared)?)?;
        let preparation = publish_prepare::Input {
            index,
            source: Some(setup.clone()),
            ..input.prepare.clone()
        };
        publish_prepare::transfer_source(options, &preparation, batch)?;
        let file = batch.files.get(index).ok_or(Failure::Arguments)?;
        let mut record = BrowserHandoffRecord {
            format: FORMAT.into(),
            session: browser.session.clone(),
            file_index: index,
            source_run: setup,
            source_transfer: None,
            transfer: file.input.transfer_input(&file.body_sha256)?,
        };
        if let Some(source) = source {
            if original(source, index)? != record {
                return Err(Failure::Binding);
            }
            record.source_transfer = Some(sources::directory(source)?);
        }
        let bytes = serde_json::to_vec_pretty(&record).map_err(|_| Failure::File)?;
        if bytes.len() as u64 > MAX_BYTES {
            return Err(Failure::ReplyLimit);
        }
        action.bytes("transfer.json", &bytes)?;
        Ok(
            json!({"schema":1,"operation":"publish_session_transfer","file_index":index,
        "native_phase":sources::directory(directory)?,"recovery":source.is_some(),
        "provider_requests":0,"service_updates":0,"retry_authorized":false,"file_live":false}),
        )
    }
}
