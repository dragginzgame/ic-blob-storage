//! Connected local upload/deletion journey; completion and billing facts are substitutes.
#![cfg(not(target_family = "wasm"))]

mod chunks;
mod content_vectors;
mod readback;
mod recovery;
mod support;
mod sync_request;
mod vectors;
use blob_test_protocol::journey::{
    JourneyCertificate, JourneyFailure, JourneyManifest, JourneyProgress, JourneyReservation,
    JourneyUpload, JourneyUsage, JourneyVerification,
};
use ic_platform::candid::decoder;
use ic_platform::principal::ExternalPrincipal as Principal;

use ic_testkit::{
    Fake,
    pocket_ic::{CanisterSettings, RejectCode},
};
use sha2::{Digest, Sha256};
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    service: Principal,
    gateway: Principal,
    first: Principal,
    second: Principal,
    operator: Principal,
    authority_operator: Principal,
}

impl Fixture {
    fn sync_request(&self) -> blob_test_protocol::GatewaySyncRequest {
        sync_request::read(&self.harness.pic, self.service, self.authority_operator)
    }

    fn new() -> Self {
        Self::with_source_operator(false)
    }

    fn with_source_operator(source_operator: bool) -> Self {
        let harness = Harness::new();
        let pic = &harness.pic;
        let first = Fake::principal(1);
        let second = Fake::principal(2);
        let operator = Fake::principal(4);
        let service = pic.create_canister_with_settings(
            Some(operator),
            Some(CanisterSettings {
                controllers: Some(vec![operator]),
                ..CanisterSettings::default()
            }),
        );
        let gateway = pic.create_canister();
        let authority_operator = if source_operator { gateway } else { operator };
        pic.install_canister(
            service,
            std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).expect("service Wasm"),
            ic_platform::candid::contract::encode_args((
                first,
                second,
                gateway,
                authority_operator,
            ))
            .expect("service init"),
            Some(operator),
        );
        pic.install_canister(
            gateway,
            std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).expect("gateway Wasm"),
            ic_platform::candid::contract::encode_args((service, gateway, operator))
                .expect("gateway init"),
            None,
        );
        Self {
            harness,
            service,
            gateway,
            first,
            second,
            operator,
            authority_operator,
        }
    }

    fn control(
        &self,
        caller: Principal,
        method: &str,
        input: JourneyUpload,
    ) -> Result<(), JourneyFailure> {
        crate::support::update_fixture(&self.harness.pic, self.service, caller, method, (input,))
            .map(|(reply,)| reply)
            .expect("local control reply")
    }
    fn usage(&self, caller: Principal) -> Result<JourneyUsage, JourneyFailure> {
        crate::support::query_fixture(&self.harness.pic, self.service, caller, "journey_usage", ())
            .map(|(reply,)| reply)
            .expect("usage reply")
    }
    fn reserve(&self, caller: Principal, input: JourneyUpload) -> Result<(), JourneyFailure> {
        self.reserve_manifest(
            caller,
            input,
            JourneyManifest {
                chunks: vec![input.root],
                headers: vec![],
            },
        )
    }
    fn reserve_manifest(
        &self,
        caller: Principal,
        input: JourneyUpload,
        manifest: JourneyManifest,
    ) -> Result<(), JourneyFailure> {
        crate::support::update_fixture(
            &self.harness.pic,
            self.service,
            caller,
            "journey_reserve",
            (JourneyReservation {
                upload: input,
                manifest,
            },),
        )
        .map(|(reply,)| reply)
        .expect("admission reply")
    }
    fn progress(
        &self,
        caller: Principal,
        input: JourneyUpload,
    ) -> Result<JourneyProgress, JourneyFailure> {
        crate::support::query_fixture(
            &self.harness.pic,
            self.service,
            caller,
            "journey_progress",
            (root(input.root),),
        )
        .map(|(reply,)| reply)
        .expect("progress reply")
    }
    fn append(
        &self,
        caller: Principal,
        input: JourneyUpload,
        index: u64,
        bytes: &[u8],
    ) -> Result<(), JourneyFailure> {
        crate::support::update_fixture(
            &self.harness.pic,
            self.service,
            caller,
            "journey_append",
            (root(input.root), index, bytes.to_vec()),
        )
        .map(|(reply,)| reply)
        .expect("verification reply")
    }
    fn certificate(
        &self,
        caller: Principal,
        seed: [u8; 32],
    ) -> Result<JourneyCertificate, RejectCode> {
        self.harness
            .pic
            .update_call(
                self.service,
                caller,
                "_immutableObjectStorageCreateCertificate",
                ic_platform::candid::contract::encode_one(root_text(seed)).expect("root text"),
            )
            .map(|bytes| {
                decoder::decode_one_with_config(&bytes, &decoder::DecoderConfig::new())
                    .expect("current certificate shape")
            })
            .map_err(|error| error.reject_code)
    }
    fn root_control(
        &self,
        caller: Principal,
        method: &str,
        seed: [u8; 32],
    ) -> Result<(), JourneyFailure> {
        crate::support::update_fixture(
            &self.harness.pic,
            self.service,
            caller,
            method,
            (root(seed),),
        )
        .map(|(reply,)| reply)
        .expect("root control")
    }
    fn pending(&self) -> Vec<Vec<u8>> {
        crate::support::query_fixture(
            &self.harness.pic,
            self.service,
            self.gateway,
            "_immutableObjectStorageBlobsToDelete",
            (),
        )
        .map(|(reply,)| reply)
        .expect("binary-root deletion list")
    }
    fn live(&self, seeds: &[[u8; 32]]) -> Vec<bool> {
        let roots: Vec<_> = seeds.iter().map(|seed| root(*seed)).collect();
        crate::support::query_fixture(
            &self.harness.pic,
            self.service,
            self.gateway,
            "_immutableObjectStorageBlobsAreLive",
            (roots,),
        )
        .map(|(reply,)| reply)
        .expect("liveness batch")
    }
    fn delete(&self, roots: Vec<Vec<u8>>) -> Result<(), u32> {
        crate::support::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.operator,
            "run_deletion",
            (roots,),
        )
        .map(|(reply,)| reply)
        .expect("real inter-canister callback")
    }
}

