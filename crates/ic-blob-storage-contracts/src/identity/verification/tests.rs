use super::*;

#[test]
fn chunk_boundaries_do_not_change_verified_digest() {
    let content: Vec<u8> = (0..=255).cycle().take(1_025).collect();
    let expected = ContentDigest::compute(&content);
    // Include boundaries around the SHA-256 block and padding lengths.
    for chunk_size in [1, 2, 7, 55, 56, 63, 64, 65, 127, 128, 1_024, 1_025] {
        let mut verifier = ContentVerifier::new(expected, 1_025).expect("valid length");
        let mut offset = 0;
        for chunk in content.chunks(chunk_size) {
            verifier.append(offset, &[]).expect("empty chunk");
            verifier.append(offset, chunk).expect("ordered chunk");
            offset += u64::try_from(chunk.len()).expect("bounded chunk");
            assert_eq!(verifier.received_bytes(), offset);
            assert_eq!(verifier.remaining_bytes(), 1_025 - offset);
        }
        assert_eq!(verifier.finish(), Ok(expected));
    }
}

#[test]
fn incremental_hash_matches_independent_million_byte_vector() {
    // Standard SHA-256 vector: one million ASCII 'a' bytes.
    let expected: ContentDigest =
        "sha256:cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
            .parse()
            .expect("fixed vector");
    let mut verifier = ContentVerifier::new(expected, 1_000_000).expect("valid length");
    let chunk = [b'a'; 1_000];
    for index in 0..1_000 {
        verifier
            .append(index * 1_000, &chunk)
            .expect("ordered chunk");
    }
    assert_eq!(verifier.finish(), Ok(expected));
}

#[test]
fn rejected_chunks_leave_hash_and_offset_unchanged() {
    let expected = ContentDigest::compute(b"abc");
    let mut verifier = ContentVerifier::new(expected, 3).expect("valid length");
    verifier.append(0, b"a").expect("first chunk");
    for (offset, chunk, error) in [
        (
            0,
            b"a".as_slice(),
            ContentVerificationError::UnexpectedOffset {
                expected_offset: 1,
                actual_offset: 0,
            },
        ),
        (
            2,
            b"c".as_slice(),
            ContentVerificationError::UnexpectedOffset {
                expected_offset: 1,
                actual_offset: 2,
            },
        ),
        (
            u64::MAX,
            b"".as_slice(),
            ContentVerificationError::UnexpectedOffset {
                expected_offset: 1,
                actual_offset: u64::MAX,
            },
        ),
        (
            1,
            b"bcd".as_slice(),
            ContentVerificationError::ChunkExceedsLength {
                remaining_bytes: 2,
                chunk_bytes: 3,
            },
        ),
    ] {
        assert_eq!(verifier.append(offset, chunk), Err(error));
        assert_eq!(verifier.received_bytes(), 1);
        assert_eq!(verifier.remaining_bytes(), 2);
    }
    verifier.append(1, b"bc").expect("correct remainder");
    assert_eq!(
        verifier.append(3, b"d"),
        Err(ContentVerificationError::ChunkExceedsLength {
            remaining_bytes: 0,
            chunk_bytes: 1
        })
    );
    assert_eq!(verifier.finish(), Ok(expected));
}

#[test]
fn truncated_and_same_length_corrupt_content_cannot_finish() {
    let expected = ContentDigest::compute(b"abc");
    let mut truncated = ContentVerifier::new(expected, 3).expect("valid length");
    truncated.append(0, b"ab").expect("prefix");
    assert_eq!(
        truncated.finish(),
        Err(ContentVerificationError::Incomplete {
            expected_bytes: 3,
            received_bytes: 2
        })
    );

    for content in [b"abd", b"bac", b"abb"] {
        let mut corrupt = ContentVerifier::new(expected, 3).expect("valid length");
        corrupt.append(0, content).expect("declared length fits");
        assert_eq!(
            corrupt.finish(),
            Err(ContentVerificationError::DigestMismatch {
                expected,
                actual: ContentDigest::compute(content)
            })
        );
    }
}

#[test]
fn empty_content_still_requires_the_expected_digest() {
    let expected = ContentDigest::compute(b"");
    let mut empty = ContentVerifier::new(expected, 0).expect("valid empty length");
    empty.append(0, b"").expect("empty chunk");
    assert_eq!(
        empty.append(0, b"a"),
        Err(ContentVerificationError::ChunkExceedsLength {
            remaining_bytes: 0,
            chunk_bytes: 1
        })
    );
    assert_eq!(empty.finish(), Ok(expected));
    let wrong = ContentDigest::compute(b"a");
    assert_eq!(
        ContentVerifier::new(wrong, 0)
            .expect("valid length")
            .finish(),
        Err(ContentVerificationError::DigestMismatch {
            expected: wrong,
            actual: expected
        })
    );
}

#[test]
fn declared_lengths_respect_the_algorithm_bound_without_allocating_content() {
    let digest = ContentDigest::compute(b"");
    let mut largest = ContentVerifier::new(digest, ContentVerifier::MAX_BYTES)
        .expect("maximum representable length");
    largest.append(0, b"a").expect("bounded first byte");
    assert_eq!(largest.remaining_bytes(), ContentVerifier::MAX_BYTES - 1);
    assert_eq!(
        largest.finish(),
        Err(ContentVerificationError::Incomplete {
            expected_bytes: ContentVerifier::MAX_BYTES,
            received_bytes: 1
        })
    );
    for length in [ContentVerifier::MAX_BYTES + 1, u64::MAX] {
        assert_eq!(
            ContentVerifier::new(digest, length).err(),
            Some(ContentVerificationError::ContentTooLong {
                declared_bytes: length
            })
        );
    }
}
