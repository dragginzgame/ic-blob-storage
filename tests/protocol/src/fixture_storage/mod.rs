//! Shared persistence plumbing for private canister probes, never a product codec.
use ic_memory::{Memory, MemoryManagerConfig, testing::FixtureCell};
use ic_platform::candid::{
    contract::{CandidType, encode_one},
    decoder::decode_one_with_config,
};
use serde::de::DeserializeOwned;

/// One explicitly bounded probe journal with its selected decoder work budget.
pub struct FixtureJournal<M: Memory> {
    cell: FixtureCell<M>,
    maximum: usize,
    decoding_quota: usize,
}
impl<M: Memory> FixtureJournal<M> {
    /// Create a fresh cell; populated memory must refuse rather than reset.
    /// # Panics
    /// Traps on invalid profile, existing state or storage failure.
    pub fn create(memory: M, key: &str, maximum: usize, decoding_quota: usize) -> Self {
        let cell = FixtureCell::create_bounded(
            memory,
            "fixture",
            "fixture.",
            key,
            MemoryManagerConfig::new(16).expect("host bucket profile"),
            maximum,
        )
        .expect("fresh bounded fixture journal");
        Self {
            cell,
            maximum,
            decoding_quota,
        }
    }
    /// Open only an existing cell under the original profile.
    /// # Panics
    /// Traps on absent, incompatible or corrupt retained storage.
    pub fn open(memory: M, key: &str, maximum: usize, decoding_quota: usize) -> Self {
        let cell = FixtureCell::open_bounded(
            memory,
            "fixture",
            "fixture.",
            key,
            MemoryManagerConfig::new(16).expect("host bucket profile"),
            maximum,
        )
        .expect("retained bounded fixture journal");
        Self {
            cell,
            maximum,
            decoding_quota,
        }
    }
    /// Save the caller's existing Candid record under its byte ceiling.
    /// # Panics
    /// Traps on encoding, byte admission or storage failure.
    pub fn save<T: CandidType>(&mut self, state: &T) {
        self.cell
            .try_set(encode_one(state).expect("bounded fixture journal"))
            .expect("journal byte budget");
    }
    /// Load the original record; absent/invalid bytes never seed an empty journal.
    /// # Panics
    /// Traps on empty, oversized, malformed or over-budget input.
    #[must_use]
    pub fn load<T: CandidType + DeserializeOwned>(&self) -> T {
        let bytes = self.cell.bytes();
        assert!(
            !bytes.is_empty() && bytes.len() <= self.maximum,
            "retained journal required"
        );
        let limits =
            ic_blob_storage_contracts::decoding::limits(self.decoding_quota, 10_000, 64, None);
        decode_one_with_config(bytes, &limits).expect("same-release journal")
    }
}

#[cfg(test)]
mod tests;
