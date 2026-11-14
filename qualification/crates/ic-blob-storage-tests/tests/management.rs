//! Exercise the actual native command through signed ingress and a local IC HTTP gateway.

use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::Limits,
    protocol::{Command, Installation, RPC_LIMITS},
};
use ic_platform::{
    Principal,
    agent::{BasicIdentity, Identity},
};
use ic_testkit::{
    pic::PocketIcStartupConfig,
    pocket_ic::{CanisterId, PocketIcBuilder},
};
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::Output,
    time::Duration,
};

// Public deterministic test seed [7; 32], used only in this disposable simulator.
const TEST_PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcH\n-----END PRIVATE KEY-----\n";

fn encode(input: &Installation) -> Vec<u8> {
    ic_wire::native::encode_request(ic_wire::native::method_id("blob_init"), input, RPC_LIMITS)
        .unwrap()
}

fn command(
    directory: &Path,
    url: &str,
    canister: CanisterId,
    version: &str,
    operation: &str,
) -> tokio::process::Command {
    let mut command = tokio::process::Command::new(
        std::env::var_os("BLOB_ADMIN").expect("set BLOB_ADMIN to the built native artifact"),
    );
    command
        .arg(operation)
        .arg("--url")
        .arg(url)
        .arg("--canister")
        .arg(canister.to_text())
        .arg("--identity")
        .arg(directory.join("manager.pem"))
        .arg("--root-key")
        .arg(directory.join("root.der"))
        .arg("--installation")
        .arg(directory.join("installation.wire"))
        .arg("--wasm")
        .arg(directory.join("host.wasm"))
        .arg("--state-version")
        .arg(version)
        .kill_on_drop(true);
    command
}

async fn inspect(directory: &Path, url: &str, canister: CanisterId, version: &str) -> Output {
    command(directory, url, canister, version, "inspect")
        .output()
        .await
        .unwrap()
}

