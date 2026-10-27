//! Bounded Wire journals over Memory's canonical test cell; no product storage codec.
use ic_memory::{Memory, MemoryManagerConfig, testing::FixtureCell};
use ic_wire::{Limits, native};
use serde::{Serialize, de::DeserializeOwned};

// A storage-only discriminator, distinct from RPCs and offline documents.
const JOURNAL_ID: u64 = 0x424c_4f42_4a4e_4c31;

/// One fresh-or-retained journal bound to its allocation key and finite Wire policy.
/// Earlier Candid payloads and custom consumer envelopes are deliberately unreadable.
/// Initialization and restoration never substitute empty state for retained bytes.
pub struct FixtureJournal<M: Memory> {
    cell: FixtureCell<M>,
    key: String,
    limits: Limits,
}
impl<M: Memory> FixtureJournal<M> {
    /// Create only on empty backing, before allocating the canonical cell.
    /// # Panics
    /// Refuses populated backing, invalid policy or storage failure.
    pub fn create(memory: M, key: &str, limits: Limits) -> Self {
        validate(limits);
        assert_eq!(memory.size(), 0, "fresh fixture backing required");
        let cell = FixtureCell::create_bounded(
            memory,
            "fixture",
            "fixture.",
            key,
            MemoryManagerConfig::new(16).expect("host bucket profile"),
            limits.bytes,
        )
        .expect("fresh bounded fixture journal");
        Self {
            cell,
            key: key.to_owned(),
            limits,
        }
    }
    /// Open only an existing cell under the selected allocation and byte ceiling.
    /// # Panics
    /// Refuses missing, incompatible, oversized or corrupt cell metadata.
    pub fn open(memory: M, key: &str, limits: Limits) -> Self {
        validate(limits);
        let cell = FixtureCell::open_bounded(
            memory,
            "fixture",
            "fixture.",
            key,
            MemoryManagerConfig::new(16).expect("host bucket profile"),
            limits.bytes,
        )
        .expect("retained bounded fixture journal");
        Self {
            cell,
            key: key.to_owned(),
            limits,
        }
    }
    /// Encode the canonical record directly, retaining the key in the storage frame.
    /// # Panics
    /// Refuses encoding/resource violations before writing any candidate bytes.
    pub fn save<T: Serialize>(&mut self, state: &T) {
        let bytes = native::encode_request(JOURNAL_ID, &(&self.key, state), self.limits)
            .expect("bounded Wire fixture journal");
        self.cell.try_set(bytes).expect("journal byte budget");
    }
    /// Decode retained state; no Candid reader, migration or initialization fallback.
    /// # Panics
    /// Refuses empty, foreign, malformed, trailing, mismatched-key or over-budget data.
    #[must_use]
    pub fn load<T: DeserializeOwned>(&self) -> T {
        let (key, state): (String, T) =
            native::decode_request(JOURNAL_ID, self.cell.bytes(), self.limits)
                .expect("same-release Wire journal");
        assert_eq!(key, self.key, "fixture journal binding");
        state
    }
}
fn validate(limits: Limits) {
    assert!(
        limits.bytes > 0 && limits.bytes <= crate::fixture_wire::MAX_BYTES,
        "journal byte policy"
    );
    assert!(
        limits.allocation > 0 && limits.nodes > 0 && limits.depth > 0,
        "journal traversal policy"
    );
}

#[cfg(test)]
mod tests;
