//! Shared subprocess harness for the maintained receipt endpoint on both local hosts.
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_testkit::pocket_ic::PocketIc;
use std::{fs, path::Path};

/// Native requests use genuine signed tenant queries with independently supplied trust.
pub(crate) async fn native_requests(
    pic: &PocketIc,
    input: ReferenceCommand,
    directory: &Path,
    url: &str,
) -> (Vec<String>, Vec<String>) {
    let (key, root) = crate::authenticated_cli::identity_files(pic, directory, "tenant.pem").await;
    let receipt = directory.join("receipt.wire");
    let status = directory.join("status.wire");
    fs::write(
        &receipt,
        ic_blob_storage_contracts::document::encode(input).unwrap(),
    )
    .unwrap();
    fs::write(
        &status,
        ic_blob_storage_contracts::document::encode(
            ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest {
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
