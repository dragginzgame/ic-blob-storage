//! Shared threshold diagnosis with real status/CLI queries and unknown spendability.
use super::*;
use blob_test_protocol::{
    billing::{BillingLimitsInput, FundingNeedView},
    status::BillingBlockerView,
};

impl Fixture {
    fn limits(&self, minimum: u128, target: u128) -> BillingLimitsInput {
        BillingLimitsInput {
            scope: self.balance_scope(),
            revision: self.balance_status().balance_observation.revision,
            reserve: u128::MAX,
            minimum,
            target,
        }
    }
    fn configure_limits_as(
        &self,
        actor: Principal,
        input: BillingLimitsInput,
    ) -> Result<(), BalanceFailure> {
        self.harness
            .pic
            .update_candid_as(self.authority, actor, "configure_billing_limits", (input,))
            .unwrap()
    }
    fn configure_limits(&self, minimum: u128, target: u128) {
        assert_eq!(
            self.configure_limits_as(self.driver, self.limits(minimum, target)),
            Ok(())
        );
    }
}

#[test]
fn threshold_shortfall_and_sufficient_balance_both_keep_spendability_unknown() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("success"), false, false);
    f.refresh_balance().unwrap();
    f.configure_limits(150, 200);
    let status = f.balance_status();
    assert!(status.billing_configured && status.billing.current);
    assert_eq!(status.billing.funding_need, FundingNeedView::TopUp(100));
    assert!(
        status
            .blockers
            .contains(&OperatorBlockerView::SpendabilityUnknown)
    );
    assert!(status.blockers.contains(&OperatorBlockerView::Billing(
        BillingBlockerView::InsufficientBalance
    )));
    assert!(!status.blockers.contains(&OperatorBlockerView::Billing(
        BillingBlockerView::ReserveWouldBeViolated
    )));
    assert_eq!(status.available_funding_cycles, None);
    let before = f.journals();
    let mut args = f.args(f.authority, f.driver, "authority", "--namespace", "1");
    args[0] = "check".into();
    let (code, json) = command(&args);
    assert_eq!(code, 4);
    assert_eq!(json["status"]["billing"]["funding_need"]["cycles"], "100");
    assert_eq!(
        json["status"]["billing"]["limits"]["reserve"],
        u128::MAX.to_string()
    );
    assert!(f.journals() == before, "diagnosis cannot refresh or fund");
    f.configure_limits(100, 200);
    let status = f.balance_status();
    assert_eq!(status.billing.funding_need, FundingNeedView::NotNeeded);
    for blocker in [
        OperatorBlockerView::SpendabilityUnknown,
        OperatorBlockerView::ProviderUnqualified,
        OperatorBlockerView::FundingUnknown,
    ] {
        assert!(status.blockers.contains(&blocker));
    }
    f.configure_limits(u128::MAX, u128::MAX);
    assert_eq!(
        f.balance_cli()["status"]["billing"]["funding_need"]["cycles"],
        (u128::MAX - 100).to_string()
    );
    assert_eq!(f.balance_source_status().requests, 1);
}

#[test]
fn malformed_unavailable_and_expired_reports_remain_distinct_under_configured_limits() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.configure_limits(10, 50);
    assert_eq!(
        f.balance_status().billing.funding_need,
        FundingNeedView::BalanceUnavailable
    );
    for (name, expected) in [
        ("negative-ledger", FundingNeedView::BalanceMalformed),
        ("not-found", FundingNeedView::BalanceUnavailable),
    ] {
        f.source_balance(bytes(name), false, false);
        assert!(f.refresh_balance().is_err());
        assert_eq!(f.balance_status().billing.funding_need, expected);
        assert!(
            f.balance_status()
                .blockers
                .contains(&OperatorBlockerView::SpendabilityUnknown)
        );
    }
    f.source_balance(bytes("success"), false, false);
    f.refresh_balance().unwrap();
    assert_eq!(
        f.balance_status().billing.funding_need,
        FundingNeedView::NotNeeded
    );
    f.harness.pic.advance_time(Duration::from_secs(31));
    f.harness.pic.tick();
    assert_eq!(
        f.balance_status().billing.funding_need,
        FundingNeedView::BalanceUnavailable
    );
    assert_eq!(
        f.balance_cli()["status"]["balance_observation"]["usability"],
        "Expired"
    );
}

#[test]
fn limits_do_not_follow_a_new_scope_revision_and_restore_cannot_clear_blockers() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.configure_limits(10, 50);
    let old_input = f.limits(10, 50);
    let id = f.hold_balance();
    f.configure_balance(f.balance_scope()).unwrap();
    let status = f.balance_status();
    assert!(!status.billing.current && !status.billing_configured);
    assert!(status.billing.limits.is_some());
    assert_eq!(status.billing.funding_need, FundingNeedView::NotConfigured);
    assert_eq!(
        f.configure_limits_as(f.driver, old_input),
        Err(BalanceFailure::Binding)
    );
    assert_eq!(f.resume_balance(id), Err(BalanceFailure::Stale));
    f.configure_limits(10, 50);
    f.source_balance(bytes("success"), false, false);
    f.refresh_balance().unwrap();
    f.upgrade_balance(f.authority, false);
    let status = f.balance_status();
    assert!(status.billing.current);
    assert_eq!(
        status.billing.funding_need,
        FundingNeedView::BalanceUnavailable
    );
    for blocker in [
        OperatorBlockerView::SpendabilityUnknown,
        OperatorBlockerView::RecoveryFenced,
    ] {
        assert!(status.blockers.contains(&blocker));
    }
    assert_eq!(
        f.configure_limits_as(f.driver, f.limits(10, 50)),
        Err(BalanceFailure::Fenced)
    );
    f.balance_cli();
}

#[test]
fn denied_invalid_and_misbound_limit_updates_leave_journals_unchanged() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    let input = f.limits(10, 50);
    let before = f.journals();
    assert_eq!(
        f.configure_limits_as(Principal::anonymous(), input),
        Err(BalanceFailure::Denied)
    );
    for invalid in [
        BillingLimitsInput {
            reserve: 0,
            ..input
        },
        BillingLimitsInput {
            minimum: 0,
            ..input
        },
        BillingLimitsInput {
            minimum: 51,
            ..input
        },
    ] {
        assert_eq!(
            f.configure_limits_as(f.driver, invalid),
            Err(BalanceFailure::InvalidLimits)
        );
    }
    let wrong = BillingLimitsInput {
        scope: BalanceScope {
            account: Fake::principal(50),
            ..input.scope
        },
        ..input
    };
    assert_eq!(
        f.configure_limits_as(f.driver, wrong),
        Err(BalanceFailure::Binding)
    );
    assert!(
        f.journals() == before,
        "invalid limits cannot alter retained state"
    );
    assert_eq!(f.balance_source_status().requests, 0);
}
