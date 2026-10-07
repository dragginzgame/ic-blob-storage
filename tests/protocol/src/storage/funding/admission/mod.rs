//! Passive preparation diagnosis; no host evidence can be supplied through ingress.
use super::Intent;
use candid::{CandidType, Deserialize};
/// Independent reasons a new intent cannot be reserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Blocker {
    /// Configured provider/account is not qualified.
    ProviderUnqualified,
    /// Actual journal is restored and fenced.
    JournalFenced,
    /// Local acceptance or reservation remains uncredited/unresolved.
    JournalUncredited,
    /// No exact intent has been reserved for this attempt.
    IntentMissing,
    /// This retained intent has already been attempted, regardless of outcome.
    AlreadyAttempted,
    /// Exact unattempted intent does not own the final reservation.
    ReservationMismatch,
    /// Exact identity already exists.
    IdentityRetained,
    /// A fresh identity must exceed retained history.
    IdentityStale,
    /// Lifetime intent slots exhausted.
    JournalFull,
    /// Full amount exceeds local attachment allowance; value is allowance only.
    AllocationReserve(u128),
    /// Independent host recovery is unestablished.
    RecoveryUnknown,
    /// Cycles after all liabilities are unestablished.
    SpendabilityUnknown,
    /// Complete external account activity is unestablished.
    AccountActivityUnknown,
    /// Host evidence independently fences recovery.
    HostRecoveryFenced,
    /// External account activity is in progress.
    AccountActivityInProgress,
    /// External account activity is unresolved.
    AccountActivityUncertain,
    /// Funding limits are absent.
    NotConfigured,
    /// Full amount exceeds authoritative spendability after reserve.
    SpendableReserve {
        /// Unchanged complete request.
        requested: u128,
        /// Diagnostic allowance, never a reduced request.
        transferable: u128,
    },
}
/// Read-only exact request and all current blockers, not a reusable authorization.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct View {
    /// Original proposed identity, offer and target.
    pub intent: Intent,
    /// Independent local constraints and external missing evidence.
    pub blockers: Vec<Blocker>,
}
/// Synchronous reservation result; it never sends a provider call.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Preparation {
    /// No mutation; retained diagnostics explain refusal.
    Blocked(Vec<Blocker>),
    /// A local reservation was made; no dispatch authority follows.
    Prepared,
}
/// First-attempt marker result, never a provider response or reusable permit.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Attempt {
    /// No mutation; the existing reservation remains charged.
    Blocked(Vec<Blocker>),
    /// Marker persisted; transport and current liquidity checks remain separate.
    Marked,
}
