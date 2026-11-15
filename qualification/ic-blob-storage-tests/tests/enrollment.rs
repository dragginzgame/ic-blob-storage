//! Installed single-use enrollment custody against explicitly synthetic Cashier replies.

use ic_blob_storage_enrollment_probe::protocol::{
    Error, Installation, LIMITS, Outcome, REPLY_BYTES, View,
};
use ic_platform::candid_codec::{
    Nat,
    decoder::{Arity, Limits, decode_args},
};
use ic_platform_types::Principal;
use ic_testkit::{
    pic::{
        PocketIcStartupConfig,
        fixtures::{
            RawReplyLimits, RawReplyMethod, ReplyMode, held::HeldReplyFixture, raw_reply_canister,
        },
    },
    pocket_ic::{
        CanisterId, PocketIcBuilder, common::rest::BlobCompression, nonblocking::PocketIc,
    },
};
use ic_wire::native;
use serde::Serialize;
use std::{path::Path, time::Duration};

const OPERATOR: &[u8] = &[1, 1];

fn actor() -> CanisterId {
    CanisterId::from_slice(OPERATOR)
}

fn request<T: Serialize>(method: &str, input: &T) -> Vec<u8> {
    native::encode_request(native::method_id(method), input, LIMITS).unwrap()
}

fn response(method: &str, bytes: &[u8]) -> Result<View, Error> {
    native::decode_response(native::method_id(method), bytes, LIMITS).unwrap()
}

fn wasm() -> Vec<u8> {
    std::fs::read(std::env::var_os("BLOB_ENROLLMENT_PROBE").expect("set BLOB_ENROLLMENT_PROBE"))
        .unwrap()
}

async fn install(pic: &PocketIc, cashier: CanisterId) -> CanisterId {
    let owner = pic.create_canister_with_settings(Some(actor()), None).await;
    pic.add_cycles(owner, 10_000_000_000_000).await;
    let config = Installation {
        operator: Principal::from_slice(OPERATOR),
        cashier: Principal::from_slice(cashier.as_slice()),
        target_balance: 1_000_000_000,
    };
    pic.install_canister(
        owner,
        wasm(),
        request("enrollment_init", &config),
        Some(actor()),
    )
    .await;
    owner
}

async fn inspect(pic: &PocketIc, owner: CanisterId) -> View {
    response(
        "enrollment_inspect",
        &pic.update_call(
            owner,
            actor(),
            "enrollment_inspect",
            request("enrollment_inspect", &()),
        )
        .await
        .unwrap(),
    )
    .unwrap()
}

async fn attempt(pic: &PocketIc, owner: CanisterId) -> View {
    response(
        "enrollment_attempt",
        &pic.update_call(
            owner,
            actor(),
            "enrollment_attempt",
            request("enrollment_attempt", &()),
        )
        .await
        .unwrap(),
    )
    .unwrap()
}

async fn upgrade(pic: &PocketIc, owner: CanisterId) {
    pic.stop_canister(owner, Some(actor())).await.unwrap();
    pic.upgrade_canister(
        owner,
        wasm(),
        request("enrollment_upgrade", &()),
        Some(actor()),
    )
    .await
    .unwrap();
    pic.start_canister(owner, Some(actor())).await.unwrap();
}

fn server() -> PocketIcStartupConfig {
    let directory = std::env::var_os("BLOB_TEST_SERVER").expect("set BLOB_TEST_SERVER");
    PocketIcStartupConfig::Spawn {
        server_binary: ic_testkit::pic::server_assets::check(Path::new(&directory)).unwrap(),
        timeout: Duration::from_secs(60),
        server_hard_ttl: Some(Duration::from_secs(180)),
        server_idle_ttl: None,
        server_output_files: None,
    }
}

ic_platform::candid_record! {
    struct TopUp { 2504008430 => target_balance: Option<Nat>, 2707029165 => account: Option<Principal> }
}

