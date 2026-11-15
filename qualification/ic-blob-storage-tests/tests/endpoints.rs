//! Installed standalone Wire contract and selected generic-hook rollback evidence.

use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::{Error, Limits, UploadRequest, Usage},
    protocol::{
        Command, Inspection, Installation, Maintenance, ManagementView, Observation, Outcome,
        RPC_LIMITS,
    },
};
use ic_platform_types::Principal;
use ic_testkit::{
    pic::PocketIcStartupConfig,
    pocket_ic::{CanisterId, PocketIcBuilder, RejectCode, RejectResponse, nonblocking::PocketIc},
};
use ic_wire as wire;
use serde::Serialize;
use std::{path::Path, time::Duration};

fn request<T: Serialize>(method: &str, value: &T) -> Vec<u8> {
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

fn response<T: Serialize>(method: &str, value: &T) -> Vec<u8> {
    wire::encode(
        value,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

async fn call(
    pic: &PocketIc,
    canister: CanisterId,
    caller: CanisterId,
    method: &str,
    bytes: Vec<u8>,
) -> Result<Vec<u8>, RejectResponse> {
    if method == "blob_query" {
        pic.query_call(canister, caller, method, bytes).await
    } else {
        pic.update_call(canister, caller, method, bytes).await
    }
}

// A valid authorization frame of exactly the requested size. Domain authority is
// deliberately wrong: an admitted frame must return typed Denied, not a transport trap.
fn sized_request(bytes: usize) -> Vec<u8> {
    let mut command = Command::Authorize {
        request: UploadRequest {
            id: 1.try_into().unwrap(),
            uploader: Principal::from_slice(&[2, 1]),
            root: [1; 32],
            bytes: 3,
            first_reference: 1.try_into().unwrap(),
            expires_at: u64::MAX,
        },
        chunks: vec![[1; 32]],
        headers: vec![("x".into(), "a".repeat(bytes))],
    };
    let limits = ic_wire::Limits {
        bytes: RPC_LIMITS.bytes * 2,
        ..RPC_LIMITS
    };
    let size = wire::encode(
        &command,
        wire::Encoding::Frame {
            method: wire::method_id("blob_update"),
            direction: wire::Direction::Request,
        },
        limits,
    )
    .unwrap()
    .len();
    let Command::Authorize { headers, .. } = &mut command else {
        unreachable!()
    };
    headers[0].1.truncate(bytes - (size - bytes));
    let frame = wire::encode(
        &command,
        wire::Encoding::Frame {
            method: wire::method_id("blob_update"),
            direction: wire::Direction::Request,
        },
        limits,
    )
    .unwrap();
    assert_eq!(frame.len(), bytes);
    frame
}

#[tokio::test]
async fn installed_frames_and_reply_failure_preserve_the_contract() {
    let server_directory = std::env::var_os("BLOB_TEST_SERVER").expect("set BLOB_TEST_SERVER");
    let server = ic_testkit::pic::server_assets::check(Path::new(&server_directory)).unwrap();
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: server,
        timeout: Duration::from_secs(60),
        server_hard_ttl: Some(Duration::from_secs(180)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(PocketIcBuilder::new().with_application_subnet())
    .await
    .unwrap();
    let pic = owned.instance();
    let application = Principal::from_slice(&[1, 1]);
    let manager = Principal::from_slice(&[9, 1]);
    let caller = CanisterId::from_slice(application.as_slice());
    let controller = CanisterId::from_slice(manager.as_slice());
    let input = Installation {
        application,
        manager,
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
    let canister = pic
        .create_canister_with_settings(Some(controller), None)
        .await;
    pic.add_cycles(canister, 10_000_000_000_000).await;
    let wasm = std::fs::read(std::env::var_os("BLOB_WASM").expect("set BLOB_WASM")).unwrap();
    pic.install_canister(
        canister,
        wasm,
        request("blob_init", &input),
        Some(controller),
    )
    .await;
    let initial = pic.get_stable_memory(canister).await;
    for (method, actor, valid, expected) in [
        (
            "blob_update",
            caller,
            request("blob_update", &Command::Drain(1)),
            response("blob_update", &Err::<Outcome, _>(Error::Denied)),
        ),
        (
            "blob_query",
            caller,
            request("blob_query", &Inspection::Usage),
            response(
                "blob_query",
                &Ok::<_, Error>(Observation::Usage(Usage::default())),
            ),
        ),
        (
            "blob_management",
            controller,
            request("blob_management", &()),
            response(
                "blob_management",
                &Ok::<_, Error>(ManagementView {
                    installation: Principal::from_slice(canister.as_slice()),
                    configuration: input,
                    state_version: 1,
                    maintenance: Maintenance {
                        attempt: 0,
                        active: true,
                    },
                }),
            ),
        ),
    ] {
        assert_eq!(
            call(pic, canister, actor, method, valid.clone())
                .await
                .unwrap(),
            expected
        );
        let mut trailing = valid.clone();
        trailing.push(0);
        for malformed in [
            vec![],
            valid[..10].to_vec(),
            valid[..valid.len() - 1].to_vec(),
            trailing,
            request("foreign_method", &()),
            response(method, &()),
        ] {
            let error = call(pic, canister, actor, method, malformed)
                .await
                .unwrap_err();
            assert_eq!(
                error.reject_code,
                RejectCode::CanisterError,
                "{method}: {error}"
            );
            assert!(error.reject_message.contains("wire "), "{method}: {error}");
        }
        let error = call(pic, canister, actor, method, vec![0; RPC_LIMITS.bytes + 1])
            .await
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert!(
            error
                .reject_message
                .contains("Wire request exceeds byte allowance")
        );
        assert_eq!(pic.get_stable_memory(canister).await, initial);
    }
    // Zero-argument management still decodes its unit frame and remains update-only.
    assert_eq!(
        call(
            pic,
            canister,
            caller,
            "blob_management",
            request("blob_management", &())
        )
        .await
        .unwrap(),
        response("blob_management", &Err::<ManagementView, _>(Error::Denied))
    );
    assert!(
        pic.query_call(
            canister,
            controller,
            "blob_management",
            request("blob_management", &())
        )
        .await
        .is_err()
    );
    assert_eq!(
        call(
            pic,
            canister,
            CanisterId::anonymous(),
            "blob_update",
            sized_request(RPC_LIMITS.bytes)
        )
        .await
        .unwrap(),
        response("blob_update", &Err::<Outcome, _>(Error::Denied))
    );
    let overflow = call(
        pic,
        canister,
        CanisterId::anonymous(),
        "blob_update",
        sized_request(RPC_LIMITS.bytes + 1),
    )
    .await
    .unwrap_err();
    assert_eq!(overflow.reject_code, RejectCode::CanisterError);
    assert!(
        overflow
            .reject_message
            .contains("Wire request exceeds byte allowance")
    );
    assert_eq!(pic.get_stable_memory(canister).await, initial);

    // Blob's fixed-size replies cannot normally overflow RPC_LIMITS. This separate
    // installed probe forces the selected Platform/Wire encode hook to fail after
    // a stable write, without adding a fault-injection endpoint to production Blob.
    let probe = pic.create_canister().await;
    pic.add_cycles(probe, 10_000_000_000_000).await;
    pic.install_canister(
        probe,
        std::fs::read(std::env::var_os("BLOB_REPLY_PROBE").expect("set BLOB_REPLY_PROBE")).unwrap(),
        vec![],
        None,
    )
    .await;
    for (fail, expected) in [(true, 0u8), (false, 1), (true, 1)] {
        let result = pic
            .update_call(probe, caller, "probe_write", request("probe_write", &fail))
            .await;
        if fail {
            let error = result.unwrap_err();
            assert_eq!(error.reject_code, RejectCode::CanisterError);
            assert!(error.reject_message.contains("wire limit"), "{error}");
        } else {
            assert_eq!(result.unwrap(), response("probe_write", &vec![expected]));
        }
        assert_eq!(
            pic.query_call(probe, caller, "probe_read", request("probe_read", &()))
                .await
                .unwrap(),
            response("probe_read", &expected)
        );
    }
    owned.close().await;
}
