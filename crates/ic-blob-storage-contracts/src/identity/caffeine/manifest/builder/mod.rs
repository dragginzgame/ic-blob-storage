//! Single-pass client manifest preparation using the shared streaming hasher.

use crate::identity::caffeine::CAFFEINE_CHUNK_BYTES;
use crate::identity::caffeine::CaffeineContentHasher;
use crate::identity::caffeine::CaffeineContentHashes;
use crate::identity::caffeine::CaffeineHashError;
use crate::identity::caffeine::CaffeineHashLimits;
use crate::identity::caffeine::CaffeineHeader;
use crate::identity::caffeine::manifest::CaffeineChunkHash;
use crate::identity::caffeine::manifest::CaffeineChunkManifest;
use crate::identity::caffeine::validate_length;
use std::num::NonZeroUsize;
use thiserror::Error;

/// Bounded client-side manifest builder; no file body or full tree is retained.
///
/// Construction reserves the declared number of 32-byte leaves under an explicit
/// limit. Each raw/leaf hash is computed once by the existing streaming hasher;
/// arbitrary append framing does not affect leaf boundaries. Original metadata
/// must be preserved separately by the caller for subsequent admission/downloads.
/// This is preparation, not a provider certificate, completion fact or resume
/// journal. Hosts must require clean input EOF and upload the same prepared bytes.
pub struct CaffeineManifestBuilder {
    hasher: CaffeineContentHasher,
    bytes: u64,
    chunks: Vec<CaffeineChunkHash>,
}

/// Computed identities and the corresponding immutable ordered manifest.
///
/// Private fields preserve their relationship. They authenticate neither the
/// publisher nor a service reference and make no claim about stored content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaffeineBuiltManifest {
    hashes: CaffeineContentHashes,
    manifest: CaffeineChunkManifest,
}

impl CaffeineBuiltManifest {
    /// Raw digest and provider root computed from the same supplied body.
    #[must_use]
    pub const fn hashes(&self) -> CaffeineContentHashes {
        self.hashes
    }

    /// Exact declared length and ordered leaves, without an extra copy.
    #[must_use]
    pub const fn manifest(&self) -> &CaffeineChunkManifest {
        &self.manifest
    }
}

impl CaffeineManifestBuilder {
    /// Bound processing and retained leaves before accepting any file bytes.
    ///
    /// Header rules are the generic Caffeine hash rules, not the stricter service
    /// metadata contract. No implicit headers or MIME inference are performed.
    /// # Errors
    /// Rejects invalid content/metadata, too many declared leaves, or inability
    /// to reserve their storage. No leaf allocation follows a limit rejection.
    pub fn new(
        expected_bytes: u64,
        headers: &[CaffeineHeader<'_>],
        limits: CaffeineHashLimits,
        max_chunks: NonZeroUsize,
    ) -> Result<Self, CaffeineManifestBuilderError> {
        validate_length(expected_bytes, limits.max_content_bytes)?;
        let count = usize::try_from(expected_bytes.div_ceil(CAFFEINE_CHUNK_BYTES as u64))
            .map_err(|_| CaffeineManifestBuilderError::TooManyChunks)?;
        if count > max_chunks.get() {
            return Err(CaffeineManifestBuilderError::TooManyChunks);
        }
        let hasher = CaffeineContentHasher::new(expected_bytes, headers, limits)?;
        let mut chunks = Vec::new();
        chunks
            .try_reserve_exact(count)
            .map_err(|_| CaffeineManifestBuilderError::CapacityUnavailable)?;
        Ok(Self {
            hasher,
            bytes: expected_bytes,
            chunks,
        })
    }

    /// Accepted input length and the required next offset, not durable progress.
    #[must_use]
    pub const fn received_bytes(&self) -> u64 {
        self.hasher.received_bytes()
    }

    /// Required remaining input length; zero still does not establish host EOF.
    #[must_use]
    pub const fn remaining_bytes(&self) -> u64 {
        self.hasher.remaining_bytes()
    }

    /// Process one frame using the shared hash checks and exact leaf boundaries.
    /// # Errors
    /// Offset, per-frame and length rejection preserve hashes and retained leaves.
    pub fn append(&mut self, offset: u64, bytes: &[u8]) -> Result<(), CaffeineHashError> {
        let chunks = &mut self.chunks;
        self.hasher
            .append_observing(offset, bytes, |hash| chunks.push(CaffeineChunkHash(hash)))
    }

    /// Finish only after the exact declared length, moving the retained leaf list.
    ///
    /// A partial final leaf is added once. An exact leaf multiple creates no
    /// empty leaf. The caller must separately require successful source EOF.
    /// # Errors
    /// Incomplete input consumes the builder without returning a partial manifest.
    pub fn finish(mut self) -> Result<CaffeineBuiltManifest, CaffeineHashError> {
        let hashes = self
            .hasher
            .finish_observing(|hash| self.chunks.push(CaffeineChunkHash(hash)))?;
        Ok(CaffeineBuiltManifest {
            hashes,
            manifest: CaffeineChunkManifest {
                root: hashes.provider_root,
                content_bytes: self.bytes,
                chunks: self.chunks,
            },
        })
    }
}

/// Manifest preparation rejected before any body bytes were accepted.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CaffeineManifestBuilderError {
    /// Shared streaming content or metadata validation failed.
    #[error(transparent)]
    Hash(#[from] CaffeineHashError),
    /// Declared content requires more leaves than the configured/portable bound.
    #[error("declared content exceeds manifest leaf budget")]
    TooManyChunks,
    /// The allocator could not reserve the bounded leaf list.
    #[error("manifest leaf storage is unavailable")]
    CapacityUnavailable,
}

#[cfg(test)]
mod tests;
