//! Actual standalone signed map checks, with locally trusted synthetic completion.
use super::*;
use crate::{
    account_native_cli::signer,
    authenticated_cli::{PEM, run},
    standalone_publish_check::freeze_files,
    upload_setup_cli::uploader,
};
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationMutation;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationRequest;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::{Value, json};
use std::path::Path;

fn setup(directory: &Path) -> (Fixture, Vec<UploadAdmissionRequest>) {
    let mut f = Fixture::with_profile(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        Fake::principal(5),
        signer(),
        uploader(),
        Envelope::Regular,
        Fake::principal(90),
        "publish-map-fixture",
    );
    f.tenant = signer();
    f.enroll(f.operator).unwrap();
    let permissions = freeze_files(&f, directory, &[1024, 2048]);
    std::fs::write(directory.join("tenant.pem"), PEM).unwrap();
    std::fs::write(
        directory.join("root.der"),
        f.harness.pic.root_key().unwrap(),
    )
    .unwrap();
    (f, permissions)
}
fn manifest(directory: &Path, index: usize) -> UploadManifestRequest {
    candid::decode_one(
        &std::fs::read(directory.join(format!("batch/file-{index:04}/manifest.candid"))).unwrap(),
    )
    .unwrap()
}
pub(super) fn confirm(f: &Fixture, manifest: &UploadManifestRequest) {
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (manifest.permission,),
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, manifest).unwrap();
    let root = crate::standalone_certificate::root(manifest);
    f.harness
        .pic
        .update_call(
            f.service,
            f.uploader,
            ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
            candid::encode_one(root).unwrap(),
        )
        .unwrap();
    let statement = UploadAttestationRequest {
        permission: manifest.permission,
        content_digest: *ContentDigest::compute(&vec![
            42;
            usize::try_from(
                manifest.permission.upload.bytes
            )
            .unwrap()
        ])
        .as_bytes(),
        observed_at_ns: f.harness.pic.get_time().as_nanos_since_unix_epoch(),
    };
    // A locally trusted statement exercises service confirmation, not provider availability.
    f.harness
        .pic
        .update_candid_as::<Result<UploadAttestationMutation, UploadAttestationFailure>, _>(
            f.service,
            Fake::principal(90),
            "blob_attest_upload",
            (statement,),
        )
        .unwrap()
        .unwrap();
}
fn args(f: &mut Fixture, directory: &Path, label: &str) -> Vec<String> {
    let mut url =
        f.harness
            .pic
            .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None);
    // PocketIC's already-live URL getter uses localhost even when started with
    // a literal domain. Preserve the maintained local CLI's literal-only policy.
    url.set_host(Some("127.0.0.1")).unwrap();
    let url = url.to_string();
    [
        "publish-map",
        "--network",
        "local",
        "--url",
        &url,
        "--identity",
        directory.join("tenant.pem").to_str().unwrap(),
        "--actor",
        &f.tenant.to_text(),
        "--operator-identity",
        directory.join("tenant.pem").to_str().unwrap(),
        "--service",
        &f.service.to_text(),
        "--namespace",
        &f.config.namespace.to_string(),
        "--root-key",
        directory.join("root.der").to_str().unwrap(),
        "--inputs",
        directory.join("batch").to_str().unwrap(),
        "--gateway",
        "https://127.0.0.1:65530",
        "--max-bytes",
        "2048",
        "--max-total-bytes",
        "3072",
        "--max-queries",
        "6",
        "--timeout-seconds",
        "30",
        "--run-dir",
        directory.join(label).to_str().unwrap(),
    ]
    .map(str::to_owned)
    .to_vec()
}
fn read_map(directory: &Path, label: &str) -> Value {
    serde_json::from_slice(&std::fs::read(directory.join(label).join("media-map.json")).unwrap())
        .unwrap()
}
#[test]
fn standalone_publish_map_requires_every_completed_live_reference_and_preserves_state() {
    let directory = tempfile::tempdir().unwrap();
    let (mut f, permissions) = setup(directory.path());
    confirm(&f, &manifest(directory.path(), 0));
    let mut file_args = args(&mut f, directory.path(), "first-file");
    file_args[0] = "publish-file-status".into();
    crate::reference_cli::change_native(&mut file_args, "--max-queries", "4");
    file_args.extend(["--file-index".into(), "0".into()]);
    let first = run(&file_args, 0);
    assert_eq!(first["file_live"], true);
    assert_eq!(first["batch_complete"], false);
    assert_eq!(first["files"][0]["index"], 0);
    assert!(!directory.path().join("first-file/media-map.json").exists());
    let incomplete = run(&args(&mut f, directory.path(), "incomplete"), 0);
    assert_eq!(incomplete["all_references_live"], false);
    assert_eq!(incomplete["blockers"][0]["code"], "completion_unmatched");
    assert!(!directory.path().join("incomplete/media-map.json").exists());
    f.harness.pic.stop_live();
    confirm(&f, &manifest(directory.path(), 1));
    let before = f.harness.pic.get_stable_memory(f.service);
    let complete = run(&args(&mut f, directory.path(), "complete"), 0);
    assert_eq!(complete["all_references_live"], true);
    assert_eq!(read_map(directory.path(), "complete"), complete);
    assert_eq!(complete["files"][1]["index"], 1);
    assert_eq!(
        complete["files"][1]["body_sha256"],
        ContentDigest::compute(&vec![42; 2048])
            .to_string()
            .trim_start_matches("sha256:")
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    f.harness
        .pic
        .update_candid_as::<Result<ReferenceMutationResponse, ReferenceFailure>, _>(
            f.service,
            f.tenant,
            "blob_apply_reference",
            (ReferenceCommand {
                upload: permissions[1].upload,
                reference: permissions[1].upload.first_reference,
                operation: 900,
                action: ReferenceAction::Release,
            },),
        )
        .unwrap()
        .unwrap();
    let released = run(&args(&mut f, directory.path(), "released"), 0);
    assert_eq!(released["all_references_live"], false);
    assert_eq!(
        released["blockers"][0],
        json!({"index":1,"code":"reference_inactive"})
    );
    assert!(!directory.path().join("released/media-map.json").exists());
    f.harness.pic.stop_live();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = run(&args(&mut f, directory.path(), "restored"), 0);
    assert_eq!(restored["all_references_live"], false);
    assert!(!directory.path().join("restored/media-map.json").exists());
}

#[test]
fn standalone_publish_map_refuses_bad_frozen_bytes_budget_and_operator_before_claiming_output() {
    use crate::{account_native_cli::OUTSIDER_PEM, reference_cli::change_native};
    let directory = tempfile::tempdir().unwrap();
    let (mut f, _) = setup(directory.path());
    let mut input = args(&mut f, directory.path(), "refused");
    input[0] = "publish-file-status".into();
    change_native(&mut input, "--max-queries", "4");
    input.extend(["--file-index".into(), "2".into()]);
    assert_eq!(run(&input, 2)["error"], "arguments");
    change_native(&mut input, "--file-index", "01");
    assert_eq!(run(&input, 2)["error"], "arguments");
    change_native(&mut input, "--file-index", "0");
    change_native(&mut input, "--max-queries", "3");
    assert_eq!(run(&input, 3)["error"], "reply_limit");
    assert!(!directory.path().join("refused").exists());
    input = args(&mut f, directory.path(), "refused");
    change_native(&mut input, "--max-queries", "5");
    assert_eq!(run(&input, 3)["error"], "reply_limit");
    assert!(!directory.path().join("refused").exists());
    change_native(&mut input, "--max-queries", "6");
    std::fs::write(directory.path().join("wrong.pem"), OUTSIDER_PEM).unwrap();
    change_native(
        &mut input,
        "--operator-identity",
        directory.path().join("wrong.pem").to_str().unwrap(),
    );
    assert_eq!(run(&input, 3)["error"], "identity_binding");
    assert!(!directory.path().join("refused").exists());
    change_native(
        &mut input,
        "--operator-identity",
        directory.path().join("tenant.pem").to_str().unwrap(),
    );
    std::fs::write(
        directory.path().join("batch/file-0001/body.bin"),
        vec![41; 2048],
    )
    .unwrap();
    assert_eq!(run(&input, 3)["error"], "content_mismatch");
    assert!(!directory.path().join("refused").exists());
}
