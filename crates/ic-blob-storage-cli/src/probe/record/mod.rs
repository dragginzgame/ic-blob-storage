use ic_host_tools::artifact::{ArtifactError, Sha256Digest};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub(super) const MAX_BODY: usize = 1024 * 1024;
pub(super) const MAX_REQUESTS: usize = 4;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PlanRecord {
    pub schema: u8,
    pub started_unix_seconds: u64,
    pub runner_version: String,
    pub runner_source_sha256: String,
    pub kind: String,
    pub identity: String,
    pub max_requests: usize,
    pub max_response_bytes: usize,
    pub request_deadline_seconds: u64,
    pub attached_cycles: String,
    pub provider_account: Option<String>,
    pub targets: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RequestRecord {
    pub index: usize,
    pub started_unix_seconds: u64,
    pub method: String,
    pub url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ResponseRecord {
    pub index: usize,
    pub finished_unix_seconds: u64,
    pub status: Option<u16>,
    pub outcome: String,
    pub bytes: usize,
    pub sha256: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SummaryRecord {
    pub finished_unix_seconds: u64,
    pub outcome: String,
    pub requests_attempted: usize,
    pub source_commit: Option<String>,
    pub limitation: String,
}
pub(super) fn now() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|n| n.as_secs())
        .map_err(|_| "clock".into())
}
pub(super) fn hash(bytes: &[u8]) -> String {
    Sha256Digest::compute(bytes).to_string()
}
pub(super) fn save(directory: &Path, name: &str, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(directory.join(name))
        .map_err(|_| "create_record")?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|_| "persist_record")?;
    // Sync the directory before any following request. Never overwrite a previous run.
    File::open(directory)
        .and_then(|file| file.sync_all())
        .map_err(|_| "persist_directory".into())
}
pub(super) fn json<T: Serialize>(directory: &Path, name: &str, record: &T) -> Result<(), String> {
    save(
        directory,
        name,
        &serde_json::to_vec_pretty(record).map_err(|_| "encode_record")?,
    )
}
pub(super) fn read(directory: &Path, name: &str, limit: usize) -> Result<Vec<u8>, String> {
    if !std::fs::symlink_metadata(directory.join(name))
        .map_err(|_| "read_record")?
        .is_file()
    {
        return Err("record_not_file".into());
    }
    #[cfg(unix)]
    let result = ic_host_tools::artifact::read_file_no_follow(&directory.join(name), limit);
    #[cfg(not(unix))]
    let result = ic_host_tools::artifact::read_file(&directory.join(name), limit);
    result.map_err(|error| match error {
        ArtifactError::LimitExceeded { .. } => "record_limit".into(),
        ArtifactError::NotRegularFile => "record_not_file".into(),
        _ => "read_record".into(),
    })
}
pub(super) fn decode<T: for<'de> Deserialize<'de>>(
    directory: &Path,
    name: &str,
) -> Result<T, String> {
    serde_json::from_slice(&read(directory, name, 16384)?).map_err(|_| "invalid_record".into())
}

/// Verify retained bytes even for failed or interrupted runs. Absence never means success.
pub(super) fn verify(directory: &Path) -> Result<serde_json::Value, String> {
    for entry in std::fs::read_dir(directory).map_err(|_| "read_directory")? {
        let entry = entry.map_err(|_| "read_directory")?;
        let name = entry.file_name();
        let name = name.to_str().ok_or("invalid_record_name")?;
        let known = matches!(name, "plan.json" | "summary.json")
            || (0..MAX_REQUESTS).any(|i| {
                name == format!("request-{i}.json")
                    || name == format!("response-{i}.json")
                    || name == format!("response-{i}.body")
            });
        if !known {
            return Err("unexpected_record".into());
        }
    }
    let plan: PlanRecord = decode(directory, "plan.json")?;
    if plan.schema != 1 || plan.max_requests != MAX_REQUESTS || plan.max_response_bytes != MAX_BODY
    {
        return Err("invalid_plan".into());
    }
    let mut completed = 0;
    let mut attempted = 0;
    let mut incomplete = false;
    let mut all_captured = true;
    for index in 0..MAX_REQUESTS {
        let request_name = format!("request-{index}.json");
        let response_name = format!("response-{index}.json");
        let body_name = format!("response-{index}.body");
        if !directory.join(&request_name).exists() {
            if directory.join(&response_name).exists() || directory.join(&body_name).exists() {
                return Err("orphan_response".into());
            }
            incomplete = true;
            continue;
        }
        if incomplete {
            return Err("request_gap".into());
        }
        let request: RequestRecord = decode(directory, &request_name)?;
        if request.index != index || request.method != "GET" {
            return Err("invalid_request".into());
        }
        attempted += 1;
        if !directory.join(&response_name).exists() {
            incomplete = true;
            continue;
        }
        let response: ResponseRecord = decode(directory, &response_name)?;
        if !matches!(
            response.outcome.as_str(),
            "captured" | "timeout" | "transport" | "body_transport" | "body_limit" | "http_status"
        ) {
            return Err("invalid_response".into());
        }
        if response.outcome == "captured"
            && !response.status.is_some_and(|s| (200..300).contains(&s))
        {
            return Err("invalid_response".into());
        }
        all_captured &= response.outcome == "captured";
        let body = read(directory, &body_name, MAX_BODY)?;
        if response.index != index || response.bytes != body.len() || response.sha256 != hash(&body)
        {
            return Err("artifact_mismatch".into());
        }
        completed += 1;
    }
    let outcome = if directory.join("summary.json").exists() {
        let summary: SummaryRecord = decode(directory, "summary.json")?;
        if summary.requests_attempted != attempted || completed != attempted {
            return Err("summary_mismatch".into());
        }
        match summary.outcome.as_str() {
            "captured" if completed == MAX_REQUESTS && all_captured => "captured",
            "failed" => "failed",
            _ => return Err("invalid_summary".into()),
        }
    } else {
        "incomplete"
    };
    Ok(
        serde_json::json!({"integrity":"checked","outcome":outcome,"requests_attempted":attempted,"responses_recorded":completed,"limitation":"Local hashes detect changed bytes, not provider authenticity or qualification."}),
    )
}
