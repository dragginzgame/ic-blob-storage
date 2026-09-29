//! Signed installed plan, real local HTTP bytes and saved statement; provider facts substituted.
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
fn attest_and_recover(f: &mut Fixture, path: &Path, args: &[String]) {
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
    // Fixture-only dispatch tests whether the independently checked statement is accepted.
    f.harness.pic.advance_time(Duration::from_secs(1));
    f.harness
        .pic
        .update_candid_as::<Result<UploadAttestationMutation, UploadAttestationFailure>, _>(
            f.service,
            f.operator,
            "blob_attest_upload",
            (statement,),
        )
        .unwrap()
        .unwrap();
    assert!(
        matches!(receipt(f, &statement).attestation, UploadAttestationLookup::Found(r) if r.request == statement)
    );
    let mut recovery = vec!["upload-attestation".into()];
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
        recovery.extend([flag.into(), args[i + 1].clone()]);
    }
    recovery.extend([
        "--verifier".into(),
        f.operator.to_text(),
        "--statement".into(),
        path.join("statement.candid").to_str().unwrap().into(),
    ]);
    change(&mut recovery, "--url", &live(f));
    assert_eq!(run(&recovery, 0)["outcome"], "matched");
    assert_eq!(std::fs::read(path.join("statement.candid")).unwrap(), saved);
    f.harness.pic.stop_live();
}
#[test]
fn observer_uses_signed_installed_mapping_and_persists_verified_statement_without_dispatch() {
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
    change(&mut args, "--url", &live(&mut f));
    let observed = dir.join("observed");
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
    f.harness.pic.stop_live();
    attest_and_recover(&mut f, &observed, &args);
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
