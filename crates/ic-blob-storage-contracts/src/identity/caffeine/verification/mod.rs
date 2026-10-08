//! Whole-stream verification against a fixed provider root and exact metadata.
//!
//! Intended for integrity consumers, including off-canister download clients.
//! No transport, destination, tenant authority or provider completion is owned.

use crate::identity::ProviderRootHash;
use crate::identity::caffeine::CaffeineContentHasher;
use crate::identity::caffeine::CaffeineContentHashes;
use crate::identity::caffeine::CaffeineHashError;
use crate::identity::caffeine::CaffeineHashLimits;
use crate::identity::caffeine::CaffeineHeader;

#[cfg(test)]
mod tests;

/// Verify a streamed body when a trusted provider root is available without an
/// independently trusted raw digest or a complete chunk manifest.
///
/// Root, exact byte length and original hash metadata must come from an
/// authenticated application descriptor. Reading them from the same untrusted
/// response as the body establishes consistency only. HTTP response headers
/// must not replace the original metadata: transport can normalize or add them.
///
/// Appends accept arbitrary transport framing under explicit work limits. Memory
/// is fixed hash/frontier state, independent of file size; no body is retained.
/// Accepted prefixes are **not verified bytes**. The host must keep them private
/// until the body ends successfully and `finish` succeeds, then publish the
/// exact checked destination. A transport error is not a successful end of body.
/// Host buffering, temporary-file durability, cancellation and concurrency need
/// their own limits; this model supplies none of those effects or guarantees.
///
/// A matching root proves byte/metadata consistency, not MIME safety, storage
/// availability, billing, tenant access or provider upload completion. A new
/// instance starts from zero; there is no imported progress or resume token.
pub struct CaffeineRootVerifier {
    expected_root: ProviderRootHash,
    hasher: CaffeineContentHasher,
}

impl CaffeineRootVerifier {
    /// Bind the expected root, length and metadata before accepting any bytes.
    ///
    /// Uses the same bounded metadata normalization and hashing as the uploader.
    /// No metadata or body-sized allocation is retained after construction.
    /// # Errors
    /// Rejects empty/unbounded content or invalid/over-budget hash metadata using
    /// the shared hasher's typed errors. Metadata hashing is not HTTP validation.
    pub fn new(
        expected_root: ProviderRootHash,
        expected_bytes: u64,
        headers: &[CaffeineHeader<'_>],
        limits: CaffeineHashLimits,
    ) -> Result<Self, CaffeineHashError> {
        Ok(Self {
            expected_root,
            hasher: CaffeineContentHasher::new(expected_bytes, headers, limits)?,
        })
    }

    /// Fixed expected provider identity; never inferred from downloaded bytes.
    #[must_use]
    pub const fn expected_root(&self) -> ProviderRootHash {
        self.expected_root
    }

    /// Accepted bytes and required next offset, not verified or durable progress.
    #[must_use]
    pub const fn received_bytes(&self) -> u64 {
        self.hasher.received_bytes()
    }

    /// Bytes still needed to reach the declared length, not proof of HTTP EOF.
    #[must_use]
    pub const fn remaining_bytes(&self) -> u64 {
        self.hasher.remaining_bytes()
    }

    /// Hash the next bounded frame, preserving all state on an admission error.
    ///
    /// Corrupt bytes cannot be detected until final root verification. Accepted
    /// input cannot be overwritten; discard this instance to restart the body.
    /// # Errors
    /// Rejects wrong offset, per-append work overflow or excess total bytes.
    pub fn append(&mut self, offset: u64, bytes: &[u8]) -> Result<(), CaffeineHashError> {
        self.hasher.append(offset, bytes)
    }

    /// Consume the stream and check exact length and the originally bound root.
    ///
    /// The returned raw digest is computed from these checked bytes; it is not
    /// compared with an independent expected raw digest. Use the existing
    /// `CaffeineContentHasher::verify` when both identities must match.
    /// The host must additionally require successful transport EOF and retain
    /// the exact checked bytes before disclosing a download as verified.
    /// # Errors
    /// Incomplete input or root mismatch consumes this instance without success.
    pub fn finish(self) -> Result<CaffeineContentHashes, CaffeineHashError> {
        let actual = self.hasher.finish()?;
        if actual.provider_root != self.expected_root {
            return Err(CaffeineHashError::ProviderRootMismatch);
        }
        Ok(actual)
    }
}
