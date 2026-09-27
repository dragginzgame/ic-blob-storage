//! Bounded fixture traversal inputs; limits are installed by the host, not callers.
use crate::admission::ContentObservation;
use candid::{CandidType, Deserialize, Principal};
/// Independently authenticated read scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Scope {
    /// Caller must equal the supplied tenant.
    Tenant(Principal),
    /// Caller must be the configured operator.
    Service,
}
/// Current local states, never an effect authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Filter {
    /// All retained history.
    All,
    /// Reserved/possibly exposed operations.
    Active,
    /// Physical deletion remains pending after last release.
    DeletionPending,
    /// Active uploads or unsettled confirmed objects.
    Outstanding,
}
/// Untrusted scope-bound forward position; not a snapshot or durable progress proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Cursor {
    /// Original service.
    pub service: Principal,
    /// Original namespace.
    pub namespace: u128,
    /// Original observer scope.
    pub scope: Scope,
    /// Original filter.
    pub filter: Filter,
    /// Tenant of last inspected operation.
    pub after_tenant: Principal,
    /// Full-width last inspected request ID.
    pub after_request: u128,
}
/// One scan with fixed host scan/result limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ScanInput {
    /// Expected running service.
    pub service: Principal,
    /// Expected namespace.
    pub namespace: u128,
    /// Scope checked independently of cursor contents.
    pub scope: Scope,
    /// Current-state predicate.
    pub filter: Filter,
    /// Resume position, if any.
    pub cursor: Option<Cursor>,
}
/// Current page with progress even when no entry matched.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Page {
    /// Matching entries only; no file bytes or manifests.
    pub entries: Vec<ContentObservation>,
    /// Next position, not authority to skip later reconciliation sweeps.
    pub next: Option<Cursor>,
    /// Inspected operation rows.
    pub scanned: u64,
}

/// Bounded operator input; root knowledge never supplies tenant/provider authority.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct RootBatchInput {
    /// Expected running service.
    pub service: Principal,
    /// Expected installed namespace.
    pub namespace: u128,
    /// Raw roots; duplicates and malformed bytes count against fixed host limits.
    pub roots: Vec<Vec<u8>>,
}
/// Local reconciliation observation, not a provider liveness/deletion boolean.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum RootObservation {
    /// Exact retained local identity and current state.
    Known(ContentObservation),
    /// No local claim; not permission to delete or retry.
    Unknown,
    /// Binary root was not 32 bytes.
    Malformed {
        /// Actual byte length, without retaining untrusted bytes.
        bytes: u64,
    },
}
