//! Exact-release, bounded verifier state for a host's trusted storage.
//!
//! These bytes are not an upload receipt or a client resume token. Their checksum
//! detects accidental damage, not forgery or rollback. Hosts must separately bind
//! tenant/operation identity, protect storage and fence restored/stale instances.
use crate::identity::caffeine::manifest::verification::ordered::CAFFEINE_CHUNK_BYTES;
use crate::identity::caffeine::manifest::verification::ordered::CaffeineChunkManifest;
use crate::identity::caffeine::manifest::verification::ordered::CaffeineOrderedChunkVerifier;
use crate::identity::caffeine::manifest::verification::ordered::ContentDigest;
use crate::identity::verification::checkpoint::ContentVerifierStateRecord;
use crate::identity::verification::checkpoint::HASH_STATE_BYTES;
use sha2::Digest;
use sha2::Sha256;
use std::fmt;
use thiserror::Error;

const MAGIC: &[u8; 8] = b"ICBV\0\0\0\x01";
const RELEASE: &str = concat!(
    "ic-blob-storage/",
    env!("CARGO_PKG_VERSION"),
    ";sha2/0.11.0;checkpoint/v1"
);
const HASH_OFFSET: usize = 120;
const CHECKSUM_OFFSET: usize = HASH_OFFSET + HASH_STATE_BYTES;
const ENCODED_BYTES: usize = CHECKSUM_OFFSET + 32;

/// Opaque lossless hash checkpoint, compatible only with the same library release.
///
/// Restore requires the independently retained manifest and expected raw digest.
/// A checkpoint contains up to 63 buffered plaintext bytes: keep it in protected
/// host storage, never public views, logs, certificates or client-supplied tokens.
/// It provides no proof of provenance, freshness, tenant authority or provider
/// effects. Restoring it alone never authorizes resuming a service workflow.
#[derive(Clone, Eq, PartialEq)]
pub struct CaffeineVerificationCheckpointRecord {
    bytes: [u8; ENCODED_BYTES],
}

impl fmt::Debug for CaffeineVerificationCheckpointRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CaffeineVerificationCheckpointRecord")
            .finish_non_exhaustive()
    }
}

impl CaffeineVerificationCheckpointRecord {
    /// Fixed storage bound for this exact release's representation.
    pub const ENCODED_BYTES: usize = ENCODED_BYTES;

    /// Protected host-storage representation; may contain buffered plaintext.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn state(&self) -> ContentVerifierStateRecord {
        ContentVerifierStateRecord {
            expected_digest: ContentDigest::try_from(&self.bytes[72..104]).expect("fixed digest"),
            expected_bytes: u64::from_le_bytes(
                self.bytes[104..112].try_into().expect("fixed length"),
            ),
            received_bytes: u64::from_le_bytes(
                self.bytes[112..120].try_into().expect("fixed position"),
            ),
            hash_state: self.bytes[HASH_OFFSET..CHECKSUM_OFFSET]
                .try_into()
                .expect("fixed state"),
        }
    }
}

impl TryFrom<&[u8]> for CaffeineVerificationCheckpointRecord {
    type Error = CaffeineCheckpointError;

    /// Decode only bounded current-release bytes from trusted storage.
    /// # Errors
    /// Rejects wrong size/format/release, checksum damage and inconsistent hash
    /// state. This cannot detect a deliberately forged or rolled-back checkpoint.
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let bytes: [u8; ENCODED_BYTES] = input
            .try_into()
            .map_err(|_| CaffeineCheckpointError::InvalidLength)?;
        if &bytes[..8] != MAGIC {
            return Err(CaffeineCheckpointError::InvalidFormat);
        }
        if bytes[8..40] != Sha256::digest(RELEASE.as_bytes())[..] {
            return Err(CaffeineCheckpointError::WrongRelease);
        }
        if bytes[CHECKSUM_OFFSET..] != Sha256::digest(&bytes[..CHECKSUM_OFFSET])[..] {
            return Err(CaffeineCheckpointError::ChecksumMismatch);
        }
        let record = Self { bytes };
        let state = record.state();
        if state.expected_bytes == 0
            || (state.received_bytes != state.expected_bytes
                && !state
                    .received_bytes
                    .is_multiple_of(CAFFEINE_CHUNK_BYTES as u64))
        {
            return Err(CaffeineCheckpointError::InvalidState);
        }
        state
            .restore()
            .ok_or(CaffeineCheckpointError::InvalidState)?;
        Ok(record)
    }
}

