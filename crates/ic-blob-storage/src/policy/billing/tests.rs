use super::*;

fn amount(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("positive test amount")
}

fn observation(balance: BalanceObservation) -> BillingObservation {
    BillingObservation {
        gateway_count: 1,
        balance,
        available_cycles: 1_000,
        recovery: RecoveryState::Reconciled,
    }
}

#[test]
fn funding_requires_full_amount_and_preserves_exact_reserve() {
    let limits = FundingLimits::new(500, 10, 100).expect("valid limits");
    assert_eq!(
        assess_funding(limits, amount(500), 1_000),
        FundingDecision::FitsReserve {
            requested_cycles: amount(500)
        }
    );
    assert_eq!(
        assess_funding(limits, amount(501), 1_000),
        FundingDecision::ReserveWouldBeViolated {
            requested_cycles: amount(501),
            transferable_cycles: 500
        }
    );
    for available in [0, 499, 500] {
        assert_eq!(
            assess_funding(limits, amount(1), available),
            FundingDecision::ReserveWouldBeViolated {
                requested_cycles: amount(1),
                transferable_cycles: 0
            }
        );
    }
}

#[test]
fn funding_conserves_cycles_at_small_and_extreme_boundaries() {
    let boundaries = [0, 1, 2, 9, 10, 99, 100, 101, u128::MAX - 1, u128::MAX];
    for reserve in boundaries.into_iter().filter(|value| *value > 0) {
        let limits = FundingLimits::new(reserve, 1, 1).expect("valid limits");
        for available in boundaries {
            for requested in boundaries.into_iter().filter_map(NonZeroU128::new) {
                match assess_funding(limits, requested, available) {
                    FundingDecision::FitsReserve { requested_cycles } => {
                        assert_eq!(requested_cycles, requested);
                        let remaining = available
                            .checked_sub(requested.get())
                            .expect("no overspend");
                        assert!(remaining >= reserve);
                        assert_eq!(remaining.checked_add(requested.get()), Some(available));
                    }
                    FundingDecision::ReserveWouldBeViolated {
                        requested_cycles,
                        transferable_cycles,
                    } => {
                        assert_eq!(requested_cycles, requested);
                        assert!(
                            available
                                .checked_sub(requested.get())
                                .is_none_or(|remaining| remaining < reserve)
                        );
                        assert!(transferable_cycles < requested.get());
                        assert!(transferable_cycles <= available);
                    }
                }
            }
        }
    }
}

#[test]
fn readiness_uses_minimum_threshold_then_full_target_top_up() {
    let limits = FundingLimits::new(100, 10, 100).expect("valid limits");
    for balance in [10, 11, 99, 100, u128::MAX] {
        let result = assess_readiness(
            Some(limits),
            observation(BalanceObservation::Available(balance)),
        );
        assert!(result.is_ready());
        assert_eq!(result.funding(), FundingStatus::NotNeeded);
        assert_eq!(result.blockers(), []);
        assert_eq!(result.warnings(), []);
    }
    for (balance, requested) in [(0, 100), (1, 99), (9, 91)] {
        let result = assess_readiness(
            Some(limits),
            observation(BalanceObservation::Available(balance)),
        );
        assert!(!result.is_ready());
        assert_eq!(
            result.funding(),
            FundingStatus::TopUp(FundingDecision::FitsReserve {
                requested_cycles: amount(requested)
            })
        );
        assert_eq!(result.blockers(), &[BillingBlocker::InsufficientBalance]);
    }
}

#[test]
fn readiness_preserves_all_simultaneous_blockers() {
    let limits = FundingLimits::new(950, 10, 100).expect("valid limits");
    let input = BillingObservation {
        gateway_count: 0,
        recovery: RecoveryState::Fenced,
        ..observation(BalanceObservation::Available(9))
    };
    let result = assess_readiness(Some(limits), input);
    assert!(!result.is_ready());
    assert_eq!(
        result.blockers(),
        &[
            BillingBlocker::RecoveryFenced,
            BillingBlocker::GatewayPrincipalsMissing,
            BillingBlocker::InsufficientBalance,
            BillingBlocker::ReserveWouldBeViolated,
        ]
    );
    assert_eq!(
        result.warnings(),
        &[BillingWarning::GatewayPrincipalSetEmpty]
    );
    assert_eq!(
        result.funding(),
        FundingStatus::TopUp(FundingDecision::ReserveWouldBeViolated {
            requested_cycles: amount(91),
            transferable_cycles: 50
        })
    );
    assert_eq!(assess_readiness(Some(limits), input), result);
}

#[test]
fn observation_failures_never_become_zero_balance_top_ups() {
    let limits = FundingLimits::new(1, 10, 100).expect("valid limits");
    for (balance, funding, blocker, warning) in [
        (
            BalanceObservation::Unavailable,
            FundingStatus::BalanceUnavailable,
            BillingBlocker::BalanceUnavailable,
            BillingWarning::BalanceUnavailable,
        ),
        (
            BalanceObservation::Malformed,
            FundingStatus::BalanceMalformed,
            BillingBlocker::BalanceMalformed,
            BillingWarning::BalanceMalformed,
        ),
    ] {
        let result = assess_readiness(Some(limits), observation(balance));
        assert!(!result.is_ready());
        assert_eq!(result.funding(), funding);
        assert_eq!(result.blockers(), &[blocker]);
        assert_eq!(result.warnings(), &[warning]);
    }
    let recovered = assess_readiness(Some(limits), observation(BalanceObservation::Available(10)));
    assert!(recovered.is_ready());
}

#[test]
fn missing_configuration_and_recovery_fences_fail_closed() {
    let limits = FundingLimits::new(1, 10, 100).expect("valid limits");
    let input = observation(BalanceObservation::Available(u128::MAX));
    let missing = assess_readiness(None, input);
    assert!(!missing.is_ready());
    assert_eq!(missing.funding(), FundingStatus::NotConfigured);
    assert_eq!(missing.blockers(), &[BillingBlocker::NotConfigured]);
    let fenced_input = BillingObservation {
        recovery: RecoveryState::Fenced,
        ..input
    };
    let fenced = assess_readiness(Some(limits), fenced_input);
    assert!(!fenced.is_ready());
    assert_eq!(fenced.funding(), FundingStatus::NotNeeded);
    assert_eq!(fenced.blockers(), &[BillingBlocker::RecoveryFenced]);
    assert_eq!(
        assess_readiness(None, fenced_input).blockers(),
        &[
            BillingBlocker::RecoveryFenced,
            BillingBlocker::NotConfigured
        ]
    );
}

#[test]
fn maximum_target_and_equal_thresholds_do_not_overflow() {
    let limits = FundingLimits::new(1, u128::MAX, u128::MAX).expect("valid limits");
    for (balance, expected) in [
        (
            0,
            FundingDecision::ReserveWouldBeViolated {
                requested_cycles: amount(u128::MAX),
                transferable_cycles: u128::MAX - 1,
            },
        ),
        (
            1,
            FundingDecision::FitsReserve {
                requested_cycles: amount(u128::MAX - 1),
            },
        ),
        (
            u128::MAX - 1,
            FundingDecision::FitsReserve {
                requested_cycles: amount(1),
            },
        ),
    ] {
        let result = assess_readiness(
            Some(limits),
            BillingObservation {
                available_cycles: u128::MAX,
                ..observation(BalanceObservation::Available(balance))
            },
        );
        assert!(!result.is_ready());
        assert_eq!(result.funding(), FundingStatus::TopUp(expected));
    }
}