fn root(seed: [u8; 32]) -> Vec<u8> {
    seed.to_vec()
}
fn root_text(seed: [u8; 32]) -> String {
    format!("sha256:{}", hex::encode(seed))
}
fn usage(logical: u128, physical: u128, liability: u128) -> JourneyUsage {
    JourneyUsage {
        logical,
        physical,
        liability,
    }
}

fn upload(id: u8, bytes: &[u8]) -> JourneyUpload {
    let mut leaf = Sha256::new();
    leaf.update(b"icfs-chunk/");
    leaf.update(bytes);
    JourneyUpload {
        id,
        root: leaf.finalize().into(),
        digest: Sha256::digest(bytes).into(),
        bytes: u64::try_from(bytes.len()).expect("small fixture"),
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One connected journey checks the same objects through interruption, release and delayed callbacks"
)]
fn upload_interruption_replay_release_and_delayed_callbacks_share_one_accounting_owner() {
    let f = Fixture::new();
    let a = upload(1, &[21; 100]);
    assert_eq!(f.reserve(f.first, a), Ok(()));
    assert_eq!(f.reserve(f.first, a), Ok(()));
    assert_eq!(f.usage(f.first), Ok(usage(100, 100, 100)));
    assert_eq!(
        f.reserve(f.first, JourneyUpload { bytes: 101, ..a }),
        Err(JourneyFailure::Conflict)
    );
    assert_eq!(f.reserve(f.second, a), Err(JourneyFailure::Conflict));
    assert_eq!(
        f.certificate(f.second, a.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(
        f.certificate(f.first, a.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(f.append(f.first, a, 0, &[21; 100]), Ok(()));
    assert_eq!(
        f.certificate(f.first, a.root),
        Ok(JourneyCertificate {
            method: "upload".to_owned(),
            blob_hash: root_text(a.root)
        })
    );
    // Authority has escaped; interrupt before supplying any completion fact.
    assert_eq!(
        f.certificate(f.first, a.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(
        f.control(f.first, "journey_cancel", a),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(
        f.delete(vec![root(a.root)]),
        Err(RejectCode::CanisterError as u32)
    );
    assert_eq!(f.live(&[a.root, [99; 32], a.root]), vec![true, true, true]);
    assert_eq!(f.usage(f.first), Ok(usage(100, 100, 100)));
    assert_eq!(f.usage(f.first), Ok(usage(100, 100, 100)));
    assert_eq!(
        f.root_control(f.operator, "journey_settle", a.root),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(
        f.root_control(f.second, "journey_release", a.root),
        Err(JourneyFailure::Denied)
    );
    assert_eq!(f.root_control(f.first, "journey_release", a.root), Ok(()));
    assert_eq!(f.root_control(f.first, "journey_release", a.root), Ok(()));
    let delayed = f.pending();
    assert_eq!(delayed, vec![root(a.root)]);
    assert_eq!(
        f.live(&[a.root, [99; 32], a.root]),
        vec![false, true, false]
    );
    assert_eq!(f.usage(f.first), Ok(usage(0, 100, 100)));

    let b = upload(2, &[22; 150]);
    assert_eq!(f.reserve(f.first, b), Ok(()));
    assert_eq!(f.append(f.first, b, 0, &[22; 150]), Ok(()));
    f.certificate(f.first, b.root).expect("second certificate");
    // The first mutation must roll back when the second root still has references.
    assert_eq!(
        f.delete(vec![root(a.root), root(b.root)]),
        Err(RejectCode::CanisterError as u32)
    );
    assert_eq!(f.usage(f.first), Ok(usage(150, 250, 250)));
    assert_eq!(f.pending(), delayed);
    assert_eq!(f.pending(), delayed);
    assert_eq!(
        f.certificate(f.first, a.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(f.delete(vec![root(a.root), root(a.root)]), Ok(()));
    assert_eq!(f.delete(delayed), Ok(()));
    assert_eq!(f.usage(f.first), Ok(usage(150, 150, 250)));
    assert_eq!(f.pending(), Vec::<Vec<u8>>::new());
    assert_eq!(
        f.root_control(f.first, "journey_settle", a.root),
        Err(JourneyFailure::Denied)
    );
    assert_eq!(f.root_control(f.operator, "journey_settle", a.root), Ok(()));
    assert_eq!(f.root_control(f.operator, "journey_settle", a.root), Ok(()));
    assert_eq!(
        f.reserve(f.first, JourneyUpload { id: 3, ..a }),
        Err(JourneyFailure::Conflict)
    );
    assert_eq!(f.usage(f.first), Ok(usage(150, 150, 150)));

    assert_eq!(f.root_control(f.first, "journey_release", b.root), Ok(()));
    let delayed = f.pending();
    assert_eq!(delayed, vec![root(b.root)]);
    let revoked: bool =
        crate::support::update_fixture(&f.harness.pic, f.service, f.operator, "revoke_gateway", ())
            .map(|(reply,)| reply)
            .expect("revoke");
    assert!(revoked);
    assert_eq!(f.delete(delayed), Err(RejectCode::CanisterError as u32));
    assert_eq!(f.delete(vec![]), Err(RejectCode::CanisterError as u32));
    assert_eq!(
        f.root_control(f.operator, "journey_settle", b.root),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(f.usage(f.first), Ok(usage(0, 150, 150)));
    let denied = f
        .harness
        .pic
        .query_call(
            f.service,
            f.gateway,
            "_immutableObjectStorageBlobsToDelete",
            ic_platform::candid::contract::encode_args(()).expect("empty args"),
        )
        .expect_err("revoked gateway");
    assert_eq!(denied.reject_code, RejectCode::CanisterError);
}

#[test]
fn cancellation_and_rejected_authority_preserve_bounds_and_other_tenants() {
    let f = Fixture::new();
    let a = upload(1, &[31; 100]);
    for caller in [f.operator, f.gateway, Principal::anonymous()] {
        assert_eq!(f.reserve(caller, a), Err(JourneyFailure::Denied));
        assert_eq!(f.usage(caller), Err(JourneyFailure::Denied));
    }
    assert_eq!(
        f.reserve(f.first, JourneyUpload { id: 0, ..a }),
        Err(JourneyFailure::InvalidInput)
    );
    let denied = f
        .harness
        .pic
        .update_call(
            f.gateway,
            f.first,
            "run_deletion",
            ic_platform::candid::contract::encode_one(vec![root(a.root)]).expect("callback roots"),
        )
        .expect_err("tenant cannot command the gateway fixture");
    assert_eq!(denied.reject_code, RejectCode::CanisterError);
    assert_eq!(f.reserve(f.first, a), Ok(()));
    assert_eq!(f.reserve(f.second, upload(1, &[32; 100])), Ok(()));
    assert_eq!(f.control(f.first, "journey_cancel", a), Ok(()));
    assert_eq!(f.control(f.first, "journey_cancel", a), Ok(()));
    assert_eq!(
        f.certificate(f.first, a.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(f.usage(f.first), Ok(usage(0, 0, 0)));
    assert_eq!(f.usage(f.second), Ok(usage(100, 100, 100)));
    for caller in [f.first, f.second, f.operator, Principal::anonymous()] {
        let rejected = f
            .harness
            .pic
            .query_call(
                f.service,
                caller,
                "_immutableObjectStorageBlobsAreLive",
                ic_platform::candid::contract::encode_one(Vec::<Vec<u8>>::new())
                    .expect("empty batch"),
            )
            .expect_err("gateway-only even for empty batch");
        assert_eq!(rejected.reject_code, RejectCode::CanisterError);
    }
    for batch in [vec![vec![1; 31]], vec![root(a.root); 9]] {
        let rejected = f
            .harness
            .pic
            .query_call(
                f.service,
                f.gateway,
                "_immutableObjectStorageBlobsAreLive",
                ic_platform::candid::contract::encode_one(batch).expect("invalid batch"),
            )
            .expect_err("bounded valid binary roots required");
        assert_eq!(rejected.reject_code, RejectCode::CanisterError);
    }
    assert_eq!(
        f.reserve(
            f.first,
            JourneyUpload {
                bytes: 10 * 1024 * 1024 + 1,
                ..a
            }
        ),
        Err(JourneyFailure::InvalidInput)
    );
    assert_eq!(f.usage(f.first), Ok(usage(0, 0, 0)));
    assert_eq!(f.usage(f.second), Ok(usage(100, 100, 100)));
}

#[test]
fn content_verification_binds_authority_to_exact_bytes_and_tenant() {
    let f = Fixture::new();
    let a = upload(1, b"abc");
    // Cross-check the test declaration against the retained independent JS vector.
    let vectors: serde_json::Value = serde_json::from_str(include_str!(
        "../../../crates/ic-blob-storage-contracts/tests/fixtures/caffeine-hashing/vectors.json"
    ))
    .expect("independent vectors");
    let vector = vectors["vectors"]
        .as_array()
        .expect("array")
        .iter()
        .find(|v| v["name"] == "abc-no-metadata")
        .expect("abc vector");
    assert_eq!(root_text(a.root), vector["provider_root"]);
    assert_eq!(root_text(a.digest), vector["raw_digest"]);
    assert_eq!(f.reserve(f.first, a), Ok(()));
    assert_eq!(
        f.certificate(f.first, a.root),
        Err(RejectCode::CanisterError)
    );
    for caller in [f.second, f.gateway, f.operator, Principal::anonymous()] {
        assert_eq!(f.append(caller, a, 0, b"abc"), Err(JourneyFailure::Denied));
    }
    for invalid in [b"".as_slice(), b"ab", b"abcd", b"abd", &[1; 401]] {
        assert_eq!(
            f.append(f.first, a, 0, invalid),
            Err(JourneyFailure::ContentMismatch)
        );
        assert_eq!(
            f.certificate(f.first, a.root),
            Err(RejectCode::CanisterError)
        );
        assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    }
    assert_eq!(f.append(f.first, a, 0, b"abc"), Ok(()));
    assert_eq!(f.append(f.first, a, 0, b"abc"), Ok(()));
    assert_eq!(f.reserve(f.first, a), Ok(()));
    // Rejected bytes cannot replace a successful immutable verification fact.
    assert_eq!(
        f.append(f.first, a, 0, b"abd"),
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.root_control(f.first, "journey_release", a.root),
        Err(JourneyFailure::InvalidPhase)
    );
    f.certificate(f.first, a.root)
        .expect("verified certificate");
    assert_eq!(
        f.append(f.first, a, 0, b"abc"),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    assert_eq!(f.usage(f.second), Ok(usage(0, 0, 0)));
}

#[test]
fn mismatched_raw_digest_cannot_issue_authority_or_rebind_the_reservation() {
    let f = Fixture::new();
    let correct = upload(1, b"xyz");
    let wrong = JourneyUpload {
        digest: [0; 32],
        ..correct
    };
    assert_eq!(f.reserve(f.first, wrong), Ok(()));
    assert_eq!(
        f.append(f.first, wrong, 0, b"xyz"),
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.certificate(f.first, wrong.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(f.reserve(f.first, correct), Err(JourneyFailure::Conflict));
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    assert_eq!(f.control(f.first, "journey_cancel", wrong), Ok(()));
    assert_eq!(
        f.append(f.first, wrong, 0, b"xyz"),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(f.usage(f.first), Ok(usage(0, 0, 0)));
}