fn refused(output: &Output, reason: &str) {
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(reason),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[tokio::test]
async fn signed_inspection_checks_trust_artifact_configuration_and_closed_admission() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../target/management-test")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    let wasm = std::fs::read(std::env::var_os("BLOB_WASM").expect("set BLOB_WASM")).unwrap();
    std::fs::write(directory.join("host.wasm"), &wasm).unwrap();
    std::fs::write(directory.join("manager.pem"), TEST_PEM).unwrap();
    let manager = BasicIdentity::from_pem(TEST_PEM).unwrap().sender().unwrap();
    let input = Installation {
        application: Principal::from_slice(&[1, 1]),
        manager: Principal::from_slice(manager.as_slice()),
        limits: Limits {
            manifest: ManifestLimits {
                max_content_bytes: 100.try_into().unwrap(),
                max_chunks: 1.try_into().unwrap(),
                max_headers: 1.try_into().unwrap(),
                max_header_bytes: 100.try_into().unwrap(),
            },
            uploads: 64.try_into().unwrap(),
            references: 64.try_into().unwrap(),
            logical_bytes: 100.try_into().unwrap(),
            exposure_bytes: 100.try_into().unwrap(),
        },
    };
    std::fs::write(directory.join("installation.wire"), encode(&input)).unwrap();
    let server_directory = std::env::var_os("BLOB_TEST_SERVER").expect("set BLOB_TEST_SERVER");
    let server = ic_testkit::pic::server_assets::check(Path::new(&server_directory)).unwrap();
    let mut owned = PocketIcStartupConfig::Spawn {
        server_binary: server,
        timeout: Duration::from_secs(60),
        server_hard_ttl: Some(Duration::from_secs(300)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    )
    .await
    .unwrap();
    let canister = owned.instance().create_canister().await;
    owned
        .instance()
        .add_cycles(canister, 10_000_000_000_000)
        .await;
    owned
        .instance()
        .install_canister(canister, wasm.clone(), encode(&input), None)
        .await;
    // Root comes from the fixture's owned server, never the command's HTTP endpoint.
    let root = owned.instance().root_key().await.unwrap();
    std::fs::write(directory.join("root.der"), &root).unwrap();
    let url = owned.instance_mut().make_live(None).await;
    let before = owned.instance().get_stable_memory(canister).await;
    let output = inspect(&directory, url.as_str(), canister, "1").await;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("attempt=0 active=true"));
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    refused(
        &inspect(&directory, url.as_str(), canister, "2").await,
        "state format mismatch",
    );
    let different = Installation {
        application: Principal::from_slice(&[2, 1]),
        ..input
    };
    std::fs::write(directory.join("installation.wire"), encode(&different)).unwrap();
    refused(
        &inspect(&directory, url.as_str(), canister, "1").await,
        "configuration or state format mismatch",
    );
    std::fs::write(directory.join("installation.wire"), encode(&input)).unwrap();
    std::fs::write(directory.join("host.wasm"), b"not the admitted Wasm").unwrap();
    refused(
        &inspect(&directory, url.as_str(), canister, "1").await,
        "Wasm does not match",
    );
    std::fs::write(directory.join("host.wasm"), &wasm).unwrap();
    let mut wrong_root = root.clone();
    *wrong_root.last_mut().unwrap() ^= 1;
    std::fs::write(directory.join("root.der"), wrong_root).unwrap();
    refused(
        &inspect(&directory, url.as_str(), canister, "1").await,
        "authentication failed",
    );
    std::fs::write(directory.join("root.der"), root).unwrap();
    // Matching a known target/hash does not grant another identity management access.
    let foreign = owned.instance().create_canister().await;
    owned
        .instance()
        .add_cycles(foreign, 10_000_000_000_000)
        .await;
    owned
        .instance()
        .install_canister(
            foreign,
            wasm.clone(),
            encode(&Installation {
                manager: input.application,
                ..input
            }),
            None,
        )
        .await;
    refused(
        &inspect(&directory, url.as_str(), foreign, "1").await,
        "host refused management inspection",
    );
    let claim = directory.join("drain-1");
    let before = owned.instance().get_stable_memory(canister).await;
    let output = command(&directory, url.as_str(), canister, "1", "prepare-drain")
        .arg("--record")
        .arg(&claim)
        .args(["--attempt", "1"])
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("NOT submitted"));
    let packet = std::fs::read(claim.join("request.cbor")).unwrap();
    let intent = std::fs::read(claim.join("intent.wire")).unwrap();
    refused(
        &command(&directory, url.as_str(), canister, "1", "prepare-drain")
            .arg("--record")
            .arg(&claim)
            .args(["--attempt", "1"])
            .output()
            .await
            .unwrap(),
        "cannot claim drain directory",
    );
    let partial = directory.join("partial-drain");
    std::fs::create_dir(&partial).unwrap();
    refused(
        &command(&directory, url.as_str(), canister, "1", "prepare-drain")
            .arg("--record")
            .arg(&partial)
            .args(["--attempt", "1"])
            .output()
            .await
            .unwrap(),
        "cannot claim drain directory",
    );
    assert!(std::fs::read_dir(&partial).unwrap().next().is_none());
    refused(
        &command(&directory, url.as_str(), canister, "1", "observe-drain")
            .arg("--record")
            .arg(&partial)
            .output()
            .await
            .unwrap(),
        "cannot read bounded regular file",
    );
    refused(
        &command(&directory, url.as_str(), canister, "1", "observe-drain")
            .arg("--record")
            .arg(&claim)
            .output()
            .await
            .unwrap(),
        "outcome unavailable",
    );
    refused(
        &command(&directory, url.as_str(), foreign, "1", "observe-drain")
            .arg("--record")
            .arg(&claim)
            .output()
            .await
            .unwrap(),
        "claim binding mismatch",
    );
    let mut corrupt = packet.clone();
    corrupt.push(0);
    std::fs::write(claim.join("request.cbor"), corrupt).unwrap();
    refused(
        &command(&directory, url.as_str(), canister, "1", "observe-drain")
            .arg("--record")
            .arg(&claim)
            .output()
            .await
            .unwrap(),
        "claim binding mismatch",
    );
    std::fs::write(claim.join("request.cbor"), &packet).unwrap();
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    let checkout = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let custody = directory.join("custody");
    let bad_checkout = directory.join("bad-checkout");
    std::fs::create_dir(&bad_checkout).unwrap();
    std::fs::write(
        bad_checkout.join(".toko-tooling.snapshot"),
        b"wrong snapshot",
    )
    .unwrap();
    refused(
        &command(&directory, url.as_str(), canister, "1", "dispatch-drain")
            .arg("--record")
            .arg(&claim)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&bad_checkout)
            .output()
            .await
            .unwrap(),
        "snapshot differs",
    );
    assert!(!custody.exists());
    std::fs::copy(
        checkout.join(".toko-tooling.snapshot"),
        bad_checkout.join(".toko-tooling.snapshot"),
    )
    .unwrap();
    std::fs::write(
        bad_checkout.join("Makefile"),
        "toko-tooling-check:\n\t@false\n",
    )
    .unwrap();
    refused(
        &command(&directory, url.as_str(), canister, "1", "dispatch-drain")
            .arg("--record")
            .arg(&claim)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&bad_checkout)
            .output()
            .await
            .unwrap(),
        "tooling admission refused",
    );
    assert!(!custody.exists());
    let network: String = Sha256::digest(owned.instance().root_key().await.unwrap())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let target_directory = custody.join(&network).join(canister.to_text());
    ic_host_fs::durable::prepare_private_parents(&target_directory).unwrap();
    let lock = ic_host_fs::durable::ExclusiveFileLock::try_acquire(
        ic_host_fs::durable::open_private_regular_lock_file(&target_directory.join("manager.lock"))
            .unwrap(),
    )
    .unwrap();
    refused(
        &command(&directory, url.as_str(), canister, "1", "dispatch-drain")
            .arg("--record")
            .arg(&claim)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&checkout)
            .output()
            .await
            .unwrap(),
        "already locked",
    );
    assert!(!target_directory.join("attempt-1.claim").exists());
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    drop(lock);
    // Two real processes compete: exactly one may claim dispatch. A separate
    // process then observes the original outcome without relying on its stdout.
    let mut first = command(&directory, url.as_str(), canister, "1", "dispatch-drain");
    first
        .arg("--record")
        .arg(&claim)
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(&checkout);
    let mut second = command(&directory, url.as_str(), canister, "1", "dispatch-drain");
    second
        .arg("--record")
        .arg(&claim)
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(&checkout);
    let (first, second) = tokio::join!(first.output(), second.output());
    let outputs = [first.unwrap(), second.unwrap()];
    assert_eq!(
        outputs.iter().filter(|out| out.status.success()).count(),
        1,
        "{outputs:?}"
    );
    let denied = outputs.iter().find(|out| !out.status.success()).unwrap();
    let reason = String::from_utf8_lossy(&denied.stderr);
    assert!(
        reason.contains("already locked") || reason.contains("already claimed"),
        "{reason}"
    );
    let content = ic_platform::ic_protocol::decode_call_content_strict(&packet, 16 * 1024).unwrap();
    let original_id = ic_platform::agent::to_request_id(&content).unwrap();
    let marker = std::fs::read(target_directory.join("attempt-1.claim")).unwrap();
    assert_eq!(marker, original_id.to_vec());
    refused(
        &command(&directory, url.as_str(), canister, "1", "dispatch-drain")
            .arg("--record")
            .arg(&claim)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&checkout)
            .output()
            .await
            .unwrap(),
        "already claimed",
    );
    let output = command(&directory, url.as_str(), canister, "1", "observe-drain")
        .arg("--record")
        .arg(&claim)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Original drain acknowledged"));
    assert_eq!(std::fs::read(claim.join("request.cbor")).unwrap(), packet);
    assert_eq!(std::fs::read(claim.join("intent.wire")).unwrap(), intent);
    let drain = ic_wire::native::encode_request(
        ic_wire::native::method_id("blob_update"),
        &Command::Drain(1),
        RPC_LIMITS,
    )
    .unwrap();
    owned
        .instance()
        .update_call(
            canister,
            CanisterId::from_slice(manager.as_slice()),
            "blob_update",
            drain,
        )
        .await
        .unwrap();
    let before = owned.instance().get_stable_memory(canister).await;
    let output = inspect(&directory, url.as_str(), canister, "1").await;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("attempt=1 active=false"));
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    // A crash after claiming but before transmission is deliberately unrecoverable
    // by dispatch. A new record/request ID cannot bypass that same target attempt.
    let stranded = owned.instance().create_canister().await;
    owned
        .instance()
        .add_cycles(stranded, 10_000_000_000_000)
        .await;
    owned
        .instance()
        .install_canister(stranded, wasm, encode(&input), None)
        .await;
    let stranded_claim = directory.join("stranded");
    let output = command(&directory, url.as_str(), stranded, "1", "prepare-drain")
        .arg("--record")
        .arg(&stranded_claim)
        .args(["--attempt", "1"])
        .output()
        .await
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let stranded_target = custody.join(&network).join(stranded.to_text());
    ic_host_fs::durable::prepare_private_parents(&stranded_target).unwrap();
    std::fs::write(stranded_target.join("attempt-1.claim"), []).unwrap();
    let retry_claim = directory.join("stranded-new-request");
    let output = command(&directory, url.as_str(), stranded, "1", "prepare-drain")
        .arg("--record")
        .arg(&retry_claim)
        .args(["--attempt", "1"])
        .output()
        .await
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_ne!(
        std::fs::read(stranded_claim.join("request.cbor")).unwrap(),
        std::fs::read(retry_claim.join("request.cbor")).unwrap()
    );
    let before = owned.instance().get_stable_memory(stranded).await;
    refused(
        &command(&directory, url.as_str(), stranded, "1", "dispatch-drain")
            .arg("--record")
            .arg(&retry_claim)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&checkout)
            .output()
            .await
            .unwrap(),
        "already claimed",
    );
    refused(
        &command(&directory, url.as_str(), stranded, "1", "observe-drain")
            .arg("--record")
            .arg(&stranded_claim)
            .output()
            .await
            .unwrap(),
        "outcome unavailable",
    );
    assert_eq!(owned.instance().get_stable_memory(stranded).await, before);
    for transition in [Command::Activate(1), Command::Drain(2)] {
        let bytes = ic_wire::native::encode_request(
            ic_wire::native::method_id("blob_update"),
            &transition,
            RPC_LIMITS,
        )
        .unwrap();
        owned
            .instance()
            .update_call(
                canister,
                CanisterId::from_slice(manager.as_slice()),
                "blob_update",
                bytes,
            )
            .await
            .unwrap();
    }
    let before = owned.instance().get_stable_memory(canister).await;
    let output = command(&directory, url.as_str(), canister, "1", "observe-drain")
        .arg("--record")
        .arg(&claim)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("historical outcome only"));
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    for forbidden in ["reinstall", "upgrade", "activate", "snapshot-load"] {
        let output = tokio::process::Command::new(std::env::var_os("BLOB_ADMIN").unwrap())
            .arg(forbidden)
            .output()
            .await
            .unwrap();
        refused(&output, "unrecognized subcommand");
    }
    owned.close().await;
}
