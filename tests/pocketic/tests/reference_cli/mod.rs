//! Shared subprocess harness for the maintained receipt endpoint on both local hosts.
use candid::Principal;
use ic_blob_storage::{
    dto::reference::{ReferenceAction, ReferenceCommand},
    model::identity::ProviderRootHash,
};
use ic_testkit::pocket_ic::PocketIc;
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

/// Native requests use genuine signed tenant queries with independently supplied trust.
pub(crate) fn native_requests(
    pic: &PocketIc,
    input: ReferenceCommand,
    directory: &Path,
    url: &str,
) -> (Vec<String>, Vec<String>) {
    let key = directory.join("tenant.pem");
    let root = directory.join("root.der");
    fs::write(&key, crate::authenticated_cli::PEM).unwrap();
    fs::write(&root, pic.root_key().unwrap()).unwrap();
    let receipt = directory.join("receipt.candid");
    let status = directory.join("status.candid");
    fs::write(&receipt, candid::encode_one(input).unwrap()).unwrap();
    fs::write(
        &status,
        candid::encode_one(
            ic_blob_storage::dto::reference::status::ReferenceStatusRequest {
                upload: input.upload,
                reference: input.reference,
            },
        )
        .unwrap(),
    )
    .unwrap();
    let args = |kind: &str, path: &Path| {
        [
            kind,
            "--network",
            "local",
            "--url",
            url,
            "--identity",
            key.to_str().unwrap(),
            "--root-key",
            root.to_str().unwrap(),
            "--actor",
            &input.upload.tenant.to_text(),
            "--service",
            &input.upload.service.to_text(),
            "--namespace",
            &input.upload.namespace.to_string(),
            "--request",
            path.to_str().unwrap(),
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    };
    (
        args("reference-receipt", &receipt),
        args("reference-status", &status),
    )
}

pub(crate) fn change_native(args: &mut [String], flag: &str, value: &str) {
    let index = args.iter().position(|s| s == flag).unwrap();
    args[index + 1] = value.into();
}

pub(crate) fn intent(path: &Path, input: ReferenceCommand) {
    let upload = input.upload;
    let root = ProviderRootHash::try_from(upload.root.as_slice()).unwrap();
    fs::write(path, serde_json::to_vec(&json!({
        "schema":1,"scope":"pocketic_fixture","asset":"release-image", "service":upload.service.to_text(),
        "tenant":upload.tenant.to_text(),"namespace":upload.namespace.to_string(),
        "upload":upload.upload.to_string(),"object":upload.object.to_string(),"incarnation":upload.incarnation.to_string(),
        "first_reference":upload.first_reference.to_string(),"root":root.to_string(),
        "bytes":upload.bytes,"reference":input.reference.to_string(),"operation":input.operation.to_string(),
        "retain":input.action == ReferenceAction::Retain,
    })).unwrap()).unwrap();
}

pub(crate) fn command(args: &[String], code: i32) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_blob-fixture-reference"))
        .args(args)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(code),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

pub(crate) fn inspect(
    pic: &PocketIc,
    service: Principal,
    caller: Principal,
    path: &Path,
    code: i32,
) -> Value {
    let url = pic.get_server_url();
    command(
        &[
            "inspect".into(),
            "--intent".into(),
            path.to_str().unwrap().into(),
            "--server".into(),
            format!("{}:{}", url.host_str().unwrap(), url.port().unwrap()),
            "--instance".into(),
            pic.instance_id().to_string(),
            "--canister".into(),
            service.to_text(),
            "--caller".into(),
            caller.to_text(),
        ],
        code,
    )
}
