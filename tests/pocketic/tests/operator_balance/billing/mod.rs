//! Shared threshold diagnosis with real status queries and unknown spendability.
use super::*;
use blob_test_protocol::{
    billing::{BillingLimitsInput, FundingNeedView},
    status::BillingBlockerView,
};
use ic_blob_storage_pocketic_tests::fixture_calls;

impl Fixture {
    async fn limits(&self, minimum: u128, target: u128) -> BillingLimitsInput {
        BillingLimitsInput {
            scope: self.balance_scope(),
            revision: self.balance_status().await.balance_observation.revision,
            reserve: u128::MAX,
            minimum,
            target,
        }
    }
    async fn configure_limits_as(
        &self,
        actor: Principal,
        input: BillingLimitsInput,
    ) -> Result<(), BalanceFailure> {
        fixture_calls::update_fixture(
            &self.harness.pic,
            self.authority,
            actor,
            "configure_billing_limits",
            (input,),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
    }
    async fn configure_limits(&self, minimum: u128, target: u128) {
        assert_eq!(
            self.configure_limits_as(self.driver, self.limits(minimum, target).await)
                .await,
            Ok(())
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn threshold_shortfall_and_sufficient_balance_both_keep_spendability_unknown() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.source_balance(bytes("success"), false, false).await;
    f.refresh_balance().await.unwrap();
    f.configure_limits(150, 200).await;
    let status = f.balance_status().await;
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
    let before = f.journals().await;
    let diagnosed = f.inspect_balance().await;
    assert_eq!(diagnosed.billing.funding_need, FundingNeedView::TopUp(100));
    assert_eq!(diagnosed.billing.limits.unwrap().reserve, u128::MAX);
    assert!(
        f.journals().await == before,
        "diagnosis cannot refresh or fund"
    );
    f.configure_limits(100, 200).await;
    let status = f.balance_status().await;
    assert_eq!(status.billing.funding_need, FundingNeedView::NotNeeded);
    for blocker in [
        OperatorBlockerView::SpendabilityUnknown,
        OperatorBlockerView::ProviderUnqualified,
        OperatorBlockerView::FundingUnknown,
    ] {
        assert!(status.blockers.contains(&blocker));
    }
    f.configure_limits(u128::MAX, u128::MAX).await;
    assert_eq!(
        f.inspect_balance().await.billing.funding_need,
        FundingNeedView::TopUp(u128::MAX - 100)
    );
    assert_eq!(f.balance_source_status().await.requests, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn malformed_unavailable_and_expired_reports_remain_distinct_under_configured_limits() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.configure_limits(10, 50).await;
    assert_eq!(
        f.balance_status().await.billing.funding_need,
        FundingNeedView::BalanceUnavailable
    );
    for (name, expected) in [
        ("negative-ledger", FundingNeedView::BalanceMalformed),
        ("not-found", FundingNeedView::BalanceUnavailable),
    ] {
        f.source_balance(bytes(name), false, false).await;
        assert!(f.refresh_balance().await.is_err());
        assert_eq!(f.balance_status().await.billing.funding_need, expected);
        assert!(
            f.balance_status()
                .await
                .blockers
                .contains(&OperatorBlockerView::SpendabilityUnknown)
        );
    }
    f.source_balance(bytes("success"), false, false).await;
    f.refresh_balance().await.unwrap();
    assert_eq!(
        f.balance_status().await.billing.funding_need,
        FundingNeedView::NotNeeded
    );
    f.harness
        .pic
        .advance_time(Duration::from_secs(31))
        .await
        .unwrap();
    f.harness.pic.tick().await.unwrap();
    assert_eq!(
        f.balance_status().await.billing.funding_need,
        FundingNeedView::BalanceUnavailable
    );
    assert_eq!(
        f.inspect_balance().await.balance_observation.usability,
        BalanceUsability::Expired
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn limits_do_not_follow_a_new_scope_revision_and_restore_cannot_clear_blockers() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.configure_limits(10, 50).await;
    let old_input = f.limits(10, 50).await;
    let id = f.hold_balance().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    let status = f.balance_status().await;
    assert!(!status.billing.current && !status.billing_configured);
    assert!(status.billing.limits.is_some());
    assert_eq!(status.billing.funding_need, FundingNeedView::NotConfigured);
    assert_eq!(
        f.configure_limits_as(f.driver, old_input).await,
        Err(BalanceFailure::Binding)
    );
    assert_eq!(f.resume_balance(id).await, Err(BalanceFailure::Stale));
    f.configure_limits(10, 50).await;
    f.source_balance(bytes("success"), false, false).await;
    f.refresh_balance().await.unwrap();
    f.upgrade_fixture(f.authority, false).await;
    let status = f.balance_status().await;
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
        f.configure_limits_as(f.driver, f.limits(10, 50).await)
            .await,
        Err(BalanceFailure::Fenced)
    );
    f.inspect_balance().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn denied_invalid_and_misbound_limit_updates_leave_journals_unchanged() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    let input = f.limits(10, 50).await;
    let before = f.journals().await;
    assert_eq!(
        f.configure_limits_as(Principal::anonymous(), input).await,
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
            f.configure_limits_as(f.driver, invalid).await,
            Err(BalanceFailure::InvalidLimits)
        );
    }
    let wrong = BillingLimitsInput {
        scope: BalanceScope {
            account: fake_principal(50),
            ..input.scope
        },
        ..input
    };
    assert_eq!(
        f.configure_limits_as(f.driver, wrong).await,
        Err(BalanceFailure::Binding)
    );
    assert!(
        f.journals().await == before,
        "invalid limits cannot alter retained state"
    );
    assert_eq!(f.balance_source_status().await.requests, 0);
}