impl CaffeineOrderedChunkVerifier {
    /// Capture a lossless bounded prefix without retaining the complete file.
    ///
    /// The host must commit this with its exact operation, manifest and accounting
    /// before exposing effects. The checksum is not authentication or freshness.
    #[must_use]
    pub fn checkpoint(&self) -> CaffeineVerificationCheckpointRecord {
        let state = self.raw.checkpoint_state();
        let mut bytes = [0; ENCODED_BYTES];
        bytes[..8].copy_from_slice(MAGIC);
        bytes[8..40].copy_from_slice(&Sha256::digest(RELEASE.as_bytes()));
        bytes[40..72].copy_from_slice(self.manifest.root().as_bytes());
        bytes[72..104].copy_from_slice(state.expected_digest.as_bytes());
        bytes[104..112].copy_from_slice(&state.expected_bytes.to_le_bytes());
        bytes[112..120].copy_from_slice(&state.received_bytes.to_le_bytes());
        bytes[HASH_OFFSET..CHECKSUM_OFFSET].copy_from_slice(&state.hash_state);
        let checksum = Sha256::digest(&bytes[..CHECKSUM_OFFSET]);
        bytes[CHECKSUM_OFFSET..].copy_from_slice(&checksum);
        CaffeineVerificationCheckpointRecord { bytes }
    }

    /// Reconstruct verification against separately trusted immutable declarations.
    ///
    /// This restores mathematical state, not operation authority or freshness.
    /// The host still owns rollback fencing and same-release recovery policy.
    /// # Errors
    /// Rejects a changed root, length or expected raw digest. Record decoding
    /// already checked the internal byte/block counts and ordered-leaf boundary.
    pub fn from_checkpoint(
        manifest: CaffeineChunkManifest,
        expected_digest: ContentDigest,
        checkpoint: &CaffeineVerificationCheckpointRecord,
    ) -> Result<Self, CaffeineCheckpointError> {
        let state = checkpoint.state();
        if checkpoint.bytes[40..72] != manifest.root().as_bytes()[..]
            || state.expected_bytes != manifest.content_bytes()
            || state.expected_digest != expected_digest
        {
            return Err(CaffeineCheckpointError::WrongDeclaration);
        }
        let raw = state
            .restore()
            .ok_or(CaffeineCheckpointError::InvalidState)?;
        Ok(Self { manifest, raw })
    }
}

/// Checkpoint representation or declaration failure; no state is resumed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum CaffeineCheckpointError {
    /// Truncated, extended or otherwise incorrectly sized input.
    #[error("invalid checkpoint length")]
    InvalidLength,
    /// Unsupported verifier representation.
    #[error("invalid checkpoint format")]
    InvalidFormat,
    /// Checkpoint belongs to another library release.
    #[error("checkpoint release mismatch")]
    WrongRelease,
    /// Encoded bytes do not match their accidental-damage checksum.
    #[error("checkpoint checksum mismatch")]
    ChecksumMismatch,
    /// Hash byte count, buffer, bounds or leaf position is inconsistent.
    #[error("invalid checkpoint state")]
    InvalidState,
    /// Trusted manifest/root/length or raw digest differs from the captured one.
    #[error("checkpoint declaration mismatch")]
    WrongDeclaration,
}

#[cfg(test)]
mod tests;
