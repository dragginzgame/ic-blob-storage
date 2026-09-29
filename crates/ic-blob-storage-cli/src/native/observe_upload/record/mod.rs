//! Fresh, no-clobber observation artifacts. Partial runs are never resumed automatically.
use super::super::Failure;
use serde::Serialize;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Serialize)]
pub(super) struct ObservationPlanRecord<'a> {
    pub schema: u8,
    pub runner_version: &'a str,
    pub runner_source_sha256: String,
    pub network: &'a str,
    pub service_url: &'a str,
    pub service: String,
    pub namespace: String,
    pub verifier: String,
    pub gateway_origin: &'a str,
    pub max_content_bytes: String,
    pub max_provider_requests: u8,
    pub request_deadline_seconds: u8,
    pub provider_charges: &'a str,
    pub started_at_ns: String,
}
#[derive(Serialize)]
pub(super) struct DownloadRequestRecord<'a> {
    pub method: &'a str,
    pub url: &'a str,
    pub service_reply_sha256: String,
    pub owner: String,
    pub project: &'a str,
    pub bytes: String,
    pub started_at_ns: String,
}
#[derive(Serialize)]
pub(super) struct HttpResponseRecord {
    pub status: u16,
}
#[derive(Serialize)]
pub(super) struct DownloadOutcomeRecord<'a> {
    pub received_bytes: String,
    pub outcome: &'a str,
    pub error: Option<&'a str>,
}
#[derive(Serialize)]
pub(super) struct FailureRecord<'a> {
    pub error: &'a str,
}
#[derive(Serialize)]
pub(super) struct ObservationSummaryRecord<'a> {
    pub schema: u8,
    pub observation: &'a str,
    pub network: &'a str,
    pub service: String,
    pub namespace: String,
    pub verifier: String,
    pub owner: String,
    pub project: &'a str,
    pub gateway_origin: &'a str,
    pub root: String,
    pub bytes: String,
    pub content_digest: String,
    pub observed_at_ns: String,
    pub statement_sha256: String,
    pub authentication: &'a str,
    pub attestation_dispatched: bool,
    pub retry_authorized: bool,
    pub future_retention: &'a str,
    pub billing_cessation: &'a str,
}

pub(super) struct Run {
    path: PathBuf,
}
impl Run {
    pub fn create(path: &Path) -> Result<Self, Failure> {
        let mut builder = std::fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                Failure::ExistingRun
            } else {
                Failure::File
            }
        })?;
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| Failure::File)?;
        Ok(Self {
            path: path.to_owned(),
        })
    }
    pub fn bytes(&self, name: &str, bytes: &[u8]) -> Result<(), Failure> {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(self.path.join(name))
            .map_err(|_| Failure::File)?;
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .map_err(|_| Failure::File)?;
        File::open(&self.path)
            .and_then(|f| f.sync_all())
            .map_err(|_| Failure::File)
    }
    pub fn json(&self, name: &str, value: &impl Serialize) -> Result<(), Failure> {
        self.bytes(
            name,
            &serde_json::to_vec_pretty(value).map_err(|_| Failure::File)?,
        )
    }
}
