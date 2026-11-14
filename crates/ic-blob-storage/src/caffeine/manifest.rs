//! Bounded declaration consistency, without retaining a manifest or issuing authority.

use std::num::{NonZeroU64, NonZeroUsize};

use super::{
    CHUNK_BYTES, Header, TREE_LEVELS, VerificationError, check_length, metadata, push_chunk,
    root_with_metadata,
};

/// Work budgets for one synchronous manifest check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ManifestLimits {
    /// Maximum declared content length; this does not authenticate that length.
    pub max_content_bytes: NonZeroU64,
    /// Maximum supplied chunk hashes, checked before hashing metadata or the tree.
    pub max_chunks: NonZeroUsize,
    /// Maximum raw metadata entries, before normalization.
    pub max_headers: NonZeroUsize,
    /// Maximum raw UTF-8 metadata bytes, including three framing bytes per entry.
    pub max_header_bytes: NonZeroUsize,
}

/// Check an ordered chunk-hash declaration against a Caffeine tree root.
///
/// This adapts the first-party legacy manifest admission check. Length and count
/// limits precede all hashing. Chunk hashes are borrowed, never copied or retained;
/// the tree uses fixed working memory. Metadata work has separate explicit bounds.
/// The caller must bound decoding/allocation before supplying these slices.
///
/// Success establishes tree consistency only. Hashes do not prove chunk lengths,
/// even if the claimed length has the right chunk count. Content-Length metadata
/// is hashed literally, not interpreted or authenticated. Use [`super::RootVerifier`]
/// with an independently trusted declaration to check actual bytes. This check
/// grants no upload authority and proves no conservative provider storage bound.
/// It is a local preflight, not a provider wire format or persistent representation.
///
/// # Errors
/// Rejects empty/over-budget content, excessive or incorrect chunk count, invalid
/// or over-budget metadata, then a root mismatch. No state changes on any result.
pub fn check_manifest(
    expected_root: [u8; 32],
    declared_bytes: u64,
    chunks: &[[u8; 32]],
    headers: &[Header<'_>],
    limits: ManifestLimits,
) -> Result<(), VerificationError> {
    check_length(declared_bytes, limits.max_content_bytes)?;
    if chunks.len() > limits.max_chunks.get() {
        return Err(VerificationError::ChunkLimit);
    }
    let expected = declared_bytes.div_ceil(CHUNK_BYTES as u64);
    if u64::try_from(chunks.len()).ok() != Some(expected) {
        return Err(VerificationError::ChunkCountMismatch);
    }
    let metadata = metadata::hash(headers, limits.max_headers, limits.max_header_bytes)?;
    let mut frontier = [None; TREE_LEVELS];
    for &chunk in chunks {
        push_chunk(&mut frontier, chunk);
    }
    if root_with_metadata(&frontier, metadata) != expected_root {
        return Err(VerificationError::RootMismatch);
    }
    Ok(())
}
