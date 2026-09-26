//! Bounded streaming identities for the reviewed Caffeine client 1.1.2 algorithm.
//!
//! This computes hashes, not upload completion, ownership or provider availability.
//! Input metadata is hashed exactly; no MIME inference, HTTP validation or implicit
//! Content-Length header is supplied. See docs/provider-review.md for source pins
//! and independent vectors. Empty provider objects remain unqualified.

pub mod manifest;

mod metadata;

#[cfg(test)]
mod tests;

use std::num::{NonZeroU64, NonZeroUsize};

use sha2::{Digest, Sha256};
use thiserror::Error;

use super::{ContentDigest, ProviderRootHash, verification::ContentVerifier};

/// Chunk boundary used by the reviewed client, independent of append boundaries.
pub const CAFFEINE_CHUNK_BYTES: usize = 1024 * 1024;

type Hash = [u8; 32];
// A u64 byte length requires at most 2^44 chunks. 64 levels also leave the bound
// obvious if the chunk constant changes; no object-sized tree is allocated.
const TREE_LEVELS: usize = 64;

/// Explicit processing budgets for one transient hash computation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaffeineHashLimits {
    /// Maximum declared object length, additionally subject to SHA-256's bound.
    pub max_content_bytes: NonZeroU64,
    /// Maximum bytes hashed by a single append call.
    pub max_append_bytes: NonZeroUsize,
    /// Maximum number of supplied metadata entries before normalization.
    pub max_headers: NonZeroUsize,
    /// Raw UTF-8 name/value bytes plus three framing bytes per entry.
    pub max_header_bytes: NonZeroUsize,
}

/// One supplied metadata entry; hashing does not validate it as an HTTP header.
///
/// Names are case-sensitive for hashing. Rust strings provide well-formed UTF-8;
/// JavaScript strings containing unpaired surrogates are outside this input model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaffeineHeader<'a> {
    /// Exact supplied name, before ECMAScript whitespace trimming.
    pub name: &'a str,
    /// Exact supplied value, before ECMAScript whitespace trimming.
    pub value: &'a str,
}

/// Computed identities of the same bytes, with different hash domains.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaffeineContentHashes {
    /// SHA-256 of raw content, independent of chunking and metadata.
    pub content_digest: ContentDigest,
    /// Provider tree root, including supplied metadata when present.
    pub provider_root: ProviderRootHash,
}

/// Incremental hash computation with fixed memory independent of content length.
///
/// Arbitrary append splits are rechunked into the client's 1 MiB leaves. The
/// frontier retains at most 64 hashes, two SHA-256 states and one metadata hash;
/// complete chunks/content and the upload tree are not retained. Rejected appends
/// change no state. This is not a persisted checkpoint or resumable upload session.
pub struct CaffeineContentHasher {
    expected_bytes: u64,
    received_bytes: u64,
    max_append_bytes: NonZeroUsize,
    raw: Sha256,
    chunk: Sha256,
    chunk_bytes: usize,
    frontier: [Option<Hash>; TREE_LEVELS],
    metadata: Option<Hash>,
}

