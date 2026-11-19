//! Installed single-use enrollment custody against explicitly synthetic Cashier replies.

#![forbid(unsafe_code)]

use ic_blob_storage_enrollment_probe::protocol::{
    Error, Installation, LIMITS, Outcome, REPLY_BYTES, View,
};
use ic_blob_storage_tests::support_tests as support;
use ic_platform_protocol::{
    caffeine::{Nat, decode_top_up},
    candid::Limits,
};
use ic_platform_types::Principal;
use ic_testkit::{
    held::{Catalog, Fixture, Limits as HeldLimits, Reply},
    pocket_ic::{
        CanisterId, PocketIcBuilder, RejectCode, common::rest::BlobCompression,
        nonblocking::PocketIc,
    },
};
use ic_wire as wire;
use serde::Serialize;

fn raw_reply(bytes: &[u8]) -> Vec<u8> {
    assert!(bytes.len() <= REPLY_BYTES + 1);
    let data: String = bytes.iter().map(|b| format!("\\{b:02x}")).collect();

    wat::parse_str(format!(
        r#"(module
      (import "ic0" "msg_reply_data_append" (func $append (param i32 i32)))
      (import "ic0" "msg_reply" (func $reply))
      (memory 2) (data (i32.const 0) "{data}")
      (func (export "canister_update account_top_up_v1")
        i32.const 0 i32.const {} call $append call $reply))"#,
        bytes.len()
    ))
    .unwrap()
}

const OPERATOR: &[u8] = &[1, 1];

fn actor() -> CanisterId {
    CanisterId::from_slice(OPERATOR)
}

fn request<T: Serialize>(method: &str, input: &T) -> Vec<u8> {
    wire::encode(
        input,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Request,
        },
        LIMITS,
    )
    .unwrap()
}

fn response(method: &str, bytes: &[u8]) -> Result<View, Error> {
    wire::decode(
        bytes,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Response,
        },
        LIMITS,
    )
    .unwrap()
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

#[tokio::test]
async fn one_attempt_survives_concurrency_revocation_lost_reply_and_upgrade() {
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 180).await;
    let pic = owned.client();
    let cashier = pic.create_canister().await;
    pic.add_cycles(cashier, 10_000_000_000_000).await;
    // The second slot witnesses an accidental retry, including one after upgrade.
    let methods = [Reply {
        method: "account_top_up_v1",
        bytes: b"raw provider observation",
    }; 2];
    let catalog = Catalog::new(
        &methods,
        HeldLimits {
            slots: 2,
            reply_bytes: 100,
            name_bytes: 40,
        },
    )
    .unwrap();
    let mut fixture = Fixture::new(
        pic,
        Principal::from_slice(cashier.as_slice()),
        &catalog,
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
    let args = decode_top_up(
        ready.arguments.as_slice(),
        Limits {
            bytes: 1024,
            header: 512,
            types: 16,
            work: 4096,
            allocation: 4096,
            vector: 1024,
            skipping: 4096,
        },
    )
    .unwrap();
    let args = args.unwrap();
    assert_eq!(args.account, Some(ready.owner));
    assert_eq!(args.target_balance, Some(Nat::from(1_000_000_000u64)));
    upgrade(pic, owner).await;
    assert_eq!(inspect(pic, owner).await, ready);
    let memory = inspect(pic, owner).await;
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
    assert_eq!(inspect(pic, owner).await, memory);
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
            ic_wire::Bytes::new(methods[0].bytes.to_vec()).unwrap()
        ))
    );
    assert_eq!(retained.attempted_at, uncertain.attempted_at);
    // Discard the ingress result and recover through inspection; replay cannot resubmit.
    let _ = attempt(pic, owner).await;
    upgrade(pic, owner).await;
    assert_eq!(inspect(pic, owner).await, retained);
    assert_eq!(attempt(pic, owner).await, retained);
    // Reopen the durable state observed before delivery of the callback to this
    // disposable owner. A missing outcome must keep the original attempt spent.
    pic.set_stable_memory(owner, pending_image, BlobCompression::NoCompression)
        .await;
    upgrade(pic, owner).await;
    assert_eq!(inspect(pic, owner).await, uncertain);
    assert_eq!(attempt(pic, owner).await, uncertain);
    for _ in 0..10 {
        pic.tick().await;
    }
    assert!(fixture.pending(pic).await.unwrap().is_empty());
    support::close(server, owned).await;
}

