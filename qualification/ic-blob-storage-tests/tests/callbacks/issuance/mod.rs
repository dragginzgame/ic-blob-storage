//! Exact local issuer ordering and authenticated original-request recovery, not gateway acceptance.

mod publication;

use super::*;
use ic_blob_storage::local::{ReferenceStatus, UploadState};
use ic_blob_storage_tests::native::{Client, identity, policy};
use ic_host_artifacts::Length;
use ic_host_fs::{Commit, Destination, Directory, Policy, Retention};
use ic_platform::certificate::{
    CertificateDecodeLimits, PreparedRootKey, TimePolicy, TimeWindow, VerificationLimits,
    authenticate_certificate, decode_certificate,
};
use ic_platform::ingress::{Content, decode_envelope};
use ic_platform::native::{PreparedUpdate, ReadError, RequestStatus};

use ic_testkit::server::{Instance, Server};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const ISSUE: &str = "_immutableObjectStorageCreateCertificate";

fn root_text(root: Root) -> String {
    let hex: String = root.iter().map(|b| format!("{b:02x}")).collect();
    format!("sha256:{hex}")
}
fn issue_args(root: Root) -> Vec<u8> {
    provider::encode_root(&root_text(root), 1024).unwrap()
}
fn assert_reply(bytes: &[u8], root: Root) {
    let reply = provider::decode_certificate(bytes, PROVIDER_LIMITS).unwrap();
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
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 180).await;
    let pic = owned.client();
    let owner = install(pic).await;
    let root = [42; 32];
    update(pic, owner, 1, grant(1, root, 2)).await.unwrap();
    let before = support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await;
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
        assert_eq!(
            support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await,
            before
        );
    }
    let mut trailing = issue_args(root);
    trailing.push(0);
    for bytes in [
        vec![],
        vec![0; 4097],
        trailing,
        roots(&[root]),
        // No type table; text and bool arguments, with a 71-byte root string.
        [
            b"DIDL\x00\x02\x71\x7e\x47".as_slice(),
            root_text(root).as_bytes(),
            &[1],
        ]
        .concat(),
        provider::encode_root(&"sha256:00".to_string(), 4096).unwrap(),
        provider::encode_root(&root_text(root).to_uppercase(), 4096).unwrap(),
        issue_args([99; 32]),
    ] {
        assert!(
            pic.update_call(owner, caller(2), ISSUE, bytes)
                .await
                .is_err()
        );
        assert_eq!(
            support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await,
            before
        );
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
    let before = support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await;
    for unavailable in [[2; 32], [3; 32]] {
        assert!(
            pic.update_call(owner, caller(2), ISSUE, issue_args(unavailable))
                .await
                .is_err()
        );
        assert_eq!(
            support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await,
            before
        );
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
    pic.upgrade_eop_canister(
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
    support::close(server, owned).await;
}

fn signer(seed: u8) -> Principal {
    identity(seed).principal()
}

async fn signed_command(agent: &Client, owner: CanisterId, command: Command) -> Outcome {
    let bytes = agent
        .update(&owner, "blob_update", &frame("blob_update", &command))
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
    result.unwrap()
}

fn certificate_command(
    operation: &str,
    directory: &Path,
    url: &str,
    owner: CanisterId,
    root: Root,
    uploader: Principal,
) -> tokio::process::Command {
    // A predecessor may prepare original custody. Through 0.100.10 it used flat
    // commands; set BLOB_PREVIOUS_CLI_GROUPED for later grouped predecessors.
    // Current grouped commands must dispatch/recover it without rewriting it.
    let previous = (operation == "prepare-certificate")
        .then(|| std::env::var_os("BLOB_PREVIOUS_CLI"))
        .flatten();
    let legacy_cli = previous.is_some();
    let mut command = tokio::process::Command::new(previous.unwrap_or_else(|| {
        std::env::var_os("BLOB_CLI").expect("set BLOB_CLI to the built native artifact")
    }));
    if legacy_cli && std::env::var_os("BLOB_PREVIOUS_CLI_GROUPED").is_none() {
        command.arg(operation);
    } else {
        command.args([
            "certificate",
            operation.strip_suffix("-certificate").unwrap(),
        ]);
    }
    command
        .args([
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
    if !legacy_cli {
        command
            .arg("--root-subnet")
            .arg(std::fs::read_to_string(directory.join("root-subnet.txt")).unwrap());
    }
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
    authenticate_certificate(
        decode_certificate(bytes, CertificateDecodeLimits { bytes: 128 * 1024 }).unwrap(),
        &PreparedRootKey::from_der(key).unwrap(),
        Principal::from_slice(owner.as_slice()),
        TimePolicy {
            certificate: TimeWindow {
                reference_ns: end,
                max_age_ns: end.checked_sub(start).unwrap(),
                max_future_skew_ns: 0,
            },
            delegation: TimeWindow {
                reference_ns: end,
                max_age_ns: end,
                max_future_skew_ns: 0,
            },
        },
        VerificationLimits {
            delegation_bytes: 128 * 1024,
            tree_nodes: 4096,
            tree_depth: 64,
            canister_ranges: 64,
        },
    )
    .unwrap_or_else(|error| panic!("certificate outside expected [{start}, {end}]: {error}"))
    .time_ns()
}

fn corrupt_signature(bytes: &[u8]) -> Vec<u8> {
    let decoded = decode_certificate(bytes, CertificateDecodeLimits { bytes: 128 * 1024 }).unwrap();
    let signature = &decoded.unverified().signature;
    let offsets: Vec<_> = bytes
        .windows(signature.len())
        .enumerate()
        .filter_map(|(offset, value)| (value == signature).then_some(offset))
        .collect();
    assert_eq!(offsets.len(), 1);
    let mut changed = bytes.to_vec();
    changed[offsets[0]] ^= 1;
    changed
}

async fn signed_issuer() -> (Server, Instance, CanisterId, Vec<u8>) {
    let (server, owned) = support::instance(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
        300,
    )
    .await;
    let (owner, trust_root) = install_issuer(&owned, signer(1), issuer_limits()).await;
    (server, owned, owner, trust_root)
}

fn issuer_limits() -> Limits {
    let mut limits = input().limits;
    limits.manifest.max_headers = 2.try_into().unwrap();
    limits
}

async fn install_issuer(
    owned: &Instance,
    application: Principal,
    limits: Limits,
) -> (CanisterId, Vec<u8>) {
    let owner = owned.client().create_canister().await;
    owned.client().add_cycles(owner, 10_000_000_000_000).await;
    let input = Installation {
        application,
        manager: signer(9),
        limits,
    };
    owned
        .client()
        .install_canister(
            owner,
            wasm(),
            frame("callback_init", &(input, signer(7))),
            None,
        )
        .await;
    let trust_root = owned.client().root_key().await.unwrap();
    (owner, trust_root)
}

fn certificate_directory(name: &str, trust_root: &[u8], root_subnet: CanisterId) -> PathBuf {
    // Public deterministic seed [2; 32]; no wallet key is exported.
    const PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIC\n-----END PRIVATE KEY-----\n";
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target")
        .join(name)
        .join(std::process::id().to_string());
    std::fs::create_dir_all(directory.parent().unwrap()).unwrap();
    Directory::open(directory.parent().unwrap(), 0)
        .unwrap()
        .unwrap()
        .create_directory(directory.file_name().unwrap(), 0o700, 0o077)
        .unwrap();
    let directory = directory.canonicalize().unwrap();
    publish_fixture(&directory.join("uploader.pem"), PEM.as_bytes()).unwrap();
    publish_fixture(&directory.join("root.der"), trust_root).unwrap();
    publish_fixture(
        &directory.join("root-subnet.txt"),
        root_subnet.to_text().as_bytes(),
    )
    .unwrap();
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
    let (server, mut owned, owner, trust_root) = signed_issuer().await;
    let url = owned.client_mut().make_live(None).await;
    let application = Client::new(
        url.as_str(),
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
        1,
    );
    let uploader = Client::new(
        url.as_str(),
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
        2,
    );
    let directory = certificate_directory(
        "certificate-refusal-test",
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
    );
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
        let original =
            PreparedUpdate::from_bytes(&packet, Principal::from_slice(owner.as_slice()), 8192)
                .unwrap();
        let id = *original.request_id();
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
            owned.client_mut().stop_live().await;
        }
        let before = support::blob_state(
            owned.client(),
            owner,
            CanisterId::from_slice(signer(1).as_slice()),
            &[signer(2), signer(3)],
        )
        .await;
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
                uploader.wait(&original).await.status(),
                RequestStatus::Rejected { .. }
            ));
        } else {
            assert!(!output.status.success());
            let restored = owned.client_mut().make_live(url.port()).await;
            assert_eq!(restored, url);
            let observed = uploader.observe(&original).await.unwrap();
            assert!(matches!(
                observed.status(),
                RequestStatus::Unknown | RequestStatus::Unavailable
            ));
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
        assert_eq!(
            support::blob_state(
                owned.client(),
                owner,
                CanisterId::from_slice(signer(1).as_slice()),
                &[signer(2), signer(3)]
            )
            .await,
            before
        );
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
        .query(
            &owner,
            "blob_query",
            &frame("blob_query", &Inspection::Usage),
        )
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
    support::close(server, owned).await;
}

#[tokio::test]
async fn original_request_recovery_authenticates_history_after_release_without_reissuance() {
    let (server, mut owned, owner, trust_root) = signed_issuer().await;
    let url = owned.client_mut().make_live(None).await;
    let application = Client::new(
        url.as_str(),
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
        1,
    );
    let uploader = Client::new(
        url.as_str(),
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
        2,
    );
    let gateway = Client::new(
        url.as_str(),
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
        7,
    );
    let root = [7; 32];
    let mut authorize = grant(1, root, 2);
    let Command::Authorize { request, .. } = &mut authorize else {
        unreachable!()
    };
    request.uploader = signer(2);
    signed_command(&application, owner, authorize).await;
    let directory = certificate_directory(
        "certificate-recovery-test",
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
    );
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
    let content = decode_envelope(&packet, 8192).unwrap().value.content;
    let id = content.request_id().unwrap();
    let original =
        PreparedUpdate::from_bytes(&packet, Principal::from_slice(owner.as_slice()), 8192).unwrap();
    let Content::Call {
        sender,
        ingress_expiry: expiry,
        ..
    } = &content
    else {
        panic!("not a call")
    };
    assert!(!prepare.output().await.unwrap().status.success());
    assert_eq!(std::fs::read(claim.join("request.cbor")).unwrap(), packet);
    assert!(!claim.join("dispatch.claim").exists());
    // Recovery also accepts an independently retained exact copy. The canonical
    // preparation directory remains the only source for dispatch.
    publish_fixture(&directory.join("request.cbor"), &packet).unwrap();
    let output = recovery_command(&directory, url.as_str(), owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no certified reply"));
    assert!(!directory.join("certificate.cbor").exists());
    assert_eq!(original.sender().as_slice(), sender.as_slice());
    assert_eq!(original.expiry(), *expiry);
    let Content::Call {
        arg, method_name, ..
    } = &content
    else {
        unreachable!()
    };
    assert_eq!(method_name, ISSUE);
    assert_eq!(arg.as_slice(), issue_args(root));
    assert_ne!(arg.as_slice(), issue_args([8; 32]));
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
    assert!(matches!(
        uploader.wait(&original).await.status(),
        RequestStatus::Replied(_)
    ));
    signed_command(&application, owner, Command::Release(1.try_into().unwrap())).await;
    let bytes = gateway.update(&owner, ACK, &roots(&[root])).await.unwrap();
    provider::decode_empty(&bytes, PROVIDER_LIMITS).unwrap();
    // Independently started CLI processes never share the fixture client or its reply.
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
    let untrusted =
        include_bytes!("../../../../../crates/ic-blob-storage-cli/src/mainnet-root.der");
    std::fs::write(directory.join("root.der"), untrusted).unwrap();
    let output = recovery_command(&directory, url.as_str(), owner, root, signer(2))
        .output()
        .await
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("authentication failed"));
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
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains(&id.iter().map(|b| format!("{b:02x}")).collect::<String>())
    );
    let saved = std::fs::read(directory.join("certificate.cbor")).unwrap();
    let proof = uploader
        .transport
        .authenticate_request(&original, &saved, policy())
        .unwrap();
    let RequestStatus::Replied(reply) = proof.status() else {
        panic!("retained proof has no reply")
    };
    assert_reply(reply, root);
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
    // A fresh client authenticates the same original result without issuing again.
    let recovered = Client::new(
        url.as_str(),
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
        2,
    );
    let observation = recovered.observe(&original).await.unwrap();
    let RequestStatus::Replied(reply) = observation.status() else {
        panic!("original result not retained")
    };
    assert_reply(reply, root);
    let another = recovered.prepare(owner, ISSUE, &issue_args(root));
    assert_ne!(another.request_id(), &id);
    let absent = recovered
        .transport
        .authenticate_request(&another, observation.certificate_bytes(), policy())
        .unwrap();
    assert!(matches!(
        absent.status(),
        RequestStatus::Unknown | RequestStatus::Unavailable
    ));
    assert!(
        recovered
            .transport
            .authenticate_request(
                &original,
                &corrupt_signature(observation.certificate_bytes()),
                policy()
            )
            .is_err()
    );
    // A different valid root is admitted structurally but cannot authenticate this network.
    let wrong_root =
        include_bytes!("../../../../../crates/ic-blob-storage-cli/src/mainnet-root.der");
    let error = Client::new(
        url.as_str(),
        wrong_root,
        owned.client().topology().await.get_nns().unwrap(),
        2,
    )
    .observe(&original)
    .await
    .unwrap_err();
    assert!(matches!(error, ReadError::Verification(_)));
    // A new request identity cannot obtain fresh authority for the consumed root.
    assert!(matches!(
        recovered.update(owner, ISSUE, &issue_args(root)).await,
        Err(ReadError::Rejected { .. })
    ));
    let bytes = application
        .update(
            owner,
            "blob_query",
            &frame("blob_query", &Inspection::Usage),
        )
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
    let earlier_time = certificate_time(&saved, &trust_root, owner, 0, *expiry);
    // Age the already-executed request past its original expiry without resigning
    // or replaying it. The old reply remains within this fixture's history retention.
    // Keep the simulator and the CLI's wall clock aligned. Advancing only the
    // simulator would violate the explicitly selected certificate future window.
    let wait_ns = expiry
        .saturating_add(1_000_000_000)
        .saturating_sub(ic_blob_storage_tests::native::now());
    tokio::time::sleep(Duration::from_nanos(wait_ns)).await;
    // set_time alone need not publish a new certified state. Execute rounds so
    // recovery reads a certificate produced after the simulated expiry boundary.
    owned.client().tick().await;
    owned.client().tick().await;
    assert!(owned.client().get_time().await.as_nanos_since_unix_epoch() > *expiry);
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
    let now = owned.client().get_time().await.as_nanos_since_unix_epoch();
    let later_time = certificate_time(&later, &trust_root, owner, expiry + 1, now);
    assert!(later_time > earlier_time);
    let proof = uploader
        .transport
        .authenticate_request(&original, &later, policy())
        .unwrap();
    assert_eq!(proof.status(), observation.status());
    assert_eq!(
        std::fs::read(directory.join("request.cbor")).unwrap(),
        packet
    );
    let after = owned
        .client()
        .query_call(
            owner,
            CanisterId::from_slice(signer(1).as_slice()),
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
    support::close(server, owned).await;
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
    let uploader = Client::new(
        url,
        trust_root,
        CanisterId::from_text(std::fs::read_to_string(directory.join("root-subnet.txt")).unwrap())
            .unwrap(),
        2,
    );
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
    let original =
        PreparedUpdate::from_bytes(&packet, Principal::from_slice(owner.as_slice()), 8192).unwrap();
    assert!(matches!(
        uploader.wait(&original).await.status(),
        RequestStatus::Replied(_)
    ));
    publish_fixture(&directory.join("request.cbor"), &packet).unwrap();
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
    publish_fixture(
        &directory.join("bad-certificate.cbor"),
        &corrupt_signature(&proof),
    )
    .unwrap();
    claim
}

#[tokio::test]
async fn owned_transfer_authenticates_proof_verifies_read_and_refuses_process_replay() {
    let (server, mut owned, owner, trust_root) = signed_issuer().await;
    let url = owned.client_mut().make_live(None).await;
    let application = Client::new(
        url.as_str(),
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
        1,
    );
    let authorize = abc_authorization();
    let Command::Authorize { request, .. } = &authorize else {
        unreachable!()
    };
    let root = request.root;
    signed_command(&application, owner, authorize).await;
    let directory = certificate_directory(
        "owned-transfer-test",
        &trust_root,
        owned.client().topology().await.get_nns().unwrap(),
    );
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
        .query(
            &owner,
            "blob_query",
            &frame("blob_query", &Inspection::Usage),
        )
        .await
        .unwrap();
    assert!(
        matches!(wire::decode::<Result<Observation, Error>>(&bytes, wire::Encoding::Frame { method: wire::method_id("blob_query"), direction: wire::Direction::Response }, RPC_LIMITS).unwrap().unwrap(),
        Observation::Usage(u) if u.live_bytes == 0 && u.unresolved_bytes == 3)
    );
    support::close(server, owned).await;
}

// Fixture setup uses the same published Host primitives, without exposing CLI internals.
fn publish_fixture(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = Directory::open(path.parent().unwrap(), 0)
        .map_err(|e| e.to_string())?
        .unwrap();
    let name = path.file_name().unwrap();
    let mut staging = name.to_os_string();
    staging.push(".pending");
    let stage = parent
        .stage(
            &staging,
            name,
            Policy {
                destination: Destination::CreateNew,
                permissions: 0o600,
                retention: Retention::Retain,
            },
        )
        .map_err(|e| e.to_string())?;
    let sealed = stage
        .write_from(bytes, Length::Exactly(bytes.len() as u64))
        .and_then(ic_host_fs::Written::seal)
        .map_err(|e| e.to_string())?;
    match sealed.commit() {
        Commit::Committed(_) => Ok(()),
        other => Err(format!("{other:?}")),
    }
}
