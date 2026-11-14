//! Exercise the actual native command through signed ingress and a local IC HTTP gateway.

mod lifecycle;

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

fn installation(manager: Principal) -> Installation {
    Installation {
        application: Principal::from_slice(&[1, 1]),
        manager,
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
    }
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
async fn signed_management_preserves_target_custody_and_closed_admission() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../target/management-test")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    let wasm = std::fs::read(std::env::var_os("BLOB_WASM").expect("set BLOB_WASM")).unwrap();
    std::fs::write(directory.join("host.wasm"), &wasm).unwrap();
    std::fs::write(directory.join("manager.pem"), TEST_PEM).unwrap();
    let manager = BasicIdentity::from_pem(TEST_PEM).unwrap().sender().unwrap();
    let input = installation(Principal::from_slice(manager.as_slice()));
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
        "cannot claim management directory",
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
        "cannot claim management directory",
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
    let stop_claim = directory.join("stop");
    refused(
        &command(&directory, url.as_str(), stranded, "1", "prepare-stop")
            .arg("--record")
            .arg(&stop_claim)
            .args(["--attempt", "1"])
            .output()
            .await
            .unwrap(),
        "current admission does not permit",
    );
    assert!(!stop_claim.exists());
    // The Blob manager is not automatically a platform controller.
    let refused_stop = command(&directory, url.as_str(), canister, "1", "prepare-stop")
        .arg("--record")
        .arg(&stop_claim)
        .args(["--attempt", "2"])
        .output()
        .await
        .unwrap();
    assert!(!refused_stop.status.success());
    assert!(!stop_claim.exists());
    let controller = CanisterId::from_slice(manager.as_slice());
    owned
        .instance()
        .set_controllers(canister, None, vec![controller])
        .await
        .unwrap();
    let output = command(&directory, url.as_str(), canister, "1", "prepare-stop")
        .arg("--record")
        .arg(&stop_claim)
        .args(["--attempt", "2"])
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stop_packet = std::fs::read(stop_claim.join("request.cbor")).unwrap();
    refused(
        &command(&directory, url.as_str(), canister, "1", "observe-drain")
            .arg("--record")
            .arg(&stop_claim)
            .output()
            .await
            .unwrap(),
        "fields do not match intent",
    );
    // A previously prepared stop cannot cross activation or a newer drain attempt.
    for transition in [Command::Activate(2), Command::Drain(3)] {
        owned
            .instance()
            .update_call(
                canister,
                controller,
                "blob_update",
                ic_wire::native::encode_request(
                    ic_wire::native::method_id("blob_update"),
                    &transition,
                    RPC_LIMITS,
                )
                .unwrap(),
            )
            .await
            .unwrap();
        refused(
            &command(&directory, url.as_str(), canister, "1", "dispatch-stop")
                .arg("--record")
                .arg(&stop_claim)
                .arg("--custody")
                .arg(&custody)
                .arg("--checkout")
                .arg(&checkout)
                .output()
                .await
                .unwrap(),
            "current admission does not permit",
        );
        assert!(!target_directory.join("attempt-2.stop.claim").exists());
    }
    assert_eq!(
        std::fs::read(stop_claim.join("request.cbor")).unwrap(),
        stop_packet
    );
    let current_stop = directory.join("current-stop");
    let output = command(&directory, url.as_str(), canister, "1", "prepare-stop")
        .arg("--record")
        .arg(&current_stop)
        .args(["--attempt", "3"])
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    refused(
        &command(&directory, url.as_str(), canister, "1", "observe-stop")
            .arg("--record")
            .arg(&current_stop)
            .output()
            .await
            .unwrap(),
        "platform=Running",
    );
    let before = owned.instance().get_stable_memory(canister).await;
    let unused_stop = directory.join("unused-stop");
    let output = command(&directory, url.as_str(), canister, "1", "prepare-stop")
        .arg("--record")
        .arg(&unused_stop)
        .args(["--attempt", "3"])
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_ne!(
        std::fs::read(unused_stop.join("request.cbor")).unwrap(),
        std::fs::read(current_stop.join("request.cbor")).unwrap()
    );
    let output = command(&directory, url.as_str(), canister, "1", "dispatch-stop")
        .arg("--record")
        .arg(&current_stop)
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(&checkout)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target_directory.join("attempt-3.stop.claim").is_file());
    let output = command(&directory, url.as_str(), canister, "1", "observe-stop")
        .arg("--record")
        .arg(&current_stop)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("original=acknowledged platform=Stopped")
    );
    // Stopped state cannot fabricate acknowledgment of another, unsent request
    // or authorize dispatch of that replacement request ID.
    let output = command(&directory, url.as_str(), canister, "1", "observe-stop")
        .arg("--record")
        .arg(&unused_stop)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("original=unavailable platform=Stopped")
    );
    refused(
        &command(&directory, url.as_str(), canister, "1", "dispatch-stop")
            .arg("--record")
            .arg(&unused_stop)
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
        &command(&directory, url.as_str(), canister, "1", "dispatch-stop")
            .arg("--record")
            .arg(&current_stop)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&checkout)
            .output()
            .await
            .unwrap(),
        "already claimed",
    );
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    let upgrade_claim = directory.join("upgrade");
    let unused_upgrade = directory.join("unused-upgrade");
    let next_wasm = directory.join("next.wasm");
    std::fs::copy(directory.join("host.wasm"), &next_wasm).unwrap();
    let upgrade_command = |operation: &str, record: &Path, predecessor: &Path| {
        let mut command = command(&directory, url.as_str(), canister, "1", operation);
        command
            .arg("--record")
            .arg(record)
            .arg("--stop-record")
            .arg(predecessor)
            .arg("--next-wasm")
            .arg(&next_wasm);
        command
    };
    // A stopped observation cannot substitute for the original stop acknowledgment.
    refused(
        &upgrade_command("prepare-upgrade", &upgrade_claim, &unused_stop)
            .args(["--attempt", "3"])
            .output()
            .await
            .unwrap(),
        "requires original stop acknowledgment",
    );
    assert!(!upgrade_claim.exists());
    refused(
        &upgrade_command("prepare-upgrade", &upgrade_claim, &current_stop)
            .args(["--attempt", "2"])
            .output()
            .await
            .unwrap(),
        "predecessor attempt mismatch",
    );
    assert!(!upgrade_claim.exists());
    for record in [&upgrade_claim, &unused_upgrade] {
        let output = upgrade_command("prepare-upgrade", record, &current_stop)
            .args(["--attempt", "3"])
            .output()
            .await
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    // A different candidate cannot be substituted when observing or dispatching
    // a retained request, even when it has a syntactically valid Wasm header.
    std::fs::write(&next_wasm, b"\0asm\x01\0\0\0\xff").unwrap();
    refused(
        &upgrade_command("observe-upgrade", &upgrade_claim, &current_stop)
            .output()
            .await
            .unwrap(),
        "fields do not match intent",
    );
    std::fs::copy(directory.join("host.wasm"), &next_wasm).unwrap();
    // Same-Wasm upgrades need an original result even before any bytes change.
    refused(
        &upgrade_command("observe-upgrade", &upgrade_claim, &current_stop)
            .output()
            .await
            .unwrap(),
        "outcome unresolved",
    );
    refused(
        &upgrade_command("observe-upgrade", &upgrade_claim, &unused_stop)
            .output()
            .await
            .unwrap(),
        "predecessor identity mismatch",
    );
    let stop_marker = target_directory.join("attempt-3.stop.claim");
    let original_marker = std::fs::read(&stop_marker).unwrap();
    // Explicit corruption substitute: wrong predecessor claim must refuse before
    // publishing upgrade exposure, and restored fixture bytes allow normal dispatch.
    std::fs::write(&stop_marker, [0; 32]).unwrap();
    refused(
        &upgrade_command("dispatch-upgrade", &upgrade_claim, &current_stop)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&checkout)
            .output()
            .await
            .unwrap(),
        "claim does not match predecessor",
    );
    assert!(!target_directory.join("attempt-3.upgrade.claim").exists());
    std::fs::write(&stop_marker, original_marker).unwrap();
    let output = upgrade_command("dispatch-upgrade", &upgrade_claim, &current_stop)
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(&checkout)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = upgrade_command("observe-upgrade", &upgrade_claim, &current_stop)
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
            .contains("replacement Wasm matched, platform=Stopped")
    );
    refused(
        &upgrade_command("observe-upgrade", &unused_upgrade, &current_stop)
            .output()
            .await
            .unwrap(),
        "outcome unresolved",
    );
    refused(
        &upgrade_command("dispatch-upgrade", &unused_upgrade, &current_stop)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&checkout)
            .output()
            .await
            .unwrap(),
        "already claimed",
    );
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    let start_claim = directory.join("start");
    let unused_start = directory.join("unused-start");
    let start_command = |operation: &str, record: &Path, upgrade: &Path, stop: &Path| {
        let mut command = upgrade_command(operation, record, stop);
        command.arg("--upgrade-record").arg(upgrade);
        command
    };
    refused(
        &start_command(
            "prepare-start",
            &start_claim,
            &unused_upgrade,
            &current_stop,
        )
        .args(["--attempt", "3"])
        .output()
        .await
        .unwrap(),
        "outcome unresolved",
    );
    assert!(!start_claim.exists());
    refused(
        &start_command("prepare-start", &start_claim, &upgrade_claim, &current_stop)
            .args(["--attempt", "2"])
            .output()
            .await
            .unwrap(),
        "predecessor attempt mismatch",
    );
    assert!(!start_claim.exists());
    for record in [&start_claim, &unused_start] {
        let output = start_command("prepare-start", record, &upgrade_claim, &current_stop)
            .args(["--attempt", "3"])
            .output()
            .await
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    // Start checks both predecessor claims, including the transitive stop.
    for marker in [
        &stop_marker,
        &target_directory.join("attempt-3.upgrade.claim"),
    ] {
        let saved = std::fs::read(marker).unwrap();
        std::fs::write(marker, [0; 32]).unwrap();
        refused(
            &start_command(
                "dispatch-start",
                &start_claim,
                &upgrade_claim,
                &current_stop,
            )
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&checkout)
            .output()
            .await
            .unwrap(),
            "claim does not match predecessor",
        );
        assert!(!target_directory.join("attempt-3.start.claim").exists());
        std::fs::write(marker, saved).unwrap();
    }
    let output = start_command(
        "dispatch-start",
        &start_claim,
        &upgrade_claim,
        &current_stop,
    )
    .arg("--custody")
    .arg(&custody)
    .arg("--checkout")
    .arg(&checkout)
    .output()
    .await
    .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("attempt=3 active=false"));
    let output = start_command("observe-start", &start_claim, &upgrade_claim, &current_stop)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    refused(
        &start_command(
            "observe-start",
            &unused_start,
            &upgrade_claim,
            &current_stop,
        )
        .output()
        .await
        .unwrap(),
        "outcome unresolved",
    );
    refused(
        &start_command(
            "dispatch-start",
            &unused_start,
            &upgrade_claim,
            &current_stop,
        )
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(&checkout)
        .output()
        .await
        .unwrap(),
        "already claimed",
    );
    // An old stop acknowledgment does not hide that the host is now running.
    refused(
        &command(&directory, url.as_str(), canister, "1", "observe-stop")
            .arg("--record")
            .arg(&current_stop)
            .output()
            .await
            .unwrap(),
        "platform=Running",
    );
    let output = inspect(&directory, url.as_str(), canister, "1").await;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("attempt=3 active=false"));
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    let activation = directory.join("activation");
    let unused_activation = directory.join("unused-activation");
    let activate_command = |operation: &str, record: &Path, start: &Path| {
        let mut command = start_command(operation, record, &upgrade_claim, &current_stop);
        command.arg("--start-record").arg(start);
        command
    };
    refused(
        &activate_command("prepare-activate", &activation, &unused_start)
            .args(["--attempt", "3"])
            .output()
            .await
            .unwrap(),
        "outcome unresolved",
    );
    assert!(!activation.exists());
    refused(
        &activate_command("prepare-activate", &activation, &start_claim)
            .args(["--attempt", "2"])
            .output()
            .await
            .unwrap(),
        "predecessor attempt mismatch",
    );
    assert!(!activation.exists());
    for record in [&activation, &unused_activation] {
        let output = activate_command("prepare-activate", record, &start_claim)
            .args(["--attempt", "3"])
            .output()
            .await
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    for marker in [
        &stop_marker,
        &target_directory.join("attempt-3.upgrade.claim"),
        &target_directory.join("attempt-3.start.claim"),
    ] {
        let saved = std::fs::read(marker).unwrap();
        std::fs::write(marker, [0; 32]).unwrap();
        refused(
            &activate_command("dispatch-activate", &activation, &start_claim)
                .arg("--custody")
                .arg(&custody)
                .arg("--checkout")
                .arg(&checkout)
                .output()
                .await
                .unwrap(),
            "claim does not match predecessor",
        );
        assert!(!target_directory.join("attempt-3.activate.claim").exists());
        std::fs::write(marker, saved).unwrap();
    }
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    let output = activate_command("dispatch-activate", &activation, &start_claim)
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(&checkout)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("attempt=3 active=true"));
    let active = owned.instance().get_stable_memory(canister).await;
    let output = activate_command("observe-activate", &activation, &start_claim)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("attempt=3 active=true"));
    refused(
        &activate_command("observe-activate", &unused_activation, &start_claim)
            .output()
            .await
            .unwrap(),
        "outcome unresolved",
    );
    for record in [&activation, &unused_activation] {
        refused(
            &activate_command("dispatch-activate", record, &start_claim)
                .arg("--custody")
                .arg(&custody)
                .arg("--checkout")
                .arg(&checkout)
                .output()
                .await
                .unwrap(),
            "already claimed",
        );
    }
    let after_active = directory.join("activation-after-active");
    refused(
        &activate_command("prepare-activate", &after_active, &start_claim)
            .args(["--attempt", "3"])
            .output()
            .await
            .unwrap(),
        "does not match the current drained attempt",
    );
    assert!(!after_active.exists());
    assert_eq!(owned.instance().get_stable_memory(canister).await, active);
    refused(
        &start_command("observe-start", &start_claim, &upgrade_claim, &current_stop)
            .output()
            .await
            .unwrap(),
        "does not match the current drained attempt",
    );
    // A newer drain must survive even an activation that reaches execution late.
    for transition in [Command::Drain(4), Command::Activate(3)] {
        let bytes = owned
            .instance()
            .update_call(
                canister,
                controller,
                "blob_update",
                ic_wire::native::encode_request(
                    ic_wire::native::method_id("blob_update"),
                    &transition,
                    RPC_LIMITS,
                )
                .unwrap(),
            )
            .await
            .unwrap();
        let outcome: Result<ic_blob_storage::protocol::Outcome, ic_blob_storage::local::Error> =
            ic_wire::native::decode_response(
                ic_wire::native::method_id("blob_update"),
                &bytes,
                RPC_LIMITS,
            )
            .unwrap();
        if matches!(transition, Command::Activate(_)) {
            assert_eq!(
                outcome,
                Err(ic_blob_storage::local::Error::MaintenanceAttempt)
            );
        } else {
            assert!(outcome.is_ok());
        }
        refused(
            &start_command("observe-start", &start_claim, &upgrade_claim, &current_stop)
                .output()
                .await
                .unwrap(),
            "does not match the current drained attempt",
        );
    }
    let drained = owned.instance().get_stable_memory(canister).await;
    refused(
        &activate_command("observe-activate", &activation, &start_claim)
            .output()
            .await
            .unwrap(),
        "acknowledged historically; current admission differs",
    );
    let stale_activation = directory.join("stale-activation");
    refused(
        &activate_command("prepare-activate", &stale_activation, &start_claim)
            .args(["--attempt", "3"])
            .output()
            .await
            .unwrap(),
        "does not match the current drained attempt",
    );
    assert!(!stale_activation.exists());
    assert_eq!(owned.instance().get_stable_memory(canister).await, drained);
    let output = inspect(&directory, url.as_str(), canister, "1").await;
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("attempt=4 active=false"));
    // A definitive install rejection retains the old state and the dispatch claim.
    let failure_stop = directory.join("failure-stop");
    let output = command(&directory, url.as_str(), canister, "1", "prepare-stop")
        .arg("--record")
        .arg(&failure_stop)
        .args(["--attempt", "4"])
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = command(&directory, url.as_str(), canister, "1", "dispatch-stop")
        .arg("--record")
        .arg(&failure_stop)
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(&checkout)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::write(&next_wasm, b"\0asm\x01\0\0\0\xff").unwrap();
    let failed_upgrade = directory.join("failed-upgrade");
    let output = upgrade_command("prepare-upgrade", &failed_upgrade, &failure_stop)
        .args(["--attempt", "4"])
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let before = owned.instance().get_stable_memory(canister).await;
    let output = upgrade_command("dispatch-upgrade", &failed_upgrade, &failure_stop)
        .arg("--custody")
        .arg(&custody)
        .arg("--checkout")
        .arg(&checkout)
        .output()
        .await
        .unwrap();
    assert!(!output.status.success());
    refused(
        &upgrade_command("observe-upgrade", &failed_upgrade, &failure_stop)
            .output()
            .await
            .unwrap(),
        "original upgrade rejected",
    );
    let failed_start = directory.join("failed-start");
    refused(
        &start_command(
            "prepare-start",
            &failed_start,
            &failed_upgrade,
            &failure_stop,
        )
        .args(["--attempt", "4"])
        .output()
        .await
        .unwrap(),
        "original upgrade rejected",
    );
    assert!(!failed_start.exists());
    refused(
        &upgrade_command("dispatch-upgrade", &failed_upgrade, &failure_stop)
            .arg("--custody")
            .arg(&custody)
            .arg("--checkout")
            .arg(&checkout)
            .output()
            .await
            .unwrap(),
        "already claimed",
    );
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    let output = command(&directory, url.as_str(), canister, "1", "observe-stop")
        .arg("--record")
        .arg(&failure_stop)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    for forbidden in ["reinstall", "upgrade", "activate", "start", "snapshot-load"] {
        let output = tokio::process::Command::new(std::env::var_os("BLOB_ADMIN").unwrap())
            .arg(forbidden)
            .output()
            .await
            .unwrap();
        refused(&output, "unrecognized subcommand");
    }
    owned.close().await;
}