#[tokio::test]
async fn one_attempt_survives_concurrency_revocation_lost_reply_and_upgrade() {
    let owned = server()
        .build_instance(PocketIcBuilder::new().with_application_subnet())
        .await
        .unwrap();
    let pic = owned.instance();
    let cashier = pic.create_canister().await;
    pic.add_cycles(cashier, 10_000_000_000_000).await;
    // The second slot witnesses an accidental retry, including one after upgrade.
    let methods = [RawReplyMethod {
        name: "account_top_up_v1",
        mode: ReplyMode::Update,
        reply: b"raw provider observation",
    }; 2];
    let mut fixture = HeldReplyFixture::new(
        pic,
        cashier,
        &methods,
        RawReplyLimits {
            methods: 2,
            reply_bytes: 100,
            method_name_bytes: 40,
        },
        50_000_000_000,
        100_000,
    )
    .await
    .unwrap();
    pic.install_canister(cashier, fixture.wasm().to_vec(), vec![], None)
        .await;
    let owner = install(pic, cashier).await;
    let ready = inspect(pic, owner).await;
    assert_eq!(ready.owner.as_slice(), owner.as_slice());
    assert_eq!((ready.attempted_at, &ready.outcome), (None, &None));
    let (args,): (Option<TopUp>,) = decode_args(
        ready.arguments.as_slice(),
        Limits {
            bytes: 1024,
            header: 512,
            types: 16,
            work: 4096,
            allocation: 4096,
        },
        Arity::Exact,
    )
    .unwrap();
    let args = args.unwrap();
    assert_eq!(args.account, Some(ready.owner));
    assert_eq!(args.target_balance, Some(Nat::from(1_000_000_000u64)));
    upgrade(pic, owner).await;
    assert_eq!(inspect(pic, owner).await, ready);
    let memory = pic.get_stable_memory(owner).await;
    for method in ["enrollment_inspect", "enrollment_attempt"] {
        let denied = pic
            .update_call(owner, CanisterId::anonymous(), method, request(method, &()))
            .await
            .unwrap();
        assert_eq!(response(method, &denied), Err(Error::Denied));
    }
    assert!(
        pic.update_call(
            owner,
            actor(),
            "enrollment_attempt",
            request("wrong_method", &())
        )
        .await
        .is_err()
    );
    assert_eq!(pic.get_stable_memory(owner).await, memory);
    let pending = pic
        .submit_call(
            owner,
            actor(),
            "enrollment_attempt",
            request("enrollment_attempt", &()),
        )
        .await
        .unwrap();
    let mut held = vec![];
    for _ in 0..30 {
        pic.tick().await;
        held = fixture.pending(pic).await.unwrap();
        if !held.is_empty() {
            break;
        }
    }
    assert_eq!(held.len(), 1);
    let uncertain = inspect(pic, owner).await;
    let pending_image = pic.get_stable_memory(owner).await;
    assert!(uncertain.attempted_at.is_some() && uncertain.outcome.is_none());
    assert_eq!(attempt(pic, owner).await, uncertain);
    assert!(
        pic.upgrade_canister(
            owner,
            wasm(),
            request("enrollment_upgrade", &()),
            Some(actor())
        )
        .await
        .is_err()
    );
    // Auth changes after dispatch cannot discard the result, but do withhold its reply.
    let replacement = CanisterId::from_slice(&[2, 1]);
    pic.set_controllers(owner, Some(actor()), vec![replacement])
        .await
        .unwrap();
    fixture.release(pic, &held[0]).await.unwrap();
    assert_eq!(
        response(
            "enrollment_attempt",
            &pic.await_call(pending).await.unwrap()
        ),
        Err(Error::Denied)
    );
    pic.set_controllers(owner, Some(replacement), vec![actor()])
        .await
        .unwrap();
    let retained = inspect(pic, owner).await;
    assert_eq!(
        retained.outcome,
        Some(Outcome::Reply(
            ic_wire::Bytes::new(methods[0].reply.to_vec()).unwrap()
        ))
    );
    assert_eq!(retained.attempted_at, uncertain.attempted_at);
    // Discard the ingress result and recover through inspection; replay cannot resubmit.
    let _ = attempt(pic, owner).await;
    upgrade(pic, owner).await;
    assert_eq!(inspect(pic, owner).await, retained);
    assert_eq!(attempt(pic, owner).await, retained);
    // Inject the same owner's committed pre-callback image after draining. This
    // models a result write that never committed; it is not snapshot continuity
    // qualification or permission to restore an earlier Ready image in production.
    pic.set_stable_memory(owner, pending_image, BlobCompression::NoCompression)
        .await;
    upgrade(pic, owner).await;
    assert_eq!(inspect(pic, owner).await, uncertain);
    assert_eq!(attempt(pic, owner).await, uncertain);
    for _ in 0..10 {
        pic.tick().await;
    }
    assert!(fixture.pending(pic).await.unwrap().is_empty());
}

