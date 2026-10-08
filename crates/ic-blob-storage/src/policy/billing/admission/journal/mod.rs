//! Combine local journal constraints with independently established host evidence.
use super::{
    FundingActivity,
    evidence::{FundingAdmissionEvidence, FundingEvidenceBlocker, assess_funding_evidence},
};
use crate::model::billing::allocation::FundingAllocationView;
use crate::policy::billing::reconciliation::assess_uncredited_allocation;
use ic_blob_storage_contracts::configuration::funding::FundingLimits;
use std::num::NonZeroU128;

/// Current facts established by the integrating host, never accepted from ingress
/// or inferred from balances. These values are not cryptographic evidence tokens.
/// No snapshot from an earlier query/await can be reused as effect authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingHostEvidence {
    /// Full installation/account scope to which every supplied observation applies.
    pub scope: crate::model::billing::journal::FundingJournalScope,
    /// Deployed provider and account relationship have been qualified by the host.
    pub provider_qualified: bool,
    /// Independent spendability, recovery and complete account-activity observations.
    pub admission: FundingAdmissionEvidence,
}

/// Current local facts for a new intent. No field establishes external account coverage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingJournalAdmissionObservation {
    /// Maintained complete local attachment totals.
    pub allocation: FundingAllocationView,
    /// Permanent restore fence, independent of claimed host recovery.
    pub fenced: bool,
    /// Retained lifetime history count.
    pub retained_intents: u64,
    /// Installed lifetime capacity.
    pub intent_capacity: u64,
    /// Highest retained identity; never independent freshness authority.
    pub last_operation: Option<NonZeroU128>,
    /// Whether the exact requested identity is already retained.
    pub already_retained: bool,
}
/// Independent local or host reason to refuse a new reservation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingPreparationBlocker {
    /// Host has not qualified the configured provider/account.
    ProviderUnqualified,
    /// The actual local journal remains permanently fenced.
    JournalFenced,
    /// Local accepted or reserved amounts remain uncredited/unresolved.
    JournalUncredited,
    /// This exact identity already exists; inspect it instead of preparing again.
    IdentityRetained,
    /// A new identity must sort after all retained history.
    IdentityStale,
    /// Lifetime history has no remaining slot.
    JournalFull,
    /// Full attachment exceeds local allocation after its reserve.
    AllocationReserve {
        /// Available attachment allowance, not a partial-transfer proposal.
        transferable_cycles: u128,
    },
    /// Missing or unsafe independently established account-wide evidence.
    Evidence(FundingEvidenceBlocker),
}
/// Passive assessment of one new intent; not dispatch, retry or recovery authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FundingPreparationAssessment {
    blockers: Vec<FundingPreparationBlocker>,
}
impl FundingPreparationAssessment {
    /// All independent blockers, including external unknowns alongside local facts.
    #[must_use]
    pub fn blockers(&self) -> &[FundingPreparationBlocker] {
        &self.blockers
    }
    /// Only the supplied observations satisfy preparation policy. Recompute against
    /// current bound facts inside a synchronous update before reserving an intent.
    #[must_use]
    pub fn can_prepare_intent(&self) -> bool {
        self.blockers.is_empty()
    }
}
/// Preserve independent obligations: external `Clear` cannot erase local liabilities,
/// and local `Clear` cannot fill missing account-wide activity. Host evidence must
/// be independently authenticated, current and scoped by the surrounding workflow.
#[must_use]
pub fn assess_journal_preparation(
    limits: Option<FundingLimits>,
    operation: NonZeroU128,
    requested_cycles: NonZeroU128,
    local: FundingJournalAdmissionObservation,
    provider_qualified: bool,
    evidence: FundingAdmissionEvidence,
) -> FundingPreparationAssessment {
    use FundingPreparationBlocker as B;
    let mut blockers = Vec::new();
    if !provider_qualified {
        blockers.push(B::ProviderUnqualified);
    }
    if local.fenced {
        blockers.push(B::JournalFenced);
    }
    if assess_uncredited_allocation(local.allocation) != FundingActivity::Clear {
        blockers.push(B::JournalUncredited);
    }
    if local.already_retained {
        blockers.push(B::IdentityRetained);
    } else if local.last_operation.is_some_and(|last| operation <= last) {
        blockers.push(B::IdentityStale);
    }
    if local.retained_intents >= local.intent_capacity {
        blockers.push(B::JournalFull);
    }
    if requested_cycles.get() > local.allocation.transferable() {
        blockers.push(B::AllocationReserve {
            transferable_cycles: local.allocation.transferable(),
        });
    }
    blockers.extend(
        assess_funding_evidence(limits, requested_cycles, evidence)
            .blockers()
            .iter()
            .copied()
            .map(B::Evidence),
    );
    FundingPreparationAssessment { blockers }
}
#[cfg(test)]
mod tests;
