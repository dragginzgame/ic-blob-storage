//! Real offline CLI output feeds the signed service commands; Rust only supplies local inputs.
use super::*;
use ic_blob_storage::model::identity::{ProviderRootHash, caffeine::manifest::CaffeineChunkHash};

pub(super) fn prepare(c: &Client, body: &[u8]) {
    let p = c.permission.permission;
    let u = p.upload;
    let root = ProviderRootHash::try_from(u.root.as_slice())
        .unwrap()
        .to_string();
    let binding = json!({"format":"ic-blob-storage/upload-inputs:original-preparation","preparation":{},"project":c.installation.project,"bucket":"fixture-bucket","service":u.service.to_text(),"namespace":u.namespace.to_string(),
        "tenant":u.tenant.to_text(),"uploader":p.uploader.to_text(),"upload":u.upload.to_string(),
        "object":u.object.to_string(),"incarnation":u.incarnation.to_string(),
        "first_reference":u.first_reference.to_string(),"root":root,"bytes":u.bytes.to_string(),
        "expires_at_ns":p.expires_at_ns.to_string()});
    let manifest = json!({"tree_type":"DSBMTWH","tree":{"hash":root},
        "chunk_hashes":c.permission.declaration.chunks.iter().map(|chunk|
            CaffeineChunkHash::try_from(chunk.as_slice()).unwrap().to_string()).collect::<Vec<_>>(),
        "headers":c.permission.declaration.headers.iter().map(|h|format!("{}: {}",h.name,h.value)).collect::<Vec<_>>()});
    save(
        &c.report.join("binding.json"),
        &serde_json::to_vec(&binding).unwrap(),
    );
    save(
        &c.report.join("upstream-format-manifest.json"),
        &serde_json::to_vec(&manifest).unwrap(),
    );
    let source = c.temporary.path().join("source.bin");
    save(&source, body);
    save(
        &c.report.join("installation.candid"),
        &candid::encode_one(&c.installation).unwrap(),
    );
    let args = vec![
        "upload-inputs".into(),
        "--installation".into(),
        c.report.join("installation.candid").display().to_string(),
        "--binding".into(),
        c.report.join("binding.json").display().to_string(),
        "--manifest".into(),
        c.report
            .join("upstream-format-manifest.json")
            .display()
            .to_string(),
        "--body".into(),
        source.display().to_string(),
        "--max-bytes".into(),
        u.bytes.to_string(),
        "--run-dir".into(),
        c.report.join("inputs").display().to_string(),
    ];
    let result = c.call(&args, 0);
    assert_eq!(result["body_verified"], true);
    assert_eq!(result["service_dispatched"], false);
    assert_eq!(result["provider_dispatched"], false);
    let generated: UploadManifestRequest =
        candid::decode_one(&std::fs::read(c.report.join("inputs/manifest.candid")).unwrap())
            .unwrap();
    assert_eq!(generated, c.permission);
    assert_eq!(
        std::fs::read(c.report.join("inputs/body.bin")).unwrap(),
        body
    );
    // A later edit to the source must not select new bytes for service verification.
    std::fs::write(&source, b"changed source").unwrap();
    assert_eq!(
        std::fs::read(c.report.join("inputs/body.bin")).unwrap(),
        body
    );
}

pub(super) fn verify(c: &Client) {
    let mut args = c.args("verify-upload", None, true);
    let n = args.iter().position(|a| a == "--request").unwrap();
    args[n] = "--permission".into();
    args.extend([
        "--body".into(),
        c.report.join("inputs/body.bin").display().to_string(),
        "--max-bytes".into(),
        c.permission.permission.upload.bytes.to_string(),
    ]);
    let result = c.call(&args, 0);
    assert_eq!(result["provider_completion"], "not_established");
    assert_eq!(result["provider_availability"], "not_observed");
}

pub(super) fn unconfirmed_reference(c: &Client) {
    let mut args = c.args("reference-status", None, false);
    change(
        &mut args,
        "--request",
        c.report
            .join("inputs/reference-status.candid")
            .to_str()
            .unwrap(),
    );
    assert_eq!(c.call(&args, 3)["error"], "reference_unconfirmed");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let mut args = c.args("download", Some("unconfirmed-download"), false);
    change(
        &mut args,
        "--request",
        c.report.join("inputs/download.candid").to_str().unwrap(),
    );
    args.extend([
        "--project".into(),
        "unconfirmed-fixture".into(),
        "--gateway".into(),
        format!("http://{}/", listener.local_addr().unwrap()),
        "--max-bytes".into(),
        c.permission.permission.upload.bytes.to_string(),
    ]);
    assert_eq!(c.call(&args, 3)["error"], "download_unavailable");
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
    assert!(!c.report.join("unconfirmed-download/body.bin").exists());
}