#[tokio::test]
async fn malformed_oversized_and_rejected_replies_never_reopen_the_attempt() {
    let owned = server()
        .build_instance(PocketIcBuilder::new().with_application_subnet())
        .await
        .unwrap();
    let pic = owned.instance();
    for bytes in [
        vec![],
        b"not candid".to_vec(),
        vec![0xff; REPLY_BYTES],
        vec![0xff; REPLY_BYTES + 1],
    ] {
        let cashier = pic.create_canister().await;
        pic.add_cycles(cashier, 10_000_000_000_000).await;
        let reply = raw_reply_canister(
            &[RawReplyMethod {
                name: "account_top_up_v1",
                mode: ReplyMode::Update,
                reply: &bytes,
            }],
            RawReplyLimits {
                methods: 1,
                reply_bytes: REPLY_BYTES + 1,
                method_name_bytes: 20,
            },
        )
        .unwrap();
        pic.install_canister(cashier, reply, vec![], None).await;
        let owner = install(pic, cashier).await;
        let retained = attempt(pic, owner).await;
        assert_eq!(
            retained.outcome,
            Some(if bytes.len() <= REPLY_BYTES {
                Outcome::Reply(ic_wire::Bytes::new(bytes).unwrap())
            } else {
                Outcome::ReplyTooLarge(bytes.len() as u64)
            })
        );
        upgrade(pic, owner).await;
        assert_eq!(attempt(pic, owner).await, retained);
    }
    // Empty target has no method; an IC rejection is not provider nonexecution proof.
    let cashier = pic.create_canister().await;
    let owner = install(pic, cashier).await;
    let retained = attempt(pic, owner).await;
    assert!(matches!(retained.outcome, Some(Outcome::Rejected(_))));
    upgrade(pic, owner).await;
    assert_eq!(attempt(pic, owner).await, retained);
}

#[tokio::test]
async fn unavailable_cashier_and_foreign_state_never_reopen_admission() {
    let owned = server()
        .build_instance(PocketIcBuilder::new().with_application_subnet())
        .await
        .unwrap();
    let pic = owned.instance();
    let cashier = pic.create_canister().await;
    pic.add_cycles(cashier, 10_000_000_000_000).await;
    pic.stop_canister(cashier, None).await.unwrap();
    let owner = install(pic, cashier).await;
    let retained = attempt(pic, owner).await;
    assert!(matches!(retained.outcome, Some(Outcome::Rejected(_))));
    upgrade(pic, owner).await;
    assert_eq!(attempt(pic, owner).await, retained);
    let foreign = install(pic, cashier).await;
    let image = pic.get_stable_memory(owner).await;
    pic.set_stable_memory(foreign, image, BlobCompression::NoCompression)
        .await;
    assert!(
        pic.upgrade_canister(
            foreign,
            wasm(),
            request("enrollment_upgrade", &()),
            Some(actor())
        )
        .await
        .is_err()
    );
    let missing = install(pic, cashier).await;
    pic.set_stable_memory(missing, vec![], BlobCompression::NoCompression)
        .await;
    assert!(
        pic.upgrade_canister(
            missing,
            wasm(),
            request("enrollment_upgrade", &()),
            Some(actor())
        )
        .await
        .is_err()
    );
}
