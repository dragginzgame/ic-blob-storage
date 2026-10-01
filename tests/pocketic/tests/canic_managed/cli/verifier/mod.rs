//! Signed managed verifier workflow over labelled local bytes/exposure and real IC updates.
use super::{Fixture, OUTSIDER_PEM, live, local_subnet_key, manifest};
use crate::{
    authenticated_cli::{PEM, run},
    observation_provider::serve,
    submission_proxy::{Dispatch, Proxy, Reply},
};
use candid::Principal;
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage::{
    dto::{
        download::{DownloadFailure, DownloadRequest, DownloadResponse},
        upload::{
            admission::{UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest},
            completion::{
                UploadAttestationFailure, UploadAttestationLookup, UploadAttestationRequest,
                UploadAttestationResponse,
            },
            manifest::{UploadManifestFailure, UploadManifestMutation},
        },
    },
    model::{
        identity::{ContentDigest, ProviderRootHash},
        service::read::download::CaffeineDownloadScope,
    },
    ops::caffeine::download::request_target,
};
use ic_testkit::pic::CandidCallExt;
use serde_json::{Value, json};
use std::{
    net::TcpListener,
    path::{Path, PathBuf},
    time::Duration,
};

fn change(args: &mut [String], flag: &str, value: &str) {
    let index = args.iter().position(|s| s == flag).unwrap();
    args[index + 1] = value.into();
}
pub(super) fn arguments(f: &Fixture, actor: Principal, url: &str, keys: &Path) -> Vec<String> {
    [
        "observe-upload",
        "--network",
        "local",
        "--url",
        url,
        "--identity",
        keys.join("identity.pem").to_str().unwrap(),
        "--root-key",
        keys.join("root.der").to_str().unwrap(),
        "--actor",
        &actor.to_text(),
        "--service",
        &f.app().to_text(),
        "--namespace",
        &u128::MAX.to_string(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
pub(super) fn command(args: &[String], name: &str, run_dir: &Path) -> Vec<String> {
    let mut result = args.to_vec();
    result[0] = name.into();
    result.extend(["--run-dir".into(), run_dir.to_str().unwrap().into()]);
    result
}
fn evidence_directory(temporary: &Path, label: &str) -> PathBuf {
    let directory = std::env::var_os("BLOB_MANAGED_VERIFIER_REPORT").map_or_else(
        || temporary.join(label),
        |root| PathBuf::from(root).join(label),
    );
    // A retained or partial run is never silently overwritten by a repeat test.
    std::fs::create_dir(&directory).unwrap();
    directory
}
fn json_file(file: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(file).unwrap()).unwrap()
}
fn prepared(f: &Fixture) -> UploadAdmissionRequest {
    let (scope, _) = super::super::installation::enroll(f);
    let declaration = manifest(f, scope.tenant, Principal::from_slice(&[6, 1]));
    f.pic()
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.app(),
            scope.tenant,
            "blob_admit_upload",
            (declaration.permission,),
        )
        .unwrap()
        .unwrap();
    f.pic()
        .update_candid_as::<Result<UploadManifestMutation, UploadManifestFailure>, _>(
            f.app(),
            declaration.permission.uploader,
            "blob_prepare_upload",
            (&declaration,),
        )
        .unwrap()
        .unwrap();
    declaration.permission
}
fn no_source_request(listener: &TcpListener) {
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
fn before_exposure(f: &Fixture, args: &[String], report: &Path, listener: &TcpListener) {
    let before = f.pic().get_stable_memory(f.app());
    let mut observe = args.to_vec();
    change(
        &mut observe,
        "--run-dir",
        report.join("unexposed").to_str().unwrap(),
    );
    assert_eq!(run(&observe, 3)["error"], "verification_refused");
    assert_eq!(
        json_file(&report.join("unexposed/failure.json"))["error"],
        "verification_refused"
    );
    assert!(!report.join("unexposed/download-request.json").exists());
    no_source_request(listener);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

fn foreign_verifier(
    f: &Fixture,
    args: &[String],
    report: &Path,
    keys: &Path,
    listener: &TcpListener,
) {
    let before = f.pic().get_stable_memory(f.app());
    let outsider = BasicIdentity::from_raw_key(&[43; 32]).sender().unwrap();
    let key = keys.join("outsider.pem");
    std::fs::write(&key, OUTSIDER_PEM).unwrap();
    let mut observe = args.to_vec();
    change(&mut observe, "--identity", key.to_str().unwrap());
    change(&mut observe, "--actor", &outsider.to_text());
    change(
        &mut observe,
        "--run-dir",
        report.join("foreign-verifier").to_str().unwrap(),
    );
    assert_eq!(run(&observe, 3)["error"], "verification_refused");
    assert!(
        !report
            .join("foreign-verifier/download-request.json")
            .exists()
    );
    no_source_request(listener);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}
fn receipt(
    f: &Fixture,
    actor: Principal,
    permission: UploadAdmissionRequest,
) -> UploadAttestationResponse {
    f.pic()
        .query_candid_as::<Result<_, UploadAttestationFailure>, _>(
            f.app(),
            actor,
            "blob_upload_attestation",
            (permission,),
        )
        .unwrap()
        .unwrap()
}
pub(super) fn observe(
    f: &Fixture,
    args: &[String],
    observed: &Path,
    listener: &TcpListener,
    permission: UploadAdmissionRequest,
) -> UploadAttestationRequest {
    let scope =
        CaffeineDownloadScope::new(f.app(), u128::MAX.try_into().unwrap(), "fixture/β?&=").unwrap();
    let target = request_target(
        &scope,
        ProviderRootHash::try_from(permission.upload.root.as_slice()).unwrap(),
    );
    let before = f.pic().get_stable_memory(f.app());
    std::thread::scope(|threads| {
        let server = threads.spawn(|| serve(listener, observed, &target, &[42; 10]));
        let report = run(args, 0);
        assert_eq!(report["owner"], f.app().to_text());
        assert_eq!(report["project"], scope.project());
        assert_eq!(report["authentication"], "query_signatures");
        assert_eq!(report["attestation_dispatched"], false);
        assert_eq!(report["retry_authorized"], false);
        server.join().unwrap();
    });
    let statement: UploadAttestationRequest =
        candid::decode_one(&std::fs::read(observed.join("statement.candid")).unwrap()).unwrap();
    assert_eq!(statement.permission, permission);
    assert_eq!(
        statement.content_digest,
        *ContentDigest::compute(&[42; 10]).as_bytes()
    );
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    statement
}
fn submit_and_recover(
    f: &Fixture,
    base: &[String],
    observed: &Path,
    verifier: Principal,
    statement: &UploadAttestationRequest,
    reply: Reply,
) -> Vec<String> {
    assert_eq!(
        receipt(f, verifier, statement.permission).attestation,
        UploadAttestationLookup::Absent
    );
    let submit = command(base, "submit-attestation", observed);
    let summary = observed.join("summary.json");
    let original = std::fs::read(&summary).unwrap();
    let mut damaged: Value = serde_json::from_slice(&original).unwrap();
    damaged["project"] = "another project".into();
    std::fs::write(
        observed.join("fixture-tampered-summary.json"),
        damaged.to_string(),
    )
    .unwrap();
    std::fs::write(&summary, damaged.to_string()).unwrap();
    let refused = run(&submit, 3);
    std::fs::write(
        observed.join("fixture-tamper-refusal.json"),
        refused.to_string(),
    )
    .unwrap();
    assert_eq!(refused["error"], "invalid_observation");
    assert!(!observed.join("attestation").exists());
    std::fs::write(&summary, original).unwrap();
    match reply {
        Reply::Pass => assert_eq!(run(&submit, 0)["outcome"], "accepted"),
        Reply::Drop => assert_eq!(run(&submit, 3)["error"], "transport"),
        Reply::Pending => assert_eq!(run(&submit, 0)["outcome"], "pending"),
    }
    let dispatch = observed.join("attestation");
    let intent = std::fs::read(dispatch.join("intent.json")).unwrap();
    assert_eq!(run(&submit, 3)["error"], "submission_already_claimed");
    assert_eq!(std::fs::read(dispatch.join("intent.json")).unwrap(), intent);
    assert_eq!(
        std::fs::read(dispatch.join("statement.candid")).unwrap(),
        candid::encode_one(statement).unwrap()
    );
    let outcome = json_file(&dispatch.join("outcome.json"));
    assert_eq!(
        outcome["outcome"],
        match reply {
            Reply::Pass => "accepted",
            Reply::Drop => "uncertain",
            Reply::Pending => "pending",
        }
    );
    assert_eq!(outcome["retry_authorized"], false);
    let mut recovery = base.to_vec();
    recovery[0] = "upload-attestation".into();
    recovery.extend([
        "--verifier".into(),
        verifier.to_text(),
        "--statement".into(),
        dispatch.join("statement.candid").to_str().unwrap().into(),
    ]);
    let recovered = run(&recovery, 0);
    assert_eq!(recovered["outcome"], "matched");
    assert_eq!(recovered["retry_authorized"], false);
    std::fs::write(
        observed.join("fixture-recovered-receipt.json"),
        recovered.to_string(),
    )
    .unwrap();
    assert!(
        matches!(receipt(f, verifier, statement.permission).attestation, UploadAttestationLookup::Found(r) if r.request == *statement)
    );
    recovery
}
fn download(
    f: &Fixture,
    permission: UploadAdmissionRequest,
) -> Result<DownloadResponse, DownloadFailure> {
    let upload = permission.upload;
    f.pic()
        .update_candid_as(
            f.app(),
            upload.tenant,
            "blob_download_descriptor",
            (DownloadRequest {
                service: upload.service,
                namespace: upload.namespace,
                tenant: upload.tenant,
                object: upload.object,
                incarnation: upload.incarnation,
                root: upload.root,
                reference: upload.first_reference,
            },),
        )
        .unwrap()
}
fn restored(
    f: &Fixture,
    observe_args: &[String],
    recovery: &[String],
    report: &Path,
    permission: UploadAdmissionRequest,
) {
    let accepted = run(recovery, 0);
    f.pic().stop_progress();
    f.upgrade_same_release(Duration::from_secs(5));
    let before = f.pic().get_stable_memory(f.app());
    f.pic().auto_progress();
    let historical = run(recovery, 0);
    assert_eq!(historical["receipt"], accepted["receipt"]);
    assert_eq!(historical["fenced"], true);
    assert_eq!(historical["retry_authorized"], false);
    assert_eq!(historical["billing_cessation"], "not_established");
    std::fs::write(
        report.join("fixture-restored-receipt.json"),
        historical.to_string(),
    )
    .unwrap();
    let mut observe = observe_args.to_vec();
    change(
        &mut observe,
        "--run-dir",
        report.join("restored").to_str().unwrap(),
    );
    assert_eq!(run(&observe, 3)["error"], "verification_refused");
    assert!(!report.join("restored/download-request.json").exists());
    assert_eq!(download(f, permission), Err(DownloadFailure::Fenced));
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}
fn journey(reply: Reply, label: &str) {
    let temporary = tempfile::tempdir().unwrap();
    let report = evidence_directory(temporary.path(), label);
    let verifier = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    let mut input = blob_canic_probe::configuration::input();
    input.completion_verifier = verifier;
    let f = Fixture::with_input(&input);
    let permission = prepared(&f);
    assert!(
        ![
            input.operator,
            permission.uploader,
            permission.upload.tenant
        ]
        .contains(&verifier)
    );
    std::fs::write(temporary.path().join("identity.pem"), PEM).unwrap();
    let root = local_subnet_key(&f);
    std::fs::write(temporary.path().join("root.der"), &root).unwrap();
    std::fs::write(report.join("fixture-root.der"), root).unwrap();
    std::fs::write(
        report.join("permission.candid"),
        candid::encode_one(permission).unwrap(),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let gateway = format!("http://{}/", listener.local_addr().unwrap());
    std::fs::write(
        report.join("fixture-plan.json"),
        json!({"evidence":"local_substitute",
        "service":f.app().to_text(),"namespace":input.namespace.to_string(),"project":input.project,
        "operator":input.operator.to_text(),"tenant":permission.upload.tenant.to_text(),
        "uploader":permission.uploader.to_text(),"verifier":verifier.to_text(),"gateway":gateway,
        "reply":label,"max_source_gets":1,"max_signed_updates":1,"max_cli_invocations":16,
        "max_content_bytes":"10","deployed_provider_requests":0,"attached_provider_cycles":"0"})
        .to_string(),
    )
    .unwrap();
    let (mut replica, url) = live(&f);
    let mut args = arguments(&f, verifier, &url, temporary.path());
    let observed = report.join("observed");
    let mut observe_args = command(&args, "observe-upload", &observed);
    observe_args.extend([
        "--permission".into(),
        report.join("permission.candid").to_str().unwrap().into(),
        "--gateway".into(),
        gateway,
        "--max-bytes".into(),
        "10".into(),
    ]);
    before_exposure(&f, &observe_args, &report, &listener);
    f.pic().stop_progress();
    f.pic()
        .update_candid_as::<Result<(), blob_canic_probe::ProbeExposureFailure>, _>(
            f.app(),
            input.operator,
            "probe_expose_upload",
            (permission,),
        )
        .unwrap()
        .unwrap();
    f.pic().auto_progress();
    foreign_verifier(&f, &observe_args, &report, temporary.path(), &listener);
    let proxy = Proxy::start(
        url,
        observed.join("attestation"),
        Dispatch {
            service: f.app(),
            actor: verifier,
            method: "blob_attest_upload",
            argument_file: "statement.candid",
        },
        reply,
    );
    change(&mut args, "--url", &proxy.url);
    change(&mut observe_args, "--url", &proxy.url);
    let statement = observe(&f, &observe_args, &observed, &listener, permission);
    assert_eq!(run(&observe_args, 3)["error"], "new_run_required");
    let recovery = submit_and_recover(&f, &args, &observed, verifier, &statement, reply);
    let descriptor = download(&f, permission).unwrap();
    assert_eq!(
        descriptor.request.reference,
        permission.upload.first_reference
    );
    assert_eq!(descriptor.request.root, permission.upload.root);
    assert_eq!(descriptor.bytes, 10);
    restored(&f, &observe_args, &recovery, &report, permission);
    no_source_request(&listener);
    assert_eq!(proxy.calls(), 1, "receipt recovery never redispatches");
    std::fs::write(report.join("fixture-summary.json"), json!({"evidence":"local_substitute", "reply":label,
        "source_gets":1,"signed_updates":proxy.calls(),"deployed_provider_requests":0,"attached_provider_cycles":"0"}).to_string()).unwrap();
    drop(proxy);
    replica.stop_live();
}

#[test]
fn managed_native_verifier_submission_accepts_and_preserves_fenced_receipt() {
    journey(Reply::Pass, "acknowledged");
}
#[test]
fn managed_native_verifier_submission_recovers_lost_reply_without_resend() {
    journey(Reply::Drop, "dropped");
}
#[test]
fn managed_native_verifier_submission_recovers_pending_reply_without_resend() {
    journey(Reply::Pending, "pending");
}
