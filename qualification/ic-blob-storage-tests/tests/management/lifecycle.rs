//! Populated obligations through native maintenance with a different artifact hash.
//! The replacement adds only custom metadata; this is not a schema migration test.

use super::*;
use ic_blob_storage::{
    local::{Error, ReferenceStatus, UploadRequest, Usage},
    protocol::{Inspection, Observation, Outcome},
    references::RetentionState,
};
use ic_testkit::pocket_ic::nonblocking::PocketIc;
use std::num::NonZeroU128;

fn id(value: u128) -> NonZeroU128 {
    value.try_into().unwrap()
}

fn request(value: u8, uploader: Principal, bytes: u64) -> UploadRequest {
    UploadRequest {
        id: id(u128::from(value)),
        uploader,
        // Synthetic single-leaf declarations, not evidence of provider bytes.
        root: [value; 32],
        bytes,
        first_reference: id(u128::from(value)),
        expires_at: u64::MAX,
    }
}

fn authorize(request: UploadRequest) -> Command {
    Command::Authorize {
        request,
        chunks: vec![request.root],
        headers: vec![],
    }
}

async fn update(
    pic: &PocketIc,
    canister: CanisterId,
    caller: Principal,
    command: Command,
) -> Result<Outcome, Error> {
    let method = ic_wire::method_id("blob_update");
    let bytes = pic
        .update_call(
            canister,
            CanisterId::from_slice(caller.as_slice()),
            "blob_update",
            ic_blob_storage_wire::frame::encode(
                &command,
                ic_wire::Encoding::Frame {
                    method,
                    direction: ic_wire::Direction::Request,
                },
                RPC_LIMITS,
            )
            .unwrap(),
        )
        .await
        .unwrap();

    ic_blob_storage_wire::frame::decode(
        &bytes,
        ic_wire::Encoding::Frame {
            method,
            direction: ic_wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

async fn query(
    pic: &PocketIc,
    canister: CanisterId,
    caller: Principal,
    request: Inspection,
) -> Result<Observation, Error> {
    let method = ic_wire::method_id("blob_query");
    let bytes = pic
        .query_call(
            canister,
            CanisterId::from_slice(caller.as_slice()),
            "blob_query",
            ic_blob_storage_wire::frame::encode(
                &request,
                ic_wire::Encoding::Frame {
                    method,
                    direction: ic_wire::Direction::Request,
                },
                RPC_LIMITS,
            )
            .unwrap(),
        )
        .await
        .unwrap();

    ic_blob_storage_wire::frame::decode(
        &bytes,
        ic_wire::Encoding::Frame {
            method,
            direction: ic_wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

async fn observations(
    pic: &PocketIc,
    canister: CanisterId,
    application: Principal,
) -> Vec<Observation> {
    let mut values = Vec::new();
    for n in 1..=4 {
        for request in [Inspection::Upload(id(n)), Inspection::Reference(id(n))] {
            values.push(query(pic, canister, application, request).await.unwrap());
        }
    }
    for root in [[1; 32], [3; 32], [4; 32]] {
        values.push(
            query(pic, canister, application, Inspection::Object(root))
                .await
                .unwrap(),
        );
    }
    values.push(
        query(pic, canister, application, Inspection::Usage)
            .await
            .unwrap(),
    );

    values
}

#[tokio::test]
async fn changed_artifact_preserves_obligations_through_native_lifecycle() {
    let checkout = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let directory = checkout
        .join("target/management-test")
        .join(std::process::id().to_string())
        .join("changed-artifact");
    std::fs::create_dir_all(&directory).unwrap();
    let wasm = std::fs::read(std::env::var_os("BLOB_WASM").expect("set BLOB_WASM")).unwrap();
    let mut replacement = wasm.clone();
    // Wasm custom section 0, with fixed one-byte lengths (<128): name + marker.
    // https://webassembly.github.io/spec/core/binary/modules.html#custom-section
    let name = b"blob-qualification";
    let payload = [&[u8::try_from(name.len()).unwrap()][..], name, b"v1"].concat();
    assert!(payload.len() < 128);
    replacement.extend([0, u8::try_from(payload.len()).unwrap()]);
    replacement.extend(payload);
    assert_ne!(Sha256::digest(&wasm), Sha256::digest(&replacement));
    std::fs::write(directory.join("host.wasm"), &wasm).unwrap();
    std::fs::write(directory.join("next.wasm"), &replacement).unwrap();
    std::fs::write(directory.join("manager.pem"), TEST_PEM).unwrap();
    let manager = CanisterId::from_slice(
        Ed25519Identity::from_pem(TEST_PEM, 4096)
            .unwrap()
            .principal()
            .as_slice(),
    );
    let input = installation(Principal::from_slice(manager.as_slice()));
    std::fs::write(directory.join("installation.wire"), encode(&input)).unwrap();
    let (server, mut owned) = support::instance(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
        300,
    )
    .await;
    let controller = CanisterId::from_slice(manager.as_slice());
    let canister = owned
        .client()
        .create_canister_with_settings(Some(controller), None)
        .await;
    owned
        .client()
        .add_cycles(canister, 10_000_000_000_000)
        .await;
    owned
        .client()
        .install_canister(canister, wasm, encode(&input), Some(controller))
        .await;
    std::fs::write(
        directory.join("root.der"),
        owned.client().root_key().await.unwrap(),
    )
    .unwrap();
    std::fs::write(
        directory.join("root-subnet.txt"),
        owned.client().topology().await.get_nns().unwrap().to_text(),
    )
    .unwrap();
    let url = owned.client_mut().make_live(None).await;
    let pic = owned.client();
    let uploader = Principal::from_slice(&[2, 1]);
    let other = Principal::from_slice(&[3, 1]);
    let open = request(1, uploader, 3);
    let attached = UploadRequest {
        id: id(2),
        first_reference: id(2),
        uploader: other,
        ..open
    };
    let closing = request(3, uploader, 5);
    let reserved = request(4, other, 7);
    update(pic, canister, input.application, authorize(open))
        .await
        .unwrap();
    update(pic, canister, uploader, Command::Register(open.root))
        .await
        .unwrap();
    for grant in [attached, closing, reserved] {
        update(pic, canister, input.application, authorize(grant))
            .await
            .unwrap();
    }
    update(pic, canister, uploader, Command::Register(closing.root))
        .await
        .unwrap();
    for reference in [open.first_reference, closing.first_reference] {
        update(
            pic,
            canister,
            input.application,
            Command::Release(reference),
        )
        .await
        .unwrap();
    }
    let expected = observations(pic, canister, input.application).await;
    let usage = Usage {
        reserved_bytes: 7,
        live_bytes: 3,
        unresolved_bytes: 8,
        uploads: 4,
        references: 4,
    };
    assert_eq!(expected.last(), Some(&Observation::Usage(usage)));
    assert!(
        matches!(query(pic, canister, input.application, Inspection::Object(open.root)).await.unwrap(),
        Observation::Object(view) if view.retention == Some(RetentionState::Open) && view.live_references == 1)
    );
    assert!(
        matches!(query(pic, canister, input.application, Inspection::Object(closing.root)).await.unwrap(),
        Observation::Object(view) if view.retention == Some(RetentionState::Closing) && view.live_references == 0)
    );
    let custody = directory.join("custody");
    // All lifecycle effects go through the actual CLI. Fixture calls seed and inspect domain state.
    for (stage, operation) in ["drain", "stop", "upgrade", "start", "activate"]
        .into_iter()
        .enumerate()
    {
        for action in ["prepare", "dispatch", "observe"] {
            let mut cli = command(
                &directory,
                url.as_str(),
                canister,
                "3",
                &format!("{action}-{operation}"),
            );
            cli.arg("--record").arg(directory.join(operation));
            if stage >= 2 {
                cli.arg("--stop-record")
                    .arg(directory.join("stop"))
                    .arg("--next-wasm")
                    .arg(directory.join("next.wasm"));
            }
            if stage >= 3 {
                cli.arg("--upgrade-record").arg(directory.join("upgrade"));
            }
            if stage >= 4 {
                cli.arg("--start-record").arg(directory.join("start"));
            }
            match action {
                "prepare" => {
                    cli.args(["--attempt", "1"]);
                }
                "dispatch" => {
                    cli.arg("--custody")
                        .arg(&custody)
                        .arg("--checkout")
                        .arg(&checkout);
                }
                _ => {}
            }
            let output = cli.output().await.unwrap();
            assert!(
                output.status.success(),
                "{action}-{operation}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        if matches!(operation, "drain" | "start" | "activate") {
            assert_eq!(
                observations(pic, canister, input.application).await,
                expected
            );
            // Reopening cannot broaden uploader or reference authority.
            assert_eq!(
                query(pic, canister, other, Inspection::Upload(open.id)).await,
                Err(Error::Denied)
            );
            assert_eq!(
                update(
                    pic,
                    canister,
                    uploader,
                    Command::Release(attached.first_reference)
                )
                .await,
                Err(Error::Denied)
            );
            if operation != "activate" {
                assert_eq!(
                    update(
                        pic,
                        canister,
                        input.application,
                        authorize(request(5, uploader, 1))
                    )
                    .await,
                    Err(Error::Maintenance)
                );
                assert_eq!(
                    update(pic, canister, other, Command::Register(reserved.root)).await,
                    Err(Error::Maintenance)
                );
            }
        }
    }
    // The old artifact is no longer an acceptable input to inspection.
    refused(
        &inspect(&directory, url.as_str(), canister, "3").await,
        "Wasm does not match",
    );
    // Historical acquisition/release cannot resurrect a released reference or settle exposure.
    update(
        pic,
        canister,
        input.application,
        Command::Retain(open.root, open.first_reference),
    )
    .await
    .unwrap();
    assert!(
        matches!(query(pic, canister, input.application, Inspection::Reference(open.first_reference)).await.unwrap(),
        Observation::Reference(view) if view.status == ReferenceStatus::Released)
    );
    for reference in [attached.first_reference, reserved.first_reference] {
        update(
            pic,
            canister,
            input.application,
            Command::Release(reference),
        )
        .await
        .unwrap();
    }
    assert_eq!(
        query(pic, canister, input.application, Inspection::Usage).await,
        Ok(Observation::Usage(Usage {
            reserved_bytes: 0,
            live_bytes: 0,
            ..usage
        }))
    );
    assert_eq!(
        update(pic, canister, other, Command::Register(reserved.root)).await,
        Err(Error::NotReserved)
    );
    let fresh = request(5, uploader, 1);
    update(pic, canister, input.application, authorize(fresh))
        .await
        .unwrap();
    update(pic, canister, uploader, Command::Register(fresh.root))
        .await
        .unwrap();
    assert_eq!(
        query(pic, canister, input.application, Inspection::Usage).await,
        Ok(Observation::Usage(Usage {
            reserved_bytes: 0,
            live_bytes: 1,
            unresolved_bytes: 9,
            uploads: 5,
            references: 5
        }))
    );
    support::close(server, owned).await;
}
