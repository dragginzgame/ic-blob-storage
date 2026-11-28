//! Installed candidate callbacks and owned loopback transfer; no Caffeine peer or live upload.

#![forbid(unsafe_code)]

mod issuance;

use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::{Error, Limits, Root, UploadRequest},
    protocol::{Command, Inspection, Installation, Observation, Outcome, RPC_LIMITS},
};
use ic_blob_storage_tests::support_tests as support;
use ic_platform_protocol::{caffeine as provider, candid::Limits as ProviderLimits};
use ic_platform_types::Principal;
use ic_testkit::pocket_ic::{CanisterId, PocketIcBuilder, RejectCode, nonblocking::PocketIc};
use ic_wire as wire;
use serde::Serialize;
use std::{path::Path, time::Duration};

const LIVE: &str = "_immutableObjectStorageBlobsAreLive";
const LIST: &str = "_immutableObjectStorageBlobsToDelete";
const ACK: &str = "_immutableObjectStorageConfirmBlobDeletion";
// Independent wire schema: type 0 = vec type 1, type 1 = vec nat8;
// exactly one argument of type 0. Kept literal, outside the supplier encoder.
const ROOTS_HEADER: &[u8] = b"DIDL\x02\x6d\x01\x6d\x7b\x01\x00";

const PROVIDER_LIMITS: ProviderLimits = ProviderLimits {
    bytes: 4096,
    header: 512,
    types: 8,
    work: 65_536,
    allocation: 65_536,
    vector: 4096,
    skipping: 65_536,
};

fn actor(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}

fn caller(n: u8) -> CanisterId {
    CanisterId::from_slice(actor(n).as_slice())
}

