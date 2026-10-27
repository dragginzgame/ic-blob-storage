//! Actual Wasm execution of byte verification using independently pinned vectors.

#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_pocketic_tests::fixture_calls;
mod support;

use blob_test_protocol::content::{ContentProbeCase, ContentProbeFailure, ContentProbeReport};
use ic_platform::pocket_ic::CanisterSettings;
use ic_testkit::fake_principal;
use support::{Harness, fixture_path};

#[test]
fn wasm_verifies_chunk_boundaries_metadata_and_rejection_recovery_within_work_budget() {
    let harness = Harness::new();
    let pic = &harness.pic;
    let operator = fake_principal(4);
    let tenant = fake_principal(1);
    let canister = pic.create_canister_with_settings(
        Some(operator),
        Some(CanisterSettings {
            controllers: Some(vec![operator]),
            ..CanisterSettings::default()
        }),
    );
    pic.install_canister(
        canister,
        std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).expect("Wasm"),
        blob_test_protocol::fixture_wire::encode_args(
            "fixture_init",
            (tenant, fake_principal(2), fake_principal(3), operator),
        )
        .expect("init"),
        Some(operator),
    );
    let denied: Result<ContentProbeReport, ContentProbeFailure> = fixture_calls::update_fixture(
        pic,
        canister,
        tenant,
        "probe_content",
        (ContentProbeCase::FullChunk,),
    )
    .map(|(reply,)| reply)
    .expect("denial");
    assert_eq!(denied, Err(ContentProbeFailure::Denied));
    let vectors: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../crates/ic-blob-storage-contracts/tests/fixtures/caffeine-hashing/vectors.json"
    )))
    .expect("vectors");
    for (case, name) in [
        (ContentProbeCase::FullChunk, "pattern-1048576"),
        (ContentProbeCase::PartialFinalChunk, "pattern-1048577"),
        (ContentProbeCase::UnicodeMetadata, "ecmascript-metadata"),
    ] {
        let expected = vectors["vectors"]
            .as_array()
            .expect("vector list")
            .iter()
            .find(|vector| vector["name"] == name)
            .expect("selected vector");
        let result: Result<ContentProbeReport, ContentProbeFailure> =
            fixture_calls::update_fixture(pic, canister, operator, "probe_content", (case,))
                .map(|(reply,)| reply)
                .expect("Wasm execution");
        let report = result.expect("verified identities");
        assert_eq!(
            report.raw_digest,
            expected["raw_digest"].as_str().expect("raw digest")
        );
        assert_eq!(
            report.provider_root,
            expected["provider_root"].as_str().expect("root")
        );
        assert_eq!(
            report.verified_bytes,
            expected["bytes"].as_u64().expect("bytes")
        );
        // A deliberate local regression budget, not a claim about production throughput.
        assert!(report.max_append_instructions > 0);
        assert!(
            report.max_append_instructions <= 1_000_000_000,
            "{name}: {report:?}"
        );
        println!(
            "{name}: max ordered append {} instructions",
            report.max_append_instructions
        );
    }
    for (case, error) in [
        (
            ContentProbeCase::WrongDigest,
            ContentProbeFailure::DigestMismatch,
        ),
        (ContentProbeCase::Truncated, ContentProbeFailure::Incomplete),
    ] {
        let result: Result<ContentProbeReport, ContentProbeFailure> =
            fixture_calls::update_fixture(pic, canister, operator, "probe_content", (case,))
                .map(|(reply,)| reply)
                .expect("expected rejection");
        assert_eq!(result, Err(error));
    }
}
