//! Fixed-vector experiment over the real Wasm library implementation.
//! Assertions live only in this unpublished fixture, never production cfg(test).

use std::num::NonZeroU64;

use blob_test_protocol::content::{ContentProbeCase, ContentProbeFailure, ContentProbeReport};
use ic_blob_storage::model::identity::{
    ContentDigest,
    caffeine::{
        CAFFEINE_CHUNK_BYTES, CaffeineHeader,
        manifest::{
            CaffeineChunkManifest, CaffeineManifestError, CaffeineManifestLimits,
            verification::{
                CaffeineChunkVerifier,
                missing::MissingChunkPageLimits,
                ordered::{CaffeineOrderedChunkVerifier, CaffeineOrderedVerificationError},
            },
        },
    },
    verification::ContentVerificationError,
};
use serde::Deserialize;

use super::bound;

#[derive(Deserialize)]
struct Vectors {
    vectors: Vec<Vector>,
}

#[derive(Deserialize)]
struct Vector {
    name: String,
    bytes: usize,
    pattern: String,
    headers: Vec<Header>,
    raw_digest: String,
    provider_root: String,
    chunk_hashes: Vec<String>,
}

#[derive(Deserialize)]
struct Header {
    name: String,
    value: String,
}

fn vector(case: ContentProbeCase) -> Vector {
    let name = match case {
        ContentProbeCase::FullChunk => "pattern-1048576",
        ContentProbeCase::UnicodeMetadata => "ecmascript-metadata",
        ContentProbeCase::PartialFinalChunk
        | ContentProbeCase::WrongDigest
        | ContentProbeCase::Truncated => "pattern-1048577",
    };
    let vectors: Vectors = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../crates/ic-blob-storage/tests/fixtures/caffeine-hashing/vectors.json"
    )))
    .expect("pinned independent client vectors");
    vectors
        .vectors
        .into_iter()
        .find(|v| v.name == name)
        .expect("selected vector")
}

fn manifest(vector: &Vector) -> CaffeineChunkManifest {
    let leaves: Vec<_> = vector
        .chunk_hashes
        .iter()
        .map(|value| value.parse().expect("leaf"))
        .collect();
    let headers: Vec<_> = vector
        .headers
        .iter()
        .map(|header| CaffeineHeader {
            name: &header.name,
            value: &header.value,
        })
        .collect();
    CaffeineChunkManifest::new(
        vector.provider_root.parse().expect("root"),
        u64::try_from(vector.bytes).expect("length"),
        &leaves,
        &headers,
        CaffeineManifestLimits {
            max_content_bytes: NonZeroU64::new(1_048_577).expect("bound"),
            max_chunks: bound(2),
            max_headers: bound(8),
            max_header_bytes: bound(1024),
        },
    )
    .expect("bounded independent manifest")
}

fn bytes(vector: &Vector, index: usize) -> Vec<u8> {
    let offset = index * CAFFEINE_CHUNK_BYTES;
    let count = (vector.bytes - offset).min(CAFFEINE_CHUNK_BYTES);
    (offset..offset + count)
        .map(|at| match vector.pattern.as_str() {
            "abc" => b"abc"[at],
            "linear_mod251" => u8::try_from((at * 31 + 7) % 251).expect("pattern byte"),
            _ => panic!("unselected vector pattern"),
        })
        .collect()
}

fn coverage(vector: &Vector) -> u64 {
    let mut verifier = CaffeineChunkVerifier::new(manifest(vector));
    for index in (0..vector.chunk_hashes.len()).rev() {
        let index64 = u64::try_from(index).expect("index");
        let data = bytes(vector, index);
        let after = verifier
            .verify_chunk(index64, &data)
            .expect("reverse leaf check");
        assert_eq!(verifier.verify_chunk(index64, &data), Ok(after));
    }
    assert!(verifier.progress().all_chunks_verified());
    let page = verifier
        .missing_chunks(
            0,
            MissingChunkPageLimits {
                max_scan: bound(1),
                max_results: bound(1),
            },
        )
        .expect("page");
    assert!(page.chunks.is_empty());
    assert_eq!(
        page.next_index,
        (vector.chunk_hashes.len() > 1).then_some(1)
    );
    verifier.progress().verified_bytes
}

pub(crate) fn run(case: ContentProbeCase) -> Result<ContentProbeReport, ContentProbeFailure> {
    let vector = vector(case);
    let verified_bytes = coverage(&vector);
    let expected = if matches!(case, ContentProbeCase::WrongDigest) {
        ContentDigest::compute(b"deliberately different")
    } else {
        vector.raw_digest.parse().expect("raw digest")
    };
    let mut ordered = CaffeineOrderedChunkVerifier::new(manifest(&vector), expected);
    let mut max_append_instructions = 0;
    for index in 0..vector.chunk_hashes.len() {
        if matches!(case, ContentProbeCase::Truncated) && index + 1 == vector.chunk_hashes.len() {
            break;
        }
        let index64 = u64::try_from(index).expect("index");
        let mut data = bytes(&vector, index);
        let before = ordered.verified_bytes();
        data[0] ^= 1;
        assert_eq!(
            ordered.append_chunk(index64, &data),
            Err(CaffeineOrderedVerificationError::Manifest(
                CaffeineManifestError::ChunkHashMismatch
            ))
        );
        assert_eq!(ordered.verified_bytes(), before);
        data[0] ^= 1;
        let start = ic_cdk::api::instruction_counter();
        ordered
            .append_chunk(index64, &data)
            .expect("correct leaf retry");
        max_append_instructions =
            max_append_instructions.max(ic_cdk::api::instruction_counter() - start);
        assert_eq!(
            ordered.append_chunk(index64, &data),
            Err(CaffeineOrderedVerificationError::UnexpectedChunk {
                expected: index64 + 1,
                actual: index64
            })
        );
    }
    let hashes = ordered.finish().map_err(|error| match error {
        CaffeineOrderedVerificationError::Content(ContentVerificationError::Incomplete {
            ..
        }) => ContentProbeFailure::Incomplete,
        CaffeineOrderedVerificationError::Content(ContentVerificationError::DigestMismatch {
            ..
        }) => ContentProbeFailure::DigestMismatch,
        _ => panic!("unexpected fixture finalization failure"),
    })?;
    Ok(ContentProbeReport {
        raw_digest: hashes.content_digest.to_string(),
        provider_root: hashes.provider_root.to_string(),
        verified_bytes,
        max_append_instructions,
    })
}
