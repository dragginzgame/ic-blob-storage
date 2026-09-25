//! In-memory verification of exact raw bytes against a claimed length and digest.
//!
//! This owns no upload session, persisted checkpoint or provider hash tree.

use sha2::{Digest, Sha256};
use thiserror::Error;

use super::ContentDigest;

/// Incrementally verifies raw content without retaining the complete object.
///
/// Each chunk must start at the next expected byte offset. Rejected chunks do
/// not change the offset or hash state. Successfully appended bytes cannot be
/// replaced; start a new verifier if those bytes are later found to be wrong.
/// Completion consumes the verifier and requires both exact length and digest.
///
/// The caller must separately enforce service object/session limits and supply
/// a trusted expected digest. A digest from the same untrusted source as the
/// bytes only establishes consistency, not authenticity or tenant authority.
/// This transient state cannot resume after process/canister restart.
///
/// ```
/// use ic_blob_storage::model::identity::{ContentDigest, verification::ContentVerifier};
///
/// let expected: ContentDigest =
///     "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".parse()?;
/// let mut verifier = ContentVerifier::new(expected, 3)?;
/// verifier.append(0, b"a")?;
/// verifier.append(1, b"bc")?;
/// assert_eq!(verifier.finish()?, expected);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct ContentVerifier {
    expected_digest: ContentDigest,
    expected_bytes: u64,
    received_bytes: u64,
    hasher: Sha256,
}

impl ContentVerifier {
    /// Largest whole-byte message shorter than SHA-256's 2^64-bit length limit.
    ///
    /// This is an algorithm bound, not the service's object-size limit. Creating
    /// a verifier does not allocate memory proportional to the declared length.
    pub const MAX_BYTES: u64 = u64::MAX / 8;

    /// Start verification against a fixed expected raw digest and byte length.
    ///
    /// # Errors
    /// Returns [`ContentVerificationError::ContentTooLong`] if the declared
    /// length exceeds [`Self::MAX_BYTES`].
    pub fn new(
        expected_digest: ContentDigest,
        expected_bytes: u64,
    ) -> Result<Self, ContentVerificationError> {
        if expected_bytes > Self::MAX_BYTES {
            return Err(ContentVerificationError::ContentTooLong {
                declared_bytes: expected_bytes,
            });
        }
        Ok(Self {
            expected_digest,
            expected_bytes,
            received_bytes: 0,
            hasher: Sha256::new(),
        })
    }

    /// Number of bytes accepted so far, also the required next chunk offset.
    #[must_use]
    pub const fn received_bytes(&self) -> u64 {
        self.received_bytes
    }

    /// Bytes still required before completion can check the digest.
    #[must_use]
    pub const fn remaining_bytes(&self) -> u64 {
        self.expected_bytes - self.received_bytes
    }

    /// Append a whole chunk at the required offset into the fixed-size hash state.
    ///
    /// Empty chunks at the required offset are accepted without advancing it.
    /// Replaying an accepted nonempty chunk at its old offset is an error.
    ///
    /// # Errors
    /// Rejects an unexpected offset or a chunk larger than the remaining declared
    /// length. Every rejection leaves both hash state and byte count unchanged.
    pub fn append(&mut self, offset: u64, chunk: &[u8]) -> Result<(), ContentVerificationError> {
        if offset != self.received_bytes {
            return Err(ContentVerificationError::UnexpectedOffset {
                expected_offset: self.received_bytes,
                actual_offset: offset,
            });
        }
        let overflow = ContentVerificationError::ChunkExceedsLength {
            remaining_bytes: self.remaining_bytes(),
            chunk_bytes: chunk.len(),
        };
        let chunk_bytes = u64::try_from(chunk.len()).map_err(|_| overflow)?;
        if chunk_bytes > self.remaining_bytes() {
            return Err(overflow);
        }
        self.hasher.update(chunk);
        // Admission above proves the sum is <= expected_bytes <= MAX_BYTES.
        self.received_bytes += chunk_bytes;
        Ok(())
    }

    /// Consume the verifier and return the digest only after full verification.
    ///
    /// Check [`Self::remaining_bytes`] before finishing if more input may arrive.
    /// Success proves only that the supplied byte sequence has the claimed raw
    /// length and digest; it does not prove provider storage or upload completion.
    ///
    /// # Errors
    /// Rejects incomplete content or a mismatched digest. The verifier is consumed
    /// on either result; no reusable success/checkpoint token is produced.
    pub fn finish(self) -> Result<ContentDigest, ContentVerificationError> {
        if self.received_bytes != self.expected_bytes {
            return Err(ContentVerificationError::Incomplete {
                expected_bytes: self.expected_bytes,
                received_bytes: self.received_bytes,
            });
        }
        let actual = ContentDigest(self.hasher.finalize().into());
        if actual != self.expected_digest {
            return Err(ContentVerificationError::DigestMismatch {
                expected: self.expected_digest,
                actual,
            });
        }
        Ok(actual)
    }
}

/// Typed failure to admit a chunk or verify complete raw content.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ContentVerificationError {
    /// The declared length exceeds the SHA-256 message-length bound.
    #[error("declared content length exceeds SHA-256's whole-byte limit")]
    ContentTooLong {
        /// Requested total byte length.
        declared_bytes: u64,
    },
    /// A chunk would skip or repeat already accepted byte positions.
    #[error("chunk offset {actual_offset} does not match expected offset {expected_offset}")]
    UnexpectedOffset {
        /// Next byte position required by the verifier.
        expected_offset: u64,
        /// Supplied chunk starting position.
        actual_offset: u64,
    },
    /// A whole chunk cannot fit within the remaining declared length.
    #[error("chunk of {chunk_bytes} bytes exceeds {remaining_bytes} remaining bytes")]
    ChunkExceedsLength {
        /// Unfilled bytes in the declared content length.
        remaining_bytes: u64,
        /// Length of the rejected chunk.
        chunk_bytes: usize,
    },
    /// Completion was attempted before all declared bytes arrived.
    #[error("content incomplete: expected {expected_bytes} bytes, received {received_bytes}")]
    Incomplete {
        /// Declared content length.
        expected_bytes: u64,
        /// Successfully accepted bytes.
        received_bytes: u64,
    },
    /// Complete raw bytes do not have the expected SHA-256 digest.
    #[error("content digest does not match expected digest")]
    DigestMismatch {
        /// Digest fixed when the verifier was created.
        expected: ContentDigest,
        /// Computed digest of the accepted bytes.
        actual: ContentDigest,
    },
}

#[cfg(test)]
mod tests {
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
}
