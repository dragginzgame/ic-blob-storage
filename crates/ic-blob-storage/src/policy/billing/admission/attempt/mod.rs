//! First-attempt checks for an exact existing reservation, never automatic retry.
use crate::{
    model::billing::{
        FundingLimits,
        allocation::FundingAllocationView,
        journal::{FundingIntent, FundingIntentState},
    },
    policy::billing::{
        RecoveryState,
        admission::{
            FundingActivity,
            evidence::{FundingAdmissionEvidence, FundingEvidenceBlocker, assess_funding_evidence},
        },
    },
};
use std::num::NonZeroU128;

/// Current trusted host observations for this exact first attempt. These plain
/// values do not authenticate themselves and must never be accepted from ingress.
/// The host must establish them in the execution that marks and dispatches the call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingAttemptEvidence {
    /// Exact exclusion from account activity and liability holds, including amount
    /// and target. Never exclude an operation merely because its amount matches.
    pub intent: FundingIntent,
    /// Independently qualified provider/account relationship.
    pub provider_qualified: bool,
    /// Funds available for this offer and operating reserve, after all OTHER
    /// liabilities. This exact offer remains included; never add it back to an
    /// arbitrary balance or count already transferred funds as available.
    pub available_for_offer: Option<u128>,
    /// Independently established identity/accounting recovery state.
    pub recovery: Option<RecoveryState>,
    /// Complete account-wide activity excluding only this exact unattempted intent.
    /// Local journal clearance cannot establish linked-payer/external completeness.
    pub other_activity: Option<FundingActivity>,
}
/// Authenticated local facts for one exact retained intent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingAttemptObservation {
    /// Actual local restore fence, independent of host claims.
    pub fenced: bool,
    /// Complete local attachment totals.
    pub allocation: FundingAllocationView,
    /// Highest retained identity in the sequential journal.
    pub last_operation: Option<NonZeroU128>,
    /// Exact retained state, or absence. Only Prepared permits a first attempt.
    pub state: Option<FundingIntentState>,
}
/// Independent reason to refuse a first-attempt marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingAttemptBlocker {
    /// Provider/account relationship is unqualified.
    ProviderUnqualified,
    /// Local restoration forbids all effects.
    JournalFenced,
    /// No exact reservation exists.
    IntentMissing,
    /// An uncertain or terminal attempt must never be sent again.
    AlreadyAttempted,
    /// The exact unattempted intent must own the entire final reservation.
    ReservationMismatch,
    /// Older accepted amounts still need independent credit evidence.
    JournalUncredited,
    /// Other account activity, recovery or spendability is missing or unsafe.
    Evidence(FundingEvidenceBlocker),
}
/// Passive current diagnosis, not a reusable dispatch permit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FundingAttemptAssessment {
    blockers: Vec<FundingAttemptBlocker>,
}
impl FundingAttemptAssessment {
    /// Independent local blockers and missing or unsafe host observations.
    #[must_use]
    pub fn blockers(&self) -> &[FundingAttemptBlocker] {
        &self.blockers
    }
    /// Supplied facts permit marking once; re-read before writing and assess
    /// actual platform liquidity after writes, before dispatch in the same message.
    #[must_use]
    pub fn can_mark_attempt(&self) -> bool {
        self.blockers.is_empty()
    }
}
/// Assess one existing reservation without charging it twice. History capacity
/// and remaining allocation apply when reserving, not again when consuming that
/// exact reservation. Unknown external activity remains unknown in every state.
#[must_use]
pub fn assess_first_attempt(
    limits: FundingLimits,
    local: FundingAttemptObservation,
    evidence: FundingAttemptEvidence,
) -> FundingAttemptAssessment {
    use FundingAttemptBlocker as B;
    let mut blockers = Vec::new();
    if !evidence.provider_qualified {
        blockers.push(B::ProviderUnqualified);
    }
    if local.fenced {
        blockers.push(B::JournalFenced);
    }
    match local.state {
        None => blockers.push(B::IntentMissing),
        Some(FundingIntentState::Prepared) => {
            let exact_last = local.last_operation == Some(evidence.intent.operation);
            let exact_hold =
                local.allocation.reserved_or_uncertain() == evidence.intent.offered.get();
            if !exact_last || !exact_hold {
                blockers.push(B::ReservationMismatch);
            }
        }
        Some(_) => blockers.push(B::AlreadyAttempted),
    }
    if local.allocation.uncredited() != 0 {
        blockers.push(B::JournalUncredited);
    }
    blockers.extend(
        assess_funding_evidence(
            Some(limits),
            evidence.intent.offered,
            FundingAdmissionEvidence {
                available_cycles: evidence.available_for_offer,
                recovery: evidence.recovery,
                activity: evidence.other_activity,
            },
        )
        .blockers()
        .iter()
        .copied()
        .map(B::Evidence),
    );
    FundingAttemptAssessment { blockers }
}
#[cfg(test)]
mod tests;
