//! Current journal observations and passive projections; no reads from a peer.

use std::num::NonZeroU128;

use blob_test_protocol::{
    funding::{
        FundingAttemptRecord, FundingAttemptStatusView, FundingOperatorStatusView,
        FundingReceiptRecord, FundingReconciliationView,
    },
    status::{
        FundingActivityView,
        conversion::{blocker, warning},
    },
};
use candid::Principal;
use ic_blob_storage::policy::billing::RecoveryState;
use ic_blob_storage::policy::billing::admission::FundingActivity;
use ic_blob_storage::policy::billing::reconciliation::FundingReconciliation;
use ic_blob_storage::policy::diagnostics::BillingAssessment;
use ic_blob_storage::policy::diagnostics::OperatorDiagnosis;
use ic_blob_storage::policy::diagnostics::OperatorObservation;
use ic_blob_storage_contracts::funding::transfer::FundingTransfer;

pub(crate) struct FundingSnapshot {
    budget: crate::model::budget::FundingBudgetSnapshot,
    peer: Principal,
    fenced: bool,
    attempts: Vec<AssessedAttempt>,
    receipts: Vec<FundingReceiptRecord>,
}

struct AssessedAttempt {
    entry: FundingAttemptRecord,
    transfer: FundingTransfer,
    reconciliation: FundingReconciliation,
}

pub(crate) fn snapshot(
    caller: Principal,
    assess: fn(FundingTransfer) -> FundingReconciliation,
) -> Option<FundingSnapshot> {
    super::read(|state| {
        Some(FundingSnapshot {
            budget: state.budget(),
            peer: state.peer(),
            fenced: state.fenced(),
            attempts: state
                .attempts(caller)?
                .into_iter()
                .map(|entry| {
                    let transfer = transfer(&entry);
                    AssessedAttempt {
                        entry,
                        transfer,
                        reconciliation: assess(transfer),
                    }
                })
                .collect(),
            receipts: state.receipts(caller)?,
        })
    })
}

pub(crate) fn reconciliations(
    snapshot: &FundingSnapshot,
) -> impl Iterator<Item = FundingReconciliation> + '_ {
    snapshot
        .attempts
        .iter()
        .map(|attempt| attempt.reconciliation)
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
            .into_iter()
            .map(
                |AssessedAttempt {
                     entry,
                     transfer,
                     reconciliation: result,
                 }| FundingAttemptStatusView {
                    id: entry.request.id,
                    offered: entry.request.offered,
                    refunded: transfer.refunded(),
                    transport_accepted: transfer.accepted(),
                    outcome: entry.observation.map(|o| o.outcome),
                    provider_credit: None,
                    reconciliation: reconciliation(result),
                },
            )
            .collect(),
        receipts: snapshot.receipts,
        blockers: diagnosis.blockers().iter().copied().map(blocker).collect(),
        warnings: diagnosis.warnings().iter().copied().map(warning).collect(),
    }
}

pub(crate) const fn reconciliation(value: FundingReconciliation) -> FundingReconciliationView {
    match value {
        FundingReconciliation::CreditConfirmed {
            accepted_cycles,
            receipt_digest,
        } => FundingReconciliationView::CreditConfirmed {
            accepted_cycles: accepted_cycles.get(),
            receipt_digest,
        },
        FundingReconciliation::NoTransfer => FundingReconciliationView::NoTransfer,
        FundingReconciliation::CreditRequired { accepted_cycles } => {
            FundingReconciliationView::CreditRequired(accepted_cycles.get())
        }
        FundingReconciliation::TransferUnknown { offered_cycles } => {
            FundingReconciliationView::TransferUnknown(offered_cycles.get())
        }
    }
}

#[cfg(test)]
mod tests;
