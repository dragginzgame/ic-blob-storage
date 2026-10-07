//! Installed local attachment envelope, not production spendability or account credit.
use candid::CandidType;
use serde::Deserialize;

/// Explicit installation budget. No reset, replenishment or inferred balance exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingBudgetInput {
    /// Maximum locally allocated attachment cycles; execution fees are separate.
    pub allocated: u128,
    /// Positive part of the allocation that attachments must preserve.
    pub reserve: u128,
    /// Additional positive liquid-cycle slack, independent of the attachment reserve.
    pub operating_reserve: u128,
    /// Explicit local liabilities still payable from liquid cycles; never inferred.
    pub other_liabilities: u128,
}

/// Accounting derived from the bounded exact transfer journal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingBudgetView {
    /// Installed additional liquid-cycle slack.
    pub operating_reserve: u128,
    /// Installed local liabilities, separate from already spent attachments.
    pub other_liabilities: u128,
    /// Installed local allocation, never increased by receipts or gross cycles.
    pub allocated: u128,
    /// Installed attachment reserve.
    pub reserve: u128,
    /// Revision consumed by each intent admission and terminal observation.
    pub revision: u64,
    /// Allocation minus accepted attachments and all unresolved reservations.
    pub available: u128,
    /// Actual transport acceptance, still requiring independent credit evidence.
    pub accepted: u128,
    /// Exact known callback refunds; excludes execution fees and unsent attachments.
    pub refunded: u128,
    /// Attachments proven not to have been enqueued, including liquidity rejection.
    pub not_enqueued: u128,
    /// Full attachments with no terminal observation, including callback rollback.
    pub reserved_or_uncertain: u128,
}
