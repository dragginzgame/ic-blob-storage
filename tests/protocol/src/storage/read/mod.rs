//! Bounded fixture traversal inputs; limits are installed by the host, not callers.
use crate::admission::ContentObservation;
use candid::{CandidType, Deserialize, Principal};
/// Explicit local test client controls, never a production service request.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct DownloadClientInput {
    /// The maintained service request, shared rather than recreated in the fixture.
    pub request: ic_blob_storage::dto::download::DownloadRequest,
    /// Client's expected actual execution identity.
    pub tenant: Principal,
    /// Independently expected fixture project.
    pub project: String,
    /// Application reply bound before decoding.
    pub max_reply_bytes: u32,
}
/// Typed local observation of the canonical client's result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum DownloadProbeFailure {
    /// Proxy caller is not the fixture operator.
    Denied,
    /// Invalid local client settings or malformed reply.
    Invalid,
    /// Client, target or reply binding differs.
    Binding,
    /// Actual execution was not replicated.
    Execution,
    /// Platform rejection code.
    Rejected(u32),
    /// Call was not enqueued.
    NotEnqueued,
    /// Reply exceeds the application budget.
    Limit,
    /// Typed refusal from the service boundary.
    Remote(ic_blob_storage::dto::download::DownloadFailure),
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
