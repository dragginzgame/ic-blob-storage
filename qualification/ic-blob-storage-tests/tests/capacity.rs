//! Capacity and resource observations of the unmodified standalone Wasm.
//! Execution charges are cycles, not raw instruction counts or provider costs.

use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::{Error, Limits, ReferenceStatus, UploadRequest, Usage, persistence::MAX_RECORDS},
    protocol::{Command, Inspection, Installation, Observation, Outcome, RPC_LIMITS},
};
use ic_platform::{
    candid_codec::{self, Nat, decoder},
    management_canister_types::CanisterIdRecord,
};
use ic_platform_types::Principal;
use ic_testkit::{
    pic::PocketIcStartupConfig,
    pocket_ic::{
        CanisterId, PocketIcBuilder, common::rest::RawEffectivePrincipal, nonblocking::PocketIc,
    },
};
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};
use std::{num::NonZeroU128, path::Path, time::Duration};

// External management protocol projection: retain the required counter, skip other use cases.
ic_platform::candid_record! {
    struct ExecutionCharges { 1832883877 => instructions: Nat }
}
ic_platform::candid_record! {
    struct Metrics { 2122851290 => cycles_consumed: ExecutionCharges }
}

const OBJECT_BYTES: u64 = 1_048_576;

fn actor(n: u8) -> Principal {
    Principal::from_slice(&[n; 29])
}

fn id(n: u128) -> NonZeroU128 {
    (u128::MAX - n).try_into().unwrap()
}

