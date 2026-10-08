//! Exact current local reference state, distinct from historical operation receipts.
use crate::dto::reference::ReferenceUpload;
use candid::CandidType;
use candid::Deserialize;

/// Tenant-owned lookup, with no mutation operation or action to replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceStatusRequest {
    /// Complete original upload binding, including independent object/first-reference IDs.
    pub upload: ReferenceUpload,
    /// Exact positive reference identity to inspect.
    pub reference: u128,
}
/// One synchronous retained-state observation, never a serving or mutation lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceStatusResponse {
    /// Full request echoed for correlation.
    pub request: ReferenceStatusRequest,
    /// Whether this exact reference is currently active in local retained state.
    /// False covers both never-retained and released references, not reusable identity.
    pub live: bool,
    /// The same upload owner's restore fence; even a live reference cannot bypass it.
    pub fenced: bool,
}
