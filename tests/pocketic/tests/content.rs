//! Actual Wasm execution of byte verification using independently pinned vectors.

#![cfg(not(target_family = "wasm"))]

mod support;

use blob_test_protocol::content::{ContentProbeCase, ContentProbeFailure, ContentProbeReport};
use ic_testkit::{Fake, pic::CandidCallExt, pocket_ic::CanisterSettings};
use support::{Harness, fixture_path};

#[test]
fn wasm_verifies_chunk_boundaries_metadata_and_rejection_recovery_within_work_budget() {
    let harness = Harness::new();
    let pic = &harness.pic;
    let operator = Fake::principal(4);
    let tenant = Fake::principal(1);
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
        candid::encode_args((tenant, Fake::principal(2), Fake::principal(3), operator))
            .expect("init"),
        Some(operator),
    );
    let denied: Result<ContentProbeReport, ContentProbeFailure> = pic
        .update_candid_as(
            canister,
            tenant,
            "probe_content",
            (ContentProbeCase::FullChunk,),
        )
        .expect("denial");
    assert_eq!(denied, Err(ContentProbeFailure::Denied));
    let vectors: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../crates/ic-blob-storage/tests/fixtures/caffeine-hashing/vectors.json"
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
        let result: Result<ContentProbeReport, ContentProbeFailure> = pic
            .update_candid_as(canister, operator, "probe_content", (case,))
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
        let result: Result<ContentProbeReport, ContentProbeFailure> = pic
            .update_candid_as(canister, operator, "probe_content", (case,))
            .expect("expected rejection");
        assert_eq!(result, Err(error));
    }
}
