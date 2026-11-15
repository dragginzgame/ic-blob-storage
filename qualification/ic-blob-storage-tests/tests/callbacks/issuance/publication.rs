//! Real reference-app decisions around the owned transfer client; no provider substitute policy.

use super::*;
use ic_blob_storage_reference_app::protocol::{
    DraftInput, DraftView, Error as AppError, Installation as AppInstallation, Phase,
};
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
    user: &Agent,
    app: CanisterId,
    method: &str,
    input: &T,
) -> Result<DraftView, AppError> {
    let bytes = user
        .update(&app, method)
        .with_arg(frame(method, input))
        .call_and_wait()
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

async fn catalog(user: &Agent, app: CanisterId) -> Vec<DraftView> {
    let bytes = user
        .query(&app, "app_catalog")
        .with_arg(frame("app_catalog", &()))
        .call()
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

#[tokio::test]
async fn verified_read_publication_and_inflight_cancellation_use_the_real_catalog() {
    let mut owned = server()
        .build_instance(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await
        .unwrap();
    let app_wasm = std::fs::read(
        std::env::var_os("BLOB_REFERENCE_APP")
            .expect("set BLOB_REFERENCE_APP to the built product Wasm"),
    )
    .unwrap();
    let url = owned.instance_mut().make_live(None).await;
    let draft = declaration();
    for (scenario, stages) in [
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
    ] {
        // Each case owns a fresh installation, not a reset of an exposed root.
        let app = owned.instance().create_canister().await;
        owned.instance().add_cycles(app, 10_000_000_000_000).await;
        let mut limits = issuer_limits();
        limits.manifest.max_content_bytes = draft.bytes.try_into().unwrap();
        limits.manifest.max_chunks = draft.chunks.len().try_into().unwrap();
        limits.logical_bytes = draft.bytes.try_into().unwrap();
        limits.exposure_bytes = draft.bytes.try_into().unwrap();
        let (owner, trust_root) =
            install_issuer(&owned, Principal::from_slice(app.as_slice()), limits).await;
        owned
            .instance()
            .install_canister(
                app,
                app_wasm.clone(),
                frame(
                    "app_init",
                    &AppInstallation {
                        blob: Principal::from_slice(owner.as_slice()),
                        users: vec![signer(2)],
                    },
                ),
                None,
            )
            .await;
        let user = agent(url.as_str(), &trust_root, 2);
        assert_eq!(
            app_update(&user, app, "app_create", &draft)
                .await
                .unwrap()
                .phase,
            Phase::Draft
        );
        let directory = certificate_directory(&format!("publication-{scenario}-test"), &trust_root);
        let claim = retain_proof(&directory, url.as_str(), owner, draft.root, &trust_root).await;
        let original_packet = std::fs::read(claim.join("request.cbor")).unwrap();
        let original_dispatch = std::fs::read(claim.join("dispatch.claim")).unwrap();
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
                "--scenario",
                scenario,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);
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
                        let bytes = user.query(&app, "app_draft").with_arg(frame("app_draft", &draft.id))
                            .call().await.unwrap();
                        let saved = wire::decode::<Result<DraftView, AppError>>(&bytes, wire::Encoding::Frame { method: wire::method_id("app_draft"), direction: wire::Direction::Response }, RPC_LIMITS).unwrap().unwrap();
                        assert_eq!(saved.input, draft);
                        assert_eq!(saved.phase, Phase::Draft);
                        before_read = Some(owned.instance().get_stable_memory(owner).await);
                        b'S'
                    }
                    b'P' => {
                        let published = app_update(&user, app, "app_publish", &draft.id).await;
                        if matches!(scenario, "success" | "reconcile") {
                            assert_eq!(published.unwrap().phase, Phase::Published);
                            assert_eq!(catalog(&user, app).await[0].input, draft);
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
            assert_eq!(owned.instance().get_stable_memory(owner).await, before);
        }
        assert_eq!(
            std::fs::read(claim.join("request.cbor")).unwrap(),
            original_packet
        );
        assert_eq!(
            std::fs::read(claim.join("dispatch.claim")).unwrap(),
            original_dispatch
        );
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
        let bytes = user
            .query(&app, "app_draft")
            .with_arg(frame("app_draft", &draft.id))
            .call()
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
        // Simulator inspection as the application authority; application operations
        // above use the real user signature and the application's outgoing calls.
        let bytes = owned
            .instance()
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
    owned.close().await;
}
