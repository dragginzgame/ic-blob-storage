use super::*;

fn amount(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("positive test amount")
}

#[test]
fn recovery_fence_dominates_activity_configuration_and_cycle_balance() {
    let limits = FundingLimits::new(1, 10, 100).expect("valid limits");
    for limits in [None, Some(limits)] {
        for activity in [
            FundingActivity::Clear,
            FundingActivity::InProgress,
            FundingActivity::Uncertain,
        ] {
            for available_cycles in [0, u128::MAX] {
                assert_eq!(
                    assess_funding_admission(
                        limits,
                        amount(1),
                        FundingAdmissionObservation {
                            available_cycles,
                            recovery: RecoveryState::Fenced,
                            activity,
                        },
                    ),
                    FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::RecoveryFenced),
                );
            }
        }
    }
}

#[test]
fn outstanding_effects_block_new_intents_despite_changed_requests_or_configuration() {
    for (activity, blocker) in [
        (
            FundingActivity::InProgress,
            FundingAdmissionBlocker::FundingInProgress,
        ),
        (
            FundingActivity::Uncertain,
            FundingAdmissionBlocker::FundingUncertain,
        ),
    ] {
        for limits in [
            None,
            Some(FundingLimits::new(1, 1, 1).expect("valid limits")),
            Some(FundingLimits::new(500, 10, 100).expect("valid limits")),
        ] {
            for requested in [1, 500, u128::MAX] {
                let observation = FundingAdmissionObservation {
                    available_cycles: u128::MAX,
                    recovery: RecoveryState::Reconciled,
                    activity,
                };
                for _ in 0..2 {
                    assert_eq!(
                        assess_funding_admission(limits, amount(requested), observation),
                        FundingAdmissionDecision::Blocked(blocker),
                    );
                }
            }
        }
    }
}

#[test]
fn clear_reconciled_state_still_requires_configuration_and_the_full_reserve() {
    let limits = FundingLimits::new(500, 10, 100).expect("valid limits");
    let observation = FundingAdmissionObservation {
        available_cycles: 1_000,
        recovery: RecoveryState::Reconciled,
        activity: FundingActivity::Clear,
    };
    assert_eq!(
        assess_funding_admission(None, amount(500), observation),
        FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::NotConfigured),
    );
    assert_eq!(
        assess_funding_admission(Some(limits), amount(500), observation),
        FundingAdmissionDecision::PrepareIntent {
            requested_cycles: amount(500)
        },
    );
    for available_cycles in [0, 499, 500, 1_000] {
        assert_eq!(
            assess_funding_admission(
                Some(limits),
                amount(501),
                FundingAdmissionObservation {
                    available_cycles,
                    ..observation
                },
            ),
            FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::ReserveWouldBeViolated {
                requested_cycles: amount(501),
                transferable_cycles: available_cycles.saturating_sub(500),
            }),
        );
    }
    let limits = FundingLimits::new(1, 1, 1).expect("valid limits");
    assert_eq!(
        assess_funding_admission(
            Some(limits),
            amount(u128::MAX - 1),
            FundingAdmissionObservation {
                available_cycles: u128::MAX,
                ..observation
            },
        ),
        FundingAdmissionDecision::PrepareIntent {
            requested_cycles: amount(u128::MAX - 1)
        },
    );
}
