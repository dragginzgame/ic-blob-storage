//! Offline exact-reference inputs; supplied identities never allocate authority.
use super::{Failure, artifacts::Run, read, references};
use candid::{de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::{
    dto::{
        download::DownloadRequest,
        reference::{
            ReferenceAction, ReferenceCommand, ReferenceUpload, status::ReferenceStatusRequest,
        },
        upload::admission::UploadAdmissionRequest,
    },
    model::identity::ContentDigest,
    ops::service::{references::reply, uploads::admission::reply::validate_request},
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};

/// Both read requests name the same exact reference and original object binding.
pub(super) struct ReferenceFiles {
    status: Vec<u8>,
    download: Vec<u8>,
}
impl ReferenceFiles {
    pub(super) fn new(upload: ReferenceUpload, reference: u128) -> Result<Self, Failure> {
        let status = reply::status_request(ReferenceStatusRequest { upload, reference })
            .map_err(|_| Failure::Arguments)?;
        let download = candid::encode_one(DownloadRequest {
            service: upload.service,
            tenant: upload.tenant,
            namespace: upload.namespace,
            object: upload.object,
            incarnation: upload.incarnation,
            reference,
            root: upload.root,
        })
        .map_err(|_| Failure::Arguments)?;
        Ok(Self { status, download })
    }
    pub(super) fn save(&self, run: &Run) -> Result<(), Failure> {
        run.bytes("reference-status.candid", &self.status)?;
        run.bytes("download.candid", &self.download)
    }
    pub(super) fn summary(&self) -> Value {
        json!({"status_file":"reference-status.candid", "download_file":"download.candid",
            "status_sha256":ContentDigest::compute(&self.status).to_string(),
            "download_sha256":ContentDigest::compute(&self.download).to_string()})
    }
}
fn positive(value: &str) -> Result<u128, Failure> {
    let parsed = value.parse::<u128>().map_err(|_| Failure::Arguments)?;
    if parsed == 0 || parsed.to_string() != value {
        return Err(Failure::Arguments);
    }
    Ok(parsed)
}
pub(super) fn run(args: &[String]) -> Result<Value, Failure> {
    let mut flags = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        if pair.len() != 2 || flags.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
            return Err(Failure::Arguments);
        }
    }
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    let path = Path::new(take("--permission")?);
    let action = match take("--action")? {
        "retain" => ReferenceAction::Retain,
        "release" => ReferenceAction::Release,
        _ => return Err(Failure::Arguments),
    };
    let reference = positive(take("--reference")?)?;
    let operation = positive(take("--operation")?)?;
    let directory = Path::new(take("--run-dir")?);
    if !flags.is_empty() {
        return Err(Failure::Arguments);
    }
    let permission_bytes = read(path, 4096)?;
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(64)
        .set_max_header_len(4096)
        .set_full_error_message(false);
    let permission: UploadAdmissionRequest =
        decode_one_with_config(&permission_bytes, &config).map_err(|_| Failure::Arguments)?;
    validate_request(permission).map_err(|_| Failure::Arguments)?;
    let request = ReferenceCommand {
        upload: permission.upload,
        reference,
        operation,
        action,
    };
    let command = reply::receipt_request(request).map_err(|_| Failure::Arguments)?;
    let files = ReferenceFiles::new(permission.upload, reference)?;
    // Validation precedes the durable claim. Exact input is retained, never renewed.
    let run = Run::create(directory)?;
    run.bytes("permission.candid", &permission_bytes)?;
    run.bytes("reference.candid", &command)?;
    files.save(&run)?;
    let result = json!({"schema":1,"observation":"local_reference_inputs",
        "upload":references::upload_json(permission.upload),"reference":reference.to_string(),
        "operation":operation.to_string(),"action":match action {
            ReferenceAction::Retain => "retain", ReferenceAction::Release => "release" },
        "permission_sha256":ContentDigest::compute(&permission_bytes).to_string(),
        "request_sha256":ContentDigest::compute(&command).to_string(),
        "request_file":"reference.candid","reads":files.summary(),
        "authenticated":false,"identities_allocated":false,
        "service_dispatched":false,"provider_dispatched":false,"retry_authorized":false});
    run.json("summary.json", &result)?;
    Ok(result)
}

#[cfg(test)]
mod tests;
