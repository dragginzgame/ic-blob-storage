//! Signed installed plan, real local HTTP bytes and saved statement; provider facts substituted.
mod proxy;
use super::*;
use crate::authenticated_cli::{PEM, run};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage::{dto::upload::completion::*, model::identity::ContentDigest};
use ic_testkit::pocket_ic::PocketIcBuilder;
use std::{
    io::{Read, Write},
    net::TcpListener,
    path::Path,
    time::{Duration, Instant},
};

fn change(args: &mut [String], flag: &str, value: &str) {
    let i = args.iter().position(|s| s == flag).unwrap();
    args[i + 1] = value.into();
}
fn live(f: &mut Fixture) -> String {
    f.harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string()
}
fn arguments(f: &Fixture, url: &str, directory: &Path, gateway: &str) -> Vec<String> {
    [
        "observe-upload",
        "--network",
        "local",
        "--url",
        url,
        "--identity",
        directory.join("identity.pem").to_str().unwrap(),
        "--root-key",
        directory.join("root.der").to_str().unwrap(),
        "--actor",
        &f.operator.to_text(),
        "--service",
        &f.service.to_text(),
        "--namespace",
        "1",
        "--permission",
        directory.join("permission.candid").to_str().unwrap(),
        "--gateway",
        gateway,
        "--max-bytes",
        "10",
        "--run-dir",
        directory.join("unexposed").to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
fn incoming(listener: &TcpListener) -> (std::net::TcpStream, String) {
    listener.set_nonblocking(true).unwrap();
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut stream = loop {
        match listener.accept() {
            Ok((stream, _)) => break stream,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(5));
            }
            other => panic!("expected one local provider request: {other:?}"),
        }
    };
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut request = Vec::new();
    while !request.ends_with(b"\r\n\r\n") {
        assert!(request.len() < 8192);
        let mut byte = [0];
        stream.read_exact(&mut byte).unwrap();
        request.push(byte[0]);
    }
    (stream, String::from_utf8(request).unwrap())
}
fn provider(listener: &TcpListener, path: &Path, expected: &str) {
    let (mut stream, request) = incoming(listener);
    assert_eq!(
        request.lines().next().unwrap(),
        format!("GET {expected} HTTP/1.1")
    );
    assert!(!request.to_ascii_lowercase().contains("authorization:"));
    for file in [
        "plan.json",
        "permission.candid",
        "service-response.candid",
        "download-request.json",
    ] {
        assert!(
            path.join(file).is_file(),
            "intent/declaration must precede provider GET"
        );
    }
    assert!(!path.join("statement.candid").exists());
    stream
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\n")
        .unwrap();
    stream.write_all(&[5; 10]).unwrap();
}
fn interrupted(args: &[String], directory: &Path) {
    let mut args = args.to_vec();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let path = directory.join("interrupted");
    change(
        &mut args,
        "--gateway",
        &format!("http://{}", listener.local_addr().unwrap()),
    );
    change(&mut args, "--run-dir", path.to_str().unwrap());
    let mut child = std::process::Command::new(fixture_path("BLOB_CLI_BIN"))
        .args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let (mut stream, _) = incoming(&listener);
    let intent = std::fs::read(path.join("download-request.json")).unwrap();
    stream
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\n")
        .unwrap();
    stream.write_all(&[5; 3]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.join("http-response.json").exists() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
    child.kill().unwrap();
    assert!(!child.wait_with_output().unwrap().status.success());
    drop(stream);
    drop(listener);
    assert!(!path.join("statement.candid").exists());
    assert!(!path.join("summary.json").exists());
    let mut submit = shared_arguments("submit-attestation", &args);
    submit.extend(["--run-dir".into(), path.to_str().unwrap().into()]);
    assert_eq!(run(&submit, 3)["error"], "invalid_observation");
    assert!(!path.join("attestation").exists());
    assert_eq!(run(&args, 3)["error"], "new_run_required");
    assert_eq!(
        std::fs::read(path.join("download-request.json")).unwrap(),
        intent
    );
}
fn receipt(f: &Fixture, statement: &UploadAttestationRequest) -> UploadAttestationResponse {
    f.harness
        .pic
        .query_candid_as::<Result<_, UploadAttestationFailure>, _>(
            f.service,
            f.operator,
            "blob_upload_attestation",
            (statement.permission,),
        )
        .unwrap()
        .unwrap()
}
fn attest_and_recover(f: &mut Fixture, path: &Path, args: &[String], reply: proxy::Reply) {
    let saved = std::fs::read(path.join("statement.candid")).unwrap();
    let statement: UploadAttestationRequest = candid::decode_one(&saved).unwrap();
    assert_eq!(
        statement.content_digest,
        *ContentDigest::compute(&[5; 10]).as_bytes()
    );
    assert_eq!(
        receipt(f, &statement).attestation,
        UploadAttestationLookup::Absent
    );
    let mut submit = shared_arguments("submit-attestation", args);
    submit.extend(["--run-dir".into(), path.to_str().unwrap().into()]);
    if reply == proxy::Reply::Pass {
        reject_damaged_runs(path, &submit);
    }
    f.harness.pic.advance_time(Duration::from_secs(1));
    if reply == proxy::Reply::Drop {
        assert_eq!(run(&submit, 3)["error"], "transport");
    } else if reply == proxy::Reply::Pending {
        assert_eq!(run(&submit, 0)["outcome"], "pending");
    } else {
        concurrent_submission(&submit);
    }
    let intent = std::fs::read(path.join("attestation/intent.json")).unwrap();
    assert_eq!(run(&submit, 3)["error"], "submission_already_claimed");
    assert_eq!(
        std::fs::read(path.join("attestation/intent.json")).unwrap(),
        intent
    );
    // PocketIC executes the request before completing its synchronous call response.
    assert!(
        matches!(receipt(f, &statement).attestation, UploadAttestationLookup::Found(r) if r.request == statement)
    );
    let mut recovery = shared_arguments("upload-attestation", args);
    recovery.extend([
        "--verifier".into(),
        f.operator.to_text(),
        "--statement".into(),
        path.join("attestation/statement.candid")
            .to_str()
            .unwrap()
            .into(),
    ]);
    assert_eq!(run(&recovery, 0)["outcome"], "matched");
    assert_eq!(std::fs::read(path.join("statement.candid")).unwrap(), saved);
    assert_eq!(
        std::fs::read(path.join("attestation/statement.candid")).unwrap(),
        saved
    );
    let outcome: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path.join("attestation/outcome.json")).unwrap())
            .unwrap();
    if reply == proxy::Reply::Drop {
        assert_eq!(outcome["outcome"], "uncertain");
    } else if reply == proxy::Reply::Pending {
        assert_eq!(outcome["outcome"], "pending");
    }
}
fn shared_arguments(command: &str, args: &[String]) -> Vec<String> {
    let mut result = vec![command.into()];
    for flag in [
        "--network",
        "--url",
        "--identity",
        "--root-key",
        "--actor",
        "--service",
        "--namespace",
    ] {
        let i = args.iter().position(|s| s == flag).unwrap();
        result.extend([flag.into(), args[i + 1].clone()]);
    }
    result
}
fn concurrent_submission(args: &[String]) {
    let barrier = std::sync::Barrier::new(2);
    let invoke = || {
        barrier.wait();
        let output = std::process::Command::new(fixture_path("BLOB_CLI_BIN"))
            .args(args)
            .output()
            .unwrap();
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        (output.status.code(), value)
    };
    let results = std::thread::scope(|threads| {
        let a = threads.spawn(invoke);
        let b = threads.spawn(invoke);
        [a.join().unwrap(), b.join().unwrap()]
    });
    let success: Vec<_> = results
        .iter()
        .filter(|(code, _)| *code == Some(0))
        .collect();
    assert_eq!(success.len(), 1);
    assert!(matches!(
        success[0].1["outcome"].as_str(),
        Some("accepted" | "pending")
    ));
    assert_eq!(success[0].1["retry_authorized"], false);
    let refused = results.iter().find(|(code, _)| *code != Some(0)).unwrap();
    assert_eq!(refused.0, Some(3));
    assert_eq!(refused.1["error"], "submission_already_claimed");
}
fn reject_damaged_runs(path: &Path, args: &[String]) {
    for name in [
        "plan.json",
        "summary.json",
        "statement.candid",
        "permission.candid",
        "service-response.candid",
        "download-request.json",
        "download-outcome.json",
        "http-response.json",
    ] {
        let file = path.join(name);
        let saved = std::fs::read(&file).unwrap();
        std::fs::remove_file(&file).unwrap();
        assert_eq!(run(args, 3)["error"], "invalid_observation");
        assert!(!path.join("attestation").exists());
        std::fs::write(file, saved).unwrap();
    }
    for (name, field, value) in [
        (
            "summary.json",
            "observation",
            serde_json::json!("local_file"),
        ),
        ("summary.json", "content_digest", serde_json::json!("00")),
        ("summary.json", "observed_at_ns", serde_json::json!("0")),
        ("summary.json", "owner", serde_json::json!("aaaaa-aa")),
        (
            "summary.json",
            "project",
            serde_json::json!("another project"),
        ),
        (
            "summary.json",
            "attestation_dispatched",
            serde_json::json!(true),
        ),
        ("plan.json", "verifier", serde_json::json!("aaaaa-aa")),
        ("plan.json", "namespace", serde_json::json!("2")),
        (
            "plan.json",
            "service_url",
            serde_json::json!("http://127.0.0.1:9/"),
        ),
        (
            "download-request.json",
            "url",
            serde_json::json!("http://127.0.0.1:9/"),
        ),
        (
            "download-request.json",
            "service_reply_sha256",
            serde_json::json!("00"),
        ),
        (
            "download-outcome.json",
            "outcome",
            serde_json::json!("failed"),
        ),
        (
            "download-outcome.json",
            "received_bytes",
            serde_json::json!("9"),
        ),
        ("http-response.json", "status", serde_json::json!(206)),
    ] {
        let file = path.join(name);
        let saved = std::fs::read(&file).unwrap();
        let mut record: serde_json::Value = serde_json::from_slice(&saved).unwrap();
        record[field] = value;
        std::fs::write(&file, serde_json::to_vec(&record).unwrap()).unwrap();
        assert_eq!(run(args, 3)["error"], "invalid_observation");
        assert!(!path.join("attestation").exists());
        std::fs::write(file, saved).unwrap();
    }
    std::fs::write(path.join("failure.json"), b"{}").unwrap();
    assert_eq!(run(args, 3)["error"], "invalid_observation");
    std::fs::remove_file(path.join("failure.json")).unwrap();
    let summary = std::fs::read(path.join("summary.json")).unwrap();
    std::fs::write(path.join("summary.json"), vec![b' '; 16 * 1024 + 1]).unwrap();
    assert_eq!(run(args, 3)["error"], "invalid_observation");
    std::fs::write(path.join("summary.json"), summary).unwrap();
    assert!(!path.join("attestation").exists());
}
#[test]
fn observer_persists_content_then_explicit_native_submission_recovers_exact_receipt() {
    journey(proxy::Reply::Pass);
}
#[test]
fn native_submission_retains_uncertainty_after_actual_lost_reply_and_recovers_without_resend() {
    journey(proxy::Reply::Drop);
}
#[test]
fn pending_transport_response_requires_receipt_inspection_without_resend() {
    journey(proxy::Reply::Pending);
}
fn journey(reply: proxy::Reply) {
    let operator = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    let mut f = Fixture::with_operator(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        operator,
    );
    f.enroll(None, true).unwrap();
    let (permission, manifest) = f.permission(u128::MAX, 5);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&manifest).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let dir = directory.path();
    std::fs::write(dir.join("identity.pem"), PEM).unwrap();
    std::fs::write(dir.join("root.der"), f.harness.pic.root_key().unwrap()).unwrap();
    std::fs::write(
        dir.join("permission.candid"),
        candid::encode_one(admission_input(permission)).unwrap(),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let gateway = format!("http://{}", listener.local_addr().unwrap());
    let url = live(&mut f);
    let mut args = arguments(&f, &url, dir, &gateway);
    assert_eq!(run(&args, 3)["error"], "verification_refused");
    assert!(dir.join("unexposed/failure.json").exists());
    assert!(!dir.join("unexposed/download-request.json").exists());
    f.harness.pic.stop_live();
    f.expose(permission.request).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let observed = dir.join("observed");
    let proxy = proxy::Proxy::start(
        live(&mut f),
        observed.join("attestation"),
        f.service,
        f.operator,
        reply,
    );
    change(&mut args, "--url", &proxy.url);
    change(&mut args, "--run-dir", observed.to_str().unwrap());
    let scope = ic_blob_storage::model::service::read::download::CaffeineDownloadScope::new(
        f.service,
        1.try_into().unwrap(),
        "fixture project/β?&=",
    )
    .unwrap();
    let root = ic_blob_storage::model::identity::ProviderRootHash::try_from(
        permission.request.root.as_slice(),
    )
    .unwrap();
    let target = ic_blob_storage::ops::caffeine::download::request_target(&scope, root);
    std::thread::scope(|threads| {
        let server = threads.spawn(|| provider(&listener, &observed, &target));
        let report = run(&args, 0);
        assert_eq!(report["project"], scope.project());
        assert_eq!(report["owner"], f.service.to_text());
        assert_eq!(report["attestation_dispatched"], false);
        assert_eq!(report["retry_authorized"], false);
        server.join().unwrap();
    });
    drop(listener);
    assert_eq!(run(&args, 3)["error"], "new_run_required");
    interrupted(&args, dir);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    attest_and_recover(&mut f, &observed, &args, reply);
    assert_eq!(proxy.calls(), 1);
    drop(proxy);
    f.harness.pic.stop_live();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    change(&mut args, "--url", &live(&mut f));
    change(
        &mut args,
        "--run-dir",
        dir.join("restored").to_str().unwrap(),
    );
    assert_eq!(run(&args, 3)["error"], "verification_refused");
    assert!(!dir.join("restored/download-request.json").exists());
    f.harness.pic.stop_live();
}
