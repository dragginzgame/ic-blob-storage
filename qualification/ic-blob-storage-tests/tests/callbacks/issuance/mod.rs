//! Exact local issuer ordering and authenticated original-request recovery, not gateway acceptance.

mod publication;

use super::*;
use ic_blob_storage::local::{ReferenceStatus, UploadState};
use ic_host_fs::durable::{self, PublicationMode, WriteOptions};
use ic_platform::agent::{
    Agent, AgentError, BasicIdentity, Identity, RequestStatusResponse, lookup_value,
    signed_update_inspect, to_request_id,
};
use ic_platform::certification::cbor::{decode_certificate, encode_certificate};
use ic_platform::certification::historical::{
    Limits as CertificateLimits, TimePolicy, TimeWindow, authenticate_certificate_at,
};
use ic_platform::ic_protocol::{EnvelopeContent, decode_call_content_strict};
use ic_testkit::pic::PocketIcInstance;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const ISSUE: &str = "_immutableObjectStorageCreateCertificate";

ic_platform::candid_record! {
    struct ProviderReply {
        156956385 => method: String,
        278101296 => blob_hash: String,
    }
}

fn root_text(root: Root) -> String {
    let hex: String = root.iter().map(|b| format!("{b:02x}")).collect();
    format!("sha256:{hex}")
}
fn issue_args(root: Root) -> Vec<u8> {
    encode_args(&(root_text(root),), 1024).unwrap()
}
fn assert_reply(bytes: &[u8], root: Root) {
    let (reply,): (ProviderReply,) = decode_args(bytes, CANDID, Arity::Exact).unwrap();
    assert_eq!(reply.method, "upload");
    assert_eq!(reply.blob_hash, root_text(root));
}
async fn inspect(pic: &PocketIc, owner: CanisterId, request: Inspection) -> Observation {
    let bytes = pic
        .query_call(
            owner,
            caller(1),
            "blob_query",
            frame("blob_query", &request),
        )
        .await
        .unwrap();
    wire::decode::<Result<Observation, Error>>(
        &bytes,
        wire::Encoding::Frame {
            method: wire::method_id("blob_query"),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
    .unwrap()
}

#[tokio::test]
async fn issuance_is_atomic_and_delayed_reply_cannot_reopen_final_release() {
    let owned = server()
        .build_instance(PocketIcBuilder::new().with_application_subnet())
        .await
        .unwrap();
    let pic = owned.instance();
    let owner = install(pic).await;
    let root = [42; 32];
    update(pic, owner, 1, grant(1, root, 2)).await.unwrap();
    let before = pic.get_stable_memory(owner).await;
    for who in [
        CanisterId::anonymous(),
        caller(1),
        caller(3),
        caller(7),
        caller(9),
    ] {
        assert!(
            pic.update_call(owner, who, ISSUE, issue_args(root))
                .await
                .is_err()
        );
        assert_eq!(pic.get_stable_memory(owner).await, before);
    }
    let mut trailing = issue_args(root);
    trailing.push(0);
    for bytes in [
        vec![],
        vec![0; 4097],
        trailing,
        roots(&[root]),
        encode_args(&(root_text(root), true), 4096).unwrap(),
        encode_args(&("sha256:00".to_string(),), 4096).unwrap(),
        encode_args(&(root_text(root).to_uppercase(),), 4096).unwrap(),
        issue_args([99; 32]),
    ] {
        assert!(
            pic.update_call(owner, caller(2), ISSUE, bytes)
                .await
                .is_err()
        );
        assert_eq!(pic.get_stable_memory(owner).await, before);
    }
    assert!(
        pic.query_call(owner, caller(2), ISSUE, issue_args(root))
            .await
            .is_err()
    );
    update(pic, owner, 1, grant(2, [2; 32], 2)).await.unwrap();
    update(pic, owner, 1, Command::Release(2.try_into().unwrap()))
        .await
        .unwrap();
    let mut expired = grant(3, [3; 32], 2);
    let Command::Authorize { request, .. } = &mut expired else {
        unreachable!()
    };
    request.expires_at = pic.get_time().await.as_nanos_since_unix_epoch() + 60_000_000_000;
    update(pic, owner, 1, expired).await.unwrap();
    pic.advance_time(Duration::from_secs(121)).await;
    let before = pic.get_stable_memory(owner).await;
    for unavailable in [[2; 32], [3; 32]] {
        assert!(
            pic.update_call(owner, caller(2), ISSUE, issue_args(unavailable))
                .await
                .is_err()
        );
        assert_eq!(pic.get_stable_memory(owner).await, before);
    }
    // Submit once; delay retrieving the client result until after final release.
    let pending = pic
        .submit_call(owner, caller(2), ISSUE, issue_args(root))
        .await
        .unwrap();
    for _ in 0..20 {
        pic.tick().await;
        if matches!(inspect(pic, owner, Inspection::Upload(1.try_into().unwrap())).await,
            Observation::Upload(view) if view.state == UploadState::Registered)
        {
            break;
        }
    }
    assert!(
        matches!(inspect(pic, owner, Inspection::Reference(1.try_into().unwrap())).await,
        Observation::Reference(view) if view.status == ReferenceStatus::Live)
    );
    assert!(
        matches!(usage(pic, owner).await, Observation::Usage(u) if u.live_bytes == 1 && u.unresolved_bytes == 1 && u.reserved_bytes == 1)
    );
    update(pic, owner, 1, Command::Release(1.try_into().unwrap()))
        .await
        .unwrap();
    assert_reply(&pic.await_call(pending).await.unwrap(), root);
    assert_eq!(liveness(pic, owner, &[root]).await, vec![false]);
    assert_eq!(candidates(pic, owner).await, vec![root.to_vec()]);
    assert!(
        pic.update_call(owner, caller(2), ISSUE, issue_args(root))
            .await
            .is_err()
    );
    assert!(update(pic, owner, 1, grant(4, root, 2)).await.is_err());
    ack(pic, owner, &[root]).await;
    let exposure = usage(pic, owner).await;
    update(pic, owner, 9, Command::Drain(1)).await.unwrap();
    pic.stop_canister(owner, Some(caller(9))).await.unwrap();
    pic.upgrade_canister(
        owner,
        wasm(),
        frame("callback_upgrade", &()),
        Some(caller(9)),
    )
    .await
    .unwrap();
    pic.start_canister(owner, Some(caller(9))).await.unwrap();
    update(pic, owner, 9, Command::Activate(1)).await.unwrap();
    assert!(
        pic.update_call(owner, caller(2), ISSUE, issue_args(root))
            .await
            .is_err()
    );
    assert!(candidates(pic, owner).await.is_empty());
    assert_eq!(usage(pic, owner).await, exposure);
}

fn signer(seed: u8) -> Principal {
    let principal = BasicIdentity::from_raw_key(&[seed; 32]).sender().unwrap();
    Principal::from_slice(principal.as_slice())
}
fn agent(url: &str, root: &[u8], seed: u8) -> Agent {
    // Public deterministic fixture keys; root comes from simulator custody, never HTTP discovery.
    let agent = Agent::builder()
        .with_url(url)
        .with_identity(BasicIdentity::from_raw_key(&[seed; 32]))
        .with_verify_query_signatures(true)
        .with_max_response_body_size(2 * 1024 * 1024)
        .with_max_polling_time(Duration::from_secs(30))
        .with_max_tcp_error_retries(0)
        .build()
        .unwrap();
    agent.set_root_key(root.to_vec());
    agent
}
async fn signed_command(agent: &Agent, owner: CanisterId, command: Command) {
    let bytes = agent
        .update(&owner, "blob_update")
        .with_arg(frame("blob_update", &command))
        .call_and_wait()
        .await
        .unwrap();
    let result: Result<Outcome, Error> = wire::decode(
        &bytes,
        wire::Encoding::Frame {
            method: wire::method_id("blob_update"),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap();
    result.unwrap();
}

fn certificate_command(
    operation: &str,
    directory: &Path,
    url: &str,
    owner: CanisterId,
    root: Root,
    uploader: Principal,
) -> tokio::process::Command {
    // A retained predecessor may prepare custody; current code must dispatch/recover it.
    let previous = (operation == "prepare-certificate")
        .then(|| std::env::var_os("BLOB_PREVIOUS_CLI"))
        .flatten();
    let mut command = tokio::process::Command::new(previous.unwrap_or_else(|| {
        std::env::var_os("BLOB_CLI").expect("set BLOB_CLI to the built native artifact")
    }));
    command
        .args([
            operation,
            "--url",
            url,
            "--canister",
            &owner.to_text(),
            "--uploader",
            &uploader.to_text(),
            "--root",
            &root_text(root),
        ])
        .arg("--identity")
        .arg(directory.join("uploader.pem"))
        .arg("--root-key")
        .arg(directory.join("root.der"))
        .kill_on_drop(true);
    command
}

fn recovery_command(
    directory: &Path,
    url: &str,
    owner: CanisterId,
    root: Root,
    uploader: Principal,
) -> tokio::process::Command {
    let mut command =
        certificate_command("recover-certificate", directory, url, owner, root, uploader);
    command
        .arg("--request")
        .arg(directory.join("request.cbor"))
        .arg("--output")
        .arg(directory.join("certificate.cbor"));
    command
}

// Authenticate the actual certified time through Platform, not a consumer LEB128
// parser. This fixture selects an explicit inclusive window; it is no provider TTL.
fn certificate_time(bytes: &[u8], key: &[u8], owner: CanisterId, start: u64, end: u64) -> u64 {
    authenticate_certificate_at(
        bytes,
        key,
        Principal::from_slice(owner.as_slice()),
        TimePolicy {
            certificate: TimeWindow {
                reference_ns: start,
                max_age_ns: 0,
                max_future_skew_ns: end.checked_sub(start).unwrap(),
            },
            // The simulator's root delegation predates the recovered certificate.
            delegation: TimeWindow {
                reference_ns: end,
                max_age_ns: end,
                max_future_skew_ns: 0,
            },
        },
        CertificateLimits {
            certificate_bytes: 128 * 1024,
            delegation_bytes: 128 * 1024,
            tree_nodes: 4096,
            tree_depth: 64,
            canister_ranges: 64,
        },
    )
    .unwrap_or_else(|error| panic!("certificate outside expected [{start}, {end}]: {error}"))
    .time_ns
}

async fn signed_issuer() -> (PocketIcInstance, CanisterId, Vec<u8>) {
    let owned = server()
        .build_instance(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await
        .unwrap();
    let (owner, trust_root) = install_issuer(&owned, signer(1), issuer_limits()).await;
    (owned, owner, trust_root)
}

fn issuer_limits() -> Limits {
    let mut limits = input().limits;
    limits.manifest.max_headers = 2.try_into().unwrap();
    limits
}

async fn install_issuer(
    owned: &PocketIcInstance,
    application: Principal,
    limits: Limits,
) -> (CanisterId, Vec<u8>) {
    let owner = owned.instance().create_canister().await;
    owned.instance().add_cycles(owner, 10_000_000_000_000).await;
    let input = Installation {
        application,
        manager: signer(9),
        limits,
    };
    owned
        .instance()
        .install_canister(
            owner,
            wasm(),
            frame("callback_init", &(input, signer(7))),
            None,
        )
        .await;
    let trust_root = owned.instance().root_key().await.unwrap();
    (owner, trust_root)
}

fn certificate_directory(name: &str, trust_root: &[u8]) -> PathBuf {
    // Public deterministic seed [2; 32]; no wallet key is exported.
    const PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIC\n-----END PRIVATE KEY-----\n";
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target")
        .join(name)
        .join(std::process::id().to_string());
    std::fs::create_dir_all(directory.parent().unwrap()).unwrap();
    durable::create_private_directory(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    let options = WriteOptions {
        mode: PublicationMode::CreateNew,
        permissions: 0o600,
    };
    durable::write_bytes(&directory.join("uploader.pem"), options, PEM.as_bytes()).unwrap();
    durable::write_bytes(&directory.join("root.der"), options, trust_root).unwrap();
    directory
}

fn certificate_claim(custody: &Path, trust_root: &[u8], owner: CanisterId, root: Root) -> PathBuf {
    let network: String = Sha256::digest(trust_root)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    custody
        .join(network)
        .join(owner.to_text())
        .join(&root_text(root)[7..])
}

#[tokio::test]
async fn cancelled_authority_and_transport_failure_never_release_dispatch_custody() {
    let (mut owned, owner, trust_root) = signed_issuer().await;
    let url = owned.instance_mut().make_live(None).await;
    let application = agent(url.as_str(), &trust_root, 1);
    let uploader = agent(url.as_str(), &trust_root, 2);
    let directory = certificate_directory("certificate-refusal-test", &trust_root);
    let custody = directory.join("custody");
    for (number, root) in [(1, [21; 32]), (2, [22; 32])] {
        let mut authorize = grant(number, root, 2);
        let Command::Authorize { request, .. } = &mut authorize else {
            unreachable!()
        };
        request.uploader = signer(2);
        signed_command(&application, owner, authorize).await;
        let mut prepare = certificate_command(
            "prepare-certificate",
            &directory,
            url.as_str(),
            owner,
            root,
            signer(2),
        );
        prepare.arg("--custody").arg(&custody);
        let output = prepare.output().await.unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let claim = certificate_claim(&custody, &trust_root, owner, root);
        let packet = std::fs::read(claim.join("request.cbor")).unwrap();
        let id = to_request_id(&decode_call_content_strict(&packet, 8192).unwrap()).unwrap();
        if number == 1 {
            // Cancellation occurs after the signature exists, before it is sent.
            signed_command(
                &application,
                owner,
                Command::Release(number.try_into().unwrap()),
            )
            .await;
        } else {
            // Stop only the owned HTTP gateway. No proxy or substitute issuer:
            // the CLI cannot know whether a general transport error means no effect.
            owned.instance_mut().stop_live().await;
        }
        let before = owned.instance().get_stable_memory(owner).await;
        let mut dispatch = certificate_command(
            "dispatch-certificate",
            &directory,
            url.as_str(),
            owner,
            root,
            signer(2),
        );
        dispatch
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
        let output = dispatch.output().await.unwrap();
        if number == 1 {
            // Dispatch may have returned Poll; only the certified original result
            // establishes rejection. A CLI exit code is not provider disposition.
            assert!(matches!(
                uploader.wait(&id, owner).await,
                Err(AgentError::CertifiedReject { .. })
            ));
        } else {
            assert!(!output.status.success());
            let restored = owned.instance_mut().make_live(url.port()).await;
            assert_eq!(restored, url);
            let (status, _) = uploader.request_status_raw(&id, owner).await.unwrap();
            assert!(matches!(status, RequestStatusResponse::Unknown));
        }
        assert_eq!(
            std::fs::read(claim.join("dispatch.claim")).unwrap(),
            id.to_vec()
        );
        assert!(!dispatch.output().await.unwrap().status.success());
        assert!(!prepare.output().await.unwrap().status.success());
        assert_eq!(std::fs::read(claim.join("request.cbor")).unwrap(), packet);
        // Recovery refuses rejected and unknown results without publishing proof.
        let mut recovery = certificate_command(
            "recover-certificate",
            &directory,
            url.as_str(),
            owner,
            root,
            signer(2),
        );
        let proof = claim.join("certificate.cbor");
        let output = recovery
            .arg("--request")
            .arg(claim.join("request.cbor"))
            .arg("--output")
            .arg(&proof)
            .output()
            .await
            .unwrap();
        assert!(!output.status.success());
        assert!(!proof.exists());
        assert_eq!(owned.instance().get_stable_memory(owner).await, before);
        if number == 2 {
            // An unused reservation can still be released through its real owner;
            // this does not erase or reset the CLI's uncertain dispatch claim.
            signed_command(
                &application,
                owner,
                Command::Release(number.try_into().unwrap()),
            )
            .await;
        }
    }
    let bytes = application
        .query(&owner, "blob_query")
        .with_arg(frame("blob_query", &Inspection::Usage))
        .call()
        .await
        .unwrap();
    let observed: Result<Observation, Error> = wire::decode(
        &bytes,
        wire::Encoding::Frame {
            method: wire::method_id("blob_query"),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap();
    assert!(
        matches!(observed, Ok(Observation::Usage(u)) if u.reserved_bytes == 0 && u.live_bytes == 0 && u.unresolved_bytes == 0 && u.uploads == 2 && u.references == 2)
    );
    owned.close().await;
}

#[tokio::test]
async fn original_request_recovery_authenticates_history_after_release_without_reissuance() {
    let (mut owned, owner, trust_root) = signed_issuer().await;
    let url = owned.instance_mut().make_live(None).await;
    let application = agent(url.as_str(), &trust_root, 1);
    let uploader = agent(url.as_str(), &trust_root, 2);
    let gateway = agent(url.as_str(), &trust_root, 7);
    let root = [7; 32];
    let mut authorize = grant(1, root, 2);
    let Command::Authorize { request, .. } = &mut authorize else {
        unreachable!()
    };
    request.uploader = signer(2);
    signed_command(&application, owner, authorize).await;
    let directory = certificate_directory("certificate-recovery-test", &trust_root);
    let options = WriteOptions {
        mode: PublicationMode::CreateNew,
        permissions: 0o600,
    };
    let custody = directory.join("custody");
    let mut prepare = certificate_command(
        "prepare-certificate",
        &directory,
        url.as_str(),
        owner,
        root,
        signer(2),
    );
    prepare.arg("--custody").arg(&custody);
    let output = prepare.output().await.unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let claim = certificate_claim(&custody, &trust_root, owner, root);
    let packet = std::fs::read(claim.join("request.cbor")).unwrap();
    let content = decode_call_content_strict(&packet, 8192).unwrap();
    let id = to_request_id(&content).unwrap();
    let EnvelopeContent::Call {
        sender,
        ingress_expiry: expiry,
        ..
    } = content
    else {
        panic!("not a call")
    };
    assert!(!prepare.output().await.unwrap().status.success());
    assert_eq!(std::fs::read(claim.join("request.cbor")).unwrap(), packet);
    assert!(!claim.join("dispatch.claim").exists());
    // Recovery also accepts an independently retained exact copy. The canonical
    // preparation directory remains the only source for dispatch.
    durable::write_bytes(&directory.join("request.cbor"), options, &packet).unwrap();
    let output = recovery_command(&directory, url.as_str(), owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no recoverable reply"));
    assert!(!directory.join("certificate.cbor").exists());
    signed_update_inspect(
        sender,
        owner,
        ISSUE,
        &issue_args(root),
        expiry,
        packet.clone(),
    )
    .unwrap();
    assert!(
        signed_update_inspect(
            sender,
            owner,
            ISSUE,
            &issue_args([8; 32]),
            expiry,
            packet.clone()
        )
        .is_err()
    );
    let mut dispatch = certificate_command(
        "dispatch-certificate",
        &directory,
        url.as_str(),
        owner,
        root,
        signer(2),
    );
    dispatch
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    // A changed packet is refused before any dispatch claim is acquired.
    std::fs::write(claim.join("request.cbor"), b"partial fixture").unwrap();
    assert!(!dispatch.output().await.unwrap().status.success());
    assert!(!claim.join("dispatch.claim").exists());
    std::fs::write(claim.join("request.cbor"), &packet).unwrap();
    // A partial marker already represents possible dispatch. Only the fixture
    // moves this synthetic, never-dispatched marker aside; the CLI cannot reset it.
    std::fs::write(claim.join("dispatch.claim"), []).unwrap();
    assert!(!dispatch.output().await.unwrap().status.success());
    assert!(
        std::fs::read(claim.join("dispatch.claim"))
            .unwrap()
            .is_empty()
    );
    std::fs::rename(
        claim.join("dispatch.claim"),
        claim.join("synthetic-partial.claim"),
    )
    .unwrap();
    // Two separate processes contend for the same permanent dispatch claim.
    dispatch
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    let first = dispatch.spawn().unwrap();
    let second = dispatch.spawn().unwrap();
    let (first, second) = tokio::join!(first.wait_with_output(), second.wait_with_output());
    assert_eq!(
        usize::from(first.unwrap().status.success())
            + usize::from(second.unwrap().status.success()),
        1
    );
    assert_eq!(
        std::fs::read(claim.join("dispatch.claim")).unwrap(),
        id.to_vec()
    );
    assert!(!dispatch.output().await.unwrap().status.success());
    // Discard completion data. Recovery receives no in-memory dispatch response.
    drop(uploader.wait(&id, owner).await.unwrap());
    signed_command(&application, owner, Command::Release(1.try_into().unwrap())).await;
    let bytes = gateway
        .update(&owner, ACK)
        .with_arg(roots(&[root]))
        .call_and_wait()
        .await
        .unwrap();
    decode_args::<()>(&bytes, CANDID, Arity::Exact).unwrap();
    // Independently started CLI processes never share the fixture Agent or its reply.
    for (target, expected_root, who, reason) in [
        (owner, [8; 32], signer(2), "retained request does not match"),
        (
            caller(3),
            root,
            signer(2),
            "retained request does not match",
        ),
        (owner, root, signer(3), "identity must match"),
    ] {
        let output = recovery_command(&directory, url.as_str(), target, expected_root, who)
            .output()
            .await
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains(reason));
        assert!(!directory.join("certificate.cbor").exists());
    }
    let mut untrusted = trust_root.clone();
    *untrusted.last_mut().unwrap() ^= 1;
    std::fs::write(directory.join("root.der"), untrusted).unwrap();
    let output = recovery_command(&directory, url.as_str(), owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("cannot authenticate"));
    assert!(!directory.join("certificate.cbor").exists());
    std::fs::write(directory.join("root.der"), &trust_root).unwrap();
    // Partial previous output is evidence too; recovery must leave it untouched.
    std::fs::write(directory.join("certificate.cbor"), b"partial fixture").unwrap();
    let output = recovery_command(&directory, url.as_str(), owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("preserve original request and any output")
    );
    assert_eq!(
        std::fs::read(directory.join("certificate.cbor")).unwrap(),
        b"partial fixture"
    );
    // Move fixture evidence aside explicitly; the product command never does so.
    std::fs::rename(
        directory.join("certificate.cbor"),
        directory.join("partial.cbor"),
    )
    .unwrap();
    let output = recovery_command(&directory, url.as_str(), owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains(&id.to_string()));
    let saved = std::fs::read(directory.join("certificate.cbor")).unwrap();
    let certificate = decode_certificate(&saved, 128 * 1024).unwrap();
    uploader.verify(&certificate, owner).unwrap();
    assert_reply(
        lookup_value(
            &certificate,
            [
                b"request_status".as_slice(),
                id.as_ref(),
                b"reply".as_slice(),
            ],
        )
        .unwrap(),
        root,
    );
    let output = recovery_command(&directory, url.as_str(), owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(
        std::fs::read(directory.join("certificate.cbor")).unwrap(),
        saved
    );
    assert_eq!(
        std::fs::read(directory.join("request.cbor")).unwrap(),
        packet
    );
    // A new Agent with the same signer recovers the original certified result.
    let recovered = agent(url.as_str(), &trust_root, 2);
    let (status, certificate) = recovered.request_status_raw(&id, owner).await.unwrap();
    let RequestStatusResponse::Replied(reply) = status else {
        panic!("original result not retained")
    };
    assert_reply(&reply.arg, root);
    let path = [
        b"request_status".as_slice(),
        id.as_ref(),
        b"reply".as_slice(),
    ];
    assert_eq!(lookup_value(&certificate, path).unwrap(), reply.arg);
    let absent = [0u8; 32];
    assert!(
        lookup_value(
            &certificate,
            [b"request_status".as_slice(), &absent, b"reply".as_slice()]
        )
        .is_err()
    );
    let mut changed = certificate;
    changed.signature[0] ^= 1;
    assert!(recovered.verify(&changed, owner).is_err());
    let mut wrong_root = trust_root.clone();
    *wrong_root.last_mut().unwrap() ^= 1;
    let error = agent(url.as_str(), &wrong_root, 2)
        .request_status_raw(&id, owner)
        .await
        .unwrap_err();
    assert!(matches!(error, AgentError::CertificateVerificationFailed()));
    let another = recovered
        .update(&owner, ISSUE)
        .with_arg(issue_args(root))
        .expire_after(Duration::from_secs(120))
        .sign()
        .unwrap();
    assert_ne!(another.request_id, id);
    // A new request identity cannot obtain fresh authority for the already consumed root.
    match recovered.update_signed(owner, another.signed_update).await {
        Err(AgentError::CertifiedReject { .. }) => {}
        Ok(ic_platform::agent::CallResponse::Poll(id)) => {
            assert!(matches!(
                recovered.wait(&id, owner).await,
                Err(AgentError::CertifiedReject { .. })
            ));
        }
        result => panic!("fresh issuance unexpectedly succeeded: {result:?}"),
    }
    let bytes = application
        .update(&owner, "blob_query")
        .with_arg(frame("blob_query", &Inspection::Usage))
        .call_and_wait()
        .await
        .unwrap();
    let observed: Result<Observation, Error> = wire::decode(
        &bytes,
        wire::Encoding::Frame {
            method: wire::method_id("blob_query"),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap();
    assert!(
        matches!(observed, Ok(Observation::Usage(u)) if u.live_bytes == 0 && u.unresolved_bytes == 1 && u.uploads == 1 && u.references == 1)
    );
    let earlier_time = certificate_time(&saved, &trust_root, owner, 0, expiry);
    // Age the already-executed request past its original expiry without resigning
    // or replaying it. The old reply remains within this fixture's history retention.
    owned
        .instance()
        .advance_time(Duration::from_secs(121))
        .await;
    // set_time alone need not publish a new certified state. Execute rounds so
    // recovery reads a certificate produced after the simulated expiry boundary.
    owned.instance().tick().await;
    owned.instance().tick().await;
    assert!(
        owned
            .instance()
            .get_time()
            .await
            .as_nanos_since_unix_epoch()
            > expiry
    );
    std::fs::rename(
        directory.join("certificate.cbor"),
        directory.join("before-expiry.cbor"),
    )
    .unwrap();
    let output = recovery_command(&directory, url.as_str(), owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let later = std::fs::read(directory.join("certificate.cbor")).unwrap();
    let now = owned
        .instance()
        .get_time()
        .await
        .as_nanos_since_unix_epoch();
    let later_time = certificate_time(&later, &trust_root, owner, expiry + 1, now);
    assert!(later_time > earlier_time);
    let later = decode_certificate(&later, 128 * 1024).unwrap();
    uploader.verify(&later, owner).unwrap();
    assert_eq!(lookup_value(&later, path).unwrap(), reply.arg);
    assert_eq!(
        std::fs::read(directory.join("request.cbor")).unwrap(),
        packet
    );
    let after = owned
        .instance()
        .query_call(
            owner,
            application.get_principal().unwrap(),
            "blob_query",
            frame("blob_query", &Inspection::Usage),
        )
        .await
        .unwrap();
    assert_eq!(
        wire::decode::<Result<Observation, Error>>(
            &after,
            wire::Encoding::Frame {
                method: wire::method_id("blob_query"),
                direction: wire::Direction::Response
            },
            RPC_LIMITS
        )
        .unwrap(),
        observed
    );
    owned.close().await;
}

fn abc_authorization() -> Command {
    // Independently retained text/plain "abc" protocol vector.
    let decode = |hex: &str| -> Root {
        std::array::from_fn(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
    };
    let root = decode("0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd");
    let chunk = decode("b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7");
    let mut authorize = grant(1, root, 2);
    let Command::Authorize {
        request,
        chunks,
        headers,
    } = &mut authorize
    else {
        unreachable!()
    };
    request.uploader = signer(2);
    request.bytes = 3;
    *chunks = vec![chunk];
    *headers = vec![
        ("Content-Length".into(), "3".into()),
        ("Content-Type".into(), "text/plain".into()),
    ];
    authorize
}

async fn retain_proof(
    directory: &Path,
    url: &str,
    owner: CanisterId,
    root: Root,
    trust_root: &[u8],
) -> PathBuf {
    let uploader = agent(url, trust_root, 2);
    let custody = directory.join("custody");
    for operation in ["prepare-certificate", "dispatch-certificate"] {
        let mut command = certificate_command(operation, directory, url, owner, root, signer(2));
        command.arg("--custody").arg(&custody);
        if operation == "dispatch-certificate" {
            command
                .arg("--checkout")
                .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
        }
        let output = command.output().await.unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let claim = certificate_claim(&custody, trust_root, owner, root);
    let packet = std::fs::read(claim.join("request.cbor")).unwrap();
    let id = to_request_id(&decode_call_content_strict(&packet, 8192).unwrap()).unwrap();
    drop(uploader.wait(&id, owner).await.unwrap());
    let options = WriteOptions {
        mode: PublicationMode::CreateNew,
        permissions: 0o600,
    };
    durable::write_bytes(&directory.join("request.cbor"), options, &packet).unwrap();
    let output = recovery_command(directory, url, owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let proof = std::fs::read(directory.join("certificate.cbor")).unwrap();
    let mut corrupted = decode_certificate(&proof, 128 * 1024).unwrap();
    corrupted.signature[0] ^= 1;
    durable::write_bytes(
        &directory.join("bad-certificate.cbor"),
        options,
        &encode_certificate(&corrupted, 128 * 1024).unwrap(),
    )
    .unwrap();
    claim
}

#[tokio::test]
async fn owned_transfer_authenticates_proof_verifies_read_and_refuses_process_replay() {
    let (mut owned, owner, trust_root) = signed_issuer().await;
    let url = owned.instance_mut().make_live(None).await;
    let application = agent(url.as_str(), &trust_root, 1);
    let authorize = abc_authorization();
    let Command::Authorize { request, .. } = &authorize else {
        unreachable!()
    };
    let root = request.root;
    signed_command(&application, owner, authorize).await;
    let directory = certificate_directory("owned-transfer-test", &trust_root);
    let claim = retain_proof(&directory, url.as_str(), owner, root, &trust_root).await;
    let proof = std::fs::read(directory.join("certificate.cbor")).unwrap();
    let mut client = tokio::process::Command::new("node");
    client
        .arg(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../clients/browser/tests/installed-transfer.js"),
        )
        .arg("--directory")
        .arg(&directory)
        .args([
            "--issuer",
            &owner.to_text(),
            "--uploader",
            &signer(2).to_text(),
            "--url",
            url.as_str(),
        ])
        .kill_on_drop(true);
    for expected in ["verified-read", "replay-refused"] {
        let output = tokio::time::timeout(Duration::from_secs(30), client.output())
            .await
            .unwrap()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), expected);
        assert_eq!(
            std::fs::read(claim.join("transfer/certificate.cbor")).unwrap(),
            proof
        );
        client.arg("--replay");
    }
    // A verified read does not close the provider obligation after final release.
    signed_command(&application, owner, Command::Release(1.try_into().unwrap())).await;
    let bytes = application
        .query(&owner, "blob_query")
        .with_arg(frame("blob_query", &Inspection::Usage))
        .call()
        .await
        .unwrap();
    assert!(
        matches!(wire::decode::<Result<Observation, Error>>(&bytes, wire::Encoding::Frame { method: wire::method_id("blob_query"), direction: wire::Direction::Response }, RPC_LIMITS).unwrap().unwrap(),
        Observation::Usage(u) if u.live_bytes == 0 && u.unresolved_bytes == 3)
    );
    owned.close().await;
}
