//! Shared subprocess harness for the maintained receipt endpoint on both local hosts.
use ic_blob_storage::dto::reference::ReferenceCommand;
use ic_testkit::pocket_ic::PocketIc;
use std::{fs, path::Path};

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
