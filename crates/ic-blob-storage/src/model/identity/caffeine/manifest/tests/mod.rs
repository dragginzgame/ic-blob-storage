use super::*;
use crate::model::identity::{caffeine::CaffeineHashLimit, verification::ContentVerifier};

fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("bound")
}

fn limits() -> CaffeineManifestLimits {
    CaffeineManifestLimits {
        max_content_bytes: NonZeroU64::new(ContentVerifier::MAX_BYTES).expect("bound"),
        max_chunks: bound(2),
        max_headers: bound(1),
        max_header_bytes: bound(5),
    }
}

fn abc() -> (ProviderRootHash, CaffeineChunkHash) {
    let value = "sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";
    (
        value.parse().expect("client root"),
        value.parse().expect("client chunk"),
    )
}

#[test]
fn representation_and_chunk_identity_are_explicit() {
    let (_, chunk) = abc();
    assert_eq!(chunk.to_string().parse(), Ok(chunk));
    assert_eq!(
        CaffeineChunkHash::try_from(chunk.as_bytes().as_slice()),
        Ok(chunk)
    );
    for length in [0, 31, 33] {
        assert_eq!(
            CaffeineChunkHash::try_from(vec![0; length].as_slice()),
            Err(HashParseError::InvalidByteLength { actual: length })
        );
    }
    assert_eq!(
        "bad".parse::<CaffeineChunkHash>(),
        Err(HashParseError::InvalidPrefix)
    );
}

#[test]
fn admission_checks_counts_and_budgets_before_tree_processing() {
    let (root, chunk) = abc();
    let create = |length, chunks: &[CaffeineChunkHash], limits| {
        CaffeineChunkManifest::new(root, length, chunks, &[], limits)
    };
    assert_eq!(
        create(0, &[], limits()),
        Err(CaffeineManifestError::Hash(
            CaffeineHashError::EmptyContentUnqualified
        ))
    );
    for length in [ContentVerifier::MAX_BYTES + 1, u64::MAX] {
        assert_eq!(
            create(length, &[chunk], limits()),
            Err(CaffeineManifestError::Hash(CaffeineHashError::Limit(
                CaffeineHashLimit::ContentBytes
            )))
        );
    }
    assert_eq!(
        create(
            3,
            &[chunk],
            CaffeineManifestLimits {
                max_content_bytes: NonZeroU64::new(2).expect("bound"),
                ..limits()
            }
        ),
        Err(CaffeineManifestError::Hash(CaffeineHashError::Limit(
            CaffeineHashLimit::ContentBytes
        )))
    );
    assert_eq!(
        create(3, &[chunk; 3], limits()),
        Err(CaffeineManifestError::TooManyChunks)
    );
    for count in [0, 2] {
        assert_eq!(
            create(3, &vec![chunk; count], limits()),
            Err(CaffeineManifestError::ChunkCountMismatch {
                expected: 1,
                actual: count
            })
        );
    }
    assert_eq!(
        create(ContentVerifier::MAX_BYTES, &[chunk], limits()),
        Err(CaffeineManifestError::ChunkCountMismatch {
            expected: ContentVerifier::MAX_BYTES.div_ceil(CAFFEINE_CHUNK_BYTES as u64),
            actual: 1
        })
    );
    let invalid = CaffeineChunkHash::try_from([0; 32].as_slice()).expect("representation");
    assert_eq!(
        create(3, &[invalid], limits()),
        Err(CaffeineManifestError::RootMismatch)
    );
    assert!(
        create(
            3,
            &[chunk],
            CaffeineManifestLimits {
                max_chunks: bound(1),
                ..limits()
            }
        )
        .is_ok()
    );
}

#[test]
fn metadata_limits_and_root_binding_cannot_be_skipped() {
    let (root, chunk) = abc();
    let header = CaffeineHeader {
        name: "X",
        value: "a",
    };
    assert_eq!(
        CaffeineChunkManifest::new(root, 3, &[chunk], &[header; 2], limits()),
        Err(CaffeineManifestError::Hash(CaffeineHashError::Limit(
            CaffeineHashLimit::HeaderCount
        )))
    );
    assert_eq!(
        CaffeineChunkManifest::new(
            root,
            3,
            &[chunk],
            &[CaffeineHeader {
                name: " X ",
                value: "a"
            }],
            limits()
        ),
        Err(CaffeineManifestError::Hash(CaffeineHashError::Limit(
            CaffeineHashLimit::HeaderBytes
        )))
    );
    assert_eq!(
        CaffeineChunkManifest::new(root, 3, &[chunk], &[header], limits()),
        Err(CaffeineManifestError::RootMismatch)
    );
}

#[test]
fn leaf_checks_reject_corruption_and_length_lies_without_remembering_success() {
    let (root, chunk) = abc();
    let manifest = CaffeineChunkManifest::new(root, 3, &[chunk], &[], limits()).expect("manifest");
    let before = manifest.clone();
    assert_eq!(manifest.root(), root);
    assert_eq!(manifest.content_bytes(), 3);
    assert_eq!(manifest.chunk_count(), 1);
    assert_eq!(manifest.chunk_bytes(0), Ok(3));
    for index in [1, u64::MAX] {
        assert_eq!(
            manifest.chunk_bytes(index),
            Err(CaffeineManifestError::ChunkIndexOutOfRange { index })
        );
        assert_eq!(
            manifest.verify_chunk(index, b"abc"),
            Err(CaffeineManifestError::ChunkIndexOutOfRange { index })
        );
    }
    for bytes in [b"".as_slice(), b"ab", b"abcd"] {
        assert_eq!(
            manifest.verify_chunk(0, bytes),
            Err(CaffeineManifestError::ChunkLengthMismatch {
                expected: 3,
                actual: bytes.len()
            })
        );
    }
    assert_eq!(
        manifest.verify_chunk(0, b"abd"),
        Err(CaffeineManifestError::ChunkHashMismatch)
    );
    assert_eq!(manifest.verify_chunk(0, b"abc"), Ok(()));
    assert_eq!(manifest.verify_chunk(0, b"abc"), Ok(()));
    assert_eq!(
        manifest.verify_chunk(0, b"abd"),
        Err(CaffeineManifestError::ChunkHashMismatch)
    );
    assert_eq!(manifest, before);
    // The same one-leaf root does not independently authenticate a length.
    let wrong_length =
        CaffeineChunkManifest::new(root, 4, &[chunk], &[], limits()).expect("consistent tree");
    assert_eq!(
        wrong_length.verify_chunk(0, b"abc"),
        Err(CaffeineManifestError::ChunkLengthMismatch {
            expected: 4,
            actual: 3
        })
    );
    assert_eq!(
        wrong_length.verify_chunk(0, b"abcd"),
        Err(CaffeineManifestError::ChunkHashMismatch)
    );
}
