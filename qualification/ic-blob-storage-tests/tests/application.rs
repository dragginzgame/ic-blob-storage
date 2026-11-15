//! The real reference application owns publication and cleanup across Blob calls.

use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::{Error as BlobError, Limits},
    protocol::{Command, Installation as BlobInstallation, Outcome, RPC_LIMITS},
};
use ic_blob_storage_reference_app::protocol::{DraftInput, DraftView, Error, Installation, Phase};
use ic_platform_types::Principal;
use ic_testkit::{
    pic::PocketIcStartupConfig,
    pocket_ic::{
        CanisterId, PocketIcBuilder, RejectCode, common::rest::BlobCompression,
        nonblocking::PocketIc,
    },
};
use ic_wire::native;
use serde::{Serialize, de::DeserializeOwned};
use std::{path::Path, time::Duration};

fn encode<T: Serialize>(method: &str, value: &T) -> Vec<u8> {
    native::encode_request(native::method_id(method), value, RPC_LIMITS).unwrap()
}

fn decode<R: DeserializeOwned>(method: &str, bytes: &[u8]) -> R {
    native::decode_response(native::method_id(method), bytes, RPC_LIMITS).unwrap()
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
    let directory = std::env::var_os("BLOB_TEST_SERVER").expect("set BLOB_TEST_SERVER");
    let server = ic_testkit::pic::server_assets::check(Path::new(&directory)).unwrap();
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: server,
        timeout: Duration::from_secs(60),
        server_hard_ttl: Some(Duration::from_secs(180)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_application_subnet()
            .with_application_subnet(),
    )
    .await
    .unwrap();
    let pic = owned.instance();
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
    pic.install_canister(
        blob,
        std::fs::read(std::env::var_os("BLOB_WASM").unwrap()).unwrap(),
        encode("blob_init", &config),
        None,
    )
    .await;
    let wasm = std::fs::read(std::env::var_os("BLOB_REFERENCE_APP").unwrap()).unwrap();
    pic.install_canister(
        app,
        wasm.clone(),
        encode(
            "app_init",
            &Installation {
                blob: principal(blob),
                users: vec![principal(alice), principal(bob)],
            },
        ),
        None,
    )
    .await;

    let first = input(1, "portrait", 7);
    // Invalid drafts must not consume their lifetime ID or reach Blob admission.
    let initial_app = pic.get_stable_memory(app).await;
    let initial_blob = pic.get_stable_memory(blob).await;
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
        assert_eq!(pic.get_stable_memory(app).await, initial_app);
        assert_eq!(pic.get_stable_memory(blob).await, initial_blob);
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
    let before = pic.get_stable_memory(app).await;
    let refused = pic
        .upgrade_canister(app, wasm.clone(), encode("app_post_upgrade", &()), None)
        .await
        .unwrap_err();
    assert!(
        refused
            .reject_message
            .contains("stop and drain application calls before upgrade")
    );
    let resumed: Result<DraftView, Error> =
        decode("app_create", &pic.await_call(pending).await.unwrap());
    assert_eq!(resumed, created);
    assert_eq!(pic.get_stable_memory(app).await, before);

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
    assert_eq!(pic.get_stable_memory(app).await, before);
    pic.start_canister(app, None).await.unwrap();
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
    let before = pic.get_stable_memory(blob).await;
    let published: Result<DraftView, Error> = call(pic, app, alice, "app_publish", &first.id).await;
    assert_eq!(published.as_ref().unwrap().phase, Phase::Published);
    assert_eq!(
        pic.get_stable_memory(blob).await,
        before,
        "publication only inspects Blob"
    );
    assert_eq!(catalog(pic, app, alice).await, vec![published.unwrap()]);
    assert!(catalog(pic, app, bob).await.is_empty());

    // Each application hook admits its own bounded Wire frame before touching live state.
    let before_app = pic.get_stable_memory(app).await;
    let before_blob = pic.get_stable_memory(blob).await;
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
            assert_eq!(pic.get_stable_memory(app).await, before_app);
            assert_eq!(pic.get_stable_memory(blob).await, before_blob);
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
    let before = pic.get_stable_memory(blob).await;
    let cleaned: Result<DraftView, Error> = call(pic, app, alice, "app_cancel", &expired.id).await;
    assert_eq!(cleaned.unwrap().phase, Phase::Cleaned);
    assert_eq!(pic.get_stable_memory(blob).await, before);
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

    // Inject invalid state only into disposable recipients. This is rejection
    // qualification, not a supported restore or repair path for the original app.
    let original_app = pic.get_stable_memory(app).await;
    let original_blob = pic.get_stable_memory(blob).await;
    for (case, bytes) in [
        ("missing", vec![]),
        ("invalid storage", vec![0; 65_536]),
        ("foreign installation", original_app.clone()),
    ] {
        let recipient = pic.create_canister_on_subnet(None, None, subnets[0]).await;
        pic.install_canister(
            recipient,
            wasm.clone(),
            encode(
                "app_init",
                &Installation {
                    blob: principal(blob),
                    users: vec![principal(alice), principal(bob)],
                },
            ),
            None,
        )
        .await;
        pic.stop_canister(recipient, None).await.unwrap();
        pic.set_stable_memory(recipient, bytes.clone(), BlobCompression::NoCompression)
            .await;
        assert_eq!(pic.get_stable_memory(recipient).await, bytes, "{case}");
        let refused = pic
            .upgrade_canister(
                recipient,
                wasm.clone(),
                encode("app_post_upgrade", &()),
                None,
            )
            .await
            .unwrap_err();
        assert_eq!(refused.reject_code, RejectCode::CanisterError, "{case}");
        assert_eq!(pic.get_stable_memory(recipient).await, bytes, "{case}");
        assert_eq!(pic.get_stable_memory(app).await, original_app, "{case}");
        assert_eq!(pic.get_stable_memory(blob).await, original_blob, "{case}");
    }
    assert_eq!(catalog(pic, app, bob).await[0].input, second);
    assert_eq!(
        draft(pic, app, alice, &replacement).await.phase,
        Phase::Cleaned
    );
    owned.close().await;
}
