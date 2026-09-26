//! Local obligation observations and explicitly substituted confirmation facts.

use candid::CandidType;
use serde::Deserialize;

/// Local lifecycle phase observed in the fixture's unsettled-object view.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ObligationProbePhase {
    /// A tenant reference remains live.
    Live,
    /// Last reference released; deletion remains unconfirmed.
    DeletionPending,
    /// Physical deletion confirmed; billing remains unresolved.
    ProviderDeleted,
}

/// Test-only projection of one tenant-owned object's current capacity charges.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ObligationProbeView {
    /// Repeated-byte fixture root identity.
    pub root: u8,
    /// Current local phase.
    pub phase: ObligationProbePhase,
    /// Physical capacity still charged.
    pub physical_bytes: u64,
    /// Billing-byte capacity still charged, not a currency amount.
    pub liability_bytes: u64,
}

/// Operator-supplied substitute for exact external evidence, never a provider contract.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub enum ObligationProbeFact {
    /// Pretend exact physical deletion evidence was independently authenticated.
    Deleted,
    /// Pretend exact final billing evidence was independently authenticated.
    BillingStopped,
}