fn frame<T: Serialize>(method: &str, value: &T) -> Vec<u8> {
    ic_blob_storage::protocol::frame::encode(
        value,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

fn wasm() -> Vec<u8> {
    std::fs::read(std::env::var_os("BLOB_CALLBACK_PROBE").expect("set BLOB_CALLBACK_PROBE"))
        .unwrap()
}

fn input() -> Installation {
    Installation {
        application: actor(1),
        manager: actor(9),
        limits: Limits {
            manifest: ManifestLimits {
                max_content_bytes: 100.try_into().unwrap(),
                max_chunks: 1.try_into().unwrap(),
                max_headers: 2.try_into().unwrap(),
                max_header_bytes: 100.try_into().unwrap(),
            },
            uploads: 64.try_into().unwrap(),
            references: 64.try_into().unwrap(),
            logical_bytes: 100.try_into().unwrap(),
            exposure_bytes: 100.try_into().unwrap(),
        },
    }
}

fn grant(n: u128, root: Root, uploader: u8) -> Command {
    Command::Authorize {
        request: UploadRequest {
            id: n.try_into().unwrap(),
            first_reference: n.try_into().unwrap(),
            root,
            uploader: actor(uploader),
            bytes: 1,
            expires_at: u64::MAX,
        },
        chunks: vec![root],
        headers: vec![],
    }
}

async fn update(
    pic: &PocketIc,
    owner: CanisterId,
    who: u8,
    command: Command,
) -> Result<Outcome, Error> {
    let bytes = pic
        .update_call(
            owner,
            caller(who),
            "blob_update",
            frame("blob_update", &command),
        )
        .await
        .unwrap();

    ic_blob_storage::protocol::frame::decode(
        &bytes,
        wire::Encoding::Frame {
            method: wire::method_id("blob_update"),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

async fn usage(pic: &PocketIc, owner: CanisterId) -> Observation {
    let bytes = pic
        .query_call(
            owner,
            caller(1),
            "blob_query",
            frame("blob_query", &Inspection::Usage),
        )
        .await
        .unwrap();

    ic_blob_storage::protocol::frame::decode::<Result<Observation, Error>>(
        &bytes,
        wire::Encoding::Frame {
            method: wire::method_id("blob_query"),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
    .unwrap()
}

fn roots(values: &[Root]) -> Vec<u8> {
    // These fixtures include the 65-root refusal; all counts fit one LEB byte.
    assert!(values.len() < 128);
    let mut bytes = ROOTS_HEADER.to_vec();
    bytes.push(u8::try_from(values.len()).unwrap());
    for root in values {
        bytes.push(32);
        bytes.extend_from_slice(root);
    }

    bytes
}

async fn liveness(pic: &PocketIc, owner: CanisterId, values: &[Root]) -> Vec<bool> {
    let bytes = pic
        .query_call(owner, CanisterId::anonymous(), LIVE, roots(values))
        .await
        .unwrap();

    provider::decode_liveness(&bytes, PROVIDER_LIMITS).unwrap()
}

async fn candidates(pic: &PocketIc, owner: CanisterId) -> Vec<Vec<u8>> {
    let bytes = pic
        .query_call(owner, caller(7), LIST, provider::encode_empty(32).unwrap())
        .await
        .unwrap();
    let result = provider::decode_roots(&bytes, PROVIDER_LIMITS).unwrap();
    let expected: Vec<Root> = result
        .iter()
        .map(|root| root.as_slice().try_into().unwrap())
        .collect();
    assert_eq!(bytes, roots(&expected), "exact vec blob wire profile");

    result
}

async fn ack(pic: &PocketIc, owner: CanisterId, values: &[Root]) {
    let bytes = pic
        .update_call(owner, caller(7), ACK, roots(values))
        .await
        .unwrap();
    provider::decode_empty(&bytes, PROVIDER_LIMITS).unwrap();
}

async fn install(pic: &PocketIc) -> CanisterId {
    let owner = pic
        .create_canister_with_settings(Some(caller(9)), None)
        .await;
    pic.add_cycles(owner, 10_000_000_000_000).await;
    pic.install_canister(
        owner,
        wasm(),
        frame("callback_init", &(input(), actor(7))),
        Some(caller(9)),
    )
    .await;

    owner
}

#[tokio::test]
async fn callbacks_share_retention_preserve_acknowledgments_and_never_settle() {
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 180).await;
    let pic = owned.client();
    let owner = install(pic).await;
    let shared = [1; 32];
    let reserved = [2; 32];
    let retired = [3; 32];
    let unknown = [4; 32];
    update(pic, owner, 1, grant(1, shared, 2)).await.unwrap();
    update(pic, owner, 2, Command::Register(shared))
        .await
        .unwrap();
    update(pic, owner, 1, grant(2, shared, 3)).await.unwrap();
    update(pic, owner, 1, grant(3, reserved, 2)).await.unwrap();
    update(pic, owner, 1, grant(4, retired, 2)).await.unwrap();
    update(pic, owner, 1, Command::Release(4.try_into().unwrap()))
        .await
        .unwrap();
    update(pic, owner, 1, Command::Release(1.try_into().unwrap()))
        .await
        .unwrap();
    assert_eq!(
        liveness(pic, owner, &[shared, reserved, retired, unknown, shared]).await,
        vec![true; 5]
    );
    assert!(candidates(pic, owner).await.is_empty());
    let before = support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await;
    for who in [CanisterId::anonymous(), caller(1), caller(2), caller(9)] {
        assert!(
            pic.query_call(owner, who, LIST, provider::encode_empty(32).unwrap())
                .await
                .is_err()
        );
        assert!(pic.update_call(owner, who, ACK, roots(&[])).await.is_err());
    }
    assert!(
        pic.update_call(owner, caller(7), ACK, roots(&[shared]))
            .await
            .is_err()
    );
    assert_eq!(
        support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await,
        before
    );
    update(pic, owner, 1, Command::Release(2.try_into().unwrap()))
        .await
        .unwrap();
    assert_eq!(
        liveness(pic, owner, &[shared, unknown, shared, reserved, retired]).await,
        vec![false, true, false, true, true]
    );
    assert_eq!(candidates(pic, owner).await, vec![shared.to_vec()]);
    let before = support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await;
    for bad in [unknown, reserved, retired] {
        assert!(
            pic.update_call(owner, caller(7), ACK, roots(&[shared, bad]))
                .await
                .is_err()
        );
        assert_eq!(
            support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await,
            before
        );
    }
    assert!(
        update(pic, owner, 2, Command::Register(shared))
            .await
            .is_err()
    );
    assert!(update(pic, owner, 1, grant(5, shared, 2)).await.is_err());
    let outstanding = usage(pic, owner).await;
    assert!(
        matches!(&outstanding, Observation::Usage(u) if u.live_bytes == 0 && u.unresolved_bytes == 1)
    );
    // Treat this successful reply as lost, then repeat by the exact original root.
    ack(pic, owner, &[shared, shared]).await;
    assert!(candidates(pic, owner).await.is_empty());
    assert_eq!(usage(pic, owner).await, outstanding);
    update(pic, owner, 9, Command::Drain(1)).await.unwrap();
    let saved = support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await;
    pic.stop_canister(owner, Some(caller(9))).await.unwrap();
    pic.upgrade_canister(
        owner,
        wasm(),
        frame("callback_upgrade", &()),
        Some(caller(9)),
    )
    .await
    .unwrap();
    pic.start_canister(owner, Some(caller(9))).await.unwrap();
    ack(pic, owner, &[shared]).await;
    assert!(candidates(pic, owner).await.is_empty());
    assert_eq!(usage(pic, owner).await, outstanding);
    assert_eq!(
        liveness(pic, owner, &[shared, unknown]).await,
        vec![false, true]
    );
    assert_eq!(
        support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await,
        saved
    );
    assert!(
        pic.query_call(owner, caller(9), LIST, provider::encode_empty(32).unwrap())
            .await
            .is_err()
    );
    support::close(server, owned).await;
}

#[tokio::test]
async fn callback_decoding_refuses_malformed_over_budget_and_wrong_profile_without_writes() {
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 180).await;
    let pic = owned.client();
    let owner = install(pic).await;
    let root = [42; 32];
    update(pic, owner, 1, grant(1, root, 2)).await.unwrap();
    update(pic, owner, 2, Command::Register(root))
        .await
        .unwrap();
    update(pic, owner, 1, Command::Release(1.try_into().unwrap()))
        .await
        .unwrap();
    let before = support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await;
    let mut trailing = roots(&[root]);
    trailing.push(0);
    let malformed = [
        vec![],
        vec![0; 4097],
        roots(&[root; 65]),
        trailing,
        [ROOTS_HEADER, &[1, 31], &[1; 31]].concat(),
        [ROOTS_HEADER, &[1, 33], &[1; 33]].concat(),
        // One vec text argument with one ten-byte string.
        b"DIDL\x01\x6d\x71\x01\x00\x01\x0anot a root".to_vec(),
        // Same two vector types, but two arguments: vec blob and bool (0x7e).
        [
            b"DIDL\x02\x6d\x01\x6d\x7b\x02\x00\x7e\x01\x20".as_slice(),
            &root,
            &[1],
        ]
        .concat(),
    ];
    for bytes in malformed {
        let error = pic
            .query_call(owner, caller(7), LIVE, bytes.clone())
            .await
            .unwrap_err();
        assert!(matches!(
            error.reject_code,
            RejectCode::CanisterError | RejectCode::CanisterReject
        ));
        assert!(pic.update_call(owner, caller(7), ACK, bytes).await.is_err());
        assert_eq!(
            support::blob_state(pic, owner, caller(1), &[actor(2), actor(3)]).await,
            before
        );
        assert_eq!(candidates(pic, owner).await, vec![root.to_vec()]);
    }
    assert!(
        pic.query_call(owner, caller(7), LIST, roots(&[]))
            .await
            .is_err()
    );
    assert_eq!(liveness(pic, owner, &[root; 64]).await, vec![false; 64]);
    ack(pic, owner, &[root; 64]).await;
    assert!(candidates(pic, owner).await.is_empty());
    support::close(server, owned).await;
}
