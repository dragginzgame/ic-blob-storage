//! Passive controls and observations for opt-in local resource measurements.
use ic_platform::principal::ExternalPrincipal;
use serde::Deserialize;

/// Existing complete ingress byte ceiling of the private storage fixture.
pub const STORAGE_PROBE_INPUT_BYTES: usize = 4096;

/// The fixture's single current installation contract, also used on same-release reopen.
#[derive(Clone, Copy, Debug, Deserialize, serde::Serialize)]
pub struct StorageProbeInstallation {
    /// Explicit fixture operator.
    pub operator: ExternalPrincipal,
    /// Complete explicit service resource envelope, shared by every fixture scenario.
    pub resources: ic_blob_storage_contracts::dto::configuration::ServiceResourceInput,
    /// Lifetime funding intent ceiling in 1..=10,000; ordinary tests use four.
    pub max_funding_attempts: u32,
}

/// One bounded population step, never a provider request or replay instruction.
#[derive(Clone, Debug, Deserialize, serde::Serialize)]
pub struct PopulationBatch {
    /// Expected lifetime permission count before this step.
    pub start: u32,
    /// At most 100 client-prepared synthetic operations; no body hashing in the canister.
    pub uploads: Vec<crate::admission::input::PreparationInput>,
}

/// Explicit local funding fact, never provider evidence or a dispatch instruction.
#[derive(Clone, Copy, Debug, Deserialize, serde::Serialize)]
pub struct FundingPopulationIntent {
    /// Exact operator/account-bound identity for the maintained funding journal.
    pub intent: super::funding::Intent,
    /// Desired local state; pending states can occur only at the end of history.
    pub phase: super::funding::Phase,
    /// Optional synthetic host receipt; never deployed-provider evidence.
    pub credit_digest: Option<[u8; 32]>,
}

/// At most sixteen local funding admissions in one atomic fixture update.
#[derive(Clone, Debug, Deserialize, serde::Serialize)]
pub struct HistoryPopulationBatch {
    /// Expected retained funding count before this message.
    pub funding_start: u64,
    /// Local journal transitions only; no cycle dispatch.
    pub funding: Vec<FundingPopulationIntent>,
}

/// Counters retained in heap for the most recent initialization only.
#[derive(Clone, Debug, Deserialize, serde::Serialize)]
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
#[derive(Clone, Debug, Deserialize, serde::Serialize)]
pub struct RestorationMemoryReads {
    /// Existing host grant key, never a second store identifier.
    pub memory: String,
    /// Calls to the logical memory's read method, saturating at `u64::MAX`.
    pub calls: u64,
    /// True only after count overflow; an exactly reached cap remains exact.
    pub calls_saturated: bool,
    /// Requested bytes, including repeated reads, saturating at `u64::MAX`.
    pub bytes: u64,
    /// True only after byte overflow; bytes remain separate from instruction units.
    pub bytes_saturated: bool,
    /// Instructions inside the wrapped read; excludes B-tree decoding/traversal outside it.
    /// Accumulated `u64` observations, saturating at `u128::MAX`.
    /// Consult the explicit overflow flag before using this as an exact total.
    pub instructions: u128,
    /// True only after total overflow, sticky until the next host measurement window.
    pub instructions_saturated: bool,
}

/// Private local measurements around the maintained host confirmation workflow.
#[derive(Clone, Debug, Deserialize, serde::Serialize)]
pub struct FundingCreditResources {
    /// Exact workflow result, including refused authority and recovery fences.
    pub result: Result<Option<bool>, super::Failure>,
    /// Instructions through confirmation, including observer overhead.
    pub instructions: u64,
    /// Allocated Wasm linear memory after the call, not live heap.
    pub heap_bytes: u64,
    /// Physical stable memory after the call.
    pub stable_bytes: u64,
    /// Logical reads around this call only, excluding previous population.
    pub reads: Vec<RestorationMemoryReads>,
}

/// Existing small Storage fixture envelope. Callers retain explicit object, tenant and byte limits.
/// # Panics
/// Refuses byte/chunk products outside the bounded fixture representation.
#[must_use]
pub fn small_resources(
    objects: u32,
    tenants: u32,
    bytes: u64,
) -> ic_blob_storage_contracts::dto::configuration::ServiceResourceInput {
    let chunks = u32::try_from(
        bytes.div_ceil(ic_blob_storage_contracts::identity::caffeine::CAFFEINE_CHUNK_BYTES as u64),
    )
    .expect("bounded fixture bytes")
        * objects;
    ic_blob_storage_contracts::dto::configuration::ServiceResourceInput {
        max_tenants: tenants,
        max_object_bytes: bytes,
        max_headers: 8,
        max_header_bytes: 1024,
        max_chunks: chunks,
        max_tenant_chunks: chunks,
        max_objects: objects,
        max_tenant_objects: objects,
        max_physical_bytes: u128::from(objects) * u128::from(bytes),
        max_liability_bytes: u128::from(objects) * u128::from(bytes),
        max_tenant_logical_bytes: u128::from(objects) * u128::from(bytes),
        max_references_per_object: 2,
        max_receipts_per_object: 3,
        max_active: objects,
        max_tenant_active: objects,
    }
}
/// Preserve the measured admission workloads using the same production owner/configuration contract.
#[must_use]
pub fn admission_resources(
    workload: crate::admission::Workload,
) -> ic_blob_storage_contracts::dto::configuration::ServiceResourceInput {
    use crate::admission::Workload;
    let (objects, tenant_objects, chunks, tenant_chunks, tenant_bytes, references, receipts) =
        match workload {
            Workload::Small => (4, 2, 15, 10, 10, 2, 3),
            Workload::RetainedHistory => (256, 128, 256, 128, 10, 2, 3),
            Workload::ReleaseHistory => (704, 704, 768, 768, 288, 4, 7),
            Workload::ReferenceHistory => (1, 1, 10, 10, 10, 256, 511),
        };
    ic_blob_storage_contracts::dto::configuration::ServiceResourceInput {
        max_tenants: 2,
        max_object_bytes: 10 * 1024 * 1024,
        max_headers: 8,
        max_header_bytes: 1024,
        max_chunks: chunks,
        max_tenant_chunks: tenant_chunks,
        max_objects: objects,
        max_tenant_objects: tenant_objects,
        max_physical_bytes: 2 * tenant_bytes * 1024 * 1024,
        max_liability_bytes: 2 * tenant_bytes * 1024 * 1024,
        max_tenant_logical_bytes: tenant_bytes * 1024 * 1024,
        max_references_per_object: references,
        max_receipts_per_object: receipts,
        max_active: objects.min(4),
        max_tenant_active: tenant_objects.min(2),
    }
}
