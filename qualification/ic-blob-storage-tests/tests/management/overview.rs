use super::{TEST_PEM, encode, installation, refused, support};
use ic_blob_storage::{
    local::{Error, UploadRequest},
    protocol::{Command, Outcome},
};
use ic_blob_storage_wire::RPC_LIMITS;
use ic_platform::native::Ed25519Identity;
use ic_platform_types::Principal;
use ic_testkit::pocket_ic::{CanisterId, PocketIcBuilder, nonblocking::PocketIc};
use std::path::{Path, PathBuf};

async fn mutate(pic: &PocketIc, canister: CanisterId, caller: CanisterId, input: Command) {
    let bytes = ic_blob_storage_wire::frame::encode(
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
    let result: Result<Outcome, Error> = ic_blob_storage_wire::frame::decode(
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
        .arg("--root-subnet")
        .arg(std::fs::read_to_string(directory.join("root-subnet.txt")).unwrap())
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
    let application = CanisterId::from_slice(
        Ed25519Identity::from_pem(TEST_PEM, 4096)
            .unwrap()
            .principal()
            .as_slice(),
    );
    let mut input = installation(Principal::from_slice(application.as_slice()));
    input.application = input.manager;
    let uploader = Principal::from_slice(&[8; 29]);
    let wasm = std::fs::read(std::env::var_os("BLOB_WASM").expect("set BLOB_WASM")).unwrap();
    let (server, mut owned) = support::instance(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
        300,
    )
    .await;
    let canister = owned.client().create_canister().await;
    owned
        .client()
        .add_cycles(canister, 10_000_000_000_000)
        .await;
    owned
        .client()
        .install_canister(canister, wasm.clone(), encode(&input), None)
        .await;
    // Full-width identities and principals, with one shared root and 62 reservations.
    for n in 1..=64u8 {
        let root = if n == 2 { [1; 32] } else { [n; 32] };
        let id = (u128::MAX - u128::from(n)).try_into().unwrap();
        mutate(
            owned.client(),
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
                owned.client(),
                canister,
                CanisterId::from_slice(uploader.as_slice()),
                Command::Register(root),
            )
            .await;
        }
    }
    std::fs::write(
        directory.join("root.der"),
        owned.client().root_key().await.unwrap(),
    )
    .unwrap();
    std::fs::write(
        directory.join("root-subnet.txt"),
        owned.client().topology().await.get_nns().unwrap().to_text(),
    )
    .unwrap();
    let url = owned.client_mut().make_live(None).await;
    let before = support::blob_state(owned.client(), canister, application, &[uploader]).await;
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
    assert_eq!(
        support::blob_state(owned.client(), canister, application, &[uploader]).await,
        before
    );
    let untrusted = directory.join("untrusted");
    std::fs::create_dir_all(&untrusted).unwrap();
    std::fs::write(untrusted.join("root.der"), [0; 32]).unwrap();
    std::fs::copy(
        directory.join("root-subnet.txt"),
        untrusted.join("root-subnet.txt"),
    )
    .unwrap();
    refused(
        &command(&untrusted, url.as_str(), canister, uploader)
            .arg("--identity")
            .arg(directory.join("application.pem"))
            .output()
            .await
            .unwrap(),
        "invalid trusted root key",
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
            owned.client(),
            canister,
            application,
            Command::Release((u128::MAX - n).try_into().unwrap()),
        )
        .await;
    }
    mutate(owned.client(), canister, application, Command::Drain(1)).await;
    owned.client().stop_canister(canister, None).await.unwrap();
    let args = ic_blob_storage_wire::frame::encode(
        &(),
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id("blob_post_upgrade"),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap();
    owned
        .client()
        .upgrade_canister(canister, wasm, args, None)
        .await
        .unwrap();
    owned.client().start_canister(canister, None).await.unwrap();
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
    support::close(server, owned).await;
}
