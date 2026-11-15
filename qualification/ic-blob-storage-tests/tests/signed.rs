//! Signed application/uploader calls to the real host through an owned local gateway.
//! Registration remains a local transition, never a provider certificate.

use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::{Error, Limits, ReferenceStatus, UploadRequest, UploadState, UploadView},
    protocol::{Command, Inspection, Installation, Observation, Outcome, RPC_LIMITS},
};
use ic_platform::{
    Principal,
    agent::{Agent, AgentError, BasicIdentity, Identity},
};
use ic_testkit::{pic::PocketIcStartupConfig, pocket_ic::PocketIcBuilder};
use ic_wire::native;
use serde::{Serialize, de::DeserializeOwned};
use std::{path::Path, time::Duration};

fn principal(seed: u8) -> Principal {
    let sender = BasicIdentity::from_raw_key(&[seed; 32]).sender().unwrap();
    Principal::from_slice(sender.as_slice())
}

fn agent(url: &str, root: &[u8], seed: u8) -> Agent {
    // Public deterministic keys for this disposable simulator only.
    let agent = Agent::builder()
        .with_url(url)
        .with_identity(BasicIdentity::from_raw_key(&[seed; 32]))
        .with_verify_query_signatures(true)
        .with_max_response_body_size(2 * 1024 * 1024)
        .with_max_polling_time(Duration::from_secs(30))
        .with_max_tcp_error_retries(0)
        .build()
        .unwrap();
    agent.set_root_key(root.to_vec());
    agent
}

fn encode<T: Serialize>(method: &str, value: &T) -> Vec<u8> {
    native::encode_request(native::method_id(method), value, RPC_LIMITS).unwrap()
}

fn decode<T: DeserializeOwned>(method: &str, bytes: &[u8]) -> T {
    native::decode_response(native::method_id(method), bytes, RPC_LIMITS).unwrap()
}

async fn update(agent: &Agent, target: Principal, command: &Command) -> Result<Outcome, Error> {
    let bytes = agent
        .update(&target.as_slice().try_into().unwrap(), "blob_update")
        .with_arg(encode("blob_update", command))
        .call_and_wait()
        .await
        .unwrap();
    decode("blob_update", &bytes)
}

async fn query(
    agent: &Agent,
    target: Principal,
    request: Inspection,
) -> Result<Observation, Error> {
    let bytes = agent
        .query(&target.as_slice().try_into().unwrap(), "blob_query")
        .with_arg(encode("blob_query", &request))
        .call()
        .await
        .unwrap();
    decode("blob_query", &bytes)
}

fn authorize(request: UploadRequest) -> Command {
    Command::Authorize {
        request,
        chunks: vec![request.root],
        headers: vec![],
    }
}

