//! Root-bound chunk identities for independently verifying nonempty file segments.
//!
//! This is a local model, not a provider wire manifest, certificate or resume
//! checkpoint. Matching a supplied root establishes consistency only. Its trusted
//! provenance, tenant binding, availability and exact length remain external.

#[cfg(test)]
mod tests;

use std::{
    fmt,
    num::{NonZeroU64, NonZeroUsize},
    str::FromStr,
};

use sha2::Digest;
use thiserror::Error;

use super::{
    CAFFEINE_CHUNK_BYTES, CaffeineHashError, CaffeineHeader, Hash, TREE_LEVELS, chunk_hasher,
    metadata, push_chunk, root_with_metadata, validate_length,
};
use crate::model::identity::{HashParseError, ProviderRootHash, format_hash, parse_hash};

/// A claimed domain-separated chunk hash, distinct from a raw digest or tree root.
///
/// Parsing validates representation only. A manifest binds its position to an
/// expected root; verifying the exact chunk bytes establishes the leaf match.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaffeineChunkHash(Hash);

impl CaffeineChunkHash {
    /// The exact 32 hash bytes; possession of them grants no authority.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl FromStr for CaffeineChunkHash {
    type Err = HashParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_hash(value).map(Self)
    }
}

impl TryFrom<&[u8]> for CaffeineChunkHash {
    type Error = HashParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        bytes
            .try_into()
            .map(Self)
            .map_err(|_| HashParseError::InvalidByteLength {
                actual: bytes.len(),
            })
    }
}

impl fmt::Display for CaffeineChunkHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_hash(&self.0, formatter)
    }
}

/// Explicit construction budgets; the manifest retains only bounded chunk hashes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaffeineManifestLimits {
    /// Maximum declared content length, additionally subject to SHA-256's bound.
    pub max_content_bytes: NonZeroU64,
    /// Maximum retained chunk identities, checked before copying input.
    pub max_chunks: NonZeroUsize,
    /// Maximum supplied metadata entries before normalization.
    pub max_headers: NonZeroUsize,
    /// Raw UTF-8 name/value bytes plus three framing bytes per entry.
    pub max_header_bytes: NonZeroUsize,
}

/// Immutable ordered chunk hashes checked against one expected provider root.
///
/// Metadata/tree rules share the streaming hasher's implementation. Construction
/// verifies tree consistency, not any chunk's bytes. Each call to `verify_chunk`
/// checks one complete leaf, independently of previous calls, so retries and
/// out-of-order delivery do not allocate receipts or advance a false completion
/// counter. No verified-chunk bitmap, storage, persistence or download is owned.
///
/// Length determines chunk count and the last chunk's size. A root alone does not
/// authenticate length; the workflow must bind the declaration to trusted state.
/// No implicit Content-Length header is inserted or interpreted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaffeineChunkManifest {
    root: ProviderRootHash,
    content_bytes: u64,
    chunks: Vec<CaffeineChunkHash>,
}

