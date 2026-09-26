use super::*;
use crate::policy::billing::admission::{
    FundingAdmissionDecision, FundingAdmissionObservation, assess_funding_admission,
};

#[test]
fn unknowns_do_not_become_zero_or_clear_independent_uncertainty() {
    let limits = FundingLimits::new(1, 1, 1).unwrap();
    let assessment = assess_funding_evidence(
        Some(limits),
        NonZeroU128::MAX,
        FundingAdmissionEvidence {
            available_cycles: None,
            recovery: None,
            activity: Some(FundingActivity::Uncertain),
        },
    );
    assert!(!assessment.can_prepare_intent());
    assert_eq!(assessment.requested_cycles(), NonZeroU128::MAX);
    assert_eq!(
        assessment.blockers(),
        &[
            FundingEvidenceBlocker::RecoveryUnknown,
            FundingEvidenceBlocker::Admission(FundingAdmissionBlocker::FundingUncertain),
            FundingEvidenceBlocker::SpendabilityUnknown,
        ]
    );
    let absent = assess_funding_evidence(
        None,
        NonZeroU128::MIN,
        FundingAdmissionEvidence {
            available_cycles: None,
            recovery: Some(RecoveryState::Fenced),
            activity: None,
        },
    );
    assert_eq!(
        absent.blockers(),
        &[
            FundingEvidenceBlocker::Admission(FundingAdmissionBlocker::RecoveryFenced),
            FundingEvidenceBlocker::FundingUnknown,
            FundingEvidenceBlocker::Admission(FundingAdmissionBlocker::NotConfigured),
            FundingEvidenceBlocker::SpendabilityUnknown,
        ]
    );
}

#[test]
fn complete_evidence_retains_existing_admission_and_full_request_reserve_behavior() {
    for limits in [None, Some(FundingLimits::new(500, 1, 2).unwrap())] {
        for recovery in [RecoveryState::Reconciled, RecoveryState::Fenced] {
            for activity in [
                FundingActivity::Clear,
                FundingActivity::InProgress,
                FundingActivity::Uncertain,
            ] {
                for available_cycles in [0, 500, 1000, u128::MAX] {
                    for requested_cycles in [
                        NonZeroU128::MIN,
                        NonZeroU128::new(501).unwrap(),
                        NonZeroU128::MAX,
                    ] {
                        let complete = assess_funding_admission(
                            limits,
                            requested_cycles,
                            FundingAdmissionObservation {
                                available_cycles,
                                recovery,
                                activity,
                            },
                        );
                        let partial = assess_funding_evidence(
                            limits,
                            requested_cycles,
                            FundingAdmissionEvidence {
                                available_cycles: Some(available_cycles),
                                recovery: Some(recovery),
                                activity: Some(activity),
                            },
                        );
                        assert_eq!(
                            partial.can_prepare_intent(),
                            matches!(complete, FundingAdmissionDecision::PrepareIntent { .. })
                        );
                        if let FundingAdmissionDecision::Blocked(blocker) = complete {
                            assert_eq!(
                                partial.blockers().first(),
                                Some(&FundingEvidenceBlocker::Admission(blocker))
                            );
                        }
                        assert_eq!(partial.requested_cycles(), requested_cycles);
                    }
                }
            }
        }
    }
}
