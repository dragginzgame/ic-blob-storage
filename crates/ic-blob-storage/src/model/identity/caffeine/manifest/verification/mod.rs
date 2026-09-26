//! Bounded, transient coverage of byte checks against one immutable manifest.
//!
//! Coverage records successful observations in this instance, not bytes retained
//! in a destination. It is not a download receipt, provider completion proof,
//! raw whole-file digest, tenant authority or persisted resume checkpoint.
//! Use [`ordered::CaffeineOrderedChunkVerifier`] when an ordered read must also
//! match an independently supplied raw whole-file digest.

pub mod missing;
pub mod ordered;

use super::{CaffeineChunkManifest, CaffeineManifestError};

#[cfg(test)]
mod tests;

/// Read-only progress of unique chunk positions whose bytes passed verification.
///
/// This view grants no authority and does not prove that previously checked
/// bytes remain available or unchanged outside the verifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaffeineChunkProgress {
    /// Unique verified positions; repeated content at different positions counts separately.
    pub verified_chunks: usize,
    /// Exact sum of verified positions' lengths, including a partial final chunk.
    pub verified_bytes: u64,
    /// Chunk count from the immutable manifest.
    pub total_chunks: usize,
    /// Declared byte length from the immutable manifest, not provider metadata proof.
    pub total_bytes: u64,
}

impl CaffeineChunkProgress {
    /// Whether every manifest position has passed byte verification in this instance.
    ///
    /// This does not establish successful destination writes or durable completion.
    #[must_use]
    pub const fn all_chunks_verified(self) -> bool {
        self.verified_chunks == self.total_chunks && self.verified_bytes == self.total_bytes
    }
}

/// Verify chunks in any order, counting each successfully checked position once.
///
/// Owns one validated manifest and one bit per chunk (rounded to whole bytes).
/// The manifest's construction limits bound allocation and lifetime metadata.
/// No content, per-call receipts or growing retry history are retained. Every
/// supplied chunk, including a duplicate, is hashed before progress can advance.
/// Rejected input leaves all coverage unchanged and can be retried with valid bytes.
///
/// Use a new instance for a new read; constructing it always starts at zero.
/// There is no serialization, imported bitmap or unverified completion setter.
/// The owner must separately authenticate root/length/tenant provenance and
/// coordinate verification with destination writes and same-release recovery.
#[derive(Debug)]
pub struct CaffeineChunkVerifier {
    manifest: CaffeineChunkManifest,
    verified: Vec<u8>,
    verified_chunks: usize,
    verified_bytes: u64,
}

impl CaffeineChunkVerifier {
    /// Start empty coverage, consuming a manifest already admitted under explicit budgets.
    ///
    /// Allocation is exactly `ceil(chunk_count / 8)` bitmap bytes in addition to
    /// the existing manifest. No content or identities are cloned.
    #[must_use]
    pub fn new(manifest: CaffeineChunkManifest) -> Self {
        let verified = vec![0; manifest.chunk_count().div_ceil(8)];
        Self {
            manifest,
            verified,
            verified_chunks: 0,
            verified_bytes: 0,
        }
    }

    /// The immutable root, declared length and per-position identities for this read.
    #[must_use]
    pub const fn manifest(&self) -> &CaffeineChunkManifest {
        &self.manifest
    }

    /// Current unique-position coverage, without hashing or changing state.
    #[must_use]
    pub fn progress(&self) -> CaffeineChunkProgress {
        CaffeineChunkProgress {
            verified_chunks: self.verified_chunks,
            verified_bytes: self.verified_bytes,
            total_chunks: self.manifest.chunk_count(),
            total_bytes: self.manifest.content_bytes(),
        }
    }

    /// Whether this position has previously passed a byte check in this instance.
    /// # Errors
    /// Rejects out-of-range indices, including values not representable as `usize`.
    pub fn is_verified(&self, index: u64) -> Result<bool, CaffeineManifestError> {
        self.manifest.chunk_bytes(index)?;
        let (slot, mask) = bitmap_position(index);
        Ok(self.verified[slot] & mask != 0)
    }

    /// Verify exact bytes and credit a previously unseen position once.
    ///
    /// At most 1 MiB is hashed per call, followed by constant-time bookkeeping.
    /// Even after all positions passed, later supplied bytes must pass their own
    /// check. A bad duplicate returns an error but does not erase the earlier
    /// successful observation; no statement about a destination buffer is made.
    /// # Errors
    /// Rejects invalid index, wrong length or corrupt bytes before any mutation.
    pub fn verify_chunk(
        &mut self,
        index: u64,
        bytes: &[u8],
    ) -> Result<CaffeineChunkProgress, CaffeineManifestError> {
        self.manifest.verify_chunk(index, bytes)?;
        self.credit_verified(index, bytes.len());
        Ok(self.progress())
    }

    // Called only after the immutable manifest checked this index and exact bytes.
    fn credit_verified(&mut self, index: u64, byte_length: usize) {
        let (position, mask) = bitmap_position(index);
        let slot = &mut self.verified[position];
        if *slot & mask == 0 {
            *slot |= mask;
            self.verified_chunks += 1;
            // Each position is credited once and its exact length was checked.
            // The sum cannot exceed the manifest's validated u64 content length.
            self.verified_bytes += u64::try_from(byte_length).expect("at most 1 MiB");
        }
    }
}

// Both callers first validate the index against the immutable manifest.
fn bitmap_position(index: u64) -> (usize, u8) {
    let index = usize::try_from(index).expect("manifest checked index representation");
    (index / 8, 1 << (index % 8))
}
