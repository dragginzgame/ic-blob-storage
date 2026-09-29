//! Shared subprocess harness for the maintained receipt endpoint on both local hosts.
use candid::Principal;
use ic_blob_storage::{
    dto::reference::{ReferenceAction, ReferenceCommand},
    model::identity::ProviderRootHash,
};
use ic_testkit::pocket_ic::PocketIc;
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

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
