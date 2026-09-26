//! Admission diagnosis with incomplete evidence; unknown accounting is never zero.
use super::{FundingActivity, FundingAdmissionBlocker};
use crate::{
    model::billing::FundingLimits,
    policy::billing::{FundingDecision, RecoveryState, assess_funding},
};
use std::num::NonZeroU128;

/// Independently established observations for one current service/provider account.
/// Missing facts cannot be supplied by a reported provider balance or gross cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingAdmissionEvidence {
    /// Available cycles after all reservations and liabilities, if known.
    pub available_cycles: Option<u128>,
    /// Independently established identity/accounting recovery state, if known.
    pub recovery: Option<RecoveryState>,
    /// Complete outstanding funding activity for the same account, if known.
    pub activity: Option<FundingActivity>,
}

/// An independent reason a new intent cannot be prepared from the supplied facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingEvidenceBlocker {
    /// Safe identity allocation and accounting have not been established.
    RecoveryUnknown,
    /// Available cycles after reservations/liabilities have not been established.
    SpendabilityUnknown,
    /// Complete outstanding funding activity has not been established.
    FundingUnknown,
    /// A known fact blocks admission under the maintained funding policy.
    Admission(FundingAdmissionBlocker),
}

/// Pure diagnosis; even an empty blocker set is not an effect or retry permit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FundingAdmissionAssessment {
    requested_cycles: NonZeroU128,
    blockers: Vec<FundingEvidenceBlocker>,
}

impl FundingAdmissionAssessment {
    /// Full requested amount; never reduced to fit a reserve.
    #[must_use]
    pub const fn requested_cycles(&self) -> NonZeroU128 {
        self.requested_cycles
    }

    /// All independent blockers, preserving unknowns alongside known unsafe facts.
    #[must_use]
    pub fn blockers(&self) -> &[FundingEvidenceBlocker] {
        &self.blockers
    }

    /// Whether the supplied observations satisfy admission policy.
    /// Authorization, exact identity, atomic reservation, persisted intent and
    /// provider binding remain the workflow's responsibility.
    #[must_use]
    pub fn can_prepare_intent(&self) -> bool {
        self.blockers.is_empty()
    }
}

/// Diagnose incomplete admission evidence without fabricating missing inputs.
/// Reserve arithmetic runs only with validated limits and known spendability.
/// Recovery and funding uncertainty remain independent blockers even when that
/// arithmetic fits. Missing/expired evidence never authorizes repeating a payment.
#[must_use]
pub fn assess_funding_evidence(
    limits: Option<FundingLimits>,
    requested_cycles: NonZeroU128,
    evidence: FundingAdmissionEvidence,
) -> FundingAdmissionAssessment {
    use FundingEvidenceBlocker::{Admission, FundingUnknown, RecoveryUnknown, SpendabilityUnknown};
    let mut blockers = Vec::new();
    match evidence.recovery {
        None => blockers.push(RecoveryUnknown),
        Some(RecoveryState::Fenced) => {
            blockers.push(Admission(FundingAdmissionBlocker::RecoveryFenced));
        }
        Some(RecoveryState::Reconciled) => {}
    }
    match evidence.activity {
        None => blockers.push(FundingUnknown),
        Some(FundingActivity::InProgress) => {
            blockers.push(Admission(FundingAdmissionBlocker::FundingInProgress));
        }
        Some(FundingActivity::Uncertain) => {
            blockers.push(Admission(FundingAdmissionBlocker::FundingUncertain));
        }
        Some(FundingActivity::Clear) => {}
    }
    if limits.is_none() {
        blockers.push(Admission(FundingAdmissionBlocker::NotConfigured));
    }
    if evidence.available_cycles.is_none() {
        blockers.push(SpendabilityUnknown);
    }
    if let (Some(limits), Some(available)) = (limits, evidence.available_cycles)
        && let FundingDecision::ReserveWouldBeViolated {
            requested_cycles,
            transferable_cycles,
        } = assess_funding(limits, requested_cycles, available)
    {
        blockers.push(Admission(FundingAdmissionBlocker::ReserveWouldBeViolated {
            requested_cycles,
            transferable_cycles,
        }));
    }
    FundingAdmissionAssessment {
        requested_cycles,
        blockers,
    }
}

#[cfg(test)]
mod tests;