impl CaffeineChunkManifest {
    /// Check length, ordered chunk identities and explicit metadata against a root.
    ///
    /// All count/length checks precede metadata processing and chunk-list copying.
    /// Allocation is bounded by the chunk and header budgets. No complete tree or
    /// content is allocated. Header input is hashed, not validated for HTTP use.
    /// # Errors
    /// Rejects unsupported empty content, exceeded budgets, wrong chunk count,
    /// invalid metadata or any tree/root mismatch.
    pub fn new(
        root: ProviderRootHash,
        content_bytes: u64,
        chunks: &[CaffeineChunkHash],
        headers: &[CaffeineHeader<'_>],
        limits: CaffeineManifestLimits,
    ) -> Result<Self, CaffeineManifestError> {
        validate_length(content_bytes, limits.max_content_bytes)?;
        if chunks.len() > limits.max_chunks.get() {
            return Err(CaffeineManifestError::TooManyChunks);
        }
        let expected = content_bytes.div_ceil(CAFFEINE_CHUNK_BYTES as u64);
        if u64::try_from(chunks.len()).ok() != Some(expected) {
            return Err(CaffeineManifestError::ChunkCountMismatch {
                expected,
                actual: chunks.len(),
            });
        }
        let metadata = metadata::hash(headers, limits.max_headers, limits.max_header_bytes)?;
        let mut frontier = [None; TREE_LEVELS];
        for chunk in chunks {
            push_chunk(&mut frontier, chunk.0);
        }
        if root_with_metadata(&frontier, metadata) != root {
            return Err(CaffeineManifestError::RootMismatch);
        }
        Ok(Self {
            root,
            content_bytes,
            chunks: chunks.to_vec(),
        })
    }

    /// Root against which the supplied chunk list and metadata were checked.
    #[must_use]
    pub const fn root(&self) -> ProviderRootHash {
        self.root
    }

    /// Declared content length, not independently authenticated provider metadata.
    #[must_use]
    pub const fn content_bytes(&self) -> u64 {
        self.content_bytes
    }

    /// Number of chunk identities retained under the construction budget.
    #[must_use]
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Required byte length at this index, including the exact final partial chunk.
    /// # Errors
    /// Rejects indices outside the manifest before offset arithmetic.
    pub fn chunk_bytes(&self, index: u64) -> Result<usize, CaffeineManifestError> {
        self.chunk(index)?;
        Ok(chunk_length(self.content_bytes, index))
    }

    /// Verify exact bytes for one index without changing the manifest or other chunks.
    ///
    /// At most 1 MiB is hashed per call, even when an oversized slice is supplied.
    /// Repeated verification is harmless; failures permit retry with correct bytes.
    /// A successful leaf check is not whole-file completion or provider presence.
    /// # Errors
    /// Rejects out-of-range index, wrong byte length, then hash mismatch.
    pub fn verify_chunk(&self, index: u64, bytes: &[u8]) -> Result<(), CaffeineManifestError> {
        let chunk = self.chunk(index)?;
        let expected = self.chunk_bytes(index)?;
        if bytes.len() != expected {
            return Err(CaffeineManifestError::ChunkLengthMismatch {
                expected,
                actual: bytes.len(),
            });
        }
        let mut hasher = chunk_hasher();
        hasher.update(bytes);
        let actual: Hash = hasher.finalize().into();
        if actual != chunk.0 {
            return Err(CaffeineManifestError::ChunkHashMismatch);
        }
        Ok(())
    }

    fn chunk(&self, index: u64) -> Result<CaffeineChunkHash, CaffeineManifestError> {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.chunks.get(index))
            .copied()
            .ok_or(CaffeineManifestError::ChunkIndexOutOfRange { index })
    }
}

fn chunk_length(content_bytes: u64, index: u64) -> usize {
    // Called only after checking index < ceil(content_bytes / chunk size).
    // The content length fits the SHA-256 bound, so multiplication fits u64.
    let offset = index * CAFFEINE_CHUNK_BYTES as u64;
    usize::try_from((content_bytes - offset).min(CAFFEINE_CHUNK_BYTES as u64))
        .expect("at most one 1 MiB chunk on supported targets")
}

/// Manifest construction or leaf verification failed without mutating any state.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CaffeineManifestError {
    /// Shared content-length or metadata hashing checks failed.
    #[error(transparent)]
    Hash(#[from] CaffeineHashError),
    /// Raw chunk-hash count exceeds its explicit allocation budget.
    #[error("chunk manifest exceeds chunk count limit")]
    TooManyChunks,
    /// Declared byte length requires a different number of provider chunks.
    #[error("chunk count {actual} does not match expected {expected}")]
    ChunkCountMismatch {
        /// Count implied by the declared nonempty length.
        expected: u64,
        /// Supplied number of chunk hashes.
        actual: usize,
    },
    /// Ordered leaf hashes and metadata do not produce the expected provider root.
    #[error("chunk manifest root mismatch")]
    RootMismatch,
    /// Index is outside this immutable manifest.
    #[error("chunk index {index} is out of range")]
    ChunkIndexOutOfRange {
        /// Supplied index.
        index: u64,
    },
    /// Supplied chunk is not exactly the expected length at its index.
    #[error("chunk length {actual} does not match expected {expected}")]
    ChunkLengthMismatch {
        /// Required full or final-partial chunk length.
        expected: usize,
        /// Supplied byte count.
        actual: usize,
    },
    /// Exact-length bytes do not match the leaf at this position.
    #[error("chunk hash mismatch")]
    ChunkHashMismatch,
}
