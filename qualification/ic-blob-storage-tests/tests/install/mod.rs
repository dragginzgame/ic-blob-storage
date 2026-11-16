//! Fresh processes prepare/inspect exact install envelopes consumed by a local IC.

mod relay;

use super::*;
use ic_blob_storage_tests::native;
use ic_platform::native::{PreparedUpdate, RequestStatus};
use sha2::{Digest, Sha256};
use std::process::Output;

const PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcH\n-----END PRIVATE KEY-----\n";

async fn cli(operation: &str, arguments: &[String]) -> Output {
    tokio::time::timeout(
        Duration::from_secs(40),
        tokio::process::Command::new(std::env::var_os("BLOB_CLI").unwrap())
            .args(["app", operation])
            .args(arguments)
            .kill_on_drop(true)
            .output(),
    )
    .await
    .unwrap()
    .unwrap()
}

#[tokio::test]
async fn retained_install_requests_execute_exact_configuration_and_refuse_repreparation() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/signed-installation-test")
        .join(format!("{}-{}", std::process::id(), native::now()));
    std::fs::create_dir_all(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    let (server, mut owned) = support::instance(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
        180,
    )
    .await;
    let app = owned.client().create_canister().await;
    let blob = owned.client().create_canister().await;
    let controller = native::identity(7).principal();
    for target in [app, blob] {
        owned.client().add_cycles(target, 10_000_000_000_000).await;
        owned
            .client()
            .set_controllers(
                target,
                None,
                vec![CanisterId::from_slice(controller.as_slice())],
            )
            .await
            .unwrap();
    }
    let root = owned.client().root_key().await.unwrap();
    let subnet = owned.client().topology().await.get_nns().unwrap();
    std::fs::write(directory.join("root.der"), &root).unwrap();
    std::fs::write(directory.join("controller.pem"), PEM).unwrap();
    let user = Principal::from_slice(&[2, 1]);
    let config = BlobInstallation {
        application: Principal::from_slice(app.as_slice()),
        manager: controller,
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
    std::fs::write(
        directory.join("blob-init.wire"),
        encode("blob_init", &config),
    )
    .unwrap();
    std::fs::write(
        directory.join("app-init.wire"),
        encode(
            "app_init",
            &Installation {
                blob: Principal::from_slice(blob.as_slice()),
                users: vec![user],
            },
        ),
    )
    .unwrap();
    let upstream = owned.client_mut().make_live(None).await;
    let relay = relay::Relay::start(upstream.port().unwrap()).await;
    let url = &relay.url;
    let client = native::Client::new(url.as_str(), &root, subnet.as_slice(), 7);
    let network = Sha256::digest(&root)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let mut last_signing = Vec::new();
    for (role, target, artifact) in [
        ("blob", blob, "BLOB_WASM"),
        ("app", app, "BLOB_REFERENCE_APP"),
    ] {
        let wasm = std::fs::read(std::env::var_os(artifact).unwrap()).unwrap();
        let path = directory.join(format!("{role}.wasm"));
        std::fs::write(&path, &wasm).unwrap();
        let arguments = [
            "--url",
            url.as_str(),
            "--canister",
            &app.to_text(),
            "--blob-canister",
            &blob.to_text(),
            "--controller",
            &controller.to_text(),
            "--role",
            role,
            "--root-key",
            directory.join("root.der").to_str().unwrap(),
            "--root-subnet",
            &subnet.to_text(),
            "--inputs",
            directory.to_str().unwrap(),
            "--custody",
            directory.join("custody").to_str().unwrap(),
            "--wasm",
            path.to_str().unwrap(),
        ]
        .map(str::to_owned)
        .to_vec();
        let mut signing = arguments.clone();
        signing.extend([
            "--identity".into(),
            directory.join("controller.pem").display().to_string(),
        ]);
        let output = cli("prepare-install", &signing).await;
        assert!(output.status.success(), "{output:?}");
        let record = directory
            .join("custody")
            .join(&network)
            .join(target.to_text())
            .join("install");
        let bytes = std::fs::read(record.join("request.cbor")).unwrap();
        let inspected = cli("inspect-install", &arguments).await;
        assert!(inspected.status.success(), "{inspected:?}");
        assert_eq!(inspected.stdout, output.stdout);
        assert!(
            owned
                .client()
                .canister_status(target, Some(CanisterId::from_slice(controller.as_slice())))
                .await
                .unwrap()
                .module_hash
                .is_none()
        );
        assert!(!cli("prepare-install", &signing).await.status.success());
        assert_eq!(std::fs::read(record.join("request.cbor")).unwrap(), bytes);
        let original = PreparedUpdate::from_bytes(
            &bytes,
            Principal::from_slice(target.as_slice()),
            2 * 1024 * 1024,
        )
        .unwrap();
        let checkout = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap();
        let mut dispatch = signing.clone();
        dispatch.extend(["--checkout".into(), checkout.display().to_string()]);
        // Fresh creation is an explicit operator assertion, not inferred from module absence.
        assert!(!cli("dispatch-install", &dispatch).await.status.success());
        assert!(!record.join("dispatch.claim").exists());
        dispatch.push("--confirm-fresh-target".into());
        let mut invalid_checkout = dispatch.clone();
        let index = invalid_checkout
            .iter()
            .position(|arg| arg == "--checkout")
            .unwrap();
        invalid_checkout[index + 1] = directory.display().to_string();
        assert!(
            !cli("dispatch-install", &invalid_checkout)
                .await
                .status
                .success()
        );
        assert!(!record.join("dispatch.claim").exists());

        // Controller admission happens again immediately before claiming dispatch.
        let foreign = CanisterId::from_slice(native::identity(9).principal().as_slice());
        owned
            .client()
            .set_controllers(
                target,
                Some(CanisterId::from_slice(controller.as_slice())),
                vec![foreign],
            )
            .await
            .unwrap();
        assert!(!cli("dispatch-install", &dispatch).await.status.success());
        assert!(!record.join("dispatch.claim").exists());
        owned
            .client()
            .set_controllers(
                target,
                Some(foreign),
                vec![CanisterId::from_slice(controller.as_slice())],
            )
            .await
            .unwrap();

        // Race independent dispatchers and drop the real HTTP response after the
        // gateway receives the exact saved install. Only one can claim submission.
        relay.drop_reply_for(&bytes);
        let before = relay.dropped();
        let (first, second) = tokio::join!(
            cli("dispatch-install", &dispatch),
            cli("dispatch-install", &dispatch)
        );
        assert!(
            !first.status.success() && !second.status.success(),
            "{first:?}\n{second:?}"
        );
        assert_eq!(
            relay.dropped(),
            before + 1,
            "exactly one installation forwarded"
        );
        assert!(
            !record.join("outcome.cbor").exists(),
            "neither dispatcher received a terminal proof"
        );
        assert_eq!(
            std::fs::read(record.join("dispatch.claim")).unwrap(),
            original.request_id()
        );
        let observed = tokio::time::timeout(Duration::from_secs(30), async {
            loop {
                let output = cli("observe-install", &signing).await;
                if output.status.success() {
                    break output;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .expect("original installation should become observable");
        assert!(
            String::from_utf8(observed.stdout)
                .unwrap()
                .contains("original=acknowledged")
        );
        let certificate = std::fs::read(record.join("outcome.cbor")).unwrap();
        let outcome = client
            .transport
            .authenticate_request(&original, &certificate, native::policy())
            .unwrap();
        assert!(matches!(outcome.status(), RequestStatus::Replied(_)));
        assert!(!cli("dispatch-install", &dispatch).await.status.success());
        assert_eq!(relay.dropped(), before + 1, "no installation resubmission");
        assert!(cli("observe-install", &signing).await.status.success());
        assert_eq!(
            std::fs::read(record.join("outcome.cbor")).unwrap(),
            certificate
        );

        // Current controller drift cannot erase the historical installation proof.
        owned
            .client()
            .set_controllers(
                target,
                Some(CanisterId::from_slice(controller.as_slice())),
                vec![CanisterId::from_slice(controller.as_slice()), foreign],
            )
            .await
            .unwrap();
        assert!(!cli("observe-install", &signing).await.status.success());
        assert_eq!(
            std::fs::read(record.join("outcome.cbor")).unwrap(),
            certificate
        );
        owned
            .client()
            .set_controllers(
                target,
                Some(foreign),
                vec![CanisterId::from_slice(controller.as_slice())],
            )
            .await
            .unwrap();
        std::fs::write(record.join("outcome.cbor"), b"corrupted proof").unwrap();
        assert!(!cli("observe-install", &signing).await.status.success());
        assert_eq!(
            std::fs::read(record.join("outcome.cbor")).unwrap(),
            b"corrupted proof"
        );
        // Restore only these test-corrupted bytes to finish the independent readback.
        std::fs::write(record.join("outcome.cbor"), &certificate).unwrap();
        let status = owned
            .client()
            .canister_status(target, Some(CanisterId::from_slice(controller.as_slice())))
            .await
            .unwrap();
        assert_eq!(status.module_hash, Some(Sha256::digest(&wasm).to_vec()));
        assert_eq!(
            status.settings.controllers,
            vec![CanisterId::from_slice(controller.as_slice())]
        );
        assert!(cli("inspect-install", &arguments).await.status.success());
        assert!(!cli("prepare-install", &signing).await.status.success());
        assert_eq!(std::fs::read(record.join("request.cbor")).unwrap(), bytes);
        last_signing = signing;
    }
    let view: Result<ic_blob_storage::protocol::ManagementView, BlobError> = call(
        owned.client(),
        blob,
        CanisterId::from_slice(controller.as_slice()),
        "blob_management",
        &(),
    )
    .await;
    assert_eq!(view.unwrap().configuration, config);
    let catalog: Result<Vec<DraftView>, Error> = decode(
        "app_catalog",
        &owned
            .client()
            .query_call(
                app,
                CanisterId::from_slice(user.as_slice()),
                "app_catalog",
                encode("app_catalog", &()),
            )
            .await
            .unwrap(),
    );
    assert!(catalog.unwrap().is_empty());

    // An unrestricted controller can remove code; the original acknowledgment
    // must not become a claim about current installed state or permission to retry.
    let app_record = directory
        .join("custody")
        .join(&network)
        .join(app.to_text())
        .join("install");
    let acknowledged = std::fs::read(app_record.join("outcome.cbor")).unwrap();
    owned
        .client()
        .uninstall_canister(app, Some(CanisterId::from_slice(controller.as_slice())))
        .await
        .unwrap();
    assert!(!cli("observe-install", &last_signing).await.status.success());
    assert_eq!(
        std::fs::read(app_record.join("outcome.cbor")).unwrap(),
        acknowledged
    );

    // A different fresh local target receives an intentionally invalid Wasm.
    // Retain its certified rejection before any follow-up state observation.
    let rejected = owned.client().create_canister().await;
    owned
        .client()
        .add_cycles(rejected, 10_000_000_000_000)
        .await;
    owned
        .client()
        .set_controllers(
            rejected,
            None,
            vec![CanisterId::from_slice(controller.as_slice())],
        )
        .await
        .unwrap();
    let rejected_inputs = directory.join("rejected-inputs");
    std::fs::create_dir(&rejected_inputs).unwrap();
    std::fs::write(
        rejected_inputs.join("blob-init.wire"),
        encode(
            "blob_init",
            &BlobInstallation {
                application: Principal::from_slice(rejected.as_slice()),
                ..config
            },
        ),
    )
    .unwrap();
    std::fs::write(
        rejected_inputs.join("app-init.wire"),
        std::fs::read(directory.join("app-init.wire")).unwrap(),
    )
    .unwrap();
    std::fs::write(rejected_inputs.join("invalid.wasm"), b"\0asm\x01\0\0\0\xff").unwrap();
    for (flag, value) in [
        ("--canister", rejected.to_text()),
        ("--inputs", rejected_inputs.display().to_string()),
        (
            "--wasm",
            rejected_inputs.join("invalid.wasm").display().to_string(),
        ),
    ] {
        let index = last_signing.iter().position(|arg| arg == flag).unwrap();
        last_signing[index + 1] = value;
    }
    assert!(cli("prepare-install", &last_signing).await.status.success());
    let mut dispatch = last_signing.clone();
    dispatch.extend([
        "--checkout".into(),
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap()
            .display()
            .to_string(),
        "--confirm-fresh-target".into(),
    ]);
    let _discarded = cli("dispatch-install", &dispatch).await;
    let rejected_record = directory
        .join("custody")
        .join(&network)
        .join(rejected.to_text())
        .join("install");
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            assert!(!cli("observe-install", &last_signing).await.status.success());
            if rejected_record.join("outcome.cbor").exists() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("certified installation rejection should be retained");
    let packet = std::fs::read(rejected_record.join("request.cbor")).unwrap();
    let original = PreparedUpdate::from_bytes(
        &packet,
        Principal::from_slice(rejected.as_slice()),
        2 * 1024 * 1024,
    )
    .unwrap();
    let proof = std::fs::read(rejected_record.join("outcome.cbor")).unwrap();
    let outcome = client
        .transport
        .authenticate_request(&original, &proof, native::policy())
        .unwrap();
    assert!(matches!(outcome.status(), RequestStatus::Rejected { .. }));
    assert!(!cli("dispatch-install", &dispatch).await.status.success());
    assert_eq!(
        std::fs::read(rejected_record.join("outcome.cbor")).unwrap(),
        proof
    );
    assert!(
        owned
            .client()
            .canister_status(
                rejected,
                Some(CanisterId::from_slice(controller.as_slice()))
            )
            .await
            .unwrap()
            .module_hash
            .is_none()
    );
    relay.close().await;
    support::close(server, owned).await;
}
