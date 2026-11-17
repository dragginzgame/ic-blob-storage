//! Real reference-app decisions around the owned transfer client; no provider substitute policy.

use super::*;
use ic_blob_storage_reference_app::protocol::{
    DraftInput, DraftView, Error as AppError, Installation as AppInstallation, Phase,
};
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn application_state(pic: &PocketIc, app: CanisterId) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for seed in [2, 3] {
        for id in [1u128, 2] {
            let id: ic_blob_storage::local::UploadId = id.try_into().unwrap();
            result.push(
                pic.query_call(
                    app,
                    CanisterId::from_slice(signer(seed).as_slice()),
                    "app_draft",
                    frame("app_draft", &id),
                )
                .await
                .unwrap(),
            );
        }
    }
    result
}

fn declaration() -> DraftInput {
    // Existing frozen six-chunk vector, independently prepared by Rust and JS.
    const ROOT: &str = "bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03";
    let bytes: Vec<u8> = (0..5 * 1024 * 1024 + 17)
        .map(|i| ((i * 31 + 7) % 251) as u8)
        .collect();
    let chunks = bytes
        .chunks(ic_blob_storage::caffeine::CHUNK_BYTES)
        .map(|chunk| {
            let mut hash = Sha256::new();
            hash.update(b"icfs-chunk/");
            hash.update(chunk);
            hash.finalize().into()
        })
        .collect();
    DraftInput {
        id: 1.try_into().unwrap(),
        asset: "portrait".into(),
        root: std::array::from_fn(|i| u8::from_str_radix(&ROOT[i * 2..i * 2 + 2], 16).unwrap()),
        bytes: bytes.len() as u64,
        expires_at: u64::MAX,
        chunks,
        headers: vec![
            ("Content-Length".into(), bytes.len().to_string()),
            ("Content-Type".into(), "application/octet-stream".into()),
        ],
    }
}

