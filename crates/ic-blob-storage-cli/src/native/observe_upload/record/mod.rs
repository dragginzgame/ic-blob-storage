//! Fresh, no-clobber observation artifacts. Partial runs are never resumed automatically.
//! String parameters permit borrowed writes and owned strict reads of the same schema.
use super::super::Failure;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct ObservationPlanRecord<S> {
    pub schema: u8,
    pub runner_version: S,
    pub runner_source_sha256: String,
    pub network: S,
    pub service_url: S,
    pub service: String,
    pub namespace: String,
    pub verifier: String,
    pub gateway_origin: S,
    pub max_content_bytes: String,
    pub max_provider_requests: u8,
    pub request_deadline_seconds: u8,
    pub provider_charges: S,
    pub started_at_ns: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct DownloadRequestRecord<S> {
    pub method: S,
    pub url: S,
    pub service_reply_sha256: String,
    pub owner: String,
    pub project: S,
    pub bytes: String,
    pub started_at_ns: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct HttpResponseRecord {
    pub status: u16,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct DownloadOutcomeRecord<S> {
    pub received_bytes: String,
    pub outcome: S,
    pub error: Option<S>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct FailureRecord<S> {
    pub error: S,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct ObservationSummaryRecord<S> {
    pub schema: u8,
    pub observation: S,
    pub network: S,
    pub service: String,
    pub namespace: String,
    pub verifier: String,
    pub owner: String,
    pub project: S,
    pub gateway_origin: S,
    pub root: String,
    pub bytes: String,
    pub content_digest: String,
    pub observed_at_ns: String,
    pub statement_sha256: String,
    pub authentication: S,
    pub attestation_dispatched: bool,
    pub retry_authorized: bool,
    pub future_retention: S,
    pub billing_cessation: S,
}

pub(in crate::native) struct Run {
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
