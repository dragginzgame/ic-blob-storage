//! Ordered verification of both manifest leaves and the expected raw file digest.
//!
//! Corrupt chunks reject before entering the whole-file hash. No content bytes,
//! destination writes, provider calls or restart checkpoints are owned here.

use thiserror::Error;

use crate::model::identity::{
    ContentDigest,
    caffeine::{
        CAFFEINE_CHUNK_BYTES, CaffeineContentHashes,
        manifest::{CaffeineChunkManifest, CaffeineManifestError},
    },
    verification::{ContentVerificationError, ContentVerifier},
};

#[cfg(test)]
mod tests;

/// Verify a file's manifest-bound chunks in order and its independent raw digest.
///
/// The immutable manifest owns expected root, length and leaves. Each append
/// checks position, exact leaf length and hash before feeding the raw verifier.
/// Rejected input leaves the verified prefix intact; retries supply the correct
/// bytes at the same next index. Successfully appended chunks cannot be replayed.
///
/// Memory is the already bounded manifest plus fixed-size raw hash state; no
/// content or coverage bitmap is allocated. A call hashes at most one 1 MiB leaf
/// twice, once for its domain-separated identity and once for the raw digest.
/// Root/length/digest provenance and tenant authority are external requirements.
/// This instance cannot resume after restart or establish destination durability.
pub struct CaffeineOrderedChunkVerifier {
    manifest: CaffeineChunkManifest,
    raw: ContentVerifier,
}

impl CaffeineOrderedChunkVerifier {
    /// Start an empty verified prefix against one manifest and expected raw digest.
    ///
    /// Consumes the manifest without copying its leaves or allocating file bytes.
    #[must_use]
    pub fn new(manifest: CaffeineChunkManifest, expected_digest: ContentDigest) -> Self {
        let raw = raw_verifier(&manifest, expected_digest);
        Self { manifest, raw }
    }

    /// Immutable expected provider root, declared length and ordered leaves.
    #[must_use]
    pub const fn manifest(&self) -> &CaffeineChunkManifest {
        &self.manifest
    }

    /// Index required by the next append; one past the last leaf after full input.
    #[must_use]
    pub const fn next_chunk(&self) -> u64 {
        self.raw
            .received_bytes()
            .div_ceil(CAFFEINE_CHUNK_BYTES as u64)
    }

    /// Bytes already checked against their manifest leaves and fed into the raw hash.
    #[must_use]
    pub const fn verified_bytes(&self) -> u64 {
        self.raw.received_bytes()
    }

    /// Bytes still needed before final raw-digest verification can run.
    #[must_use]
    pub const fn remaining_bytes(&self) -> u64 {
        self.raw.remaining_bytes()
    }

    /// Check one complete leaf at the next index, then advance the verified prefix.
    ///
    /// A manifest-consistent chunk is still subject to the final raw digest check.
    /// No successful append alone establishes both identities or provider completion.
    /// # Errors
    /// Rejects wrong order, invalid index, wrong length or leaf hash mismatch
    /// before advancing either byte progress or raw hash state.
    pub fn append_chunk(
        &mut self,
        index: u64,
        bytes: &[u8],
    ) -> Result<(), CaffeineOrderedVerificationError> {
        if index != self.next_chunk() {
            return Err(CaffeineOrderedVerificationError::UnexpectedChunk {
                expected: self.next_chunk(),
                actual: index,
            });
        }
        self.manifest.verify_chunk(index, bytes)?;
        self.raw.append(self.raw.received_bytes(), bytes)?;
        Ok(())
    }

    /// Consume this instance and return both identities only after all bytes match.
    ///
    /// Check `remaining_bytes` before finishing when more chunks may arrive.
    /// This proves consistency of the observed sequence with the supplied manifest
    /// and raw digest, not successful writes, retained bytes or provider availability.
    /// # Errors
    /// Incomplete input or a conflicting expected raw digest consumes the instance
    /// and returns no successful identity pair.
    pub fn finish(self) -> Result<CaffeineContentHashes, CaffeineOrderedVerificationError> {
        Ok(CaffeineContentHashes {
            provider_root: self.manifest.root(),
            content_digest: self.raw.finish()?,
        })
    }
}

fn raw_verifier(
    manifest: &CaffeineChunkManifest,
    expected_digest: ContentDigest,
) -> ContentVerifier {
    // Manifest construction already enforces the same SHA-256 length bound.
    ContentVerifier::new(expected_digest, manifest.content_bytes())
        .expect("validated manifest length fits raw verifier")
}

/// Failure to append the next verified leaf or finalize the raw identity.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CaffeineOrderedVerificationError {
    /// Supplied chunk would repeat or skip a position in the verified prefix.
    #[error("chunk index {actual} does not match next index {expected}")]
    UnexpectedChunk {
        /// Next index, derived from the verified byte prefix.
        expected: u64,
        /// Supplied index.
        actual: u64,
    },
    /// Manifest position, exact length or domain-separated leaf digest rejected.
    #[error(transparent)]
    Manifest(#[from] CaffeineManifestError),
    /// Exact raw-content verification rejected append or finalization.
    #[error(transparent)]
    Content(#[from] ContentVerificationError),
}
