use super::*;
use crate::policy::billing::{BillingObservation, FundingStatus, assess_readiness};

#[test]
fn threshold_shortfalls_preserve_extremes_and_do_not_assert_affordability() {
    let limits = FundingLimits::new(u128::MAX, 100, u128::MAX).unwrap();
    for (balance, expected) in [
        (0, Some(u128::MAX)),
        (99, Some(u128::MAX - 99)),
        (100, None),
        (u128::MAX, None),
    ] {
        let result = assess_balance(
            Some(limits),
            BalanceContext {
                gateway_count: 1,
                balance: BalanceObservation::Available(balance),
                recovery: RecoveryState::Fenced,
            },
        );
        assert!(result.blockers().contains(&BillingBlocker::RecoveryFenced));
        assert!(
            !result
                .blockers()
                .contains(&BillingBlocker::ReserveWouldBeViolated)
        );
        assert_eq!(
            result.funding_need(),
            expected.map_or(FundingNeed::NotNeeded, |n| FundingNeed::TopUp(
                NonZeroU128::new(n).unwrap()
            ))
        );
    }
}

#[test]
fn complete_reserve_diagnosis_keeps_the_same_balance_failures_and_warnings() {
    let limits = FundingLimits::new(100, 10, 50).unwrap();
    for configured in [None, Some(limits)] {
        for balance in [
            BalanceObservation::Malformed,
            BalanceObservation::Unavailable,
            BalanceObservation::Available(50),
        ] {
            let context = BalanceContext {
                gateway_count: 0,
                balance,
                recovery: RecoveryState::Fenced,
            };
            let partial = assess_balance(configured, context);
            let complete = assess_readiness(
                configured,
                BillingObservation {
                    gateway_count: context.gateway_count,
                    balance,
                    recovery: context.recovery,
                    available_cycles: 0,
                },
            );
            assert_eq!(partial.blockers(), complete.blockers());
            assert_eq!(partial.warnings(), complete.warnings());
            assert!(!matches!(complete.funding(), FundingStatus::TopUp(_)));
        }
    }
}