impl CaffeineContentHasher {
    /// Begin hashing nonempty content with exact metadata and explicit budgets.
    ///
    /// Metadata follows the pinned client's trim, line framing, UTF-16 sorting
    /// and UTF-8 encoding. No headers means no metadata node. Original duplicate
    /// names reject because the upstream object cannot represent duplicate keys.
    /// Header budgets are checked before allocating normalized lines.
    /// # Errors
    /// Rejects empty content (provider behavior is unqualified), lengths beyond
    /// configured/algorithm bounds, oversized metadata or duplicate original names.
    pub fn new(
        expected_bytes: u64,
        headers: &[CaffeineHeader<'_>],
        limits: CaffeineHashLimits,
    ) -> Result<Self, CaffeineHashError> {
        validate_length(expected_bytes, limits.max_content_bytes)?;
        Ok(Self {
            expected_bytes,
            received_bytes: 0,
            max_append_bytes: limits.max_append_bytes,
            raw: Sha256::new(),
            chunk: chunk_hasher(),
            chunk_bytes: 0,
            frontier: [None; TREE_LEVELS],
            metadata: metadata::hash(headers, limits.max_headers, limits.max_header_bytes)?,
        })
    }

    /// Accepted byte count and required offset for the next append.
    #[must_use]
    pub const fn received_bytes(&self) -> u64 {
        self.received_bytes
    }

    /// Remaining bytes before completion can compute identities.
    #[must_use]
    pub const fn remaining_bytes(&self) -> u64 {
        self.expected_bytes - self.received_bytes
    }

    /// Hash bytes at the exact next offset, splitting at provider chunk boundaries.
    ///
    /// Empty appends at the expected offset are harmless. An accepted nonempty
    /// append cannot be replayed at its old offset or overwritten later.
    /// # Errors
    /// Rejects wrong offset, excessive per-call work or bytes beyond the declared
    /// length, in that order. Rejection preserves all hash states and counters.
    pub fn append(&mut self, offset: u64, bytes: &[u8]) -> Result<(), CaffeineHashError> {
        if offset != self.received_bytes {
            return Err(CaffeineHashError::UnexpectedOffset {
                expected: self.received_bytes,
                actual: offset,
            });
        }
        if bytes.len() > self.max_append_bytes.get() {
            return Err(CaffeineHashError::Limit(CaffeineHashLimit::AppendBytes));
        }
        let length = u64::try_from(bytes.len()).map_err(|_| CaffeineHashError::ExceedsLength)?;
        if length > self.remaining_bytes() {
            return Err(CaffeineHashError::ExceedsLength);
        }
        self.raw.update(bytes);
        let mut remaining = bytes;
        while !remaining.is_empty() {
            let take = remaining.len().min(CAFFEINE_CHUNK_BYTES - self.chunk_bytes);
            self.chunk.update(&remaining[..take]);
            self.chunk_bytes += take;
            remaining = &remaining[take..];
            if self.chunk_bytes == CAFFEINE_CHUNK_BYTES {
                self.complete_chunk();
            }
        }
        self.received_bytes += length;
        Ok(())
    }

    /// Compute both identities after receiving exactly the declared content length.
    /// # Errors
    /// Rejects incomplete content. Success proves supplied-byte consistency only.
    pub fn finish(mut self) -> Result<CaffeineContentHashes, CaffeineHashError> {
        if self.received_bytes != self.expected_bytes {
            return Err(CaffeineHashError::Incomplete {
                remaining: self.remaining_bytes(),
            });
        }
        if self.chunk_bytes != 0 {
            self.complete_chunk();
        }
        Ok(CaffeineContentHashes {
            content_digest: ContentDigest(self.raw.finalize().into()),
            provider_root: root_with_metadata(&self.frontier, self.metadata),
        })
    }

    /// Compare the completed stream with independently trusted expected identities.
    ///
    /// Neither an expected digest nor a matching tree grants tenant authority or
    /// establishes that these bytes remain stored at the provider.
    /// # Errors
    /// Rejects incomplete content, raw digest mismatch, then provider-root mismatch.
    pub fn verify(
        self,
        expected: CaffeineContentHashes,
    ) -> Result<CaffeineContentHashes, CaffeineHashError> {
        let actual = self.finish()?;
        if actual.content_digest != expected.content_digest {
            return Err(CaffeineHashError::ContentDigestMismatch);
        }
        if actual.provider_root != expected.provider_root {
            return Err(CaffeineHashError::ProviderRootMismatch);
        }
        Ok(actual)
    }

    fn complete_chunk(&mut self) {
        let hash: Hash = std::mem::replace(&mut self.chunk, chunk_hasher())
            .finalize()
            .into();
        self.chunk_bytes = 0;
        push_chunk(&mut self.frontier, hash);
    }
}

fn validate_length(bytes: u64, maximum: NonZeroU64) -> Result<(), CaffeineHashError> {
    if bytes == 0 {
        return Err(CaffeineHashError::EmptyContentUnqualified);
    }
    if bytes > maximum.get() || bytes > ContentVerifier::MAX_BYTES {
        return Err(CaffeineHashError::Limit(CaffeineHashLimit::ContentBytes));
    }
    Ok(())
}

fn push_chunk(frontier: &mut [Option<Hash>; TREE_LEVELS], mut hash: Hash) {
    for slot in frontier {
        if let Some(left) = slot.take() {
            hash = node_hash(left, Some(hash));
        } else {
            *slot = Some(hash);
            return;
        }
    }
    unreachable!("u64 content length cannot fill 64 chunk-tree levels");
}

fn root_with_metadata(
    frontier: &[Option<Hash>; TREE_LEVELS],
    metadata: Option<Hash>,
) -> ProviderRootHash {
    let root = tree_root(frontier);
    ProviderRootHash(metadata.map_or(root, |hash| node_hash(root, Some(hash))))
}

fn tree_root(frontier: &[Option<Hash>; TREE_LEVELS]) -> Hash {
    let mut occupied = frontier
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(level, hash)| hash.map(|hash| (level, hash)));
    // Construction forbids empty input and finish requires all declared bytes.
    let (mut height, mut root) = occupied.next().expect("nonempty completed content");
    for (level, left) in occupied {
        // A short right subtree is padded at each missing level, exactly as
        // the client's bottom-up tree pairs its odd final node with UNBALANCED.
        while height < level {
            root = node_hash(root, None);
            height += 1;
        }
        root = node_hash(left, Some(root));
        height += 1;
    }
    root
}

