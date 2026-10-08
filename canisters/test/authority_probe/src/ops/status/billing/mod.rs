//! Snapshot conversion; threshold decisions remain in shared pure policy.
use crate::model::{archive::AuthorityArchiveRecord, balance::OutcomeRecord};
use blob_test_protocol::{
    balance::{BalanceFailure, BalanceScope, BalanceUsability},
    billing::{BillingLimitsView, BillingStatusView, FundingNeedView},
};
use ic_blob_storage::policy::billing::BalanceObservation;
use ic_blob_storage::policy::billing::RecoveryState;
use ic_blob_storage::policy::billing::balance::BalanceContext;
use ic_blob_storage::policy::billing::balance::BalanceDiagnosis;
use ic_blob_storage::policy::billing::balance::FundingNeed;
use ic_blob_storage_contracts::configuration::funding::FundingLimits;

pub(crate) struct BillingSnapshot {
    pub limits: Option<FundingLimits>,
    pub context: BalanceContext,
    pub now: u64,
}

pub(crate) fn snapshot(record: &AuthorityArchiveRecord) -> BillingSnapshot {
    let now = ic_cdk::api::time();
    let recovery = if record.fenced {
        RecoveryState::Fenced
    } else {
        RecoveryState::Reconciled
    };
    BillingSnapshot {
        limits: record.balance.current_limits(),
        now,
        context: BalanceContext {
            gateway_count: record.gateways.len(),
            recovery,
            balance: observed_balance(record, now),
        },
    }
}

fn observed_balance(record: &AuthorityArchiveRecord, now: u64) -> BalanceObservation {
    if let Some(total) = record.balance.total(record.fenced, now) {
        return BalanceObservation::Available(total);
    }
    if record.balance.usability(record.fenced, now) == BalanceUsability::Failed
        && record.balance.attempts.last().is_some_and(|a| {
            matches!(
                a.outcome,
                Some(OutcomeRecord::Failed(
                    BalanceFailure::Malformed
                        | BalanceFailure::Oversized
                        | BalanceFailure::AccountMismatch
                ))
            )
        })
    {
        return BalanceObservation::Malformed;
    }
    BalanceObservation::Unavailable
}

pub(crate) fn view(
    record: &AuthorityArchiveRecord,
    diagnosis: &BalanceDiagnosis,
) -> BillingStatusView {
    let configured = record.balance.billing.configured;
    BillingStatusView {
        scope: configured.map(|v| BalanceScope {
            service: v.scope.service,
            namespace: v.scope.namespace,
            source: v.scope.source,
            account: v.scope.account,
        }),
        revision: configured.map(|v| v.revision),
        limits: configured.map(|v| BillingLimitsView {
            reserve: v.reserve,
            minimum: v.minimum,
            target: v.target,
        }),
        current: record.balance.current_limits().is_some(),
        funding_need: match diagnosis.funding_need() {
            FundingNeed::NotConfigured => FundingNeedView::NotConfigured,
            FundingNeed::NotNeeded => FundingNeedView::NotNeeded,
            FundingNeed::BalanceUnavailable => FundingNeedView::BalanceUnavailable,
            FundingNeed::BalanceMalformed => FundingNeedView::BalanceMalformed,
            FundingNeed::TopUp(amount) => FundingNeedView::TopUp(amount.get()),
        },
    }
}
