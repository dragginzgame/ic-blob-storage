//! Installed production-host checks. The simulator substitutes for IC execution,
//! not for the Caffeine contract; no provider methods or HTTP requests are involved.

#![forbid(unsafe_code)]

use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::{Error, Limits, ReferenceStatus, UploadRequest, UploadState},
    protocol::{
        Command, Inspection, Installation, Maintenance, ManagementView, Observation, Outcome,
        RPC_LIMITS,
    },
};
use ic_blob_storage_tests::support_tests as support;
use ic_platform_types::Principal;
use ic_testkit::pocket_ic::{CanisterId, PocketIcBuilder, nonblocking::PocketIc};
use serde::{Serialize, de::DeserializeOwned};
use std::num::NonZeroU128;

fn actor(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn pic_actor(n: u8) -> CanisterId {
    CanisterId::from_slice(actor(n).as_slice())
}
fn id(n: u128) -> NonZeroU128 {
    n.try_into().unwrap()
}
fn input() -> Installation {
    Installation {
        application: actor(1),
        manager: actor(9),
        limits: Limits {
            manifest: ManifestLimits {
                max_content_bytes: 10_000_000.try_into().unwrap(),
                max_chunks: 10.try_into().unwrap(),
                max_headers: 8.try_into().unwrap(),
                max_header_bytes: 4096.try_into().unwrap(),
            },
            uploads: 64.try_into().unwrap(),
            references: 64.try_into().unwrap(),
            logical_bytes: 10_000_000.try_into().unwrap(),
            exposure_bytes: 10_000_000.try_into().unwrap(),
        },
    }
}
fn encode<T: Serialize>(method: &str, value: &T) -> Vec<u8> {
    ic_wire::encode(
        value,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap()
}
fn decode<T: DeserializeOwned>(method: &str, bytes: &[u8]) -> T {
    ic_wire::decode(
        bytes,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
}
async fn update(
    pic: &PocketIc,
    canister: CanisterId,
    caller: u8,
    command: Command,
) -> Result<Outcome, Error> {
    decode(
        "blob_update",
        &pic.update_call(
            canister,
            pic_actor(caller),
            "blob_update",
            encode("blob_update", &command),
        )
        .await
        .unwrap(),
    )
}
async fn query(
    pic: &PocketIc,
    canister: CanisterId,
    caller: u8,
    request: Inspection,
) -> Result<Observation, Error> {
    decode(
        "blob_query",
        &pic.query_call(
            canister,
            pic_actor(caller),
            "blob_query",
            encode("blob_query", &request),
        )
        .await
        .unwrap(),
    )
}
async fn management(
    pic: &PocketIc,
    canister: CanisterId,
    caller: u8,
) -> Result<ManagementView, Error> {
    decode(
        "blob_management",
        &pic.update_call(
            canister,
            pic_actor(caller),
            "blob_management",
            encode("blob_management", &()),
        )
        .await
        .unwrap(),
    )
}
fn root() -> [u8; 32] {
    let hex = "b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";
    std::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
}
fn authorize(request: UploadRequest) -> Command {
    Command::Authorize {
        request,
        chunks: vec![request.root],
        headers: vec![],
    }
}

#[tokio::test]
async fn callers_replay_cleanup_and_upgrade_preserve_one_owner() {
    let wasm = std::fs::read(
        std::env::var_os("BLOB_WASM").expect("set BLOB_WASM to the built production artifact"),
    )
    .unwrap();
    // Explicit previous artifact for a cross-version host migration witness;
    // ordinary runs continue to exercise same-artifact reopening.
    let installed_wasm = std::env::var_os("BLOB_PREVIOUS_WASM")
        .map(|path| std::fs::read(path).unwrap())
        .unwrap_or_else(|| wasm.clone());
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 300).await;
    let pic = owned.client();
    let canister = pic
        .create_canister_with_settings(Some(pic_actor(9)), None)
        .await;
    pic.add_cycles(canister, 10_000_000_000_000).await;
    pic.install_canister(
        canister,
        installed_wasm,
        encode("blob_init", &input()),
        Some(pic_actor(9)),
    )
    .await;
    let initial = ManagementView {
        installation: Principal::from_slice(canister.as_slice()),
        configuration: input(),
        state_version: 1,
        maintenance: Maintenance {
            attempt: 0,
            active: true,
        },
    };
    assert_eq!(management(pic, canister, 9).await, Ok(initial));
    for caller in [1, 2, 3] {
        assert_eq!(management(pic, canister, caller).await, Err(Error::Denied));
    }
    assert!(
        pic.query_call(
            canister,
            pic_actor(9),
            "blob_management",
            encode("blob_management", &())
        )
        .await
        .is_err()
    );
    let valid = encode("blob_management", &());
    let mut trailing = valid.clone();
    trailing.push(0);
    for invalid in [valid[..valid.len() - 1].to_vec(), trailing] {
        assert!(
            pic.update_call(canister, pic_actor(9), "blob_management", invalid)
                .await
                .is_err()
        );
    }
    let request = UploadRequest {
        id: id(1),
        uploader: actor(2),
        root: root(),
        bytes: 3,
        first_reference: id(1),
        expires_at: u64::MAX,
    };
    assert_eq!(
        update(pic, canister, 2, authorize(request)).await,
        Err(Error::Denied)
    );
    assert!(
        matches!(update(pic, canister, 1, authorize(request)).await.unwrap(), Outcome::Upload(v) if v.state == UploadState::Reserved)
    );
    assert_eq!(
        query(pic, canister, 3, Inspection::Upload(id(1))).await,
        Err(Error::Denied)
    );
    assert_eq!(
        update(pic, canister, 3, Command::Register(root())).await,
        Err(Error::Denied)
    );
    assert_eq!(
        update(pic, canister, 2, Command::Register(root())).await,
        Ok(Outcome::Done)
    );
    let second = UploadRequest {
        id: id(2),
        first_reference: id(2),
        uploader: actor(3),
        ..request
    };
    assert!(
        matches!(update(pic, canister, 1, authorize(second)).await.unwrap(), Outcome::Upload(v) if v.state == UploadState::Attached)
    );
    // Exercise cold reopening at the durable profile's full history limit on
    // Wasm32, including wide IDs. Released history still occupies every slot.
    for n in 3..=64 {
        let retained = id(u128::MAX - n);
        let attached = UploadRequest {
            id: retained,
            first_reference: retained,
            ..request
        };
        update(pic, canister, 1, authorize(attached)).await.unwrap();
        update(pic, canister, 1, Command::Release(retained))
            .await
            .unwrap();
    }
    assert_eq!(
        update(
            pic,
            canister,
            1,
            authorize(UploadRequest {
                id: id(65),
                first_reference: id(65),
                ..request
            })
        )
        .await,
        Err(Error::Capacity)
    );
    assert_eq!(
        update(pic, canister, 3, Command::Release(id(1))).await,
        Err(Error::Denied)
    );
    update(pic, canister, 1, Command::Release(id(1)))
        .await
        .unwrap();
    let before = query(pic, canister, 1, Inspection::Usage).await.unwrap();
    assert!(
        matches!(&before, Observation::Usage(v) if v.live_bytes == 3 && v.unresolved_bytes == 3 && v.uploads == 64 && v.references == 64)
    );
    // An upgrade without draining traps in pre_upgrade and preserves the current instance.
    assert!(
        pic.upgrade_eop_canister(
            canister,
            wasm.clone(),
            encode("blob_post_upgrade", &()),
            Some(pic_actor(9))
        )
        .await
        .is_err()
    );
    assert_eq!(
        query(pic, canister, 1, Inspection::Usage).await.unwrap(),
        before
    );
    assert_eq!(
        update(pic, canister, 1, Command::Drain(1)).await,
        Err(Error::Denied)
    );
    update(pic, canister, 9, Command::Drain(1)).await.unwrap();
    let drained = ManagementView {
        maintenance: Maintenance {
            attempt: 1,
            active: false,
        },
        ..initial
    };
    assert_eq!(management(pic, canister, 9).await, Ok(drained));
    assert_eq!(
        update(pic, canister, 1, Command::Retain(root(), id(3))).await,
        Err(Error::Maintenance)
    );
    update(pic, canister, 1, Command::Retain(root(), id(1)))
        .await
        .unwrap();
    assert!(
        matches!(query(pic, canister, 1, Inspection::Reference(id(1))).await.unwrap(), Observation::Reference(v) if v.status == ReferenceStatus::Released)
    );
    // Supported local procedure: explicit drain, stop, ordinary upgrade, start, inspect, activate.
    pic.stop_canister(canister, Some(pic_actor(9)))
        .await
        .unwrap();
    assert!(
        pic.upgrade_eop_canister(
            canister,
            wasm.clone(),
            encode("blob_init", &input()),
            Some(pic_actor(9))
        )
        .await
        .is_err()
    );
    pic.upgrade_eop_canister(
        canister,
        wasm.clone(),
        encode("blob_post_upgrade", &()),
        Some(pic_actor(9)),
    )
    .await
    .unwrap();
    pic.start_canister(canister, Some(pic_actor(9)))
        .await
        .unwrap();
    assert_eq!(management(pic, canister, 9).await, Ok(drained));
    assert_eq!(
        query(pic, canister, 1, Inspection::Usage).await.unwrap(),
        before
    );
    assert_eq!(
        update(pic, canister, 1, Command::Retain(root(), id(3))).await,
        Err(Error::Maintenance)
    );
    update(pic, canister, 9, Command::Activate(1))
        .await
        .unwrap();
    assert_eq!(
        management(pic, canister, 9).await,
        Ok(ManagementView {
            maintenance: Maintenance {
                attempt: 1,
                active: true
            },
            ..initial
        })
    );
    update(pic, canister, 9, Command::Drain(2)).await.unwrap();
    assert_eq!(
        update(pic, canister, 9, Command::Activate(1)).await,
        Err(Error::MaintenanceAttempt)
    );
    update(pic, canister, 1, Command::Release(id(2)))
        .await
        .unwrap();
    pic.stop_canister(canister, Some(pic_actor(9)))
        .await
        .unwrap();
    pic.upgrade_eop_canister(
        canister,
        wasm,
        encode("blob_post_upgrade", &()),
        Some(pic_actor(9)),
    )
    .await
    .unwrap();
    pic.start_canister(canister, Some(pic_actor(9)))
        .await
        .unwrap();
    assert!(
        matches!(query(pic, canister, 1, Inspection::Usage).await.unwrap(), Observation::Usage(v) if v.live_bytes == 0 && v.unresolved_bytes == 3)
    );
    assert!(
        matches!(update(pic, canister, 1, authorize(second)).await.unwrap(), Outcome::Upload(v) if v.state == UploadState::Attached)
    );
    assert!(
        matches!(query(pic, canister, 1, Inspection::Reference(id(2))).await.unwrap(), Observation::Reference(v) if v.status == ReferenceStatus::Released)
    );
    // Truncated/foreign frames reject without changing state; known IDs convey no privilege.
    assert!(
        pic.update_call(
            canister,
            pic_actor(9),
            "blob_update",
            encode("blob_query", &Inspection::Usage)
        )
        .await
        .is_err()
    );
    assert!(
        pic.update_call(canister, pic_actor(9), "blob_update", vec![0; 65_537])
            .await
            .is_err()
    );
    assert_eq!(
        management(pic, canister, 9).await,
        Ok(ManagementView {
            maintenance: Maintenance {
                attempt: 2,
                active: false
            },
            ..initial
        })
    );
    support::close(server, owned).await;
}
