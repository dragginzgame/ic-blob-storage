//! Passive local funding admission; never a transfer request or Cashier schema.
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Exact proposed identity/amount against an explicitly selected local journal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingPreviewRequest {
    /// Actual sender fixture.
    pub service: Principal,
    /// Journal-bound local peer.
    pub peer: Principal,
    /// Proposed operation identity; preview never consumes it.
    pub id: u64,
    /// Full positive amount to diagnose; no partial transfer is proposed.
    pub requested_cycles: u128,
    /// Exact expected attachment-budget revision from current status.
    pub revision: u64,
}

/// Input/authority errors, separate from a valid but blocked preview.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum FundingPreviewFailure {
    /// Caller lacks explicit driver authority.
    Denied,
    /// Service or peer differs from the current journal.
    Binding,
    /// Requested amount is zero.
    InvalidAmount,
}

/// Independent missing evidence or known reason to refuse new funding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum FundingPreviewBlocker {
    /// The local peer is not a qualified provider/account binding.
    ProviderUnqualified,
    /// This identity already exists, irrespective of amount or outcome.
    AlreadyAdmitted,
    /// The bounded journal cannot retain another intent.
    JournalFull,
    /// The full amount exceeds the maintained local transfer protocol bound.
    AmountLimitExceeded {
        /// Largest permitted local attachment; not a partial-transfer proposal.
        maximum_cycles: u128,
    },
    /// The attachment journal changed since the selected budget revision.
    BudgetRevisionStale,
    /// The complete request exceeds the local envelope above its reserve.
    BudgetReserveWouldBeViolated {
        /// Maximum local attachment above reserve; not a partial-transfer proposal.
        transferable_cycles: u128,
    },
    /// Platform call costs and installed operating holds leave too few liquid cycles.
    LiquidityWouldBeViolated {
        /// Maximum full attachment from this local query observation.
        transferable_cycles: u128,
    },
    /// No independent recovery authority is established.
    RecoveryUnknown,
    /// A restored owner is permanently inspection-only.
    RecoveryFenced,
    /// The complete funding activity has not been established.
    FundingUnknown,
    /// An intent is still running.
    FundingInProgress,
    /// A transfer or its provider credit remains unverified.
    FundingUncertain,
    /// Validated billing limits are absent.
    NotConfigured,
    /// Cycles after reservations/liabilities remain unknown.
    SpendabilityUnknown,
    /// The full request cannot preserve the configured reserve.
    ReserveWouldBeViolated {
        /// Exact full request, never a reduced transfer amount.
        requested_cycles: u128,
        /// Amount above reserve, for diagnosis only.
        transferable_cycles: u128,
    },
}

/// Bound passive assessment. A fresh query never authorizes a later effect.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingPreviewView {
    /// Exact request assessed, including amount and operation identity.
    pub request: FundingPreviewRequest,
    /// Known spendable cycles; the fixture has no authoritative observation.
    pub available_cycles: Option<u128>,
    /// Local attachment accounting, distinct from authoritative spendable cycles.
    pub budget: super::budget::FundingBudgetView,
    /// Local platform observation; recomputed after intent persistence on update.
    pub liquidity: FundingLiquidityView,
    /// All independent blockers, including provider qualification.
    pub blockers: Vec<FundingPreviewBlocker>,
}

/// Query-time platform figures for the controlled receive call, not Caffeine credit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingLiquidityView {
    /// Current liquid balance; it can change independently of the budget revision.
    pub liquid_cycles: u128,
    /// Cost bound using the largest valid acceptance encoding for this offer.
    /// Includes platform response/callback reservations, excludes the attachment.
    pub call_cost: u128,
}
