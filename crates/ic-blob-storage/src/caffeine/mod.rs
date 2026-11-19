//! Local integrity checks for the reviewed Caffeine SDK 1.1.2 tree profile.
//!
//! Adapted from the first-party legacy hasher, metadata and root verifier; source
//! pins and reuse decisions are in docs/design/caffeine-contract-review.md.
//! This verifies supplied bytes, not upload authority or provider availability.

mod manifest;
mod metadata;
#[cfg(test)]
mod tests;

pub use manifest::{ManifestLimits, check_manifest};

use sha2::{Digest, Sha256};
use std::num::{NonZeroU64, NonZeroUsize};

/// Fixed provider leaf size, independent of transport framing.
pub const CHUNK_BYTES: usize = 1024 * 1024;

type Hash = [u8; 32];
// At most 2^44 leaves for a u64 byte count; no whole-file tree is retained.
const TREE_LEVELS: usize = 64;

/// Explicit input and per-call work budgets for a single verification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum declared content length.
    pub max_content_bytes: NonZeroU64,
    /// Maximum bytes processed in one append.
    pub max_append_bytes: NonZeroUsize,
    /// Maximum raw metadata entries, before normalization.
    pub max_headers: NonZeroUsize,
    /// Maximum raw UTF-8 metadata bytes, including three framing bytes per entry.
    pub max_header_bytes: NonZeroUsize,
}

/// Original metadata used when computing the expected root, not response headers.
///
/// Names are case-sensitive. Hash normalization is not HTTP validation. Rust
/// strings cannot represent unpaired JavaScript surrogates; those are unsupported.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header<'a> {
    /// Name before ECMAScript whitespace trimming.
    pub name: &'a str,
    /// Value before ECMAScript whitespace trimming.
    pub value: &'a str,
}

/// A refused input or failed integrity comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VerificationError {
    /// No empty-object profile is qualified.
    EmptyContent,
    /// Declared content exceeds the configured budget.
    ContentLimit,
    /// Supplied chunk hashes exceed the manifest work budget.
    ChunkLimit,
    /// Declared length requires a different number of chunks.
    ChunkCountMismatch,
    /// An append exceeds the configured per-call work budget.
    AppendLimit,
    /// Raw metadata has too many entries.
    HeaderCountLimit,
    /// Raw metadata plus framing exceeds its byte budget.
    HeaderBytesLimit,
    /// Original metadata names repeat; an SDK object cannot represent this.
    DuplicateHeader,
    /// An append does not start at the next expected offset.
    UnexpectedOffset,
    /// Input extends past the trusted declared length.
    ExceedsLength,
    /// Finish was called before all declared bytes arrived.
    Incomplete,
    /// Content or claimed chunk hashes, with metadata, do not produce the root.
    RootMismatch,
}

/// A single streaming comparison against an independently trusted declaration.
///
/// Supply the application's original root, byte length and metadata. Taking all
/// three from an untrusted download only establishes self-consistency. The root
/// is a Caffeine tree hash, not a raw SHA-256 digest or a provider-object owner.
///
/// After bounded metadata processing, retained memory is fixed: one SHA-256 state,
/// a 64-slot tree frontier and scalar/hash fields. No body or chunk is retained.
/// Accepted prefixes are not verified bytes. The caller must require successful
/// transport EOF and successful [`Self::finish`] before publishing the exact bytes
/// it checked. Any transport error, including trailing data, fails that download.
/// This component owns no HTTP, output file, checkpoint or persistent state.
pub struct RootVerifier {
    expected_root: Hash,
    expected_bytes: u64,
    received_bytes: u64,
    max_append_bytes: NonZeroUsize,
    chunk: Sha256,
    chunk_bytes: usize,
    frontier: [Option<Hash>; TREE_LEVELS],
    metadata: Option<Hash>,
}

