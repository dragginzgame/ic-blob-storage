use std::num::{NonZeroU64, NonZeroUsize};

use super::*;
use crate::model::identity::caffeine::manifest::CaffeineManifestLimits;

pub(super) fn abc() -> CaffeineChunkManifest {
    // Published-client vector for the one-leaf, metadata-free "abc" tree.
    let hash = "sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";
    CaffeineChunkManifest::new(
        hash.parse().expect("root"),
        3,
        &[hash.parse().expect("leaf")],
        &[],
        CaffeineManifestLimits {
            max_content_bytes: NonZeroU64::new(3).expect("bound"),
            max_chunks: NonZeroUsize::new(1).expect("bound"),
            max_headers: NonZeroUsize::new(1).expect("bound"),
            max_header_bytes: NonZeroUsize::new(1).expect("bound"),
        },
    )
    .expect("independent manifest")
}

#[test]
fn failures_preserve_coverage_before_and_after_success() {
    let mut verifier = CaffeineChunkVerifier::new(abc());
    assert_eq!(
        verifier.progress(),
        CaffeineChunkProgress {
            verified_chunks: 0,
            verified_bytes: 0,
            total_chunks: 1,
            total_bytes: 3,
        }
    );
    assert!(!verifier.progress().all_chunks_verified());
    for verified in [false, true] {
        if verified {
            verifier.verify_chunk(0, b"abc").expect("valid bytes");
        }
        let before = verifier.progress();
        for (index, bytes, expected) in [
            (
                1,
                b"abc".as_slice(),
                CaffeineManifestError::ChunkIndexOutOfRange { index: 1 },
            ),
            (
                u64::MAX,
                b"",
                CaffeineManifestError::ChunkIndexOutOfRange { index: u64::MAX },
            ),
            (
                0,
                b"ab",
                CaffeineManifestError::ChunkLengthMismatch {
                    expected: 3,
                    actual: 2,
                },
            ),
            (
                0,
                b"abcd",
                CaffeineManifestError::ChunkLengthMismatch {
                    expected: 3,
                    actual: 4,
                },
            ),
            (0, b"abd", CaffeineManifestError::ChunkHashMismatch),
        ] {
            assert_eq!(verifier.verify_chunk(index, bytes), Err(expected));
            assert_eq!(verifier.progress(), before);
            assert_eq!(verifier.is_verified(0), Ok(verified));
        }
        assert_eq!(
            verifier.is_verified(u64::MAX),
            Err(CaffeineManifestError::ChunkIndexOutOfRange { index: u64::MAX })
        );
    }
    let completed = verifier.progress();
    assert!(completed.all_chunks_verified());
    assert_eq!(verifier.verify_chunk(0, b"abc"), Ok(completed));
    let fresh = CaffeineChunkVerifier::new(verifier.manifest().clone());
    assert_eq!(fresh.progress().verified_bytes, 0);
    assert_eq!(fresh.is_verified(0), Ok(false));
}
