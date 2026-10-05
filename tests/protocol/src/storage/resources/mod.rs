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
    /// Lifetime funding intent ceiling in 1..=10,000; ordinary tests use four.
    pub max_funding_attempts: u32,
    /// Concurrent read ceiling in 1..=1,024; ordinary tests use one.
    pub max_read_sessions: u32,
    /// Concurrent reads per tenant, in `1..=max_read_sessions`; ordinarily one.
    pub max_tenant_read_sessions: u32,
}

/// One bounded population step, never a provider request or replay instruction.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PopulationBatch {
    /// Expected lifetime permission count before this step.
    pub start: u32,
    /// At most 100 client-prepared synthetic operations; no body hashing in the canister.
    pub uploads: Vec<crate::admission::input::PreparationInput>,
}

/// Explicit local funding fact, never provider evidence or a dispatch instruction.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub struct FundingPopulationIntent {
    /// Exact operator/account-bound identity for the maintained funding journal.
    pub intent: super::funding::Intent,
    /// Desired local state; pending states can occur only at the end of history.
    pub phase: super::funding::Phase,
}

/// At most sixteen local funding/read admissions in one atomic fixture update.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HistoryPopulationBatch {
    /// Expected retained funding count before this message.
    pub funding_start: u64,
    /// Expected read high-water sequence before this message.
    pub read_start: u64,
    /// Local journal transitions only; no cycle dispatch.
    pub funding: Vec<FundingPopulationIntent>,
    /// Workflow-admitted occupied sessions; no transport or completion ticket export.
    pub reads: Vec<super::gateways::ReadSessionInput>,
}

/// Bounded operator traversal of exact occupied read targets, including while fenced.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub struct RestorationReadPage {
    /// Exclude this sequence from the next page.
    pub after: u64,
    /// In 1..=64, matching the authoritative read owner's inspection bound.
    pub limit: u32,
}

/// Passive retained read observation; cannot authorize dispatch or completion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct RestorationRead {
    /// Exact durable identity, not an allocation or freshness proof.
    pub sequence: u64,
    /// Original bound chunk/reference/gateway; fixture fault selectors are always absent.
    pub target: super::gateways::ReadSessionInput,
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
