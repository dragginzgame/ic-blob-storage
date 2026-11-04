//! Pinned SHA-256 state conversion, kept private to the verification model.
use crate::identity::verification::ContentDigest;
use crate::identity::verification::ContentVerifier;
use sha2::Sha256;
use sha2::digest::common::hazmat::SerializableState;
use sha2::digest::common::hazmat::SerializedState;

// sha2 0.11.0: 32 chaining bytes, u64 little-endian block count, then the
// eager buffer's one-byte position and up to 63 canonical data bytes. No unsafe layout cast.
pub(crate) const HASH_STATE_BYTES: usize = 104;

pub(crate) struct ContentVerifierStateRecord {
    pub expected_digest: ContentDigest,
    pub expected_bytes: u64,
    pub received_bytes: u64,
    pub hash_state: [u8; HASH_STATE_BYTES],
}

impl ContentVerifier {
    pub(crate) fn checkpoint_state(&self) -> ContentVerifierStateRecord {
        ContentVerifierStateRecord {
            expected_digest: self.expected_digest,
            expected_bytes: self.expected_bytes,
            received_bytes: self.received_bytes,
            hash_state: self.hasher.serialize().into(),
        }
    }
}

impl ContentVerifierStateRecord {
    pub(crate) fn restore(self) -> Option<ContentVerifier> {
        if self.expected_bytes > ContentVerifier::MAX_BYTES
            || self.received_bytes > self.expected_bytes
        {
            return None;
        }
        let blocks = u64::from_le_bytes(self.hash_state[32..40].try_into().ok()?);
        let buffered = u64::from(self.hash_state[40]);
        if blocks.checked_mul(64)?.checked_add(buffered)? != self.received_bytes {
            return None;
        }
        let state: SerializedState<Sha256> = self.hash_state.into();
        let hasher = Sha256::deserialize(&state).ok()?;
        Some(ContentVerifier {
            expected_digest: self.expected_digest,
            expected_bytes: self.expected_bytes,
            received_bytes: self.received_bytes,
            hasher,
        })
    }
}

#[cfg(test)]
mod tests;
