use super::*;
use crate::{
    model::billing::FundingLimits,
    policy::billing::{BalanceObservation, BillingObservation, assess_readiness},
};

fn input() -> OperatorObservation<'static> {
    OperatorObservation {
        recovery: Some(RecoveryState::Reconciled),
        provider_qualified: true,
        gateway_count: 1,
        billing: BillingAssessment::Unavailable,
        funding: None,
        sync_pending: false,
        read_pending: false,
        uncertain_uploads: 0,
        pending_deletions: 0,
        pending_billing: 0,
    }
}

#[test]
fn unknowns_never_imply_zero_balance_or_clear_funding() {
    let unknown = assess_operator(OperatorObservation {
        recovery: None,
        ..input()
    });
    assert!(
        unknown
            .blockers()
            .contains(&OperatorBlocker::RecoveryUnknown)
    );
    assert!(
        !unknown
            .blockers()
            .contains(&OperatorBlocker::RecoveryFenced)
    );
    let report = assess_operator(input());
    assert_eq!(
        report.blockers(),
        &[
            OperatorBlocker::FundingUnknown,
            OperatorBlocker::BillingUnavailable
        ]
    );
    for (activity, blocker) in [
        (
            FundingActivity::InProgress,
            OperatorBlocker::FundingInProgress,
        ),
        (
            FundingActivity::Uncertain,
            OperatorBlocker::FundingUncertain,
        ),
    ] {
        let report = assess_operator(OperatorObservation {
            funding: Some(activity),
            recovery: Some(RecoveryState::Fenced),
            provider_qualified: false,
            gateway_count: 0,
            billing: BillingAssessment::NotConfigured,
            ..input()
        });
        assert_eq!(
            report.blockers(),
            &[
                OperatorBlocker::RecoveryFenced,
                OperatorBlocker::ProviderUnqualified,
                OperatorBlocker::GatewaysMissing,
                blocker,
                OperatorBlocker::BillingNotConfigured
            ]
        );
    }
}

#[test]
fn shared_billing_retains_malformed_unavailable_and_reserve_failures() {
    let limits = FundingLimits::new(950, 10, 100).unwrap();
    for (balance, blocker) in [
        (
            BalanceObservation::Unavailable,
            BillingBlocker::BalanceUnavailable,
        ),
        (
            BalanceObservation::Malformed,
            BillingBlocker::BalanceMalformed,
        ),
        (
            BalanceObservation::Available(0),
            BillingBlocker::ReserveWouldBeViolated,
        ),
    ] {
        let billing = assess_readiness(
            Some(limits),
            BillingObservation {
                gateway_count: 0,
                balance,
                available_cycles: 1000,
                recovery: RecoveryState::Fenced,
            },
        );
        let before = billing.clone();
        let report = assess_operator(OperatorObservation {
            billing: BillingAssessment::Observed(&billing),
            funding: Some(FundingActivity::Clear),
            ..input()
        });
        assert!(
            report
                .blockers()
                .contains(&OperatorBlocker::Billing(blocker))
        );
        assert!(report.blockers().contains(&OperatorBlocker::RecoveryFenced));
        assert!(
            report
                .blockers()
                .contains(&OperatorBlocker::GatewaysMissing)
        );
        assert_eq!(billing, before);
        let duplicate = assess_operator(OperatorObservation {
            billing: BillingAssessment::Observed(&billing),
            recovery: Some(RecoveryState::Fenced),
            gateway_count: 0,
            ..input()
        });
        assert_eq!(
            duplicate
                .blockers()
                .iter()
                .filter(|b| **b == OperatorBlocker::RecoveryFenced)
                .count(),
            1
        );
        assert_eq!(
            duplicate
                .blockers()
                .iter()
                .filter(|b| **b == OperatorBlocker::GatewaysMissing)
                .count(),
            1
        );
    }
}

#[test]
fn normal_work_is_visible_without_claiming_a_global_shutdown() {
    let billing = assess_readiness(
        Some(FundingLimits::new(1, 10, 100).unwrap()),
        BillingObservation {
            gateway_count: 1,
            balance: BalanceObservation::Available(10),
            available_cycles: 1000,
            recovery: RecoveryState::Reconciled,
        },
    );
    let report = assess_operator(OperatorObservation {
        billing: BillingAssessment::Observed(&billing),
        funding: Some(FundingActivity::Clear),
        sync_pending: true,
        read_pending: true,
        uncertain_uploads: usize::MAX,
        pending_deletions: usize::MAX,
        pending_billing: usize::MAX,
        ..input()
    });
    assert_eq!(report.blockers(), []);
    assert_eq!(
        report.warnings(),
        &[
            OperatorWarning::SyncPending,
            OperatorWarning::ReadPending,
            OperatorWarning::UploadCompletionUnknown,
            OperatorWarning::DeletionPending,
            OperatorWarning::BillingCessationPending
        ]
    );
}

#[test]
fn healthy_billing_cannot_hide_recovery_or_uncertain_funding() {
    let billing = assess_readiness(
        Some(FundingLimits::new(1, 10, 100).unwrap()),
        BillingObservation {
            gateway_count: 1,
            balance: BalanceObservation::Available(u128::MAX),
            available_cycles: u128::MAX,
            recovery: RecoveryState::Reconciled,
        },
    );
    let input = OperatorObservation {
        billing: BillingAssessment::Observed(&billing),
        recovery: Some(RecoveryState::Fenced),
        funding: Some(FundingActivity::Uncertain),
        ..input()
    };
    let report = assess_operator(input);
    assert_eq!(
        report.blockers(),
        &[
            OperatorBlocker::RecoveryFenced,
            OperatorBlocker::FundingUncertain
        ]
    );
    assert_eq!(assess_operator(input), report);
}

#[test]
fn observed_balance_never_substitutes_for_unknown_spendability_or_recovery() {
    use crate::policy::billing::balance::{BalanceContext, assess_balance};
    let limits = FundingLimits::new(u128::MAX, 100, 200).unwrap();
    for balance in [
        BalanceObservation::Available(0),
        BalanceObservation::Available(100),
        BalanceObservation::Malformed,
        BalanceObservation::Unavailable,
    ] {
        let billing = assess_balance(
            Some(limits),
            BalanceContext {
                gateway_count: 0,
                balance,
                recovery: RecoveryState::Fenced,
            },
        );
        let report = assess_operator(OperatorObservation {
            recovery: None,
            billing: BillingAssessment::BalanceOnly(&billing),
            funding: Some(FundingActivity::Uncertain),
            ..input()
        });
        for expected in [
            OperatorBlocker::SpendabilityUnknown,
            OperatorBlocker::RecoveryUnknown,
            OperatorBlocker::RecoveryFenced,
            OperatorBlocker::FundingUncertain,
            OperatorBlocker::GatewaysMissing,
        ] {
            assert!(report.blockers().contains(&expected));
        }
        assert!(!report.blockers().contains(&OperatorBlocker::Billing(
            BillingBlocker::ReserveWouldBeViolated
        )));
    }
}
