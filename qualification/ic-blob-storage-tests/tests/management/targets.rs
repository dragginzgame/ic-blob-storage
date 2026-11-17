//! Actual controller-only target observations through the native CLI and owned gateway.

use super::*;

fn check(
    directory: &Path,
    url: &str,
    app: CanisterId,
    blob: CanisterId,
) -> tokio::process::Command {
    let mut command = tokio::process::Command::new(std::env::var_os("BLOB_CLI").unwrap());
    command
        .args([
            "app",
            "check-targets",
            "--url",
            url,
            "--canister",
            &app.to_text(),
            "--blob-canister",
            &blob.to_text(),
        ])
        .arg("--identity")
        .arg(directory.join("manager.pem"))
        .arg("--root-key")
        .arg(directory.join("root.der"))
        .arg("--root-subnet")
        .arg(std::fs::read_to_string(directory.join("root-subnet.txt")).unwrap())
        .kill_on_drop(true);
    command
}

#[tokio::test]
async fn proposed_targets_require_current_controller_and_absent_modules() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/target-observation-test")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    std::fs::write(directory.join("manager.pem"), TEST_PEM).unwrap();
    let signer = CanisterId::from_slice(
        Ed25519Identity::from_pem(TEST_PEM, 4096)
            .unwrap()
            .principal()
            .as_slice(),
    );
    let manager = CanisterId::from_slice(signer.as_slice());
    let (server, mut owned) = support::instance(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
        180,
    )
    .await;
    let app = owned.client().create_canister().await;
    let blob = owned.client().create_canister().await;
    for target in [app, blob] {
        owned.client().add_cycles(target, 10_000_000_000_000).await;
        owned
            .client()
            .set_controllers(target, None, vec![manager])
            .await
            .unwrap();
    }
    let root = owned.client().root_key().await.unwrap();
    std::fs::write(directory.join("root.der"), &root).unwrap();
    std::fs::write(
        directory.join("root-subnet.txt"),
        owned.client().topology().await.get_nns().unwrap().to_text(),
    )
    .unwrap();
    let url = owned.client_mut().make_live(None).await;
    refused(
        &check(&directory, url.as_str(), app, app)
            .output()
            .await
            .unwrap(),
        "targets must differ",
    );
    let wrong_root = include_bytes!("../../../../crates/ic-blob-storage-cli/src/mainnet-root.der");
    std::fs::write(directory.join("root.der"), wrong_root).unwrap();
    refused(
        &check(&directory, url.as_str(), app, blob)
            .output()
            .await
            .unwrap(),
        "authentication failed",
    );
    std::fs::write(directory.join("root.der"), &root).unwrap();
    let output = check(&directory, url.as_str(), app, blob)
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report = String::from_utf8(output.stdout).unwrap();
    assert!(report.contains(&format!("Application: {} module=absent", app.to_text())));
    assert!(report.contains(&format!("Blob: {} module=absent", blob.to_text())));
    assert!(report.contains("not a reservation"));
    for target in [app, blob] {
        let status = owned
            .client()
            .canister_status(target, Some(manager))
            .await
            .unwrap();
        assert!(status.module_hash.is_none());
        assert_eq!(status.settings.controllers, vec![manager]);
    }
    // A successful observation cannot hide a later controller change.
    let other = CanisterId::from_slice(&[4, 1]);
    owned
        .client()
        .set_controllers(app, Some(manager), vec![manager, other])
        .await
        .unwrap();
    refused(
        &check(&directory, url.as_str(), app, blob)
            .output()
            .await
            .unwrap(),
        "sole controller",
    );
    owned
        .client()
        .set_controllers(app, Some(manager), vec![other])
        .await
        .unwrap();
    refused(
        &check(&directory, url.as_str(), app, blob)
            .output()
            .await
            .unwrap(),
        "uncertified dispatch rejection",
    );
    owned
        .client()
        .set_controllers(app, Some(other), vec![manager])
        .await
        .unwrap();
    let wasm = std::fs::read(std::env::var_os("BLOB_WASM").unwrap()).unwrap();
    owned
        .client()
        .install_canister(
            blob,
            wasm,
            encode(&installation(Principal::from_slice(manager.as_slice()))),
            Some(manager),
        )
        .await;
    refused(
        &check(&directory, url.as_str(), app, blob)
            .output()
            .await
            .unwrap(),
        "Blob target already has an installed module",
    );
    let status = owned
        .client()
        .canister_status(blob, Some(manager))
        .await
        .unwrap();
    assert!(status.module_hash.is_some());
    assert_eq!(status.settings.controllers, vec![manager]);
    support::close(server, owned).await;
}
