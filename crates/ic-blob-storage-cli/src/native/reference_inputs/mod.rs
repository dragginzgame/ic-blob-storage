//! Offline exact-reference inputs; supplied identities never allocate authority.

use super::parsing::positive;
use super::{Failure, artifacts::Run, read, references};
use ic_blob_storage_contracts::dto::download::DownloadRequest;
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_contracts::reference::reply;
use ic_blob_storage_contracts::upload::admission::reply::validate_request;
use serde_json::{Value, json};
use std::path::Path;

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
pub(super) fn run(args: &[String]) -> Result<Value, Failure> {
    let mut flags = super::parsing::flags(&args[1..])?;
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
    let permission: UploadAdmissionRequest =
        crate::native::exact_candid::decode(&permission_bytes, 4096, 64, 100_000)?;
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
