//! Actual signed session phases, bounded control and one-pass frozen-body reuse.
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
        ic_blob_storage::model::identity::ContentDigest::compute(&[42; 1024])
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
    use crate::reference_cli::change_native;
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
    use crate::reference_cli::change_native;
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
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    let recovered = phase(
        &mut session,
        &json!({"phase":"prepare","index":0,"source_run":original}),
    );
    assert_eq!(recovered["prepared"], true);
    assert_eq!(recovered["service_updates_this_run"], 0);
    assert_eq!(recovered["original_run"], original);
    assert_eq!(session.finish(0)["code"], "step_budget_exhausted");
    assert_eq!(
        std::fs::read(Path::new(original).join("admission/signed-request.cbor")).unwrap(),
        signed
    );
    std::fs::write(Path::new(original).join("manifest.candid"), b"DIDL").unwrap();
    change_native(
        &mut args,
        "--run-dir",
        directory.path().join("tampered").to_str().unwrap(),
    );
    let mut session = NativeSession::start(&args);
    assert_eq!(session.read()["event"], "ready");
    let before = f.harness.pic.get_stable_memory(f.service);
    session.send(&json!({"phase":"prepare","index":0,"source_run":original}));
    assert_eq!(session.finish(3)["error"], "binding");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert!(!directory.path().join("tampered/step-0000/setup").exists());
    f.harness.pic.stop_live();
}
