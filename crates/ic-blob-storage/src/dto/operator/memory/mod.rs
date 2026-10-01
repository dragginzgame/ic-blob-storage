//! Passive host backing-capacity observations, independent of tenant/provider usage.
use super::OperatorScope;
use candid::{CandidType, Deserialize};

/// Numeric allocation totals for the host's entire shared memory runtime.
/// Capacity includes the ledger and other owners; it is never payload occupancy,
/// tenant quota, provider storage or permission to activate a restored service.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct HostMemoryStatus {
    /// Exact authenticated service/provider/account scope.
    pub scope: OperatorScope,
    /// Actual persisted bucket size in Wasm pages.
    pub bucket_size_pages: u16,
    /// Host backing extent in bytes.
    pub physical_extent_bytes: u64,
    /// Sum of addressable virtual extents.
    pub virtual_extent_bytes: u64,
    /// Assigned bucket capacity in bytes.
    pub allocated_bucket_bytes: u64,
    /// Assigned bucket capacity beyond virtual extents.
    pub bucket_slack_bytes: u64,
    /// Remaining finite bucket-table slots, not a backing-growth guarantee.
    pub remaining_buckets: u32,
    /// Allocated capacity currently bound to application declarations.
    pub current_binding_bytes: u64,
    /// Allocated capacity in the substrate ledger slot.
    pub ledger_binding_bytes: u64,
    /// Allocated capacity with no current declaration; not unused capacity.
    pub unknown_binding_bytes: u64,
    /// Backing bytes outside manager metadata and assigned buckets.
    pub unmanaged_bytes: u64,
    /// Manager metadata capacity in bytes.
    pub manager_metadata_bytes: u64,
}
