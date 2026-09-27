//! Host access and passive boundary conversion for shared preparation workflow.
use super::{ProbeMemory, STATE};
use blob_test_protocol::storage::funding::admission::Blocker;
use ic_blob_storage::{
    model::billing::journal::{FundingIntent, FundingJournalScope},
    ops::service::funding::StableFundingJournal,
    policy::billing::admission::{
        FundingAdmissionBlocker,
        attempt::{FundingAttemptBlocker, FundingAttemptEvidence},
        evidence::{FundingAdmissionEvidence, FundingEvidenceBlocker},
        journal::{FundingHostEvidence, FundingPreparationBlocker},
    },
};
pub(crate) fn with_journal<R>(f: impl FnOnce(&StableFundingJournal<ProbeMemory>) -> R) -> R {
    STATE.with_borrow(|state| f(&state.as_ref().unwrap().funding))
}
pub(crate) fn with_journal_mut<R>(
    f: impl FnOnce(&mut StableFundingJournal<ProbeMemory>) -> R,
) -> R {
    STATE.with_borrow_mut(|state| f(&mut state.as_mut().unwrap().funding))
}
pub(crate) const fn unknown_evidence(scope: FundingJournalScope) -> FundingHostEvidence {
    // Fixed fixture facts. Ingress carries only the intent, never these assertions.
    FundingHostEvidence {
        scope,
        provider_qualified: false,
        admission: FundingAdmissionEvidence {
            available_cycles: None,
            recovery: None,
            activity: None,
        },
    }
}
pub(crate) fn blockers(values: &[FundingPreparationBlocker]) -> Vec<Blocker> {
    values
        .iter()
        .copied()
        .map(|value| match value {
            FundingPreparationBlocker::ProviderUnqualified => Blocker::ProviderUnqualified,
            FundingPreparationBlocker::JournalFenced => Blocker::JournalFenced,
            FundingPreparationBlocker::JournalUncredited => Blocker::JournalUncredited,
            FundingPreparationBlocker::IdentityRetained => Blocker::IdentityRetained,
            FundingPreparationBlocker::IdentityStale => Blocker::IdentityStale,
            FundingPreparationBlocker::JournalFull => Blocker::JournalFull,
            FundingPreparationBlocker::AllocationReserve {
                transferable_cycles,
            } => Blocker::AllocationReserve(transferable_cycles),
            FundingPreparationBlocker::Evidence(value) => evidence_blocker(value),
        })
        .collect()
}
fn evidence_blocker(value: FundingEvidenceBlocker) -> Blocker {
    match value {
        FundingEvidenceBlocker::RecoveryUnknown => Blocker::RecoveryUnknown,
        FundingEvidenceBlocker::SpendabilityUnknown => Blocker::SpendabilityUnknown,
        FundingEvidenceBlocker::FundingUnknown => Blocker::AccountActivityUnknown,
        FundingEvidenceBlocker::Admission(value) => match value {
            FundingAdmissionBlocker::RecoveryFenced => Blocker::HostRecoveryFenced,
            FundingAdmissionBlocker::FundingInProgress => Blocker::AccountActivityInProgress,
            FundingAdmissionBlocker::FundingUncertain => Blocker::AccountActivityUncertain,
            FundingAdmissionBlocker::NotConfigured => Blocker::NotConfigured,
            FundingAdmissionBlocker::ReserveWouldBeViolated {
                requested_cycles,
                transferable_cycles,
            } => Blocker::SpendableReserve {
                requested: requested_cycles.get(),
                transferable: transferable_cycles,
            },
        },
    }
}
pub(crate) const fn unknown_attempt_evidence(intent: FundingIntent) -> FundingAttemptEvidence {
    FundingAttemptEvidence {
        intent,
        provider_qualified: false,
        available_for_offer: None,
        recovery: None,
        other_activity: None,
    }
}
pub(crate) fn attempt_blockers(values: &[FundingAttemptBlocker]) -> Vec<Blocker> {
    values
        .iter()
        .copied()
        .map(|value| match value {
            FundingAttemptBlocker::ProviderUnqualified => Blocker::ProviderUnqualified,
            FundingAttemptBlocker::JournalFenced => Blocker::JournalFenced,
            FundingAttemptBlocker::JournalUncredited => Blocker::JournalUncredited,
            FundingAttemptBlocker::IntentMissing => Blocker::IntentMissing,
            FundingAttemptBlocker::AlreadyAttempted => Blocker::AlreadyAttempted,
            FundingAttemptBlocker::ReservationMismatch => Blocker::ReservationMismatch,
            FundingAttemptBlocker::Evidence(value) => evidence_blocker(value),
        })
        .collect()
}
