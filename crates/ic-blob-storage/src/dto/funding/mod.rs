//! Passive local funding history; no payment or provider-credit authority.
pub mod outcome;
use super::operator::OperatorScope;
use candid::{CandidType, Deserialize};

/// Descending local operation position, never a snapshot or retry permission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingHistoryCursor {
    /// Complete original journal scope.
    pub scope: OperatorScope,
    /// Exclusive positive upper bound for the next operation.
    pub before_operation: u128,
}
/// One operator history query with host-owned result bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingHistoryRequest {
    /// Explicit installed service, namespace, Cashier and payer.
    pub scope: OperatorScope,
    /// Continuation; resweep to observe activity behind this position.
    pub cursor: Option<FundingHistoryCursor>,
}
/// Local attachment progress, separate from any provider response or credit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingPhase {
    /// Attachment reserved before a local attempt marker.
    Prepared,
    /// Attempt marked; full attachment remains potentially spent.
    Uncertain,
    /// Trusted local observation proves the call was not enqueued.
    NotEnqueued,
    /// Exact callback refund; accepted remainder is not proof of provider credit.
    Callback {
        /// Cycles returned in the exact callback, bounded by the original offer.
        refunded: u128,
    },
}
/// Original identity and amounts recovered without a saved caller request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingHistoryEntry {
    /// Full retained scope, not a caller-selected transport destination.
    pub scope: OperatorScope,
    /// Positive lifetime local operation identity; not a provider receipt ID.
    pub operation: u128,
    /// Original positive cycle attachment, independent of execution fees.
    pub offered: u128,
    /// Exact optional target balance; absence is never defaulted.
    pub target_balance: Option<u128>,
    /// Retained local progress only.
    pub phase: FundingPhase,
}
/// Bounded current observations, newest local operation first.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingHistoryPage {
    /// Complete request echoed for correlation.
    pub request: FundingHistoryRequest,
    /// Exact local observations; no provider call or reconciliation is implied.
    pub entries: Vec<FundingHistoryEntry>,
    /// Absent only at the end of this local range, not proof of account completeness.
    pub next: Option<FundingHistoryCursor>,
    /// Restored journal permits inspection only.
    pub fenced: bool,
}
/// Inspection refusals never become empty history or retry permission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingHistoryFailure {
    /// Zero namespace or continuation identity.
    Invalid,
    /// Actual caller is not the configured operator.
    Denied,
    /// Actual service or requested journal scope differs from installation.
    Binding,
    /// Cursor belongs to a different journal scope.
    CursorScope,
    /// Retained records cannot be read consistently.
    Internal,
}