async fn app_update<T: Serialize>(
    user: &Client,
    app: CanisterId,
    method: &str,
    input: &T,
) -> Result<DraftView, AppError> {
    let bytes = user
        .update(&app, method, &frame(method, input))
        .await
        .unwrap();
    wire::decode(
        &bytes,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

async fn catalog(user: &Client, app: CanisterId) -> Vec<DraftView> {
    let bytes = user
        .query(&app, "app_catalog", &frame("app_catalog", &()))
        .await
        .unwrap();
    wire::decode::<Result<Vec<DraftView>, AppError>>(
        &bytes,
        wire::Encoding::Frame {
            method: wire::method_id("app_catalog"),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
    .unwrap()
}

async fn cli_application(
    directory: &Path,
    url: &str,
    app: CanisterId,
    id: Option<u128>,
) -> std::process::Output {
    let mut command =
        tokio::process::Command::new(std::env::var_os("BLOB_CLI").expect("set BLOB_CLI"));
    command
        .args([
            "app",
            if id.is_some() { "draft" } else { "catalog" },
            "--url",
            url,
            "--canister",
            &app.to_text(),
        ])
        .arg("--identity")
        .arg(directory.join("uploader.pem"))
        .arg("--root-key")
        .arg(directory.join("root.der"))
        .arg("--root-subnet")
        .arg(std::fs::read_to_string(directory.join("root-subnet.txt")).unwrap())
        .kill_on_drop(true);
    if let Some(id) = id {
        command.args(["--id", &id.to_string()]);
    }
    tokio::time::timeout(Duration::from_secs(30), command.output())
        .await
        .unwrap()
        .unwrap()
}

fn cli_report(output: &std::process::Output) -> &str {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::str::from_utf8(&output.stdout).unwrap()
}

#[tokio::test]
async fn verified_read_publication_and_inflight_cancellation_use_the_real_catalog() {
    publication_cases(false).await;
}

#[tokio::test]
#[ignore = "requires explicit BROWSER; run make test-browser-application"]
async fn browser_publication_and_late_read_use_the_real_catalog() {
    assert!(std::env::var_os("BROWSER").is_some(), "set BROWSER");
    publication_cases(true).await;
}

async fn publication_cases(browser: bool) {
    let (server, mut owned) = support::instance(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
        180,
    )
    .await;
    let app_wasm = std::fs::read(
        std::env::var_os("BLOB_REFERENCE_APP")
            .expect("set BLOB_REFERENCE_APP to the built product Wasm"),
    )
    .unwrap();
    let url = owned.client_mut().make_live(None).await;
    let draft = declaration();
    let cases: &[(&str, &str)] = if browser {
        &[("success", "SAUPX"), ("cancel-read", "SARUPX")]
    } else {
        &[
            ("success", "APX"),
            ("corrupt", "AFX"),
            ("cancel-transfer", "TPX"),
            ("cancel-read", "ARPX"),
            ("lost-final-reply", "LFX"),
            ("interrupted-read", "ADFX"),
            ("crash-transfer", "K"),
            ("reconcile", "LFSPX"),
            ("reconcile-corrupt", "LFSFX"),
            ("reconcile-cancelled", "LFSRPX"),
        ]
    };
    for &(scenario, stages) in cases {
        let mut draft = draft.clone();
        let inspect_cli = !browser && scenario == "success";
        if inspect_cli {
            draft.asset = "portrait\n\u{1b}[31m".into();
        }
        // Each case owns a fresh installation, not a reset of an exposed root.
        let app = owned.client().create_canister().await;
        owned.client().add_cycles(app, 10_000_000_000_000).await;
        let mut limits = issuer_limits();
        limits.manifest.max_content_bytes = draft.bytes.try_into().unwrap();
        limits.manifest.max_chunks = draft.chunks.len().try_into().unwrap();
        limits.logical_bytes = draft.bytes.try_into().unwrap();
        limits.exposure_bytes = draft.bytes.try_into().unwrap();
        let (owner, trust_root) =
            install_issuer(&owned, Principal::from_slice(app.as_slice()), limits).await;
        owned
            .client()
            .install_canister(
                app,
                app_wasm.clone(),
                frame(
                    "app_init",
                    &AppInstallation {
                        blob: Principal::from_slice(owner.as_slice()),
                        users: vec![signer(2), signer(3)],
                    },
                ),
                None,
            )
            .await;
        let user = Client::new(
            url.as_str(),
            &trust_root,
            owned.client().topology().await.get_nns().unwrap(),
            2,
        );
        assert_eq!(
            app_update(&user, app, "app_create", &draft)
                .await
                .unwrap()
                .phase,
            Phase::Draft
        );
        let directory = certificate_directory(
            &format!("publication-{browser}-{scenario}-test"),
            &trust_root,
            owned.client().topology().await.get_nns().unwrap(),
        );
        let claim = retain_proof(&directory, url.as_str(), owner, draft.root, &trust_root).await;
        let original_packet = std::fs::read(claim.join("request.cbor")).unwrap();
        let original_dispatch = std::fs::read(claim.join("dispatch.claim")).unwrap();
        let original_proof = std::fs::read(directory.join("certificate.cbor")).unwrap();
        if inspect_cli {
            // A second allowed user owns a published use of this same object.
            // The CLI's signer cannot inspect it just by knowing its draft ID.
            let other = Client::new(
                url.as_str(),
                &trust_root,
                owned.client().topology().await.get_nns().unwrap(),
                3,
            );
            let mut other_draft = draft.clone();
            other_draft.id = 2.try_into().unwrap();
            app_update(&other, app, "app_create", &other_draft)
                .await
                .unwrap();
            app_update(&other, app, "app_publish", &other_draft.id)
                .await
                .unwrap();
            let app_before = application_state(owned.client(), app).await;
            let blob_before =
                support::blob_state(owned.client(), owner, app, &[signer(2), signer(3)]).await;
            let output = cli_application(&directory, url.as_str(), app, None).await;
            assert!(cli_report(&output).contains("Published catalog entries: 0"));
            let output = cli_application(&directory, url.as_str(), app, Some(1)).await;
            let report = cli_report(&output);
            assert!(report.contains("Draft (unpublished)"));
            assert!(report.contains("portrait\\n\\u{1b}[31m"));
            assert!(!report.contains('\u{1b}'));
            for (id, refusal) in [(2, "Denied"), (99, "Unknown")] {
                let output = cli_application(&directory, url.as_str(), app, Some(id)).await;
                assert!(!output.status.success());
                assert!(output.stdout.is_empty());
                assert!(
                    std::str::from_utf8(&output.stderr)
                        .unwrap()
                        .contains(refusal)
                );
            }
            assert_eq!(application_state(owned.client(), app).await, app_before);
            assert_eq!(
                support::blob_state(owned.client(), owner, app, &[signer(2), signer(3)]).await,
                blob_before
            );
            app_update(&other, app, "app_cancel", &other_draft.id)
                .await
                .unwrap();
        }
        let mut client = tokio::process::Command::new("node");
        client
            .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(if browser {
                "../../clients/browser/tests/runtime.js"
            } else {
                "../../clients/browser/tests/installed-transfer.js"
            }))
            .arg("--directory")
            .arg(&directory)
            .args([
                "--issuer",
                &owner.to_text(),
                "--uploader",
                &signer(2).to_text(),
                "--url",
                url.as_str(),
                "--scenario",
                scenario,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);
        if browser {
            client.arg("--request").arg(claim.join("request.cbor"));
        }
        let mut child = client.spawn().unwrap();
        let mut events = child.stdout.take().unwrap();
        let mut replies = child.stdin.take().unwrap();
        let mut before_read = None;
        tokio::time::timeout(Duration::from_secs(45), async {
            for stage in stages.bytes() {
                assert_eq!(events.read_u8().await.unwrap(), stage, "{scenario}");
                let reply = match stage {
                    b'T' | b'R' => {
                        assert!(catalog(&user, app).await.is_empty());
                        let cancelled = app_update(&user, app, "app_cancel", &draft.id).await.unwrap();
                        assert_eq!(cancelled.phase, Phase::Cleaned);
                        b'C'
                    }
                    b'A' | b'D' | b'L' | b'F' => {
                        // HTTP acknowledgments and corrupt reads cannot publish the draft.
                        assert!(catalog(&user, app).await.is_empty());
                        stage
                    }
                    b'K' => {
                        assert!(catalog(&user, app).await.is_empty());
                        child.kill().await.unwrap();
                        break;
                    }
                    b'S' => {
                        let bytes = user.query(&app, "app_draft", &frame("app_draft", &draft.id)).await.unwrap();
                        let saved = wire::decode::<Result<DraftView, AppError>>(&bytes, wire::Encoding::Frame { method: wire::method_id("app_draft"), direction: wire::Direction::Response }, RPC_LIMITS).unwrap().unwrap();
                        assert_eq!(saved.input, draft);
                        assert_eq!(saved.phase, Phase::Draft);
                        before_read = Some(support::blob_state(owned.client(), owner, app, &[signer(2), signer(3)]).await);
                        b'S'
                    }
                    b'U' => {
                        // The HTTP read is outside both canisters. Stop/drain their
                        // runtime calls, preserving the draft and issued obligation.
                        let pic = owned.client();
                        assert!(catalog(&user, app).await.is_empty());
                        let app_before = application_state(pic, app).await;
                        pic.stop_canister(app, None).await.unwrap();
                        let manager = Client::new(url.as_str(), &trust_root, owned.client().topology().await.get_nns().unwrap(), 9);
                        let drained = signed_command(&manager, owner, Command::Drain(1)).await;
                        assert!(matches!(drained, Outcome::Maintenance(m) if m.attempt == 1 && !m.active));
                        let owner_before = support::blob_state(pic, owner, app, &[signer(2), signer(3)]).await;
                        pic.stop_canister(owner, None).await.unwrap();
                        pic.upgrade_eop_canister(owner, wasm(), frame("callback_upgrade", &()), None)
                            .await.unwrap();
                        pic.upgrade_eop_canister(app, app_wasm.clone(), frame("app_post_upgrade", &()), None)
                            .await.unwrap();
                        pic.start_canister(owner, None).await.unwrap();
                        assert_eq!(support::blob_state(pic, owner, app, &[signer(2), signer(3)]).await, owner_before);
                        // Reopening must not silently activate admission.
                        assert_eq!(signed_command(&manager, owner, Command::Drain(1)).await, drained);
                        assert_eq!(support::blob_state(pic, owner, app, &[signer(2), signer(3)]).await, owner_before);
                        let active = signed_command(&manager, owner, Command::Activate(1)).await;
                        assert!(matches!(active, Outcome::Maintenance(m) if m.attempt == 1 && m.active));
                        pic.start_canister(app, None).await.unwrap();
                        assert_eq!(application_state(pic, app).await, app_before);
                        assert!(catalog(&user, app).await.is_empty());
                        before_read = Some(support::blob_state(pic, owner, app, &[signer(2), signer(3)]).await);
                        b'U'
                    }
                    b'P' => {
                        let published = app_update(&user, app, "app_publish", &draft.id).await;
                        if matches!(scenario, "success" | "reconcile") {
                            assert_eq!(published.unwrap().phase, Phase::Published);
                            assert_eq!(catalog(&user, app).await[0].input, draft);
                            if inspect_cli {
                                let output = cli_application(&directory, url.as_str(), app, None).await;
                                let report = cli_report(&output);
                                assert!(report.contains("Published catalog entries: 1"));
                                assert!(report.contains("Published (in application catalog)"));
                            }
                            b'P'
                        } else {
                            // A correct late read, or a stale client command, cannot undo cancellation.
                            assert_eq!(published, Err(AppError::Cancelled));
                            assert!(catalog(&user, app).await.is_empty());
                            b'C'
                        }
                    }
                    b'X' => b'X',
                    _ => unreachable!(),
                };
                replies.write_all(&[reply]).await.unwrap();
            }
            assert_eq!(child.wait().await.unwrap().success(), scenario != "crash-transfer", "{scenario}");
            assert!(matches!(events.read_u8().await, Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof));
        }).await.unwrap();
        if let Some(before) = before_read
            && scenario != "reconcile-cancelled"
        {
            // Verified GET and catalog publication do not mutate Blob's state.
            assert_eq!(
                support::blob_state(owned.client(), owner, app, &[signer(2), signer(3)]).await,
                before
            );
        }
        assert_eq!(
            std::fs::read(claim.join("request.cbor")).unwrap(),
            original_packet
        );
        assert_eq!(
            std::fs::read(claim.join("dispatch.claim")).unwrap(),
            original_dispatch
        );
        assert_eq!(
            std::fs::read(directory.join("certificate.cbor")).unwrap(),
            original_proof
        );
        if !browser {
            let intent = std::fs::read(claim.join("transfer/intent.wire")).unwrap();
            assert!(!intent.is_empty());
            let proof = std::fs::read(claim.join("transfer/certificate.cbor")).unwrap();
            assert_eq!(
                proof,
                std::fs::read(directory.join("certificate.cbor")).unwrap()
            );
            if scenario.starts_with("reconcile")
                || matches!(
                    scenario,
                    "lost-final-reply" | "interrupted-read" | "crash-transfer"
                )
            {
                // An independently started client reopens the original claim. It uses
                // a fresh prepared handle/gateway but must not issue another PUT.
                let output =
                    tokio::time::timeout(Duration::from_secs(30), client.arg("--replay").output())
                        .await
                        .unwrap()
                        .unwrap();
                assert!(output.status.success(), "{scenario}");
                assert_eq!(
                    String::from_utf8_lossy(&output.stdout).trim(),
                    "replay-refused"
                );
                assert_eq!(
                    std::fs::read(claim.join("transfer/intent.wire")).unwrap(),
                    intent
                );
                assert_eq!(
                    std::fs::read(claim.join("transfer/certificate.cbor")).unwrap(),
                    proof
                );
            }
        }
        let bytes = user
            .query(&app, "app_draft", &frame("app_draft", &draft.id))
            .await
            .unwrap();
        let observed = wire::decode::<Result<DraftView, AppError>>(
            &bytes,
            wire::Encoding::Frame {
                method: wire::method_id("app_draft"),
                direction: wire::Direction::Response,
            },
            RPC_LIMITS,
        )
        .unwrap()
        .unwrap();
        assert_eq!(observed.input, draft);
        assert_eq!(
            observed.phase,
            match scenario {
                "success" | "reconcile" => Phase::Published,
                "cancel-transfer" | "cancel-read" | "reconcile-cancelled" => Phase::Cleaned,
                _ => Phase::Draft,
            }
        );
        assert_eq!(
            catalog(&user, app).await.len(),
            usize::from(matches!(scenario, "success" | "reconcile"))
        );
        assert_eq!(
            app_update(&user, app, "app_cancel", &draft.id)
                .await
                .unwrap()
                .phase,
            Phase::Cleaned
        );
        assert!(catalog(&user, app).await.is_empty());
        if inspect_cli {
            let app_before = application_state(owned.client(), app).await;
            let blob_before =
                support::blob_state(owned.client(), owner, app, &[signer(2), signer(3)]).await;
            let output = cli_application(&directory, url.as_str(), app, None).await;
            assert!(cli_report(&output).contains("Published catalog entries: 0"));
            let output = cli_application(&directory, url.as_str(), app, Some(1)).await;
            assert!(cli_report(&output).contains("Cleaned (application cleanup finished)"));
            assert_eq!(application_state(owned.client(), app).await, app_before);
            assert_eq!(
                support::blob_state(owned.client(), owner, app, &[signer(2), signer(3)]).await,
                blob_before
            );
        }
        // Simulator inspection as the application authority; application operations
        // above use the real user signature and the application's outgoing calls.
        let bytes = owned
            .client()
            .query_call(
                owner,
                app,
                "blob_query",
                frame("blob_query", &Inspection::Usage),
            )
            .await
            .unwrap();
        assert!(
            matches!(wire::decode::<Result<Observation, Error>>(&bytes, wire::Encoding::Frame { method: wire::method_id("blob_query"), direction: wire::Direction::Response }, RPC_LIMITS).unwrap().unwrap(),
            Observation::Usage(u) if u.live_bytes == 0 && u.unresolved_bytes == draft.bytes)
        );
    }
    support::close(server, owned).await;
}
