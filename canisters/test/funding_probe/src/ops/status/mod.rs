//! Current journal observations and passive projections; no reads from a peer.

use std::num::NonZeroU128;

use blob_test_protocol::{
    funding::{
        FundingAttemptRecord, FundingAttemptStatusView, FundingOperatorStatusView,
        FundingReceiptRecord, FundingReconciliationView,
    },
    status::{
        BillingBlockerView, BillingWarningView, FundingActivityView, OperatorBlockerView,
        OperatorWarningView,
    },
};
use candid::Principal;
use ic_blob_storage::{
    model::billing::transfer::FundingTransfer,
    policy::{
        billing::{
            BillingBlocker, BillingWarning, RecoveryState, admission::FundingActivity,
            reconciliation::FundingReconciliation,
        },
        diagnostics::{
            BillingAssessment, OperatorBlocker, OperatorDiagnosis, OperatorObservation,
            OperatorWarning,
        },
    },
};

pub(crate) struct FundingSnapshot {
    budget: crate::model::budget::FundingBudgetSnapshot,
    peer: Principal,
    fenced: bool,
    attempts: Vec<FundingAttemptRecord>,
    receipts: Vec<FundingReceiptRecord>,
}

pub(crate) fn snapshot(caller: Principal) -> Option<FundingSnapshot> {
    super::read(|state| {
        Some(FundingSnapshot {
            budget: state.budget(),
            peer: state.peer(),
            fenced: state.fenced(),
            attempts: state.attempts(caller)?,
            receipts: state.receipts(caller)?,
        })
    })
}

pub(crate) fn transfers(snapshot: &FundingSnapshot) -> Vec<FundingTransfer> {
    snapshot.attempts.iter().map(transfer).collect()
}

pub(super) fn transfer(entry: &FundingAttemptRecord) -> FundingTransfer {
    let offered = NonZeroU128::new(entry.request.offered).expect("admitted positive attachment");
    crate::model::checked_transfer(entry).unwrap_or_else(|_| FundingTransfer::unknown(offered))
}

pub(crate) fn observation(
    snapshot: &FundingSnapshot,
    activity: FundingActivity,
) -> OperatorObservation<'static> {
    OperatorObservation {
        // The fence is enforced after restore; an active experiment still has no
        // independent recovery qualification. Never infer it from a local counter.
        recovery: snapshot.fenced.then_some(RecoveryState::Fenced),
        provider_qualified: false,
        // The local transfer peer is not a validated storage gateway.
        gateway_count: 0,
        billing: BillingAssessment::NotConfigured,
        funding: Some(activity),
        sync_pending: false,
        read_pending: false,
        uncertain_uploads: 0,
        pending_deletions: 0,
        pending_billing: 0,
    }
}

pub(crate) fn view(
    service: Principal,
    snapshot: FundingSnapshot,
    transfers: &[FundingTransfer],
    reconciliations: &[FundingReconciliation],
    activity: FundingActivity,
    diagnosis: &OperatorDiagnosis,
) -> FundingOperatorStatusView {
    FundingOperatorStatusView {
        service,
        peer: snapshot.peer,
        fenced: snapshot.fenced,
        provider_qualified: false,
        billing_configured: false,
        provider_balance: None,
        available_funding_cycles: None,
        budget: super::preview::budget_view(&snapshot.budget),
        funding_activity: match activity {
            FundingActivity::Clear => FundingActivityView::Clear,
            FundingActivity::InProgress => FundingActivityView::InProgress,
            FundingActivity::Uncertain => FundingActivityView::Uncertain,
        },
        attempts: snapshot
            .attempts
            .iter()
            .zip(transfers)
            .zip(reconciliations)
            .map(|((entry, transfer), result)| FundingAttemptStatusView {
                id: entry.request.id,
                offered: entry.request.offered,
                refunded: transfer.refunded(),
                transport_accepted: transfer.accepted(),
                outcome: entry.observation.map(|o| o.outcome),
                provider_credit: None,
                reconciliation: reconciliation(*result),
            })
            .collect(),
        receipts: snapshot.receipts,
        blockers: diagnosis.blockers().iter().copied().map(blocker).collect(),
        warnings: diagnosis.warnings().iter().copied().map(warning).collect(),
    }
}

pub(crate) const fn reconciliation(value: FundingReconciliation) -> FundingReconciliationView {
    match value {
        FundingReconciliation::NoTransfer => FundingReconciliationView::NoTransfer,
        FundingReconciliation::CreditRequired { accepted_cycles } => {
            FundingReconciliationView::CreditRequired(accepted_cycles.get())
        }
        FundingReconciliation::TransferUnknown { offered_cycles } => {
            FundingReconciliationView::TransferUnknown(offered_cycles.get())
        }
    }
}

fn blocker(value: OperatorBlocker) -> OperatorBlockerView {
    match value {
        OperatorBlocker::RecoveryUnknown => OperatorBlockerView::RecoveryUnknown,
        OperatorBlocker::RecoveryFenced => OperatorBlockerView::RecoveryFenced,
        OperatorBlocker::ProviderUnqualified => OperatorBlockerView::ProviderUnqualified,
        OperatorBlocker::GatewaysMissing => OperatorBlockerView::GatewaysMissing,
        OperatorBlocker::BillingNotConfigured => OperatorBlockerView::BillingNotConfigured,
        OperatorBlocker::BillingUnavailable => OperatorBlockerView::BillingUnavailable,
        OperatorBlocker::SpendabilityUnknown => OperatorBlockerView::SpendabilityUnknown,
        OperatorBlocker::FundingUnknown => OperatorBlockerView::FundingUnknown,
        OperatorBlocker::FundingInProgress => OperatorBlockerView::FundingInProgress,
        OperatorBlocker::FundingUncertain => OperatorBlockerView::FundingUncertain,
        OperatorBlocker::Billing(value) => OperatorBlockerView::Billing(match value {
            BillingBlocker::RecoveryFenced => BillingBlockerView::RecoveryFenced,
            BillingBlocker::NotConfigured => BillingBlockerView::NotConfigured,
            BillingBlocker::GatewayPrincipalsMissing => {
                BillingBlockerView::GatewayPrincipalsMissing
            }
            BillingBlocker::BalanceUnavailable => BillingBlockerView::BalanceUnavailable,
            BillingBlocker::BalanceMalformed => BillingBlockerView::BalanceMalformed,
            BillingBlocker::InsufficientBalance => BillingBlockerView::InsufficientBalance,
            BillingBlocker::ReserveWouldBeViolated => BillingBlockerView::ReserveWouldBeViolated,
        }),
    }
}

fn warning(value: OperatorWarning) -> OperatorWarningView {
    match value {
        OperatorWarning::SyncPending => OperatorWarningView::SyncPending,
        OperatorWarning::ReadPending => OperatorWarningView::ReadPending,
        OperatorWarning::UploadCompletionUnknown => OperatorWarningView::UploadCompletionUnknown,
        OperatorWarning::DeletionPending => OperatorWarningView::DeletionPending,
        OperatorWarning::BillingCessationPending => OperatorWarningView::BillingCessationPending,
        OperatorWarning::Billing(value) => OperatorWarningView::Billing(match value {
            BillingWarning::GatewayPrincipalSetEmpty => {
                BillingWarningView::GatewayPrincipalSetEmpty
            }
            BillingWarning::BalanceUnavailable => BillingWarningView::BalanceUnavailable,
            BillingWarning::BalanceMalformed => BillingWarningView::BalanceMalformed,
        }),
    }
}

#[cfg(test)]
mod tests;