#[tokio::test]
async fn malformed_oversized_and_rejected_replies_never_reopen_the_attempt() {
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 180).await;
    let pic = owned.client();
    for bytes in [
        vec![],
        b"not candid".to_vec(),
        vec![0xff; REPLY_BYTES],
        vec![0xff; REPLY_BYTES + 1],
    ] {
        let cashier = pic.create_canister().await;
        pic.add_cycles(cashier, 10_000_000_000_000).await;
        let reply = raw_reply(&bytes);
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
    support::close(server, owned).await;
}

#[tokio::test]
async fn unavailable_cashier_and_missing_stable_directory_never_reopen_admission() {
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 180).await;
    let pic = owned.client();
    let cashier = pic.create_canister().await;
    pic.add_cycles(cashier, 10_000_000_000_000).await;
    pic.stop_canister(cashier, None).await.unwrap();
    let owner = install(pic, cashier).await;
    let retained = attempt(pic, owner).await;
    assert!(matches!(retained.outcome, Some(Outcome::Rejected(_))));
    upgrade(pic, owner).await;
    assert_eq!(attempt(pic, owner).await, retained);
    let missing = pic.create_canister_with_settings(Some(actor()), None).await;
    pic.add_cycles(missing, 10_000_000_000_000).await;
    pic.install_canister(
        missing,
        wat::parse_str("(module)").unwrap(),
        vec![],
        Some(actor()),
    )
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
    support::close(server, owned).await;
}

// A consumed enrollment owner must refuse an incompatible service declaration.
#[tokio::test]
async fn incompatible_service_upgrade_preserves_consumed_probe() {
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 180).await;
    let pic = owned.client();
    let cashier = pic.create_canister().await;
    pic.add_cycles(cashier, 10_000_000_000_000).await;
    let hex = include_str!(
        "../../ic-blob-storage-enrollment-probe/examples/enrollment/fixtures/top-up-without-cycles.hex"
    )
    .trim();
    let refusal: Vec<_> = (0..hex.len())
        .step_by(2)
        .map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap())
        .collect();
    let peer = raw_reply(&refusal);
    pic.install_canister(cashier, peer, vec![], None).await;
    let owner = install(pic, cashier).await;
    let retained = attempt(pic, owner).await;
    assert!(retained.attempted_at.is_some());
    assert_eq!(
        retained.outcome,
        Some(Outcome::Reply(ic_wire::Bytes::new(refusal).unwrap()))
    );
    // A stopped peer makes accidental resubmission observable as a changed result.
    pic.stop_canister(cashier, None).await.unwrap();
    let status = pic.canister_status(owner, Some(actor())).await.unwrap();
    pic.stop_canister(owner, Some(actor())).await.unwrap();
    let host = std::fs::read(std::env::var_os("BLOB_WASM").expect("set BLOB_WASM")).unwrap();
    let error = pic
        .upgrade_canister(
            owner,
            host,
            request("blob_post_upgrade", &()),
            Some(actor()),
        )
        .await
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    let after = pic.canister_status(owner, Some(actor())).await.unwrap();
    assert_eq!(after.module_hash, status.module_hash);
    assert_eq!(after.settings.controllers, status.settings.controllers);
    pic.start_canister(owner, Some(actor())).await.unwrap();
    assert_eq!(inspect(pic, owner).await, retained);
    assert_eq!(attempt(pic, owner).await, retained);
    upgrade(pic, owner).await;
    assert_eq!(inspect(pic, owner).await, retained);
    assert_eq!(attempt(pic, owner).await, retained);
    support::close(server, owned).await;
}
