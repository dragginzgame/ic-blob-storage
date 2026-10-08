//! Actual signed HTTP ingress and certificate recovery against local `PocketIC` only.
mod browser;
mod proof;
use super::*;
use ic_agent::{
    Agent, Identity,
    agent::{RequestStatusResponse, signed::SignedUpdate},
    identity::BasicIdentity,
};
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
    runtime: tokio::runtime::Runtime,
    url: String,
}
impl Headless {
    fn new() -> Self {
        let harness = Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        );
        let mut fixture = Fixture::with_operator(harness, Fake::principal(1));
        // Fixed test-only key, never a deployment identity.
        let identity = BasicIdentity::from_raw_key(&[42; 32]);
        fixture.uploader = identity.sender().unwrap();
        fixture.enroll(None, true).unwrap();
        let root_key = fixture.harness.pic.root_key().unwrap();
        let url = fixture.harness.pic.make_live(None).to_string();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
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
            runtime,
            url,
        }
    }
    fn prepare(&self, id: u128, bytes: u8) -> (ExposureInput, ExpectedUpload) {
        let f = &self.fixture;
        let (permission, declaration) = f.permission(id, bytes);
        f.admit(f.tenant, permission).unwrap();
        f.prepare(&declaration).unwrap();
        let input = input(f, permission);
        configure(f, input);
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
            .with_arg(candid::encode_one(&expected.root).unwrap())
            .sign()
            .unwrap()
    }
}

#[test]
fn headless_ingress_extracts_exact_certificate_and_rejects_forged_or_unrelated_proofs() {
    let h = Headless::new();
    let (input, expected) = h.prepare(1, 1);
    let signed = h.sign(&expected);
    let certificate = h.runtime.block_on(submit(&h.url, &signed));
    let reply = extract(&h.agent, &signed, &expected, &certificate).unwrap();
    assert_ne!(reply, Vec::<u8>::new());
    assert_eq!(
        inspect(&h.fixture, h.fixture.uploader, input)
            .unwrap()
            .state,
        UploadState::ExposurePossible
    );
    let mut forged: ic_agent::Certificate = serde_cbor::from_slice(&certificate).unwrap();
    forged.signature[0] ^= 1;
    assert_eq!(
        extract(
            &h.agent,
            &signed,
            &expected,
            &serde_cbor::to_vec(&forged).unwrap()
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
    let (_, other) = h.prepare(2, 2);
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

#[test]
fn headless_ingress_recovers_saved_request_without_reissuing_and_certifies_refusal() {
    let h = Headless::new();
    let (input, expected) = h.prepare(1, 1);
    let signed = h.sign(&expected);
    // Retain the transport envelope/request ID before dispatch. This is not a
    // production consumer journal or a claim about provider receipt retention.
    let saved = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(saved.path(), serde_json::to_vec(&signed).unwrap()).unwrap();
    let _lost_reply = h.runtime.block_on(submit(&h.url, &signed));
    let before = h.fixture.harness.pic.get_stable_memory(h.fixture.service);
    let restored: SignedUpdate =
        serde_json::from_slice(&std::fs::read(saved.path()).unwrap()).unwrap();
    let (status, cert) = h
        .runtime
        .block_on(
            h.agent
                .request_status_raw(&restored.request_id, expected.service),
        )
        .unwrap();
    let RequestStatusResponse::Replied(reply) = status else {
        panic!("expected retained ingress reply")
    };
    let recovered = serde_cbor::to_vec(&cert).unwrap();
    assert_eq!(
        extract(&h.agent, &restored, &expected, &recovered),
        Ok(reply.arg)
    );
    assert_eq!(
        h.fixture.harness.pic.get_stable_memory(h.fixture.service),
        before
    );
    assert_eq!(
        inspect(&h.fixture, h.fixture.uploader, input)
            .unwrap()
            .state,
        UploadState::ExposurePossible
    );
    // A new signed ingress is a new attempt; its certified rejection is not an
    // upload certificate even though HTTP successfully transports a certificate.
    let repeated = h.sign(&expected);
    assert_ne!(repeated.request_id, restored.request_id);
    let rejected = h.runtime.block_on(submit(&h.url, &repeated));
    assert_eq!(
        extract(&h.agent, &repeated, &expected, &rejected),
        Err(ProofError::Status)
    );
    assert_eq!(
        h.fixture.harness.pic.get_stable_memory(h.fixture.service),
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
        .update_candid_as(
            h.fixture.service,
            h.fixture.tenant,
            "blob_revoke_upload",
            (input.permission,),
        )
        .unwrap();
    assert!(revoked.unwrap().admission.revoked);
    let before = h.fixture.harness.pic.get_stable_memory(h.fixture.service);
    let (_, history) = h
        .runtime
        .block_on(
            h.agent
                .request_status_raw(&restored.request_id, expected.service),
        )
        .unwrap();
    assert!(
        extract(
            &h.agent,
            &restored,
            &expected,
            &serde_cbor::to_vec(&history).unwrap()
        )
        .is_ok()
    );
    assert!(
        inspect(&h.fixture, h.fixture.uploader, input)
            .unwrap()
            .revoked
    );
    assert_eq!(
        h.fixture.harness.pic.get_stable_memory(h.fixture.service),
        before
    );
}