#[tokio::test]
async fn signed_uploaders_cannot_substitute_authority_or_repeat_registration() {
    let server_directory = std::env::var_os("BLOB_TEST_SERVER").expect("set BLOB_TEST_SERVER");
    let server = ic_testkit::pic::server_assets::check(Path::new(&server_directory)).unwrap();
    let mut owned = PocketIcStartupConfig::Spawn {
        server_binary: server,
        timeout: Duration::from_secs(60),
        server_hard_ttl: Some(Duration::from_secs(180)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    )
    .await
    .unwrap();
    let input = Installation {
        application: principal(1),
        manager: principal(9),
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
    let canister = owned.instance().create_canister().await;
    owned
        .instance()
        .add_cycles(canister, 10_000_000_000_000)
        .await;
    owned
        .instance()
        .install_canister(
            canister,
            std::fs::read(std::env::var_os("BLOB_WASM").expect("set BLOB_WASM")).unwrap(),
            encode("blob_init", &input),
            None,
        )
        .await;
    let target = Principal::from_slice(canister.as_slice());
    // Independent fixture custody supplies the trust root; never fetch it from HTTP.
    let root = owned.instance().root_key().await.unwrap();
    let url = owned.instance_mut().make_live(None).await;
    let application = agent(url.as_str(), &root, 1);
    let uploader = agent(url.as_str(), &root, 2);
    let other = agent(url.as_str(), &root, 3);
    let manager = agent(url.as_str(), &root, 9);
    let request = UploadRequest {
        id: 1.try_into().unwrap(),
        uploader: principal(2),
        root: [7; 32],
        bytes: 3,
        first_reference: 1.try_into().unwrap(),
        expires_at: u64::MAX,
    };
    for actor in [&uploader, &other, &manager] {
        assert_eq!(
            update(actor, target, &authorize(request)).await,
            Err(Error::Denied)
        );
    }
    assert_eq!(
        update(&application, target, &authorize(request)).await,
        Ok(Outcome::Upload(UploadView {
            request,
            state: UploadState::Reserved
        }))
    );
    for actor in [&other, &application, &manager] {
        assert_eq!(
            update(actor, target, &Command::Register(request.root)).await,
            Err(Error::Denied)
        );
    }
    assert_eq!(
        query(&other, target, Inspection::Upload(request.id)).await,
        Err(Error::Denied)
    );

    // Execute once, deliberately discard its authenticated reply without decoding.
    // This models caller-side reply loss, not a network fault or certificate recovery.
    drop(
        uploader
            .update(&canister, "blob_update")
            .with_arg(encode("blob_update", &Command::Register(request.root)))
            .call_and_wait()
            .await
            .unwrap(),
    );
    // A fresh signer/Agent inspects the original identity without another registration.
    let uploader = agent(url.as_str(), &root, 2);
    assert_eq!(
        query(&uploader, target, Inspection::Upload(request.id)).await,
        Ok(Observation::Upload(UploadView {
            request,
            state: UploadState::Registered
        }))
    );
    assert_eq!(
        update(&uploader, target, &Command::Register(request.root)).await,
        Err(Error::NotReserved)
    );
    for command in [
        Command::Release(request.first_reference),
        Command::Retain(request.root, 3.try_into().unwrap()),
    ] {
        assert_eq!(
            update(&uploader, target, &command).await,
            Err(Error::Denied)
        );
    }
    assert_eq!(
        query(
            &uploader,
            target,
            Inspection::Reference(request.first_reference)
        )
        .await,
        Err(Error::Denied)
    );
    assert_eq!(
        query(&uploader, target, Inspection::Usage).await,
        Err(Error::Denied)
    );

    let attached = UploadRequest {
        id: 2.try_into().unwrap(),
        first_reference: 2.try_into().unwrap(),
        uploader: principal(3),
        ..request
    };
    assert_eq!(
        update(&application, target, &authorize(attached)).await,
        Ok(Outcome::Upload(UploadView {
            request: attached,
            state: UploadState::Attached
        }))
    );
    assert_eq!(
        query(&other, target, Inspection::Upload(attached.id)).await,
        Ok(Observation::Upload(UploadView {
            request: attached,
            state: UploadState::Attached
        }))
    );
    assert_eq!(
        update(&other, target, &Command::Register(request.root)).await,
        Err(Error::Denied)
    );
    assert_eq!(
        update(
            &application,
            target,
            &Command::Release(request.first_reference)
        )
        .await,
        Ok(Outcome::Done)
    );
    let Observation::Object(view) = query(&application, target, Inspection::Object(request.root))
        .await
        .unwrap()
    else {
        panic!("expected object")
    };
    assert_eq!(view.live_references, 1);
    assert_eq!(
        update(
            &application,
            target,
            &Command::Release(attached.first_reference)
        )
        .await,
        Ok(Outcome::Done)
    );
    // Historical authorization replay reports its old registration; it cannot revive use.
    assert_eq!(
        update(&application, target, &authorize(request)).await,
        Ok(Outcome::Upload(UploadView {
            request,
            state: UploadState::Registered
        }))
    );
    let Observation::Reference(view) = query(
        &application,
        target,
        Inspection::Reference(request.first_reference),
    )
    .await
    .unwrap() else {
        panic!("expected reference")
    };
    assert_eq!(view.status, ReferenceStatus::Released);
    let Observation::Usage(usage) = query(&application, target, Inspection::Usage)
        .await
        .unwrap()
    else {
        panic!("expected usage")
    };
    assert_eq!(
        (
            usage.reserved_bytes,
            usage.live_bytes,
            usage.unresolved_bytes
        ),
        (0, 0, 3)
    );
    assert_eq!((usage.uploads, usage.references), (2, 2));

    // A matching target and correct signer cannot make an untrusted response acceptable.
    let mut wrong_root = root;
    *wrong_root.last_mut().unwrap() ^= 1;
    let untrusted = agent(url.as_str(), &wrong_root, 1);
    let error = untrusted
        .query(&canister, "blob_query")
        .with_arg(encode("blob_query", &Inspection::Usage))
        .call()
        .await
        .unwrap_err();
    assert!(
        matches!(error, AgentError::CertificateVerificationFailed()),
        "{error}"
    );
    owned.close().await;
}