fn chunk_hasher() -> Sha256 {
    let mut hasher = Sha256::new();
    hasher.update(b"icfs-chunk/");
    hasher
}

fn node_hash(left: Hash, right: Option<Hash>) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(b"ynode/");
    hasher.update(left);
    hasher.update(
        right
            .as_ref()
            .map_or(b"UNBALANCED".as_slice(), |value| value.as_slice()),
    );
    hasher.finalize().into()
}

/// Explicit hash-computation budget that was exceeded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaffeineHashLimit {
    /// Declared content exceeds configured or SHA-256 length capacity.
    ContentBytes,
    /// A single append exceeds its configured byte budget.
    AppendBytes,
    /// Raw metadata entry count exceeds the configured maximum.
    HeaderCount,
    /// Raw metadata plus framing exceeds the configured byte maximum.
    HeaderBytes,
}

/// Rejected computation or comparison; no provider effects occur.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CaffeineHashError {
    /// No supported empty-object root has been established for the provider.
    #[error("empty provider content remains unqualified")]
    EmptyContentUnqualified,
    /// An explicit resource or algorithm bound was exceeded.
    #[error("Caffeine hash limit exceeded: {0:?}")]
    Limit(CaffeineHashLimit),
    /// Two original metadata names are equal before trimming.
    #[error("duplicate original metadata name")]
    DuplicateHeader,
    /// Append does not start immediately after the accepted bytes.
    #[error("unexpected offset {actual}, expected {expected}")]
    UnexpectedOffset {
        /// Required next offset.
        expected: u64,
        /// Supplied offset.
        actual: u64,
    },
    /// Supplied bytes exceed the remaining declared content length.
    #[error("append exceeds declared content length")]
    ExceedsLength,
    /// Completion attempted before every declared byte arrived.
    #[error("content is incomplete: {remaining} bytes remain")]
    Incomplete {
        /// Bytes still required by the declaration.
        remaining: u64,
    },
    /// Actual raw digest differs from the independently supplied expected digest.
    #[error("raw content digest mismatch")]
    ContentDigestMismatch,
    /// Actual provider tree differs from the independently supplied expected root.
    #[error("provider root mismatch")]
    ProviderRootMismatch,
}