fn encode<T: Serialize>(method: &str, request: &T) -> Vec<u8> {
    wire::encode(
        request,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

fn decode<T: DeserializeOwned>(method: &str, bytes: &[u8]) -> T {
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

struct Resources {
    execution_cycles: u128,
    balance: u128,
    stable: u128,
    heap: u128,
}

async fn resources(pic: &PocketIc, canister: CanisterId) -> Resources {
    let bytes = pic
        .query_call_with_effective_principal(
            CanisterId::management_canister(),
            RawEffectivePrincipal::CanisterId(canister.as_slice().to_vec()),
            CanisterId::anonymous(),
            "canister_metrics",
            candid_codec::encode_one(
                &CanisterIdRecord {
                    canister_id: Principal::from_slice(canister.as_slice()),
                },
                1024,
            )
            .unwrap(),
        )
        .await
        .unwrap();
    let metrics: Metrics = decoder::decode_one(
        &bytes,
        decoder::Limits {
            bytes: 4096,
            header: 1024,
            types: 32,
            work: 16_384,
            allocation: 16_384,
        },
    )
    .unwrap();
    let status = pic.canister_status(canister, None).await.unwrap();
    Resources {
        execution_cycles: metrics.cycles_consumed.instructions.0.try_into().unwrap(),
        balance: status.cycles.0.try_into().unwrap(),
        stable: status
            .memory_metrics
            .stable_memory_size
            .0
            .try_into()
            .unwrap(),
        heap: status.memory_metrics.wasm_memory_size.0.try_into().unwrap(),
    }
}

fn report(label: &str, before: &Resources, after: &Resources) {
    println!(
        "{label}\t{}\t{}\t{}\t{}",
        after
            .execution_cycles
            .checked_sub(before.execution_cycles)
            .unwrap(),
        before.balance.checked_sub(after.balance).unwrap(),
        after.stable,
        after.heap
    );
    assert!(
        after.stable <= 16 * 65_536,
        "host's granted stable-page budget"
    );
}

async fn call<T: Serialize, R: DeserializeOwned>(
    pic: &PocketIc,
    canister: CanisterId,
    caller: Principal,
    method: &str,
    request: &T,
    label: Option<&str>,
) -> R {
    let before = if label.is_some() {
        Some(resources(pic, canister).await)
    } else {
        None
    };
    // Query exports executed as updates measure the replicated inspection path.
    let result = pic
        .update_call(
            canister,
            CanisterId::from_slice(caller.as_slice()),
            method,
            encode(method, request),
        )
        .await
        .unwrap();
    if let Some(before) = before {
        report(label.unwrap(), &before, &resources(pic, canister).await);
    }
    decode(method, &result)
}

async fn usage(pic: &PocketIc, canister: CanisterId, label: Option<&str>) -> Usage {
    let result: Result<Observation, Error> = call(
        pic,
        canister,
        actor(1),
        "blob_query",
        &Inspection::Usage,
        label,
    )
    .await;
    let Observation::Usage(usage) = result.unwrap() else {
        panic!("usage reply")
    };
    usage
}

#[tokio::test]
async fn full_history_remains_inspectable_cleanable_and_upgradeable() {
    let directory = std::env::var_os("BLOB_TEST_SERVER").expect("set BLOB_TEST_SERVER");
    let server = ic_testkit::pic::server_assets::check(Path::new(&directory)).unwrap();
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: server,
        timeout: Duration::from_secs(60),
        server_hard_ttl: Some(Duration::from_secs(300)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(PocketIcBuilder::new().with_application_subnet())
    .await
    .unwrap();
    let pic = owned.instance();
    let wasm = std::fs::read(std::env::var_os("BLOB_WASM").unwrap()).unwrap();
    let config = Installation {
        application: actor(1),
        manager: actor(9),
        limits: Limits {
            manifest: ManifestLimits {
                max_content_bytes: OBJECT_BYTES.try_into().unwrap(),
                max_chunks: 1.try_into().unwrap(),
                max_headers: 1.try_into().unwrap(),
                max_header_bytes: 1.try_into().unwrap(),
            },
            uploads: MAX_RECORDS.try_into().unwrap(),
            references: MAX_RECORDS.try_into().unwrap(),
            logical_bytes: (OBJECT_BYTES * MAX_RECORDS as u64).try_into().unwrap(),
            exposure_bytes: (OBJECT_BYTES * MAX_RECORDS as u64).try_into().unwrap(),
        },
    };
    println!("operation\texecution_cycles\tbalance_debit_cycles\tstable_bytes\theap_bytes");
    for shape in ["distinct", "shared", "reserved"] {
        let canister = pic.create_canister().await;
        pic.install_canister(canister, wasm.clone(), encode("blob_init", &config), None)
            .await;
        let empty = resources(pic, canister).await;
        report(&format!("{shape}/empty"), &empty, &empty);
        let mut requests = Vec::new();
        for n in 1..=MAX_RECORDS {
            let root = [if shape == "shared" {
                1
            } else {
                u8::try_from(n).unwrap()
            }; 32];
            let request = UploadRequest {
                id: id(n as u128),
                first_reference: id(n as u128),
                uploader: actor(2),
                root,
                bytes: OBJECT_BYTES,
                expires_at: u64::MAX,
            };
            let command = Command::Authorize {
                request,
                chunks: vec![root],
                headers: vec![],
            };
            let sample = [1, 16, 32, MAX_RECORDS].contains(&n);
            let label = format!("{shape}/authorize/{n}");
            let result: Result<Outcome, Error> = call(
                pic,
                canister,
                actor(1),
                "blob_update",
                &command,
                sample.then_some(label.as_str()),
            )
            .await;
            assert!(matches!(result, Ok(Outcome::Upload(view)) if view.request == request));
            if shape == "distinct" || (shape == "shared" && n == 1) {
                let label = format!("{shape}/register/{n}");
                let result: Result<Outcome, Error> = call(
                    pic,
                    canister,
                    actor(2),
                    "blob_update",
                    &Command::Register(root),
                    sample.then_some(label.as_str()),
                )
                .await;
                assert_eq!(result, Ok(Outcome::Done));
            }
            requests.push(command);
        }
        let full = usage(pic, canister, Some(&format!("{shape}/inspect/full"))).await;
        assert_eq!(full.uploads, MAX_RECORDS);
        assert_eq!(full.references, MAX_RECORDS);
        let stored_bytes = OBJECT_BYTES
            * if shape == "shared" {
                1
            } else {
                MAX_RECORDS as u64
            };
        assert_eq!(
            full.reserved_bytes,
            if shape == "reserved" { stored_bytes } else { 0 }
        );
        assert_eq!(
            full.live_bytes,
            if shape == "reserved" { 0 } else { stored_bytes }
        );
        assert_eq!(full.unresolved_bytes, full.live_bytes);
        let extra = Command::Authorize {
            request: UploadRequest {
                id: id(65),
                first_reference: id(65),
                uploader: actor(2),
                root: [65; 32],
                bytes: OBJECT_BYTES,
                expires_at: u64::MAX,
            },
            chunks: vec![[65; 32]],
            headers: vec![],
        };
        let before = pic.get_stable_memory(canister).await;
        let refused: Result<Outcome, Error> = call(
            pic,
            canister,
            actor(1),
            "blob_update",
            &extra,
            Some(&format!("{shape}/refuse/full")),
        )
        .await;
        assert_eq!(refused, Err(Error::Capacity));
        assert_eq!(pic.get_stable_memory(canister).await, before);
        if shape != "reserved" {
            let refused: Result<Outcome, Error> = call(
                pic,
                canister,
                actor(1),
                "blob_update",
                &Command::Retain([1; 32], id(65)),
                Some(&format!("{shape}/refuse/reference")),
            )
            .await;
            assert_eq!(refused, Err(Error::Capacity));
            assert_eq!(pic.get_stable_memory(canister).await, before);
        }
        let drained: Result<Outcome, Error> = call(
            pic,
            canister,
            actor(9),
            "blob_update",
            &Command::Drain(1),
            Some(&format!("{shape}/drain/full")),
        )
        .await;
        assert!(matches!(drained, Ok(Outcome::Maintenance(state)) if !state.active));
        pic.stop_canister(canister, None).await.unwrap();
        let before_state = pic.get_stable_memory(canister).await;
        let before = resources(pic, canister).await;
        pic.upgrade_canister(
            canister,
            wasm.clone(),
            encode("blob_post_upgrade", &()),
            None,
        )
        .await
        .unwrap();
        report(
            &format!("{shape}/upgrade/full"),
            &before,
            &resources(pic, canister).await,
        );
        assert_eq!(pic.get_stable_memory(canister).await, before_state);
        pic.start_canister(canister, None).await.unwrap();
        assert_eq!(usage(pic, canister, None).await, full);
        for (index, request) in requests.iter().enumerate() {
            let replay: Result<Outcome, Error> =
                call(pic, canister, actor(1), "blob_update", request, None).await;
            let Command::Authorize { request, .. } = request else {
                unreachable!()
            };
            assert!(matches!(replay, Ok(Outcome::Upload(view)) if view.request == *request));
            let label = format!("{shape}/release/{}", index + 1);
            let released: Result<Outcome, Error> = call(
                pic,
                canister,
                actor(1),
                "blob_update",
                &Command::Release(request.first_reference),
                (index == 0 || index == MAX_RECORDS - 1).then_some(label.as_str()),
            )
            .await;
            assert_eq!(released, Ok(Outcome::Done));
            if shape == "shared" && index == 0 {
                assert_eq!(
                    usage(pic, canister, None).await,
                    full,
                    "releasing one of 64 uses must retain the shared object"
                );
            }
            let bytes = pic
                .query_call(
                    canister,
                    CanisterId::from_slice(actor(1).as_slice()),
                    "blob_query",
                    encode(
                        "blob_query",
                        &Inspection::Reference(request.first_reference),
                    ),
                )
                .await
                .unwrap();
            let observed: Result<Observation, Error> = decode("blob_query", &bytes);
            assert!(
                matches!(observed, Ok(Observation::Reference(view)) if view.root == request.root
                && view.status == if shape == "reserved" { ReferenceStatus::Retired } else { ReferenceStatus::Released })
            );
        }
        let cleaned = usage(pic, canister, Some(&format!("{shape}/inspect/cleaned"))).await;
        assert_eq!(
            cleaned,
            Usage {
                reserved_bytes: 0,
                live_bytes: 0,
                ..full
            }
        );
        let activated: Result<Outcome, Error> = call(
            pic,
            canister,
            actor(9),
            "blob_update",
            &Command::Activate(1),
            None,
        )
        .await;
        assert!(matches!(activated, Ok(Outcome::Maintenance(state)) if state.active));
        let refused: Result<Outcome, Error> =
            call(pic, canister, actor(1), "blob_update", &extra, None).await;
        assert_eq!(
            refused,
            Err(Error::Capacity),
            "cleanup does not reclaim lifetime identities"
        );
    }
    owned.close().await;
}