impl RootVerifier {
    /// Bind the trusted declaration before accepting body bytes.
    ///
    /// Metadata uses the SDK's trim, UTF-16 sort and UTF-8 line encoding. No
    /// headers means no metadata node; no Content-Length or MIME value is added.
    /// # Errors
    /// Rejects empty or over-budget content and over-budget or duplicate metadata.
    pub fn new(
        expected_root: [u8; 32],
        expected_bytes: u64,
        headers: &[Header<'_>],
        limits: Limits,
    ) -> Result<Self, VerificationError> {
        check_length(expected_bytes, limits.max_content_bytes)?;

        Ok(Self {
            expected_root,
            expected_bytes,
            received_bytes: 0,
            max_append_bytes: limits.max_append_bytes,
            chunk: chunk_hasher(),
            chunk_bytes: 0,
            frontier: [None; TREE_LEVELS],
            metadata: metadata::hash(headers, limits.max_headers, limits.max_header_bytes)?,
        })
    }

    /// Accepted byte count and next append offset, not verified or durable progress.
    #[must_use]
    pub const fn received_bytes(&self) -> u64 {
        self.received_bytes
    }

    /// Hash a bounded frame at the next offset, rechunking at fixed leaf boundaries.
    ///
    /// Rejected calls leave all state unchanged. Empty input at the next offset is
    /// harmless. Accepted input cannot be replayed or replaced; corruption needs a
    /// fresh verifier. Callers must not discard trailing bytes to manufacture EOF.
    /// # Errors
    /// Rejects wrong offset, per-call work overflow, then excess total bytes.
    pub fn append(&mut self, offset: u64, bytes: &[u8]) -> Result<(), VerificationError> {
        if offset != self.received_bytes {
            return Err(VerificationError::UnexpectedOffset);
        }
        if bytes.len() > self.max_append_bytes.get() {
            return Err(VerificationError::AppendLimit);
        }
        let length = u64::try_from(bytes.len()).map_err(|_| VerificationError::ExceedsLength)?;
        if length > self.expected_bytes - self.received_bytes {
            return Err(VerificationError::ExceedsLength);
        }
        let mut remaining = bytes;
        while !remaining.is_empty() {
            let take = remaining.len().min(CHUNK_BYTES - self.chunk_bytes);
            self.chunk.update(&remaining[..take]);
            self.chunk_bytes += take;
            remaining = &remaining[take..];
            if self.chunk_bytes == CHUNK_BYTES {
                self.complete_chunk();
            }
        }
        self.received_bytes += length;

        Ok(())
    }

    /// Consume this stream and compare exact length and the original root.
    ///
    /// Success establishes byte/metadata integrity only, not availability,
    /// content safety, registration, billing settlement or provider completion.
    /// # Errors
    /// Rejects incomplete content, then root mismatch. No success flag is retained.
    pub fn finish(mut self) -> Result<(), VerificationError> {
        if self.received_bytes != self.expected_bytes {
            return Err(VerificationError::Incomplete);
        }
        if self.chunk_bytes != 0 {
            self.complete_chunk();
        }
        let root = root_with_metadata(&self.frontier, self.metadata);
        if root != self.expected_root {
            return Err(VerificationError::RootMismatch);
        }

        Ok(())
    }

    fn complete_chunk(&mut self) {
        let hash = std::mem::replace(&mut self.chunk, chunk_hasher())
            .finalize()
            .into();
        self.chunk_bytes = 0;
        push_chunk(&mut self.frontier, hash);
    }
}

fn check_length(bytes: u64, maximum: NonZeroU64) -> Result<(), VerificationError> {
    if bytes == 0 {
        return Err(VerificationError::EmptyContent);
    }
    if bytes > maximum.get() {
        return Err(VerificationError::ContentLimit);
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
    unreachable!("u64 content length cannot fill 64 tree levels");
}

fn root_with_metadata(frontier: &[Option<Hash>; TREE_LEVELS], metadata: Option<Hash>) -> Hash {
    let root = tree_root(frontier);

    metadata.map_or(root, |metadata| node_hash(root, Some(metadata)))
}

fn tree_root(frontier: &[Option<Hash>; TREE_LEVELS]) -> Hash {
    let mut occupied = frontier
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(level, hash)| hash.map(|hash| (level, hash)));
    // Both callers require nonempty content and its exact number of leaves.
    let (mut height, mut root) = occupied.next().expect("nonempty completed content");
    for (level, left) in occupied {
        // Pad each missing level of an odd right subtree, matching the SDK tree.
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
            .map_or(b"UNBALANCED".as_slice(), |hash| hash.as_slice()),
    );

    hasher.finalize().into()
}
