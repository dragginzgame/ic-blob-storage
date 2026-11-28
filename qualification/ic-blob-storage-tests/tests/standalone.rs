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

const POSITIONAL_LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: RPC_LIMITS.bytes,
    allocation: RPC_LIMITS.allocation,
    nodes: RPC_LIMITS.nodes,
    depth: RPC_LIMITS.depth,
};

fn encode<T: Serialize>(positional: bool, method: &str, value: &T) -> Vec<u8> {
    if positional {
        return ic_blob_storage::protocol::frame::encode_retained(
            value,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id(method),
                direction: ic_wire::Direction::Request,
            },
            POSITIONAL_LIMITS,
        )
        .unwrap();
    }
    ic_blob_storage::protocol::frame::encode(
        value,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

fn decode<T: DeserializeOwned>(positional: bool, method: &str, bytes: &[u8]) -> T {
    if positional {
        return ic_blob_storage::protocol::frame::decode_retained(
            bytes,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id(method),
                direction: ic_wire::Direction::Response,
            },
            POSITIONAL_LIMITS,
        )
        .unwrap();
    }
    ic_blob_storage::protocol::frame::decode(
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
    positional: bool,
    canister: CanisterId,
    caller: u8,
    command: Command,
) -> Result<Outcome, Error> {
    decode(
        positional,
        "blob_update",
        &pic.update_call(
            canister,
            pic_actor(caller),
            "blob_update",
            encode(positional, "blob_update", &command),
        )
        .await
        .unwrap(),
    )
}

async fn query(
    pic: &PocketIc,
    positional: bool,
    canister: CanisterId,
    caller: u8,
    request: Inspection,
) -> Result<Observation, Error> {
    decode(
        positional,
        "blob_query",
        &pic.query_call(
            canister,
            pic_actor(caller),
            "blob_query",
            encode(positional, "blob_query", &request),
        )
        .await
        .unwrap(),
    )
}

async fn management(
    pic: &PocketIc,
    positional: bool,
    canister: CanisterId,
    caller: u8,
) -> Result<ManagementView, Error> {
    decode(
        positional,
        "blob_management",
        &pic.update_call(
            canister,
            pic_actor(caller),
            "blob_management",
            encode(positional, "blob_management", &()),
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
    let positional = std::env::var_os("BLOB_PREVIOUS_WASM_POSITIONAL").is_some();
    assert!(!positional || std::env::var_os("BLOB_PREVIOUS_WASM").is_some());
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
        encode(positional, "blob_init", &input()),
        Some(pic_actor(9)),
    )
    .await;
    let initial = ManagementView {
        installation: Principal::from_slice(canister.as_slice()),
        configuration: input(),
        state_version: if positional { 1 } else { 2 },
        maintenance: Maintenance {
            attempt: 0,
            active: true,
        },
    };
    assert_eq!(management(pic, positional, canister, 9).await, Ok(initial));
    for caller in [1, 2, 3] {
        assert_eq!(
            management(pic, positional, canister, caller).await,
            Err(Error::Denied)
        );
    }
    assert!(
        pic.query_call(
            canister,
            pic_actor(9),
            "blob_management",
            encode(positional, "blob_management", &())
        )
        .await
        .is_err()
    );
    let valid = encode(positional, "blob_management", &());
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
        update(pic, positional, canister, 2, authorize(request)).await,
        Err(Error::Denied)
    );
    assert!(
        matches!(update(pic, positional, canister, 1, authorize(request)).await.unwrap(), Outcome::Upload(v) if v.state == UploadState::Reserved)
    );
    assert_eq!(
        query(pic, positional, canister, 3, Inspection::Upload(id(1))).await,
        Err(Error::Denied)
    );
    assert_eq!(
        update(pic, positional, canister, 3, Command::Register(root())).await,
        Err(Error::Denied)
    );
    assert_eq!(
        update(pic, positional, canister, 2, Command::Register(root())).await,
        Ok(Outcome::Done)
    );
    let second = UploadRequest {
        id: id(2),
        first_reference: id(2),
        uploader: actor(3),
        ..request
    };
    assert!(
        matches!(update(pic, positional, canister, 1, authorize(second)).await.unwrap(), Outcome::Upload(v) if v.state == UploadState::Attached)
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
        update(pic, positional, canister, 1, authorize(attached))
            .await
            .unwrap();
        update(pic, positional, canister, 1, Command::Release(retained))
            .await
            .unwrap();
    }
    assert_eq!(
        update(
            pic,
            positional,
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
        update(pic, positional, canister, 3, Command::Release(id(1))).await,
        Err(Error::Denied)
    );
    update(pic, positional, canister, 1, Command::Release(id(1)))
        .await
        .unwrap();
    let before = query(pic, positional, canister, 1, Inspection::Usage)
        .await
        .unwrap();
    assert!(
        matches!(&before, Observation::Usage(v) if v.live_bytes == 3 && v.unresolved_bytes == 3 && v.uploads == 64 && v.references == 64)
    );
    // An upgrade without draining traps in pre_upgrade and preserves the current instance.
    assert!(
        pic.upgrade_canister(
            canister,
            wasm.clone(),
            encode(false, "blob_post_upgrade", &()),
            Some(pic_actor(9))
        )
        .await
        .is_err()
    );
    assert_eq!(
        query(pic, positional, canister, 1, Inspection::Usage)
            .await
            .unwrap(),
        before
    );
    assert_eq!(
        update(pic, positional, canister, 1, Command::Drain(1)).await,
        Err(Error::Denied)
    );
    update(pic, positional, canister, 9, Command::Drain(1))
        .await
        .unwrap();
    let drained = ManagementView {
        maintenance: Maintenance {
            attempt: 1,
            active: false,
        },
        ..initial
    };
    assert_eq!(management(pic, positional, canister, 9).await, Ok(drained));
    assert_eq!(
        update(pic, positional, canister, 1, Command::Retain(root(), id(3))).await,
        Err(Error::Maintenance)
    );
    update(pic, positional, canister, 1, Command::Retain(root(), id(1)))
        .await
        .unwrap();
    assert!(
        matches!(query(pic, positional, canister, 1, Inspection::Reference(id(1))).await.unwrap(), Observation::Reference(v) if v.status == ReferenceStatus::Released)
    );
    // Supported local procedure: explicit drain, stop, ordinary upgrade, start, inspect, activate.
    pic.stop_canister(canister, Some(pic_actor(9)))
        .await
        .unwrap();
    assert!(
        pic.upgrade_canister(
            canister,
            wasm.clone(),
            encode(positional, "blob_init", &input()),
            Some(pic_actor(9))
        )
        .await
        .is_err()
    );
    pic.upgrade_canister(
        canister,
        wasm.clone(),
        encode(false, "blob_post_upgrade", &()),
        Some(pic_actor(9)),
    )
    .await
    .unwrap();
    pic.start_canister(canister, Some(pic_actor(9)))
        .await
        .unwrap();
    let positional = false;
    let initial = ManagementView {
        state_version: 2,
        ..initial
    };
    let drained = ManagementView {
        state_version: initial.state_version,
        ..drained
    };
    assert_eq!(management(pic, positional, canister, 9).await, Ok(drained));
    assert_eq!(
        query(pic, positional, canister, 1, Inspection::Usage)
            .await
            .unwrap(),
        before
    );
    assert_eq!(
        update(pic, positional, canister, 1, Command::Retain(root(), id(3))).await,
        Err(Error::Maintenance)
    );
    update(pic, positional, canister, 9, Command::Activate(1))
        .await
        .unwrap();
    assert_eq!(
        management(pic, positional, canister, 9).await,
        Ok(ManagementView {
            maintenance: Maintenance {
                attempt: 1,
                active: true
            },
            ..initial
        })
    );
    update(pic, positional, canister, 9, Command::Drain(2))
        .await
        .unwrap();
    assert_eq!(
        update(pic, positional, canister, 9, Command::Activate(1)).await,
        Err(Error::MaintenanceAttempt)
    );
    update(pic, positional, canister, 1, Command::Release(id(2)))
        .await
        .unwrap();
    pic.stop_canister(canister, Some(pic_actor(9)))
        .await
        .unwrap();
    pic.upgrade_canister(
        canister,
        wasm,
        encode(false, "blob_post_upgrade", &()),
        Some(pic_actor(9)),
    )
    .await
    .unwrap();
    pic.start_canister(canister, Some(pic_actor(9)))
        .await
        .unwrap();
    assert!(
        matches!(query(pic, positional, canister, 1, Inspection::Usage).await.unwrap(), Observation::Usage(v) if v.live_bytes == 0 && v.unresolved_bytes == 3)
    );
    assert!(
        matches!(update(pic, positional, canister, 1, authorize(second)).await.unwrap(), Outcome::Upload(v) if v.state == UploadState::Attached)
    );
    assert!(
        matches!(query(pic, positional, canister, 1, Inspection::Reference(id(2))).await.unwrap(), Observation::Reference(v) if v.status == ReferenceStatus::Released)
    );
    // Truncated/foreign frames reject without changing state; known IDs convey no privilege.
    assert!(
        pic.update_call(
            canister,
            pic_actor(9),
            "blob_update",
            encode(positional, "blob_query", &Inspection::Usage)
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
        management(pic, positional, canister, 9).await,
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

/// Exercise the OP layout using a prepared standalone module from tag v0.104.0.
#[tokio::test]
#[ignore = "requires the retained 0.104.0 OP artifact in BLOB_OP_WASM"]
async fn populated_op_source_survives_refused_stable_upgrade() {
    incompatible_upgrade("BLOB_OP_WASM", true).await;
}

/// A stable-memory predecessor can also have an incompatible directory format.
#[tokio::test]
#[ignore = "requires a previous stable-layout host in BLOB_INCOMPATIBLE_STABLE_WASM"]
async fn populated_stable_source_survives_refused_directory_upgrade() {
    incompatible_upgrade("BLOB_INCOMPATIBLE_STABLE_WASM", false).await;
}

async fn incompatible_upgrade(source: &str, expect_empty_stable: bool) {
    use sha2::{Digest, Sha256};

    let positional = true;
    let old = std::fs::read(std::env::var_os(source).expect(source)).unwrap();
    let expected_hash = Sha256::digest(&old).to_vec();
    let replacement = std::fs::read(std::env::var_os("BLOB_WASM").unwrap()).unwrap();
    let (server, owned) =
        support::instance(PocketIcBuilder::new().with_application_subnet(), 120).await;
    let pic = owned.client();
    let owner = Some(pic_actor(9));
    let canister = pic.create_canister_with_settings(owner, None).await;
    pic.add_cycles(canister, 10_000_000_000_000).await;
    pic.install_canister(
        canister,
        old,
        encode(positional, "blob_init", &input()),
        owner,
    )
    .await;
    let request = UploadRequest {
        id: id(1),
        first_reference: id(1),
        uploader: actor(2),
        root: root(),
        bytes: 3,
        expires_at: u64::MAX,
    };
    update(pic, positional, canister, 1, authorize(request))
        .await
        .unwrap();
    update(pic, positional, canister, 2, Command::Register(root()))
        .await
        .unwrap();
    update(pic, positional, canister, 9, Command::Drain(1))
        .await
        .unwrap();
    let usage = query(pic, positional, canister, 1, Inspection::Usage)
        .await
        .unwrap();
    assert!(matches!(usage, Observation::Usage(ref u) if u.unresolved_bytes == 3));
    let maintenance = management(pic, positional, canister, 9).await.unwrap();
    let stable = pic.get_stable_memory(canister).await;
    assert_eq!(stable.is_empty(), expect_empty_stable);
    pic.stop_canister(canister, owner).await.unwrap();
    pic.upgrade_canister(
        canister,
        replacement,
        encode(false, "blob_post_upgrade", &()),
        owner,
    )
    .await
    .expect_err("ordinary upgrade cannot adopt an incompatible layout");
    assert_eq!(
        pic.canister_status(canister, owner)
            .await
            .unwrap()
            .module_hash,
        Some(expected_hash)
    );
    assert_eq!(pic.get_stable_memory(canister).await, stable);
    pic.start_canister(canister, owner).await.unwrap();
    assert_eq!(
        management(pic, positional, canister, 9).await,
        Ok(maintenance)
    );
    assert_eq!(
        query(pic, positional, canister, 1, Inspection::Usage).await,
        Ok(usage)
    );
    support::close(server, owned).await;
}
