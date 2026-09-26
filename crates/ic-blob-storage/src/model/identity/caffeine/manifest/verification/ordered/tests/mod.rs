use super::super::tests::abc as manifest;
use super::*;

fn abc(expected: ContentDigest) -> CaffeineOrderedChunkVerifier {
    CaffeineOrderedChunkVerifier::new(manifest(), expected)
}

#[test]
fn exact_length_is_required_and_a_consistent_manifest_cannot_override_raw_digest() {
    let digest = ContentDigest::compute(b"abc");
    assert_eq!(
        abc(digest).finish(),
        Err(CaffeineOrderedVerificationError::Content(
            ContentVerificationError::Incomplete {
                expected_bytes: 3,
                received_bytes: 0
            }
        ))
    );
    let wrong = ContentDigest::compute(b"abd");
    let mut verifier = abc(wrong);
    verifier
        .append_chunk(0, b"abc")
        .expect("valid leaf does not prove raw digest");
    assert_eq!(verifier.remaining_bytes(), 0);
    assert_eq!(verifier.next_chunk(), 1);
    assert_eq!(
        verifier.finish(),
        Err(CaffeineOrderedVerificationError::Content(
            ContentVerificationError::DigestMismatch {
                expected: wrong,
                actual: digest
            }
        ))
    );
}

#[test]
fn rejected_chunks_never_poison_the_raw_hash_or_advance_the_prefix() {
    let digest = ContentDigest::compute(b"abc");
    let mut verifier = abc(digest);
    for (index, bytes, expected) in [
        (
            u64::MAX,
            b"".as_slice(),
            CaffeineOrderedVerificationError::UnexpectedChunk {
                expected: 0,
                actual: u64::MAX,
            },
        ),
        (
            0,
            b"ab",
            CaffeineOrderedVerificationError::Manifest(
                CaffeineManifestError::ChunkLengthMismatch {
                    expected: 3,
                    actual: 2,
                },
            ),
        ),
        (
            0,
            b"abcd",
            CaffeineOrderedVerificationError::Manifest(
                CaffeineManifestError::ChunkLengthMismatch {
                    expected: 3,
                    actual: 4,
                },
            ),
        ),
        (
            0,
            b"abd",
            CaffeineOrderedVerificationError::Manifest(CaffeineManifestError::ChunkHashMismatch),
        ),
    ] {
        assert_eq!(verifier.append_chunk(index, bytes), Err(expected));
        assert_eq!(verifier.next_chunk(), 0);
        assert_eq!(verifier.verified_bytes(), 0);
        assert_eq!(verifier.remaining_bytes(), 3);
    }
    verifier.append_chunk(0, b"abc").expect("valid retry");
    assert_eq!(
        verifier.append_chunk(0, b"abc"),
        Err(CaffeineOrderedVerificationError::UnexpectedChunk {
            expected: 1,
            actual: 0
        })
    );
    assert_eq!(
        verifier.append_chunk(1, b""),
        Err(CaffeineOrderedVerificationError::Manifest(
            CaffeineManifestError::ChunkIndexOutOfRange { index: 1 }
        ))
    );
    let root = verifier.manifest().root();
    assert_eq!(
        verifier.finish(),
        Ok(CaffeineContentHashes {
            content_digest: digest,
            provider_root: root
        })
    );
}
