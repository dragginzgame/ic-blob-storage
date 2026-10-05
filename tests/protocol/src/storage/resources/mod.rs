//! Passive controls and observations for opt-in local resource measurements.
use candid::{CandidType, Deserialize, Principal};

/// Existing complete ingress byte ceiling of the private storage fixture.
pub const STORAGE_PROBE_INPUT_BYTES: usize = 4096;

/// The fixture's single current installation contract, also used on same-release reopen.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub struct StorageProbeInstallation {
    /// Explicit fixture operator.
    pub operator: Principal,
    /// Lifetime operation ceiling, in 1..=10,000; ordinary tests use two.
    pub max_objects: u32,
    /// Explicit tenant ceiling in 1..=32; ordinary tests use two.
    pub max_tenants: u32,
    /// Object length ceiling in 10..=64 MiB; ordinary tests use ten bytes.
    pub max_object_bytes: u64,
}

/// One bounded population step, never a provider request or replay instruction.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PopulationBatch {
    /// Expected lifetime permission count before this step.
    pub start: u32,
    /// At most 100 client-prepared synthetic operations; no body hashing in the canister.
    pub uploads: Vec<crate::admission::input::PreparationInput>,
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
