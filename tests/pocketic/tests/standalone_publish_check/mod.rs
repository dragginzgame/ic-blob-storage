//! Real signed batch queries preserve reservation history and restored fences.
use super::*;
use crate::{
    account_native_cli::signer,
    authenticated_cli::{PEM, run},
    upload_setup_cli::uploader,
};
use ic_blob_storage::model::identity::{ProviderRootHash, caffeine::manifest::CaffeineChunkHash};
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::path::Path;

pub(super) fn setup() -> Fixture {
    setup_profile(Envelope::Single)
}
pub(super) fn setup_batch() -> Fixture {
    setup_profile(Envelope::Regular)
}
fn setup_profile(envelope: Envelope) -> Fixture {
    let mut f = Fixture::with_profile(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        Fake::principal(5),
        Fake::principal(2),
        uploader(),
        envelope,
        Fake::principal(90),
        "publish-check-fixture",
    );
    f.tenant = signer();
    f.enroll(f.operator).unwrap();
    f
}
pub(super) fn freeze(fixture: &Fixture, directory: &Path) -> UploadAdmissionRequest {
    freeze_files(fixture, directory, &[1024])[0]
}
pub(super) fn freeze_files(
    fixture: &Fixture,
    directory: &Path,
    sizes: &[u64],
) -> Vec<UploadAdmissionRequest> {
    let bodies: Vec<_> = sizes
        .iter()
        .map(|size| vec![42; usize::try_from(*size).unwrap()])
        .collect();
    freeze_bodies(fixture, directory, &bodies)
}
pub(super) fn freeze_bodies(
    fixture: &Fixture,
    directory: &Path,
    bodies: &[Vec<u8>],
) -> Vec<UploadAdmissionRequest> {
    let host = fixture.configuration(fixture.operator).unwrap();
    std::fs::write(
        directory.join("installation.candid"),
        candid::encode_one(ServiceInstallationInput {
            configuration: host.configuration,
            project: host.project.clone(),
            completion_verifier: host.completion_verifier,
            trusted_uploader: host.trusted_uploader,
        })
        .unwrap(),
    )
    .unwrap();
    let mut permissions = Vec::new();
    let mut entries = Vec::new();
    for (index, body) in bodies.iter().enumerate() {
        let mut manifest = fixture.manifest_body(body);
        manifest.permission.upload.upload += index as u128;
        manifest.permission.upload.object += index as u128;
        manifest.permission.upload.first_reference += index as u128;
        let permission = manifest.permission;
        let upload = permission.upload;
        let root = ProviderRootHash::try_from(upload.root.as_slice())
            .unwrap()
            .to_string();
        let binding = json!({"format":"ic-blob-storage/upload-inputs:original-preparation","preparation":{"content_type":"image/png"},"project":host.project,"bucket":"fixture-bucket","service":upload.service.to_text(),"namespace":upload.namespace.to_string(),"tenant":upload.tenant.to_text(),"uploader":permission.uploader.to_text(),"upload":upload.upload.to_string(),"object":upload.object.to_string(),"incarnation":upload.incarnation.to_string(),"first_reference":upload.first_reference.to_string(),"root":root,"bytes":upload.bytes.to_string(),"expires_at_ns":permission.expires_at_ns.to_string()});
        let manifest = json!({"tree_type":"DSBMTWH","tree":{"hash":root},"chunk_hashes":manifest.declaration.chunks.iter().map(|c|CaffeineChunkHash::try_from(c.as_slice()).unwrap().to_string()).collect::<Vec<_>>(),"headers":manifest.declaration.headers.iter().map(|h|format!("{}: {}",h.name,h.value)).collect::<Vec<_>>()});
        let binding = serde_json::to_vec(&binding).unwrap();
        let manifest = serde_json::to_vec(&manifest).unwrap();
        std::fs::write(directory.join(format!("binding-{index}.json")), &binding).unwrap();
        std::fs::write(directory.join(format!("manifest-{index}.json")), &manifest).unwrap();
        std::fs::write(directory.join(format!("body-{index}.bin")), body).unwrap();
        let hash = |b: &[u8]| {
            Sha256::digest(b)
                .iter()
                .fold(String::with_capacity(64), |mut output, byte| {
                    write!(output, "{byte:02x}").unwrap();
                    output
                })
        };
        entries.push(
            json!({"binding":format!("binding-{index}.json"),"binding_sha256":hash(&binding),
        "manifest":format!("manifest-{index}.json"),"manifest_sha256":hash(&manifest),
        "body":format!("body-{index}.bin"),"body_sha256":hash(body)}),
        );
        permissions.push(permission);
    }

    std::fs::write(
        directory.join("inventory.json"),
        json!({"schema":1,"files":entries}).to_string(),
    )
    .unwrap();
    let maximum = bodies.iter().map(Vec::len).max().unwrap().to_string();
    let total = bodies.iter().map(Vec::len).sum::<usize>().to_string();
    let args = [
        "publish-inputs",
        "--inventory",
        directory.join("inventory.json").to_str().unwrap(),
        "--root",
        directory.to_str().unwrap(),
        "--installation",
        directory.join("installation.candid").to_str().unwrap(),
        "--max-bytes",
        &maximum,
        "--max-total-bytes",
        &total,
        "--run-dir",
        directory.join("batch").to_str().unwrap(),
    ]
    .map(str::to_owned);
    assert_eq!(run(&args, 0)["all_bodies_verified"], true);
    permissions
}
fn args(f: &Fixture, d: &Path, url: &str, label: &str) -> Vec<String> {
    std::fs::write(d.join("tenant.pem"), PEM).unwrap();
    std::fs::write(d.join("root.der"), f.harness.pic.root_key().unwrap()).unwrap();
    [
        "publish-check",
        "--network",
        "local",
        "--url",
        url,
        "--identity",
        d.join("tenant.pem").to_str().unwrap(),
        "--actor",
        &f.tenant.to_text(),
        "--service",
        &f.service.to_text(),
        "--namespace",
        &f.config.namespace.to_string(),
        "--root-key",
        d.join("root.der").to_str().unwrap(),
        "--inputs",
        d.join("batch").to_str().unwrap(),
        "--max-bytes",
        "1024",
        "--max-total-bytes",
        "1024",
        "--max-queries",
        "2",
        "--timeout-seconds",
        "30",
        "--run-dir",
        d.join(label).to_str().unwrap(),
    ]
    .map(str::to_owned)
    .to_vec()
}
#[test]
fn standalone_batch_check_observes_absence_original_reservation_and_same_release_restore_without_mutation()
 {
    let mut f = setup();
    let d = tempfile::tempdir().unwrap();
    let p = freeze(&f, d.path());
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    let before = f.harness.pic.get_stable_memory(f.service);
    let absent = run(&args(&f, d.path(), &url, "absent"), 0);
    assert_eq!(absent["blocked"], false);
    assert_eq!(absent["files"][0]["status"], "not_visible");
    assert_eq!(absent["not_visible_demand"]["objects"], 1);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (p,),
        )
        .unwrap()
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let pending = run(&args(&f, d.path(), &url, "pending"), 0);
    assert_eq!(pending["blocked"], true);
    assert_eq!(pending["files"][0]["status"], "recover_existing_operation");
    assert_eq!(
        pending["files"][0]["original_upload"]["upload"],
        p.upload.upload.to_string()
    );
    assert_eq!(pending["capacity_reserved"], false);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let repeated = run(&args(&f, d.path(), &url, "pending"), 3);
    assert_eq!(repeated["error"], "new_run_required");
    f.harness.pic.stop_progress();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    f.harness.pic.auto_progress();
    let before = f.harness.pic.get_stable_memory(f.service);
    let restored = run(&args(&f, d.path(), &url, "restored"), 0);
    assert_eq!(restored["capacity"]["fenced"], true);
    assert!(
        restored["blockers"]
            .as_array()
            .unwrap()
            .contains(&json!("service_fenced"))
    );
    assert_eq!(restored["files"][0]["status"], "recover_existing_operation");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
}
#[test]
fn standalone_batch_check_retains_history_exhaustion_after_unexposed_cancellation() {
    let mut f = setup();
    let d = tempfile::tempdir().unwrap();
    let mut p = freeze(&f, d.path());
    p.upload.root = [77; 32];
    p.upload.upload = 99;
    p.upload.object = 99;
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (p,),
        )
        .unwrap()
        .unwrap();
    f.harness
        .pic
        .update_candid_as::<Result<UploadRevocationResponse, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_revoke_upload",
            (p,),
        )
        .unwrap()
        .unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    let before = f.harness.pic.get_stable_memory(f.service);
    let report = run(&args(&f, d.path(), &url, "capacity"), 0);
    assert_eq!(report["files"][0]["status"], "not_visible");
    assert_eq!(report["capacity"]["remaining_objects"], 0);
    assert!(
        report["blockers"]
            .as_array()
            .unwrap()
            .contains(&json!("object_history_capacity"))
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
}
