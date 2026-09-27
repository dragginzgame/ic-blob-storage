use super::*;
use crate::model::billing::{allocation::FundingAllocation, transfer::FundingTransfer};
use candid::Principal;
use std::num::NonZeroUsize;

fn n(v: u128) -> NonZeroU128 {
    NonZeroU128::new(v).unwrap()
}
fn evidence() -> FundingAttemptEvidence {
    FundingAttemptEvidence {
        intent: FundingIntent {
            service: Principal::from_slice(&[1]),
            cashier: Principal::from_slice(&[2]),
            account: Principal::from_slice(&[3]),
            namespace: n(1),
            operation: n(2),
            offered: n(900),
            target_balance: None,
        },
        provider_qualified: true,
        available_for_offer: Some(1000),
        recovery: Some(RecoveryState::Reconciled),
        other_activity: Some(FundingActivity::Clear),
    }
}
fn local() -> FundingAttemptObservation {
    FundingAttemptObservation {
        fenced: false,
        allocation: FundingAllocation::new(1000, n(100), NonZeroUsize::new(2).unwrap())
            .unwrap()
            .reconstruct(&[
                FundingTransfer::not_enqueued(n(900)),
                FundingTransfer::unknown(n(900)),
            ])
            .unwrap(),
        last_operation: Some(n(2)),
        state: Some(FundingIntentState::Prepared),
    }
}
fn assess(
    local: FundingAttemptObservation,
    evidence: FundingAttemptEvidence,
) -> FundingAttemptAssessment {
    assess_first_attempt(FundingLimits::new(100, 10, 100).unwrap(), local, evidence)
}
#[test]
fn exact_reservation_is_not_charged_twice_and_exclusion_requires_both_identity_and_amount() {
    use FundingAttemptBlocker as B;
    let local = local();
    assert_eq!(local.allocation.transferable(), 0);
    assert!(assess(local, evidence()).can_mark_attempt());
    for intent in [
        FundingIntent {
            operation: n(1),
            ..evidence().intent
        },
        FundingIntent {
            offered: n(899),
            ..evidence().intent
        },
    ] {
        assert_eq!(
            assess(
                local,
                FundingAttemptEvidence {
                    intent,
                    ..evidence()
                }
            )
            .blockers(),
            &[B::ReservationMismatch]
        );
    }
    let scarce = assess(
        local,
        FundingAttemptEvidence {
            available_for_offer: Some(999),
            ..evidence()
        },
    );
    assert!(matches!(
        scarce.blockers(),
        [B::Evidence(FundingEvidenceBlocker::Admission(
            super::super::FundingAdmissionBlocker::ReserveWouldBeViolated {
                transferable_cycles: 899,
                ..
            }
        ))]
    ));
}
#[test]
fn missing_uncertain_and_terminal_states_never_become_first_attempts() {
    use FundingAttemptBlocker as B;
    for (state, blocker) in [
        (None, B::IntentMissing),
        (Some(FundingIntentState::Uncertain), B::AlreadyAttempted),
        (Some(FundingIntentState::NotEnqueued), B::AlreadyAttempted),
        (
            Some(FundingIntentState::Callback { refunded: 900 }),
            B::AlreadyAttempted,
        ),
    ] {
        let result = assess(
            FundingAttemptObservation {
                state,
                fenced: true,
                ..local()
            },
            FundingAttemptEvidence {
                provider_qualified: false,
                available_for_offer: None,
                recovery: None,
                other_activity: None,
                ..evidence()
            },
        );
        for expected in [
            blocker,
            B::JournalFenced,
            B::ProviderUnqualified,
            B::Evidence(FundingEvidenceBlocker::SpendabilityUnknown),
            B::Evidence(FundingEvidenceBlocker::RecoveryUnknown),
            B::Evidence(FundingEvidenceBlocker::FundingUnknown),
        ] {
            assert!(result.blockers().contains(&expected));
        }
    }
}
