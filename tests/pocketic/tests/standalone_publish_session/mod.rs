//! Actual signed session phases, bounded control and one-pass frozen-body reuse.
use super::reference_cli::change_native;
use super::*;
use crate::{
    authenticated_cli::PEM, native_session::NativeSession, standalone_publish_check::freeze_files,
    standalone_publish_map::confirm, standalone_publish_prepare::Directory,
};
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn arguments(
    f: &Fixture,
    directory: &Path,
    url: &str,
    gateway: &str,
    label: &str,
) -> Vec<String> {
    std::fs::write(directory.join("tenant.pem"), PEM).unwrap();
    std::fs::write(
        directory.join("root.der"),
        f.harness.pic.root_key().unwrap(),
    )
    .unwrap();
    [
        "publish-session",
        "--network",
        "local",
        "--url",
        url,
        "--identity",
        directory.join("tenant.pem").to_str().unwrap(),
        "--actor",
        &f.tenant.to_text(),
        "--operator-identity",
        directory.join("operator.pem").to_str().unwrap(),
        "--uploader-identity",
        directory.join("uploader.pem").to_str().unwrap(),
        "--root-key",
        directory.join("root.der").to_str().unwrap(),
        "--service",
        &f.service.to_text(),
        "--namespace",
        &f.config.namespace.to_string(),
        "--gateway",
        gateway,
        "--inputs",
        directory.join("batch").to_str().unwrap(),
        "--max-bytes",
        "2048",
        "--max-total-bytes",
        "3072",
        "--max-steps",
        "17",
        "--timeout-seconds",
        "30",
        "--run-dir",
        directory.join(label).to_str().unwrap(),
    ]
    .map(str::to_owned)
    .to_vec()
}
fn fixture(directory: &Path) -> (Fixture, Vec<UploadManifestRequest>, String) {
    // Here the three setup roles intentionally coincide for scope checks. The
    // browser suite keeps tenant/verifier independent of the uploader/operator.
    let mut f = Fixture::with_profile(
        Harness::with_builder(
            ic_testkit::pocket_ic::PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        Fake::principal(5),
        crate::account_native_cli::signer(),
        crate::account_native_cli::signer(),
        Envelope::Regular,
        Fake::principal(90),
        "persistent-session-fixture",
    );
    f.tenant = crate::account_native_cli::signer();
    f.enroll(f.operator).unwrap();
    freeze_files(&f, directory, &[1024, 2048]);
    for name in ["operator.pem", "uploader.pem"] {
        std::fs::write(directory.join(name), PEM).unwrap();
    }
    let files = (0..2)
        .map(|index| {
            candid::decode_one(
                &std::fs::read(directory.join(format!("batch/file-{index:04}/manifest.candid")))
                    .unwrap(),
            )
            .unwrap()
        })
        .collect();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    (f, files, url)
}
fn phase(session: &mut NativeSession, frame: &Value) -> Value {
    session.send(frame);
    let result = session.read();
    assert_eq!(result["event"], "phase");
    result["report"].clone()
}

#[test]
fn standalone_publication_session_retains_transfer_before_dispatch_and_recovers_without_upload() {
    use crate::reference_cli::change_native;
    let directory = Directory::new("session-transfer-");
    let path = directory.path();
    let (mut f, _, url) = fixture(path);
    let mut args = arguments(&f, path, &url, "https://127.0.0.1:65530", "original");
    let binding: Value =
        serde_json::from_slice(&std::fs::read(path.join("batch/file-0000/binding.json")).unwrap())
            .unwrap();
    let selection = json!({"format":"ic-blob-storage/browser-selection",
        "session":path.join("original"),"profile":path.join("profile"),
        "project":binding["project"],"bucket":binding["bucket"],
        "asset_port":65529,"signer_sha256":"1".repeat(64),"host_sha256":"2".repeat(64),
        "worker_sha256":"3".repeat(64),"database":"native-transfer-fixture","max_slots":2});
    std::fs::write(
        path.join("selection.json"),
        serde_json::to_vec(&selection).unwrap(),
    )
    .unwrap();
    args.extend([
        "--browser-selection".into(),
        path.join("selection.json").display().to_string(),
    ]);
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    assert_eq!(
        phase(&mut session, &json!({"phase":"transfer","index":1}))["state"],
        "blocked"
    );
    let prepared = phase(&mut session, &json!({"phase":"prepare","index":0}));
    assert_eq!(prepared["prepared"], true);
    assert!(prepared["transfer"].is_null());
    let first = phase(&mut session, &json!({"phase":"transfer","index":0}));
    assert_eq!(first["recovery"], false);
    let original = Path::new(first["native_phase"].as_str().unwrap());
    let bytes = std::fs::read(original.join("transfer.json")).unwrap();
    let handoff: Value = serde_json::from_slice(&bytes).unwrap();
    assert!(handoff["source_transfer"].is_null());
    assert_eq!(handoff["source_run"], prepared["original_run"]);
    let repeated = phase(&mut session, &json!({"phase":"transfer","index":0}));
    assert_eq!(repeated["recovery"], true);
    assert_eq!(repeated["service_updates"], 0);
    assert_eq!(session.finish(3)["error"], "transport");
    // No browser was launched: the proposed fingerprints above are fixture
    // metadata, not evidence of real browser or provider authority.
    std::fs::create_dir(path.join("profile")).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    args.extend([
        "--source-session".into(),
        path.join("original").display().to_string(),
    ]);
    change_native(
        &mut args,
        "--run-dir",
        path.join("recovered").to_str().unwrap(),
    );
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    let recovered = phase(&mut session, &json!({"phase":"transfer","index":0}));
    assert_eq!(recovered["recovery"], true);
    assert_eq!(recovered["provider_requests"], 0);
    let record: Value = serde_json::from_slice(
        &std::fs::read(
            Path::new(recovered["native_phase"].as_str().unwrap()).join("transfer.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(record["source_transfer"], original.display().to_string());
    assert_eq!(record["transfer"], handoff["transfer"]);
    assert_eq!(session.finish(3)["error"], "transport");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert_eq!(
        std::fs::read(original.join("transfer.json")).unwrap(),
        bytes
    );
    std::fs::write(path.join("original-transfer.json"), &bytes).unwrap();
    std::fs::write(original.join("transfer.json"), b"{").unwrap();
    change_native(
        &mut args,
        "--run-dir",
        path.join("partial").to_str().unwrap(),
    );
    assert_eq!(NativeSession::start(&args).finish(3)["error"], "binding");
    assert!(!path.join("partial").exists());
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
}

fn recover_preparation(
    args: &mut [String],
    directory: &Path,
    source: &str,
    label: &str,
    original: &str,
) {
    use crate::reference_cli::change_native;
    change_native(
        args,
        "--source-session",
        directory.join(source).to_str().unwrap(),
    );
    change_native(args, "--run-dir", directory.join(label).to_str().unwrap());
    let mut session = NativeSession::start(args);
    assert_eq!(session.read()["event"], "ready");
    let recovered = phase(&mut session, &json!({"phase":"prepare","index":0}));
    assert_eq!(recovered["original_run"], original);
    assert_eq!(recovered["service_updates_this_run"], 0);
    assert_eq!(session.finish(0)["code"], "step_budget_exhausted");
}

#[test]
fn standalone_publication_session_requires_selected_installed_verifier_before_effects() {
    let directory = Directory::new("session-verifier-authority-");
    let (mut f, _, url) = fixture(directory.path());
    let mut args = arguments(
        &f,
        directory.path(),
        &url,
        "https://127.0.0.1:65530",
        "missing-verifier",
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    session.send(&json!({"phase":"verify","index":0}));
    assert_eq!(session.finish(3)["error"], "denied");
    assert!(
        !directory
            .path()
            .join("missing-verifier/verification-0000")
            .exists()
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    change_native(
        &mut args,
        "--run-dir",
        directory.path().join("wrong-verifier").to_str().unwrap(),
    );
    args.extend([
        "--verifier-identity".into(),
        directory.path().join("tenant.pem").display().to_string(),
    ]);
    let session = NativeSession::start(&args);
    assert_eq!(session.finish(3)["error"], "identity_binding");
    assert!(!directory.path().join("wrong-verifier").exists());
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
}

#[test]
fn standalone_publication_session_reuses_cached_batch_and_checks_selected_bytes_before_setup() {
    let directory = Directory::new("session-cached-");
    let (mut f, files, url) = fixture(directory.path());
    let args = arguments(
        &f,
        directory.path(),
        &url,
        "https://127.0.0.1:65530",
        "session",
    );
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    assert_eq!(
        phase(&mut session, &json!({"phase":"prepare","index":1}))["state"],
        "blocked"
    );
    assert_eq!(
        phase(&mut session, &json!({"phase":"map"}))["code"],
        "files_incomplete"
    );
    assert_eq!(
        phase(&mut session, &json!({"phase":"status","index":0}))["file_live"],
        false
    );
    let prepared = phase(&mut session, &json!({"phase":"prepare","index":0}));
    assert_eq!(prepared["prepared"], true);
    let original = prepared["original_run"].as_str().unwrap();
    assert_eq!(
        phase(&mut session, &json!({"phase":"prepare","index":0}))["service_updates_this_run"],
        0
    );
    assert!(
        Path::new(original)
            .join("admission/signed-request.cbor")
            .exists()
    );
    confirm(&f, &files[0]);
    assert_eq!(
        phase(&mut session, &json!({"phase":"status","index":0}))["file_live"],
        true
    );
    // Corrupt a completed local copy. Preparing the next file must use the
    // initially validated intent without re-reading every completed body.
    std::fs::write(
        directory.path().join("batch/file-0000/body.bin"),
        vec![0; 1024],
    )
    .unwrap();
    assert_eq!(
        phase(&mut session, &json!({"phase":"prepare","index":1}))["prepared"],
        true
    );
    confirm(&f, &files[1]);
    assert_eq!(
        phase(&mut session, &json!({"phase":"status","index":1}))["file_live"],
        true
    );
    let map = phase(&mut session, &json!({"phase":"map"}));
    assert_eq!(map["all_references_live"], true);
    assert_eq!(
        map["files"][0]["body_sha256"],
        ic_blob_storage_contracts::identity::ContentDigest::compute(&[42; 1024])
            .to_string()
            .trim_start_matches("sha256:")
    );
    assert_eq!(session.finish(0), map);
    assert!(directory.path().join("session/media-map.json").exists());
    f.harness.pic.stop_live();
}

#[test]
fn standalone_publication_session_rejects_changed_selected_body_before_admission_and_preserves_intent()
 {
    let directory = Directory::new("session-changed-");
    let (mut f, _, url) = fixture(directory.path());
    let args = arguments(
        &f,
        directory.path(),
        &url,
        "https://127.0.0.1:65530",
        "changed",
    );
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    let before = f.harness.pic.get_stable_memory(f.service);
    std::fs::write(
        directory.path().join("batch/file-0000/body.bin"),
        vec![0; 1024],
    )
    .unwrap();
    session.send(&json!({"phase":"prepare","index":0}));
    assert_eq!(session.finish(3)["error"], "content_mismatch");
    assert!(!directory.path().join("changed/step-0000/setup").exists());
    assert!(
        directory
            .path()
            .join("changed/step-0000/request.json")
            .exists()
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
}

#[test]
fn standalone_publication_session_bounds_steps_and_exits_idle_with_stdin_still_open() {
    let directory = Directory::new("session-bounded-");
    let (mut f, _, url) = fixture(directory.path());
    let mut args = arguments(
        &f,
        directory.path(),
        &url,
        "https://127.0.0.1:65530",
        "budget",
    );
    change_native(&mut args, "--max-steps", "1");
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    assert_eq!(
        phase(&mut session, &json!({"phase":"status","index":0}))["file_live"],
        false
    );
    assert_eq!(session.finish(0)["code"], "step_budget_exhausted");
    change_native(
        &mut args,
        "--run-dir",
        directory.path().join("idle").to_str().unwrap(),
    );
    change_native(&mut args, "--timeout-seconds", "1");
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    session.wait_for_deadline();
    assert_eq!(session.finish(3)["error"], "timeout");
    assert!(!directory.path().join("idle/media-map.json").exists());
    f.harness.pic.stop_live();
}

#[test]
fn standalone_publication_session_recovers_original_setup_after_control_loss_and_refuses_tampering()
{
    let directory = Directory::new("session-recovery-");
    let (mut f, _, url) = fixture(directory.path());
    let mut args = arguments(
        &f,
        directory.path(),
        &url,
        "https://127.0.0.1:65530",
        "original",
    );
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    let prepared = phase(&mut session, &json!({"phase":"prepare","index":0}));
    let original = prepared["original_run"].as_str().unwrap();
    let signed = std::fs::read(Path::new(original).join("admission/signed-request.cbor")).unwrap();
    assert_eq!(session.finish(3)["error"], "transport");
    change_native(
        &mut args,
        "--run-dir",
        directory.path().join("recovery").to_str().unwrap(),
    );
    change_native(&mut args, "--max-steps", "1");
    args.extend([
        "--source-session".into(),
        directory.path().join("original").display().to_string(),
    ]);
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    let recovered = phase(&mut session, &json!({"phase":"prepare","index":0}));
    assert_eq!(recovered["prepared"], true);
    assert_eq!(recovered["service_updates_this_run"], 0);
    assert_eq!(recovered["original_run"], original);
    assert_eq!(session.finish(0)["code"], "step_budget_exhausted");
    assert_eq!(
        std::fs::read(Path::new(original).join("admission/signed-request.cbor")).unwrap(),
        signed
    );
    // Reopen the recovery run itself. Its resolved control record names the
    // original claim directly, with no recursive session-history traversal.
    let before = f.harness.pic.get_stable_memory(f.service);
    recover_preparation(
        &mut args,
        directory.path(),
        "recovery",
        "repeated-recovery",
        original,
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    // A status-only recovery must retain sources that this run never used.
    change_native(
        &mut args,
        "--source-session",
        directory.path().join("repeated-recovery").to_str().unwrap(),
    );
    change_native(
        &mut args,
        "--run-dir",
        directory.path().join("status-only").to_str().unwrap(),
    );
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    assert_eq!(
        phase(&mut session, &json!({"phase":"status","index":0}))["file_live"],
        false
    );
    assert_eq!(session.finish(0)["code"], "step_budget_exhausted");
    recover_preparation(
        &mut args,
        directory.path(),
        "status-only",
        "after-status",
        original,
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    std::fs::write(Path::new(original).join("manifest.candid"), b"DIDL").unwrap();
    change_native(
        &mut args,
        "--run-dir",
        directory.path().join("tampered").to_str().unwrap(),
    );
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    let before = f.harness.pic.get_stable_memory(f.service);
    session.send(&json!({"phase":"prepare","index":0}));
    assert_eq!(session.finish(3)["error"], "binding");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert!(!directory.path().join("tampered/step-0000/setup").exists());
    f.harness.pic.stop_live();
}

#[test]
fn standalone_publication_session_refuses_changed_or_partial_source_before_new_run() {
    let directory = Directory::new("session-source-binding-");
    let (mut f, _, url) = fixture(directory.path());
    let mut args = arguments(
        &f,
        directory.path(),
        &url,
        "https://127.0.0.1:65530",
        "original",
    );
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    let prepared = phase(&mut session, &json!({"phase":"prepare","index":0}));
    let signed_path =
        Path::new(prepared["original_run"].as_str().unwrap()).join("admission/signed-request.cbor");
    let signed = std::fs::read(&signed_path).unwrap();
    assert_eq!(session.finish(3)["error"], "transport");
    args.extend([
        "--source-session".into(),
        directory.path().join("original").display().to_string(),
    ]);
    change_native(
        &mut args,
        "--run-dir",
        directory.path().join("changed").to_str().unwrap(),
    );
    change_native(&mut args, "--gateway", "https://127.0.0.1:65529");
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(NativeSession::start(&args).finish(3)["error"], "binding");
    assert!(!directory.path().join("changed").exists());
    change_native(&mut args, "--gateway", "https://127.0.0.1:65530");
    change_native(
        &mut args,
        "--run-dir",
        directory.path().join("partial").to_str().unwrap(),
    );
    std::fs::create_dir(directory.path().join("original/step-0001")).unwrap();
    assert_eq!(NativeSession::start(&args).finish(3)["error"], "binding");
    assert!(!directory.path().join("partial").exists());
    assert_eq!(std::fs::read(signed_path).unwrap(), signed);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
}
