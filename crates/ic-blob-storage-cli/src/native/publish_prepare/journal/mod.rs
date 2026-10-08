//! Bind recovery to the original private claim, request bytes and signed envelope.
use super::{Failure, Input, Options, Value, json, publish_inputs, upload_setup};
use crate::native::{
    read,
    upload_inputs::{PreparedInput, digest},
};
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
        "file_index":input.index,"inventory_sha256":digest(&batch.inventory),"installation_sha256":digest(&batch.installation),
        "root_key_sha256":ContentDigest::compute(root).to_string(),"max_service_updates":2,"max_service_queries":4,
        "provider_requests":0,"automatic_retries":0,"certificate_issued":false,"publication_authorized":false})
}
pub(super) fn read_at(origin: &Path, name: &str, maximum: u64) -> Result<Vec<u8>, Failure> {
    let mut path = origin.to_path_buf();
    for part in name.split('/') {
        path.push(part);
        if std::fs::symlink_metadata(&path)
            .map_err(|_| Failure::File)?
            .file_type()
            .is_symlink()
        {
            return Err(Failure::Binding);
        }
    }
    read(&path, maximum)
}
pub(super) fn validate(
    origin: &Path,
    expected: &Value,
    selected: &PreparedInput,
) -> Result<(), Failure> {
    let metadata = std::fs::symlink_metadata(origin).map_err(|_| Failure::File)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(Failure::Binding);
    }
    let intent: Value = serde_json::from_slice(&read_at(origin, "intent.json", 16384)?)
        .map_err(|_| Failure::Arguments)?;
    let (permission, manifest) = selected.setup_requests();
    if intent != *expected
        || read_at(origin, "permission.candid", 4096)? != permission
        || read_at(origin, "manifest.candid", 65536)? != manifest
    {
        return Err(Failure::Binding);
    }
    Ok(())
}
/// Existing/partial claims never become permission to repeat an update.
pub(super) fn claimed(origin: &Path, kind: upload_setup::Kind) -> Result<bool, Failure> {
    match std::fs::symlink_metadata(origin.join(name(kind))) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => Ok(true),
        Ok(_) => Err(Failure::Binding),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(Failure::File),
    }
}
pub(super) const fn name(kind: upload_setup::Kind) -> &'static str {
    match kind {
        upload_setup::Kind::Admit => "admission",
        _ => "preparation",
    }
}
pub(super) fn validate_attempt(
    origin: &Path,
    kind: upload_setup::Kind,
    options: &Options,
    selected: &PreparedInput,
) -> Result<(), Failure> {
    let prefix = name(kind);
    let packet = |name: &str, maximum| read_at(origin, &format!("{prefix}/{name}"), maximum);
    let intent: Value =
        serde_json::from_slice(&packet("intent.json", 16384)?).map_err(|_| Failure::Arguments)?;
    let (permission, manifest) = selected.setup_requests();
    let request = if kind == upload_setup::Kind::Admit {
        permission
    } else {
        manifest
    };
    let signed = packet("signed-request.cbor", 128 * 1024)?;
    let actor = if kind == upload_setup::Kind::Admit {
        options.actor
    } else {
        selected.permission.uploader
    };
    let packets_match = packet("request.candid", 65536)? == request
        && packet("permission.candid", 4096)? == permission
        && intent["request_sha256"] == ContentDigest::compute(request).to_string()
        && intent["signed_request_sha256"] == ContentDigest::compute(&signed).to_string();
    let role_and_scope_match = intent["actor"] == actor.to_text()
        && intent["service"] == selected.permission.upload.service.to_text()
        && intent["namespace"] == selected.permission.upload.namespace.to_string()
        && intent["permission"] == upload_setup::permission_json(selected.permission);
    let transport_matches = intent["network"] == options.network
        && intent["service_url"] == options.url.as_str()
        && intent["root_key_sha256"]
            == ContentDigest::compute(&crate::native::agent(options)?.read_root_key()).to_string();
    if !packets_match
        || !role_and_scope_match
        || !transport_matches
        || intent["schema"] != 1
        || intent["method"] != kind.method()
    {
        return Err(Failure::Binding);
    }
    let expiry = intent["ingress_expiry_ns"]
        .as_str()
        .ok_or(Failure::Binding)?
        .parse()
        .map_err(|_| Failure::Binding)?;
    ic_agent::agent::signed_update_inspect(
        actor,
        selected.permission.upload.service,
        kind.method(),
        request,
        expiry,
        signed,
    )
    .map_err(|_| Failure::Binding)?;
    // The inspected envelope remains historical. It is never sent again, even after expiry.
    Ok(())
}
