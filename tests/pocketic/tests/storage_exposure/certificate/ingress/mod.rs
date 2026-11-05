//! Actual signed HTTP ingress and certificate recovery against local `PocketIC` only.
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
use ic_host_fs::testing::FixtureDirectory;
mod browser;
mod proof;
use super::*;
use ic_platform::agent::{Agent, BasicIdentity, Identity, RequestStatusResponse, SignedUpdate};
use ic_testkit::pocket_ic::PocketIcBuilder;
use proof::{ProofError, extract, submit};
use std::time::Duration;

struct ExpectedUpload {
    service: Principal,
    uploader: Principal,
    root: String,
}

struct Headless {
    fixture: Fixture,
    agent: Agent,
    url: String,
}
impl Headless {
    async fn new() -> Self {
        let harness = Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await;
        let mut fixture = Fixture::with_operator(harness, fake_principal(1)).await;
        // Fixed test-only key, never a deployment identity.
        let identity = BasicIdentity::from_raw_key(&[42; 32]);
        fixture.uploader = identity.sender().unwrap();
        fixture.enroll(None, true).await.unwrap();
        let root_key = fixture.harness.pic.root_key().await.unwrap().unwrap();
        let url = fixture
            .harness
            .pic
            .make_live(None)
            .await
            .unwrap()
            .to_string();
        let agent = Agent::builder()
            .with_url(&url)
            .with_identity(identity)
            .with_max_tcp_error_retries(0)
            .with_max_response_body_size(256 * 1024)
            .with_max_polling_time(Duration::from_secs(20))
            .build()
            .unwrap();
        // Trust only the explicitly owned emulator key; never fetch a mainnet root key.
        agent.set_root_key(root_key);
        Self {
            fixture,
            agent,
            url,
        }
    }
    async fn prepare(&self, id: u128, bytes: u8) -> (ExposureInput, ExpectedUpload) {
        let f = &self.fixture;
        let (permission, declaration) = f.permission(id, bytes);
        f.admit(f.tenant, permission).await.unwrap();
        f.prepare(&declaration).await.unwrap();
        let input = input(f, permission);
        configure(f, input).await;
        (
            input,
            ExpectedUpload {
                service: f.service,
                uploader: f.uploader,
                root: root(permission),
            },
        )
    }
    fn sign(&self, expected: &ExpectedUpload) -> SignedUpdate {
        self.agent
            .update(&expected.service, METHOD)
            .with_arg(ic_platform::candid::contract::encode_one(&expected.root).unwrap())
            .sign()
            .unwrap()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn headless_ingress_extracts_exact_certificate_and_rejects_forged_or_unrelated_proofs() {
    let h = Headless::new().await;
    let (input, expected) = h.prepare(1, 1).await;
    let signed = h.sign(&expected);
    let certificate = (submit(&h.url, &signed)).await;
    let reply = extract(&h.agent, &signed, &expected, &certificate).unwrap();
    assert_ne!(reply, Vec::<u8>::new());
    assert_eq!(
        inspect(&h.fixture, h.fixture.uploader, input)
            .await
            .unwrap()
            .state,
        UploadState::Registered
    );
    let mut forged: ic_platform::certification::Certificate =
        ic_platform::certification::cbor::decode_certificate(
            &certificate,
            proof::MAX_CERTIFICATE_BYTES,
        )
        .unwrap();
    let untagged = ic_platform::certification::cbor::encode_certificate_untagged(
        &forged,
        proof::MAX_CERTIFICATE_BYTES,
    )
    .unwrap();
    assert!(!untagged.starts_with(&[0xd9, 0xd9, 0xf7]));
    assert_eq!(extract(&h.agent, &signed, &expected, &untagged), Ok(reply));
    let mut trailing = certificate.clone();
    trailing.push(0);
    assert_eq!(
        extract(&h.agent, &signed, &expected, &trailing),
        Err(ProofError::Encoding)
    );
    forged.signature[0] ^= 1;
    assert_eq!(
        extract(
            &h.agent,
            &signed,
            &expected,
            &ic_platform::certification::cbor::encode_certificate_untagged(
                &forged,
                proof::MAX_CERTIFICATE_BYTES
            )
            .unwrap()
        ),
        Err(ProofError::Signature)
    );
    assert_eq!(
        extract(&h.agent, &signed, &expected, b"garbage"),
        Err(ProofError::Encoding)
    );
    assert_eq!(
        extract(&h.agent, &signed, &expected, &vec![0; 128 * 1024 + 1]),
        Err(ProofError::Size)
    );
    let (_, other) = h.prepare(2, 2).await;
    let other_signed = h.sign(&other);
    assert_eq!(
        extract(&h.agent, &other_signed, &other, &certificate),
        Err(ProofError::Status)
    );
    assert_eq!(
        extract(&h.agent, &signed, &other, &certificate),
        Err(ProofError::Intent)
    );
    let mut changed = signed.clone();
    changed.request_id = other_signed.request_id;
    assert_eq!(
        extract(&h.agent, &changed, &expected, &certificate),
        Err(ProofError::Intent)
    );
    let untrusted = Agent::builder().with_url(&h.url).build().unwrap();
    assert_eq!(
        extract(&untrusted, &signed, &expected, &certificate),
        Err(ProofError::Signature)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered installed scenario retains lifecycle and byte-preservation assertions while making fallible client operations explicit"
)]
async fn headless_ingress_recovers_saved_request_without_reissuing_and_certifies_refusal() {
    let h = Headless::new().await;
    let (input, expected) = h.prepare(1, 1).await;
    let signed = h.sign(&expected);
    // Retain the transport envelope/request ID before dispatch. This is not a
    // production consumer journal or a claim about provider receipt retention.
    let saved_directory = FixtureDirectory::new(&std::env::temp_dir(), "blob-file").unwrap();
    let saved =
        ic_host_fs::scratch::create_private_bytes(saved_directory.path(), "input", b"", 128)
            .unwrap();
    std::fs::write(saved.as_path(), serde_json::to_vec(&signed).unwrap()).unwrap();
    let _lost_reply = (submit(&h.url, &signed)).await;
    let before = h
        .fixture
        .harness
        .pic
        .get_stable_memory(h.fixture.service)
        .await
        .unwrap();
    let restored: SignedUpdate =
        serde_json::from_slice(&std::fs::read(saved.as_path()).unwrap()).unwrap();
    let (status, cert) = (h
        .agent
        .request_status_raw(&restored.request_id, expected.service))
    .await
    .unwrap();
    let RequestStatusResponse::Replied(reply) = status else {
        panic!("expected retained ingress reply")
    };
    let recovered = ic_platform::certification::cbor::encode_certificate_untagged(
        &cert,
        proof::MAX_CERTIFICATE_BYTES,
    )
    .unwrap();
    assert_eq!(
        extract(&h.agent, &restored, &expected, &recovered),
        Ok(reply.arg)
    );
    assert_eq!(
        h.fixture
            .harness
            .pic
            .get_stable_memory(h.fixture.service)
            .await
            .unwrap(),
        before
    );
    assert_eq!(
        inspect(&h.fixture, h.fixture.uploader, input)
            .await
            .unwrap()
            .state,
        UploadState::Registered
    );
    // A new signed ingress is a new attempt; its certified rejection is not an
    // upload certificate even though HTTP successfully transports a certificate.
    let repeated = h.sign(&expected);
    assert_ne!(repeated.request_id, restored.request_id);
    let rejected = (submit(&h.url, &repeated)).await;
    assert_eq!(
        extract(&h.agent, &repeated, &expected, &rejected),
        Err(ProofError::Status)
    );
    assert_eq!(
        h.fixture
            .harness
            .pic
            .get_stable_memory(h.fixture.service)
            .await
            .unwrap(),
        before
    );
    // A certified historical reply survives local withdrawal. It is not renewed
    // permission or authority to dispatch/retry a gateway upload.
    let revoked: Result<
        ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse,
        ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
    > = h
        .fixture
        .harness
        .pic
        .update_blob_as(
            h.fixture.service,
            h.fixture.tenant,
            "blob_revoke_upload",
            input.permission,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert!(revoked.unwrap().admission.revoked);
    let before = h
        .fixture
        .harness
        .pic
        .get_stable_memory(h.fixture.service)
        .await
        .unwrap();
    let (_, history) = (h
        .agent
        .request_status_raw(&restored.request_id, expected.service))
    .await
    .unwrap();
    assert!(
        extract(
            &h.agent,
            &restored,
            &expected,
            &ic_platform::certification::cbor::encode_certificate_untagged(
                &history,
                proof::MAX_CERTIFICATE_BYTES
            )
            .unwrap()
        )
        .is_ok()
    );
    assert!(
        inspect(&h.fixture, h.fixture.uploader, input)
            .await
            .unwrap()
            .revoked
    );
    assert_eq!(
        h.fixture
            .harness
            .pic
            .get_stable_memory(h.fixture.service)
            .await
            .unwrap(),
        before
    );
}
