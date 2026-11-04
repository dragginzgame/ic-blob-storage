//! In-memory verification of exact raw bytes against a claimed length and digest.
//!
//! This owns no upload session, persisted checkpoint or provider hash tree.

use sha2::Digest;
use sha2::Sha256;
use thiserror::Error;
pub(crate) mod checkpoint;

use crate::identity::ContentDigest;

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
/// Ordered verification additionally provides a host-owned trusted checkpoint;
/// this raw verifier exposes no standalone persistence API.
///
/// ```
/// use ic_blob_storage_contracts::identity::{ContentDigest, verification::ContentVerifier};
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
mod tests;
