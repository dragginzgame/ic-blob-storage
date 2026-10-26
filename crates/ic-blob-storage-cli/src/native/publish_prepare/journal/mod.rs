//! Bind recovery to the original private claim, request bytes and signed envelope.
use super::{Failure, Input, Options, Value, json, publish_inputs, upload_setup};
use crate::native::upload_inputs::{PreparedInput, digest};
use ic_blob_storage_contracts::identity::ContentDigest;
use std::path::Path;

pub(super) fn binding(
    options: &Options,
    input: &Input,
    batch: &publish_inputs::PreparedBatch,
    root: &[u8],
) -> Value {
    json!({"schema":1,"operation":"publish_prepare","network":options.network,"url":options.url.as_str(),
        "service":input.service.to_text(),"namespace":input.namespace.to_string(),"tenant":options.actor.to_text(),
        "file_index":input.index,"inventory_sha256":digest(&batch.inventory),"installation_sha256":digest(&batch.installation.bytes),
        "root_key_sha256":ContentDigest::compute(root).to_string(),"max_service_updates":super::MAX_UPDATES,"max_service_queries":super::MAX_QUERIES,
        "provider_requests":0,"automatic_retries":0,"certificate_issued":false,"publication_authorized":false})
}
/// One recovery validation reads every packet through the same directory handle.
/// Unlike input roots, recovery roots are never canonicalized through a symlink.
pub(super) struct RecoveryDirectory(std::fs::File);
impl RecoveryDirectory {
    pub(super) fn open(origin: &Path) -> Result<Self, Failure> {
        let metadata = std::fs::symlink_metadata(origin).map_err(|_| Failure::File)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(Failure::Binding);
        }
        #[cfg(unix)]
        {
            ic_host_fs::read::open_directory_no_follow(origin)
                .map(Self)
                .map_err(|error| {
                    if error.raw_os_error() == Some(libc::ELOOP) {
                        Failure::Binding
                    } else {
                        Failure::File
                    }
                })
        }
        #[cfg(not(unix))]
        {
            let _ = origin;
            Err(Failure::File)
        }
    }
    pub(super) fn read(&self, name: &str, maximum: u64) -> Result<Vec<u8>, Failure> {
        #[cfg(unix)]
        {
            use ic_host_artifacts::artifact::ArtifactError;
            use std::os::fd::AsFd;
            let file = ic_host_fs::read::open_regular_file_beneath(
                self.0.as_fd(),
                Path::new(name),
                maximum,
            )
            .map_err(|error| match error {
                ArtifactError::Io(error)
                    if error.kind() == std::io::ErrorKind::InvalidInput
                        || error.raw_os_error() == Some(libc::ELOOP) =>
                {
                    Failure::Binding
                }
                _ => Failure::File,
            })?;
            let limit = usize::try_from(maximum).map_err(|_| Failure::File)?;
            let bytes =
                ic_host_fs::read::read_opened_file(file, limit).map_err(|_| Failure::File)?;
            if bytes.is_empty() {
                return Err(Failure::File);
            }
            Ok(bytes)
        }
        #[cfg(not(unix))]
        {
            let _ = (name, maximum);
            Err(Failure::File)
        }
    }
}
pub(super) fn validate(
    directory: &RecoveryDirectory,
    expected: &Value,
    selected: &PreparedInput,
) -> Result<(), Failure> {
    let intent: Value = serde_json::from_slice(&directory.read("intent.json", 16384)?)
        .map_err(|_| Failure::Arguments)?;
    let (permission, manifest) = selected.setup_requests();
    if intent != *expected
        || directory.read("permission.wire", 4096)? != permission
        || directory.read("manifest.wire", 65536)? != manifest
    {
        return Err(Failure::Binding);
    }
    Ok(())
}
/// Only admission and manifest preparation have publication recovery claims.
#[derive(Clone, Copy)]
pub(super) enum Step {
    Admission,
    Preparation,
}
impl Step {
    const fn kind(self) -> upload_setup::Kind {
        match self {
            Self::Admission => upload_setup::Kind::Admit,
            Self::Preparation => upload_setup::Kind::Prepare,
        }
    }
}
/// Existing/partial claims never become permission to repeat an update.
pub(super) fn claimed(origin: &Path, step: Step) -> Result<bool, Failure> {
    match std::fs::symlink_metadata(origin.join(name(step))) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => Ok(true),
        Ok(_) => Err(Failure::Binding),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(Failure::File),
    }
}
pub(super) const fn name(step: Step) -> &'static str {
    match step {
        Step::Admission => "admission",
        Step::Preparation => "preparation",
    }
}
pub(super) fn validate_attempt(
    directory: &RecoveryDirectory,
    step: Step,
    options: &Options,
    selected: &PreparedInput,
    root: &[u8],
) -> Result<(), Failure> {
    let kind = step.kind();
    let prefix = name(step);
    let packet = |name: &str, maximum| directory.read(&format!("{prefix}/{name}"), maximum);
    let (permission, _) = selected.setup_requests();
    let request = if kind == upload_setup::Kind::Admit {
        ic_blob_storage_contracts::wire::encode_request(kind.method(), selected.permission)
    } else {
        ic_blob_storage_contracts::wire::encode_request(kind.method(), selected.request())
    }
    .map_err(|_| Failure::Arguments)?;
    let signed = packet("signed-request.cbor", 128 * 1024)?;
    let actor = if kind == upload_setup::Kind::Admit {
        options.actor
    } else {
        selected.permission.uploader
    };
    let intent: crate::native::upload_setup::record::SetupIntentRecord =
        crate::native::signed_update::record::inspect(
            &packet("intent.json", 16384)?,
            signed,
            actor,
            selected.permission.upload.service,
            kind.method(),
            &request,
            root,
        )?;
    let packets_match = packet("request.wire", 65536)? == request
        && packet("permission.wire", 4096)? == permission
        && intent.request_sha256 == ContentDigest::compute(&request).to_string();
    let role_and_scope_match = intent.actor == actor.to_text()
        && intent.service == selected.permission.upload.service.to_text()
        && intent.namespace == selected.permission.upload.namespace.to_string()
        && intent.permission == upload_setup::permission_json(selected.permission);
    let transport_matches =
        intent.network == options.network && intent.service_url == options.url.as_str();
    if !packets_match
        || !role_and_scope_match
        || !transport_matches
        || intent.schema != 1
        || intent.method != kind.method()
    {
        return Err(Failure::Binding);
    }
    // The inspected envelope remains historical. It is never sent again, even after expiry.
    Ok(())
}

#[cfg(all(test, unix))]
mod tests;
