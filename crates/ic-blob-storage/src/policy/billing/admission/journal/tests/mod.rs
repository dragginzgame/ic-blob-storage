use super::*;
use crate::model::billing::allocation::FundingAllocation;
use crate::policy::billing::RecoveryState;
use ic_blob_storage_contracts::funding::transfer::FundingTransfer;
use std::num::NonZeroUsize;
fn n(v: u128) -> NonZeroU128 {
    NonZeroU128::new(v).unwrap()
}
fn local(transfers: &[FundingTransfer]) -> FundingJournalAdmissionObservation {
    FundingJournalAdmissionObservation {
        allocation: FundingAllocation::new(1000, n(100), NonZeroUsize::new(4).unwrap())
            .unwrap()
            .reconstruct(transfers)
            .unwrap(),
        fenced: false,
        retained_intents: transfers.len() as u64,
        intent_capacity: 4,
        last_operation: NonZeroU128::new(transfers.len() as u128),
        already_retained: false,
    }
}
fn trusted() -> FundingAdmissionEvidence {
    FundingAdmissionEvidence {
        available_cycles: Some(1000),
        recovery: Some(RecoveryState::Reconciled),
        activity: Some(FundingActivity::Clear),
    }
}
fn assess(
    local: FundingJournalAdmissionObservation,
    qualified: bool,
    evidence: FundingAdmissionEvidence,
) -> FundingPreparationAssessment {
    assess_journal_preparation(
        Some(FundingLimits::new(100, 10, 100).unwrap()),
        n(9),
        n(100),
        local,
        qualified,
        evidence,
    )
}
#[test]
fn local_and_external_blockers_remain_independent_in_every_combination() {
    use crate::policy::billing::admission::FundingAdmissionBlocker;
    use FundingPreparationBlocker as B;
    for transfer in [
        FundingTransfer::unknown(n(300)),
        FundingTransfer::unbounded_callback(n(300), 100).unwrap(),
    ] {
        let local = local(&[transfer]);
        let known = assess(local, true, trusted());
        assert_eq!(known.blockers(), &[B::JournalUncredited]);
        let missing = assess(
            local,
            false,
            FundingAdmissionEvidence {
                available_cycles: None,
                recovery: None,
                activity: None,
            },
        );
        for blocker in [
            B::ProviderUnqualified,
            B::JournalUncredited,
            B::Evidence(FundingEvidenceBlocker::FundingUnknown),
            B::Evidence(FundingEvidenceBlocker::SpendabilityUnknown),
            B::Evidence(FundingEvidenceBlocker::RecoveryUnknown),
        ] {
            assert!(missing.blockers().contains(&blocker));
        }
    }
    let clear = local(&[]);
    assert!(assess(clear, true, trusted()).can_prepare_intent());
    assert_eq!(
        assess(
            clear,
            true,
            FundingAdmissionEvidence {
                activity: None,
                ..trusted()
            }
        )
        .blockers(),
        &[B::Evidence(FundingEvidenceBlocker::FundingUnknown)]
    );
    assert_eq!(
        assess(
            FundingJournalAdmissionObservation {
                fenced: true,
                ..clear
            },
            true,
            trusted()
        )
        .blockers(),
        &[B::JournalFenced]
    );
    assert_eq!(
        assess(
            clear,
            true,
            FundingAdmissionEvidence {
                recovery: Some(RecoveryState::Fenced),
                ..trusted()
            }
        )
        .blockers(),
        &[B::Evidence(FundingEvidenceBlocker::Admission(
            FundingAdmissionBlocker::RecoveryFenced
        ))]
    );
}
#[test]
fn identity_capacity_and_both_reserves_are_checked_without_reducing_the_offer() {
    use FundingPreparationBlocker as B;
    let local = FundingJournalAdmissionObservation {
        retained_intents: 4,
        last_operation: Some(n(10)),
        ..local(&[])
    };
    let assessment = assess_journal_preparation(
        Some(FundingLimits::new(100, 10, 100).unwrap()),
        n(9),
        NonZeroU128::MAX,
        local,
        true,
        FundingAdmissionEvidence {
            available_cycles: Some(150),
            ..trusted()
        },
    );
    assert!(assessment.blockers().contains(&B::IdentityStale));
    assert!(assessment.blockers().contains(&B::JournalFull));
    assert!(assessment.blockers().contains(&B::AllocationReserve {
        transferable_cycles: 900
    }));
    assert!(assessment.blockers().iter().any(|b| matches!(b, B::Evidence(FundingEvidenceBlocker::Admission(crate::policy::billing::admission::FundingAdmissionBlocker::ReserveWouldBeViolated { requested_cycles, transferable_cycles: 50 })) if *requested_cycles == NonZeroU128::MAX)));
    let retained = assess(
        FundingJournalAdmissionObservation {
            already_retained: true,
            ..local
        },
        true,
        trusted(),
    );
    assert!(retained.blockers().contains(&B::IdentityRetained));
    assert!(!retained.blockers().contains(&B::IdentityStale));
}
