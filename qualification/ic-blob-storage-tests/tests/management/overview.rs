use super::{TEST_PEM, encode, installation, refused};
use ic_blob_storage::{
    local::{Error, UploadRequest},
    protocol::{Command, Outcome, RPC_LIMITS},
};
use ic_platform_legacy::agent::{BasicIdentity, Identity};
use ic_platform_types::Principal;
use ic_testkit::{
    pic::PocketIcStartupConfig,
    pocket_ic::{CanisterId, PocketIcBuilder, nonblocking::PocketIc},
};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

async fn mutate(pic: &PocketIc, canister: CanisterId, caller: CanisterId, input: Command) {
    let bytes = ic_wire::encode(
        &input,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_update"),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap();
    let reply = pic
        .update_call(canister, caller, "blob_update", bytes)
        .await
        .unwrap();
    let result: Result<Outcome, Error> = ic_wire::decode(
        &reply,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_update"),
            direction: ic_wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap();
    result.unwrap();
}

fn command(
    directory: &Path,
    url: &str,
    canister: CanisterId,
    uploader: Principal,
) -> tokio::process::Command {
    let mut command =
        tokio::process::Command::new(std::env::var_os("BLOB_CLI").expect("set BLOB_CLI"));
    command
        .args([
            "overview",
            "--url",
            url,
            "--canister",
            &canister.to_text(),
            "--principal",
            &uploader.to_text(),
        ])
        .arg("--root-key")
        .arg(directory.join("root.der"))
        .kill_on_drop(true);
    command
}

#[tokio::test]
async fn principal_overview_at_capacity_is_read_only_and_survives_reopening() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/overview-test")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join("application.pem"), TEST_PEM).unwrap();
    let application = BasicIdentity::from_pem(TEST_PEM).unwrap().sender().unwrap();
    let mut input = installation(Principal::from_slice(application.as_slice()));
    input.application = input.manager;
    let uploader = Principal::from_slice(&[8; 29]);
    let wasm = std::fs::read(std::env::var_os("BLOB_WASM").expect("set BLOB_WASM")).unwrap();
    let server = ic_testkit::pic::server_assets::check(Path::new(
        &std::env::var_os("BLOB_TEST_SERVER").expect("set BLOB_TEST_SERVER"),
    ))
    .unwrap();
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
    // Full-width identities and principals, with one shared root and 62 reservations.
    for n in 1..=64u8 {
        let root = if n == 2 { [1; 32] } else { [n; 32] };
        let id = (u128::MAX - u128::from(n)).try_into().unwrap();
        mutate(
            owned.instance(),
            canister,
            application,
            Command::Authorize {
                request: UploadRequest {
                    id,
                    uploader,
                    root,
                    bytes: 1,
                    first_reference: id,
                    expires_at: u64::MAX,
                },
                chunks: vec![root],
                headers: vec![],
            },
        )
        .await;
        if n == 1 {
            mutate(
                owned.instance(),
                canister,
                CanisterId::from_slice(uploader.as_slice()),
                Command::Register(root),
            )
            .await;
        }
    }
    std::fs::write(
        directory.join("root.der"),
        owned.instance().root_key().await.unwrap(),
    )
    .unwrap();
    let url = owned.instance_mut().make_live(None).await;
    let before = owned.instance().get_stable_memory(canister).await;
    let output = command(&directory, url.as_str(), canister, uploader)
        .arg("--identity")
        .arg(directory.join("application.pem"))
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("Distinct associated roots: 63"));
    assert!(text.contains("Matching uploads: 64"));
    assert!(text.contains("reserved=62 live=1 unresolved=1"));
    assert!(text.contains("Application reference names on these roots: 64"));
    assert!(text.contains(&(u128::MAX - 1).to_string()));
    assert!(text.contains("Attached"));
    assert!(text.contains("provider storage not observed"));
    assert_eq!(owned.instance().get_stable_memory(canister).await, before);
    let untrusted = directory.join("untrusted");
    std::fs::create_dir_all(&untrusted).unwrap();
    std::fs::write(untrusted.join("root.der"), [0; 32]).unwrap();
    refused(
        &command(&untrusted, url.as_str(), canister, uploader)
            .arg("--identity")
            .arg(directory.join("application.pem"))
            .output()
            .await
            .unwrap(),
        "Blob query failed",
    );
    refused(
        &command(&directory, url.as_str(), canister, uploader)
            .output()
            .await
            .unwrap(),
        "Denied",
    );
    let empty = command(
        &directory,
        url.as_str(),
        canister,
        Principal::from_slice(&[9; 29]),
    )
    .arg("--identity")
    .arg(directory.join("application.pem"))
    .output()
    .await
    .unwrap();
    assert!(empty.status.success());
    assert!(
        String::from_utf8(empty.stdout)
            .unwrap()
            .contains("Distinct associated roots: 0")
    );
    for n in 1..=2 {
        mutate(
            owned.instance(),
            canister,
            application,
            Command::Release((u128::MAX - n).try_into().unwrap()),
        )
        .await;
    }
    mutate(owned.instance(), canister, application, Command::Drain(1)).await;
    owned
        .instance()
        .stop_canister(canister, None)
        .await
        .unwrap();
    let args = ic_wire::encode(
        &(),
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_post_upgrade"),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap();
    owned
        .instance()
        .upgrade_canister(canister, wasm, args, None)
        .await
        .unwrap();
    owned
        .instance()
        .start_canister(canister, None)
        .await
        .unwrap();
    let output = command(&directory, url.as_str(), canister, uploader)
        .arg("--identity")
        .arg(directory.join("application.pem"))
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("reserved=62 live=0 unresolved=1"));
    assert!(text.contains("Closing"));
    assert!(text.contains("Released"));
    assert!(text.contains("Matching uploads: 64"));
    owned.close().await;
}
