//! The real reference application owns publication and cleanup across Blob calls.

#![forbid(unsafe_code)]

mod install;

use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::{Error as BlobError, Limits},
    protocol::{Command, Installation as BlobInstallation, Outcome, RPC_LIMITS},
};
use ic_blob_storage_reference_app::protocol::{DraftInput, DraftView, Error, Installation, Phase};
use ic_blob_storage_tests::support_tests as support;
use ic_platform_types::Principal;
use ic_testkit::pocket_ic::{CanisterId, PocketIcBuilder, RejectCode, nonblocking::PocketIc};
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};
use std::{path::Path, time::Duration};

fn encode<T: Serialize>(method: &str, value: &T) -> Vec<u8> {
    wire::encode(
        value,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

fn decode<R: DeserializeOwned>(method: &str, bytes: &[u8]) -> R {
    wire::decode(
        bytes,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

async fn call<T: Serialize, R: DeserializeOwned>(
    pic: &PocketIc,
    target: CanisterId,
    caller: CanisterId,
    method: &str,
    request: &T,
) -> R {
    decode(
        method,
        &pic.update_call(target, caller, method, encode(method, request))
            .await
            .unwrap(),
    )
}

async fn draft(pic: &PocketIc, app: CanisterId, user: CanisterId, input: &DraftInput) -> DraftView {
    let bytes = pic
        .query_call(app, user, "app_draft", encode("app_draft", &input.id))
        .await
        .unwrap();

    decode::<Result<DraftView, Error>>("app_draft", &bytes).unwrap()
}

async fn catalog(pic: &PocketIc, app: CanisterId, user: CanisterId) -> Vec<DraftView> {
    let bytes = pic
        .query_call(app, user, "app_catalog", encode("app_catalog", &()))
        .await
        .unwrap();

    decode::<Result<Vec<DraftView>, Error>>("app_catalog", &bytes).unwrap()
}

// This fixture uses lifetime IDs 1..=17. Include unpublished and released drafts,
// plus denied/unknown outcomes in the observed application state.
async fn app_state(pic: &PocketIc, app: CanisterId, users: &[CanisterId]) -> Vec<Vec<u8>> {
    let mut state = Vec::new();
    for user in users {
        for id in 1u128..=17 {
            let id: ic_blob_storage::local::UploadId = id.try_into().unwrap();
            state.push(
                pic.query_call(app, *user, "app_draft", encode("app_draft", &id))
                    .await
                    .unwrap(),
            );
        }
    }

    state
}

fn input(id: u128, name: &str, root: u8) -> DraftInput {
    DraftInput {
        id: id.try_into().unwrap(),
        asset: name.into(),
        root: [root; 32],
        bytes: 3,
        expires_at: u64::MAX,
        chunks: vec![[root; 32]],
        headers: vec![],
    }
}

#[tokio::test]
async fn catalog_cancellation_and_recovery_use_original_references() {
    let (server, owned) = support::instance(
        PocketIcBuilder::new()
            .with_application_subnet()
            .with_application_subnet(),
        180,
    )
    .await;
    let pic = owned.client();
    let subnets = pic.topology().await.get_app_subnets();
    let app = pic.create_canister_on_subnet(None, None, subnets[0]).await;
    let blob = pic.create_canister_on_subnet(None, None, subnets[1]).await;
    let alice = CanisterId::from_slice(&[2, 1]);
    let bob = CanisterId::from_slice(&[3, 1]);
    let manager = CanisterId::from_slice(&[9, 1]);
    let principal = |id: CanisterId| Principal::from_slice(id.as_slice());
    let config = BlobInstallation {
        application: principal(app),
        manager: principal(manager),
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
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/application-installation-inputs")
        .join(format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    std::fs::create_dir_all(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    let output = directory.join("inputs");
    let prepared = tokio::time::timeout(
        Duration::from_secs(10),
        tokio::process::Command::new(std::env::var_os("BLOB_CLI").expect("set BLOB_CLI"))
            .args([
                "app",
                "prepare-installation",
                "--canister",
                &app.to_text(),
                "--blob-canister",
                &blob.to_text(),
                "--manager",
                &manager.to_text(),
                "--user",
                &alice.to_text(),
                "--user",
                &bob.to_text(),
                "--max-content-bytes",
                "100",
                "--max-chunks",
                "1",
                "--max-headers",
                "1",
                "--max-header-bytes",
                "100",
                "--uploads",
                "64",
                "--references",
                "64",
                "--logical-bytes",
                "100",
                "--exposure-bytes",
                "100",
                "--output",
            ])
            .arg(&output)
            .kill_on_drop(true)
            .output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(
        prepared.status.success(),
        "{}",
        String::from_utf8_lossy(&prepared.stderr)
    );
    let blob_init = std::fs::read(output.join("blob-init.wire")).unwrap();
    let app_init = std::fs::read(output.join("app-init.wire")).unwrap();
    assert_eq!(blob_init, encode("blob_init", &config));
    assert_eq!(
        app_init,
        encode(
            "app_init",
            &Installation {
                blob: principal(blob),
                users: vec![principal(alice), principal(bob)],
            }
        )
    );
    pic.install_canister(
        blob,
        std::fs::read(std::env::var_os("BLOB_WASM").unwrap()).unwrap(),
        blob_init,
        None,
    )
    .await;
    let wasm = std::fs::read(std::env::var_os("BLOB_REFERENCE_APP").unwrap()).unwrap();
    // Optional retained predecessor exercises stored catalog reopening across a codec update.
    let installed_app = std::env::var_os("BLOB_PREVIOUS_REFERENCE_APP")
        .map(|path| std::fs::read(path).unwrap())
        .unwrap_or_else(|| wasm.clone());
    pic.install_canister(app, installed_app, app_init, None)
        .await;

    let first = input(1, "portrait", 7);
    // Invalid drafts must not consume their lifetime ID or reach Blob admission.
    let initial_app = app_state(pic, app, &[alice, bob]).await;
    let initial_blob =
        support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await;
    for invalid in [
        DraftInput {
            asset: "é".repeat(33), // Asset names are bounded in UTF-8 bytes, not characters.
            ..first.clone()
        },
        DraftInput {
            chunks: vec![],
            ..first.clone()
        },
        DraftInput {
            headers: vec![("x".into(), "v".repeat(2048))],
            ..first.clone()
        },
    ] {
        let refused: Result<DraftView, Error> = call(pic, app, alice, "app_create", &invalid).await;
        assert_eq!(refused, Err(Error::Invalid));
        assert_eq!(app_state(pic, app, &[alice, bob]).await, initial_app);
        assert_eq!(
            support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await,
            initial_blob
        );
    }
    let denied: Result<DraftView, Error> =
        call(pic, app, CanisterId::anonymous(), "app_create", &first).await;
    assert_eq!(denied, Err(Error::Denied));
    let created: Result<DraftView, Error> = call(pic, app, alice, "app_create", &first).await;
    assert_eq!(created.as_ref().unwrap().phase, Phase::Draft);
    assert_eq!(created.as_ref().unwrap().uploader, principal(alice));
    let replay: Result<DraftView, Error> = call(pic, app, alice, "app_create", &first).await;
    assert_eq!(replay, created);

    // Reject Wasm replacement while a real cross-subnet authorization is outstanding.
    let pending = pic
        .submit_call(app, alice, "app_create", encode("app_create", &first))
        .await
        .unwrap();
    pic.tick().await;
    assert!(pic.ingress_status(pending.clone()).await.is_none());
    let before = app_state(pic, app, &[alice, bob]).await;
    let refused = pic
        .upgrade_canister(app, wasm.clone(), encode("app_post_upgrade", &()), None)
        .await
        .unwrap_err();
    // Runtime preserves the pre-upgrade trap; panic prose is not an API.
    assert_eq!(refused.reject_code, RejectCode::CanisterError);
    assert!(refused.certified);
    assert!(
        refused.reject_message.contains("canister_pre_upgrade"),
        "{refused:?}"
    );
    let resumed: Result<DraftView, Error> =
        decode("app_create", &pic.await_call(pending).await.unwrap());
    assert_eq!(resumed, created);
    assert_eq!(app_state(pic, app, &[alice, bob]).await, before);

    // Upgrade arguments cannot replace the immutable target/users or reset saved drafts.
    pic.stop_canister(app, None).await.unwrap();
    assert!(
        pic.upgrade_canister(
            app,
            wasm.clone(),
            encode(
                "app_post_upgrade",
                &Installation {
                    blob: principal(manager),
                    users: vec![principal(bob)],
                },
            ),
            None,
        )
        .await
        .is_err()
    );
    pic.start_canister(app, None).await.unwrap();
    assert_eq!(app_state(pic, app, &[alice, bob]).await, before);
    assert_eq!(
        draft(pic, app, alice, &first).await,
        created.clone().unwrap()
    );

    let changed = DraftInput {
        bytes: 4,
        ..first.clone()
    };
    let conflict: Result<DraftView, Error> = call(pic, app, alice, "app_create", &changed).await;
    assert_eq!(conflict, Err(Error::Conflict));
    for method in ["app_draft", "app_publish", "app_cancel"] {
        let denied: Result<DraftView, Error> = call(pic, app, bob, method, &first.id).await;
        assert_eq!(denied, Err(Error::Denied));
    }
    let unpublished: Result<DraftView, Error> =
        call(pic, app, alice, "app_publish", &first.id).await;
    assert_eq!(unpublished, Err(Error::NotLive));
    let wrong: Result<Outcome, BlobError> = call(
        pic,
        blob,
        bob,
        "blob_update",
        &Command::Register(first.root),
    )
    .await;
    assert_eq!(wrong, Err(BlobError::Denied));
    let registered: Result<Outcome, BlobError> = call(
        pic,
        blob,
        alice,
        "blob_update",
        &Command::Register(first.root),
    )
    .await;
    assert_eq!(registered, Ok(Outcome::Done));
    let before = support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await;
    let published: Result<DraftView, Error> = call(pic, app, alice, "app_publish", &first.id).await;
    assert_eq!(published.as_ref().unwrap().phase, Phase::Published);
    assert_eq!(
        support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await,
        before,
        "publication only inspects Blob"
    );
    assert_eq!(catalog(pic, app, alice).await, vec![published.unwrap()]);
    assert!(catalog(pic, app, bob).await.is_empty());

    // Each application hook admits its own bounded Wire frame before touching live state.
    let before_app = app_state(pic, app, &[alice, bob]).await;
    let before_blob =
        support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await;
    for (method, valid) in [
        ("app_create", encode("app_create", &input(2, "new", 7))),
        ("app_publish", encode("app_publish", &first.id)),
        ("app_cancel", encode("app_cancel", &first.id)),
        ("app_draft", encode("app_draft", &first.id)),
        ("app_catalog", encode("app_catalog", &())),
    ] {
        let mut trailing = valid;
        trailing.push(0);
        for malformed in [
            trailing,
            encode("foreign_method", &first.id),
            vec![0; RPC_LIMITS.bytes + 1],
        ] {
            let result = if matches!(method, "app_draft" | "app_catalog") {
                pic.query_call(app, alice, method, malformed).await
            } else {
                pic.update_call(app, alice, method, malformed).await
            };
            assert_eq!(result.unwrap_err().reject_code, RejectCode::CanisterError);
            assert_eq!(app_state(pic, app, &[alice, bob]).await, before_app);
            assert_eq!(
                support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await,
                before_blob
            );
        }
    }

    // A second use shares the object, but has its own immutable reference and owner.
    let second = input(2, "portrait", 7);
    let attached: Result<DraftView, Error> = call(pic, app, bob, "app_create", &second).await;
    assert!(attached.is_ok());
    let published: Result<DraftView, Error> = call(pic, app, bob, "app_publish", &second.id).await;
    assert_eq!(published.unwrap().phase, Phase::Published);
    let replacement = input(3, "portrait", 7);
    let created: Result<DraftView, Error> = call(pic, app, alice, "app_create", &replacement).await;
    assert!(created.is_ok());
    let occupied: Result<DraftView, Error> =
        call(pic, app, alice, "app_publish", &replacement.id).await;
    assert_eq!(occupied, Err(Error::Conflict));
    let cancelled: Result<DraftView, Error> = call(pic, app, alice, "app_cancel", &first.id).await;
    assert_eq!(cancelled.unwrap().phase, Phase::Cleaned);
    let published: Result<DraftView, Error> =
        call(pic, app, alice, "app_publish", &replacement.id).await;
    assert_eq!(published.unwrap().phase, Phase::Published);
    let replay: Result<DraftView, Error> = call(pic, app, alice, "app_create", &first).await;
    assert_eq!(replay.unwrap().phase, Phase::Cleaned);
    let replay: Result<DraftView, Error> = call(pic, app, alice, "app_cancel", &first.id).await;
    assert_eq!(replay.unwrap().phase, Phase::Cleaned);
    assert_eq!(catalog(pic, app, alice).await[0].input, replacement);
    assert_eq!(catalog(pic, app, bob).await[0].input, second);

    // An actual cross-subnet read is pending when cancellation is admitted locally.
    let racing = input(4, "racing", 7);
    let created: Result<DraftView, Error> = call(pic, app, alice, "app_create", &racing).await;
    assert!(created.is_ok());
    let publication = pic
        .submit_call(app, alice, "app_publish", encode("app_publish", &racing.id))
        .await
        .unwrap();
    pic.tick().await;
    assert!(pic.ingress_status(publication.clone()).await.is_none());
    assert_eq!(draft(pic, app, alice, &racing).await.phase, Phase::Draft);
    let cancellation = pic
        .submit_call(app, alice, "app_cancel", encode("app_cancel", &racing.id))
        .await
        .unwrap();
    pic.tick().await;
    assert_eq!(
        draft(pic, app, alice, &racing).await.phase,
        Phase::Cancelled
    );
    let result: Result<DraftView, Error> =
        decode("app_publish", &pic.await_call(publication).await.unwrap());
    assert_eq!(result, Err(Error::Cancelled));
    let result: Result<DraftView, Error> =
        decode("app_cancel", &pic.await_call(cancellation).await.unwrap());
    assert_eq!(result.unwrap().phase, Phase::Cleaned);

    // A rejected remote call cannot undo durable intent or cancellation. Upgrade then retry it.
    pic.stop_canister(blob, None).await.unwrap();
    let interrupted = input(5, "interrupted", 8);
    let failed: Result<DraftView, Error> = call(pic, app, alice, "app_create", &interrupted).await;
    assert_eq!(failed, Err(Error::Uncertain));
    assert_eq!(
        draft(pic, app, alice, &interrupted).await.phase,
        Phase::Draft
    );
    let failed: Result<DraftView, Error> =
        call(pic, app, alice, "app_cancel", &interrupted.id).await;
    assert_eq!(failed, Err(Error::Uncertain));
    assert_eq!(
        draft(pic, app, alice, &interrupted).await.phase,
        Phase::Cancelled
    );
    assert_eq!(catalog(pic, app, alice).await.len(), 1);
    pic.stop_canister(app, None).await.unwrap();
    pic.upgrade_canister(app, wasm.clone(), encode("app_post_upgrade", &()), None)
        .await
        .unwrap();
    pic.start_canister(app, None).await.unwrap();
    assert_eq!(
        draft(pic, app, alice, &interrupted).await.phase,
        Phase::Cancelled
    );
    assert_eq!(draft(pic, app, alice, &first).await.phase, Phase::Cleaned);
    assert_eq!(catalog(pic, app, bob).await[0].input, second);
    let refused: Result<DraftView, Error> =
        call(pic, app, alice, "app_publish", &interrupted.id).await;
    assert_eq!(refused, Err(Error::Cancelled));
    pic.start_canister(blob, None).await.unwrap();
    let cleaned: Result<DraftView, Error> =
        call(pic, app, alice, "app_cancel", &interrupted.id).await;
    assert_eq!(cleaned.unwrap().phase, Phase::Cleaned);
    let stale: Result<DraftView, Error> = call(pic, app, alice, "app_create", &interrupted).await;
    assert_eq!(stale.unwrap().phase, Phase::Cleaned);

    // Cancellation resolves the original authorization even while its first call is pending.
    let late = input(6, "late", 9);
    let creation = pic
        .submit_call(app, alice, "app_create", encode("app_create", &late))
        .await
        .unwrap();
    pic.tick().await;
    assert!(pic.ingress_status(creation.clone()).await.is_none());
    assert_eq!(draft(pic, app, alice, &late).await.phase, Phase::Draft);
    let cancellation = pic
        .submit_call(app, alice, "app_cancel", encode("app_cancel", &late.id))
        .await
        .unwrap();
    pic.tick().await;
    assert_eq!(draft(pic, app, alice, &late).await.phase, Phase::Cancelled);
    let created: Result<DraftView, Error> =
        decode("app_create", &pic.await_call(creation).await.unwrap());
    // Creation may have replied before cancellation ran in that round. Its historical
    // result must not overwrite the current cancelled disposition.
    assert_eq!(created.unwrap().input, late);
    assert_eq!(draft(pic, app, alice, &late).await.phase, Phase::Cancelled);
    // Deliberately discard cleanup's response, then recover from its durable disposition.
    let _discarded = pic.await_call(cancellation).await.unwrap();
    assert_eq!(draft(pic, app, alice, &late).await.phase, Phase::Cleaned);
    let replay: Result<DraftView, Error> = call(pic, app, alice, "app_cancel", &late.id).await;
    assert_eq!(replay.unwrap().phase, Phase::Cleaned);
    let stale: Result<Outcome, BlobError> = call(
        pic,
        blob,
        alice,
        "blob_update",
        &Command::Register(late.root),
    )
    .await;
    assert!(stale.is_err());

    // A typed expiry refusal proves this ID was never admitted; late retries cannot admit it.
    let expired = DraftInput {
        expires_at: 1,
        ..input(7, "expired", 10)
    };
    let refused: Result<DraftView, Error> = call(pic, app, alice, "app_create", &expired).await;
    assert_eq!(refused, Err(Error::Blob(BlobError::Expired)));
    let before = support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await;
    let cleaned: Result<DraftView, Error> = call(pic, app, alice, "app_cancel", &expired.id).await;
    assert_eq!(cleaned.unwrap().phase, Phase::Cleaned);
    assert_eq!(
        support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await,
        before
    );
    let replay: Result<DraftView, Error> = call(pic, app, alice, "app_create", &expired).await;
    assert_eq!(replay.unwrap().phase, Phase::Cleaned);
    let renewed = DraftInput {
        expires_at: u64::MAX,
        ..expired.clone()
    };
    let refused: Result<DraftView, Error> = call(pic, app, alice, "app_create", &renewed).await;
    assert_eq!(refused, Err(Error::Conflict));

    // An admitted grant must still be released after its deadline: expiry alone is not cleanup.
    let admitted = DraftInput {
        expires_at: pic.get_time().await.as_nanos_since_unix_epoch() + 1_000_000_000,
        ..input(8, "admitted", 11)
    };
    let created: Result<DraftView, Error> = call(pic, app, alice, "app_create", &admitted).await;
    assert!(created.is_ok());
    pic.advance_time(Duration::from_secs(2)).await;
    let cleaned: Result<DraftView, Error> = call(pic, app, alice, "app_cancel", &admitted.id).await;
    assert_eq!(cleaned.unwrap().phase, Phase::Cleaned);
    let retired: Result<Outcome, BlobError> = call(
        pic,
        blob,
        alice,
        "blob_update",
        &Command::Register(admitted.root),
    )
    .await;
    assert_eq!(retired, Err(BlobError::NotReserved));

    // Lifetime history is bounded. Exhaustion must leave inspection and cleanup usable.
    for id in 9..=16 {
        let created: Result<DraftView, Error> =
            call(pic, app, alice, "app_create", &input(id, "unused", 7)).await;
        assert!(created.is_ok());
    }
    let full: Result<DraftView, Error> =
        call(pic, app, alice, "app_create", &input(17, "excess", 7)).await;
    assert_eq!(full, Err(Error::Capacity));
    let drained: Result<Outcome, BlobError> =
        call(pic, blob, manager, "blob_update", &Command::Drain(1)).await;
    assert!(matches!(drained, Ok(Outcome::Maintenance(state)) if !state.active));
    let cleaned: Result<DraftView, Error> =
        call(pic, app, alice, "app_cancel", &replacement.id).await;
    assert_eq!(cleaned.unwrap().phase, Phase::Cleaned);
    assert!(catalog(pic, app, alice).await.is_empty());
    assert_eq!(catalog(pic, app, bob).await[0].input, second);
    pic.stop_canister(app, None).await.unwrap();
    pic.upgrade_canister(app, wasm.clone(), encode("app_post_upgrade", &()), None)
        .await
        .unwrap();
    pic.start_canister(app, None).await.unwrap();
    assert_eq!(
        draft(pic, app, alice, &replacement).await.phase,
        Phase::Cleaned
    );

    // A foreign schema and a module without stable memory must fail to
    // reopen. These use disposable recipients with separate state.
    let original_app = app_state(pic, app, &[alice, bob]).await;
    let original_blob =
        support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await;
    pic.stop_canister(app, None).await.unwrap();
    let refused = pic
        .upgrade_canister(
            app,
            std::fs::read(std::env::var_os("BLOB_WASM").unwrap()).unwrap(),
            encode("blob_post_upgrade", &()),
            None,
        )
        .await
        .unwrap_err();
    assert_eq!(refused.reject_code, RejectCode::CanisterError);
    pic.start_canister(app, None).await.unwrap();
    assert_eq!(app_state(pic, app, &[alice, bob]).await, original_app);
    assert_eq!(
        support::blob_state(pic, blob, app, &[principal(alice), principal(bob)]).await,
        original_blob
    );
    let recipient = pic.create_canister_on_subnet(None, None, subnets[0]).await;
    pic.install_canister(recipient, wat::parse_str("(module)").unwrap(), vec![], None)
        .await;
    pic.stop_canister(recipient, None).await.unwrap();
    let refused = pic
        .upgrade_canister(recipient, wasm, encode("app_post_upgrade", &()), None)
        .await
        .unwrap_err();
    assert_eq!(refused.reject_code, RejectCode::CanisterError);
    assert_eq!(catalog(pic, app, bob).await[0].input, second);
    assert_eq!(
        draft(pic, app, alice, &replacement).await.phase,
        Phase::Cleaned
    );
    support::close(server, owned).await;
}
