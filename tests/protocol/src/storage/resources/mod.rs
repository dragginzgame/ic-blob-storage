//! Passive controls and observations for opt-in local resource measurements.
use candid::{CandidType, Deserialize, Principal};

/// The fixture's single current installation contract, also used on same-release reopen.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub struct StorageProbeInstallation {
    /// Explicit fixture operator.
    pub operator: Principal,
    /// Lifetime operation ceiling, in 1..=10,000; ordinary tests use two.
    pub max_objects: u32,
}

/// One bounded population step, never a provider request or replay instruction.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub struct PopulationBatch {
    /// Expected lifetime permission count before this step.
    pub start: u32,
    /// At most 100 fresh synthetic operations.
    pub count: u32,
}

/// Counters retained in heap for the most recent initialization only.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RestorationResources {
    /// Whether this execution opened existing stores.
    pub restored: bool,
    /// Instructions in host initialization through publication of state.
    pub initialization_instructions: u64,
    /// Instructions in the existing `ServiceStores` install/open call.
    pub stores_instructions: u64,
    /// Allocated Wasm linear memory at the end of initialization, not live heap bytes.
    pub heap_bytes: u64,
    /// Physical stable memory at the end of initialization.
    pub stable_bytes: u64,
    /// Instrumented logical-memory reads during store reopening only.
    pub reads: Vec<RestorationMemoryReads>,
}

/// Read-only observations of one fixture memory; not persisted accounting.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RestorationMemoryReads {
    /// Existing host grant key, never a second store identifier.
    pub memory: String,
    /// Calls to the logical memory's read method.
    pub calls: u64,
    /// Requested bytes, including repeated reads.
    pub bytes: u64,
    /// Instructions inside the wrapped read; excludes B-tree decoding/traversal outside it.
    pub instructions: u64,
}
