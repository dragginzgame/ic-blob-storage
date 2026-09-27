//! Operator discovery with fixed host page bounds; no dispatch or unfence authority.
use super::{Intent, Phase};
use candid::{CandidType, Deserialize, Principal};

/// Explicit installed journal scope, independent of the continuation position.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Scope {
    /// Running service.
    pub service: Principal,
    /// Installed Cashier candidate.
    pub cashier: Principal,
    /// Installed payment account.
    pub account: Principal,
    /// Installed provider namespace.
    pub namespace: u128,
}
/// Untrusted descending position, never a snapshot or retry permission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Cursor {
    /// Original complete query scope.
    pub scope: Scope,
    /// Exclusive upper bound on the next operation ID.
    pub before_operation: u128,
}
/// Discovery request; the host fixes the result limit, not the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Input {
    /// Independently checked installation scope.
    pub scope: Scope,
    /// Continue an earlier page; new activity requires a fresh sweep.
    pub cursor: Option<Cursor>,
}
/// Complete original intent and current local progress, not provider credit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Entry {
    /// Identity and amounts recovered without a saved caller request.
    pub intent: Intent,
    /// Current retained state.
    pub phase: Phase,
}
/// Bounded current observations in descending operation-ID order.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Page {
    /// Exact local observations, newest first.
    pub entries: Vec<Entry>,
    /// Resume with the same scope, including after same-release restoration.
    pub next: Option<Cursor>,
}
