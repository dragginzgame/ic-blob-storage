//! The same executable refuses an unconfirmed or restored standalone object before HTTP.
use super::super::*;
use crate::{
    account_native_cli::signer,
    authenticated_cli::{PEM, run},
};
use ic_blob_storage_contracts::dto::download::DownloadRequest;
use ic_testkit::pocket_ic::PocketIcBuilder;
use std::{
    net::TcpListener,
    path::{Path, PathBuf},
};

fn arguments(
    f: &Fixture,
    temp: &Path,
    report: &Path,
    directory: &Path,
    url: &str,
    gateway: &str,
) -> Vec<String> {
    [
        "download",
        "--network",
        "local",
        "--url",
        url,
        "--identity",
        temp.join("identity.pem").to_str().unwrap(),
        "--root-key",
        temp.join("root.der").to_str().unwrap(),
        "--actor",
        &f.tenant.to_text(),
        "--service",
        &f.service.to_text(),
        "--namespace",
        &f.config.namespace.to_string(),
        "--request",
        report.join("request.candid").to_str().unwrap(),
        "--project",
        PROJECT,
        "--gateway",
        gateway,
        "--max-bytes",
        "10485760",
        "--run-dir",
        directory.to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[test]
fn standalone_native_download_refuses_unconfirmed_and_restored_references_without_provider_get() {
    let temp = tempfile::tempdir().unwrap();
    let report = std::env::var_os("BLOB_TENANT_DOWNLOAD_REPORT").map_or_else(
        || temp.path().join("standalone"),
        |p| PathBuf::from(p).join("standalone"),
    );
    std::fs::create_dir(&report).unwrap();
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.tenant = signer();
    f.enroll(f.operator).unwrap();
    let permission = f.manifest();
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (permission.permission,),
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &permission).unwrap();
    let upload = permission.permission.upload;
    let request = DownloadRequest {
        service: f.service,
        tenant: f.tenant,
        namespace: upload.namespace,
        root: upload.root,
        object: upload.object,
        incarnation: upload.incarnation,
        reference: upload.first_reference,
    };
    std::fs::write(
        report.join("request.candid"),
        candid::encode_one(request).unwrap(),
    )
    .unwrap();
    std::fs::write(temp.path().join("identity.pem"), PEM).unwrap();
    let root = f.harness.pic.root_key().unwrap();
    std::fs::write(temp.path().join("root.der"), &root).unwrap();
    std::fs::write(report.join("root.der"), root).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let gateway = format!("http://{}/", listener.local_addr().unwrap());
    std::fs::write(report.join("fixture-plan.json"),serde_json::to_vec(&serde_json::json!({"evidence":"local_unconfirmed_standalone","max_cli_invocations":2,"max_source_gets":0,"deployed_provider_requests":0,"attached_provider_cycles":"0"})).unwrap()).unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    for (label, error) in [
        ("unconfirmed", "download_unavailable"),
        ("restored", "download_fenced"),
    ] {
        if label == "restored" {
            f.harness.pic.stop_progress();
            f.upgrade(candid::encode_args(()).unwrap()).unwrap();
            f.harness.pic.auto_progress();
        }
        let directory = report.join(label);
        let args = arguments(&f, temp.path(), &report, &directory, &url, &gateway);
        std::fs::write(
            report.join(format!("{label}-command.json")),
            serde_json::to_vec(&args).unwrap(),
        )
        .unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        let value = run(&args, 3);
        assert_eq!(value["error"], error);
        std::fs::write(
            report.join(format!("{label}-result.json")),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
        assert!(!directory.join("download-request.json").exists());
        assert!(!directory.join("body.part").exists());
        assert!(!directory.join("body.bin").exists());
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    f.harness.pic.stop_live();
}
