//! Exact local intent lookup, never a provider completion query or retry permit.
use super::{FundingObservation, FundingRequest};
use candid::{CandidType, Principal};
use serde::Deserialize;

/// Caller-supplied original request and explicit journal bindings.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingLookupRequest {
    /// Actual service whose retained journal is queried.
    pub service: Principal,
    /// Expected transfer peer, not a qualified Cashier account.
    pub peer: Principal,
    /// Every immutable input must match, including local experiment controls.
    pub attempt: FundingRequest,
}

/// A failed lookup exposes no retained outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingLookupFailure {
    /// Caller lacks explicit driver authority.
    Denied,
    /// Service or peer differs from the journal.
    Binding,
    /// Input could not be a valid local transfer request.
    InvalidRequest,
    /// Identity exists with different immutable inputs.
    Conflict,
}

/// Current retained evidence; none of these states authorizes another effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingLookupState {
    /// No entry in this journal. Old backups can omit paid operations.
    Absent,
    /// Intent exists without terminal evidence, including callback rollback.
    Pending,
    /// Exact retained transport observation, not independent provider credit.
    Observed(FundingObservation),
}

/// Query-time projection; recovery and provider credit remain unqualified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingLookupView {
    /// Exact query binding, echoed even for absent/pending results.
    pub request: FundingLookupRequest,
    /// Whether the current owner is permanently fenced after restore.
    pub fenced: bool,
    /// Evidence for this request in the current retained journal only.
    pub state: FundingLookupState,
}
