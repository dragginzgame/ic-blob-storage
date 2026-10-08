//! Scope conversion and read-only status projection; policy belongs to workflow.
use super::{Failure, STATE, UploadContext, failure};
use blob_test_protocol::{
    status::FundingActivityView,
    storage::funding::{Allocation, summary::Summary},
};
use ic_blob_storage::model::billing::journal::FundingJournalScope;
use ic_blob_storage::ops::service::funding::summary::FundingJournalSummary;
use ic_blob_storage::policy::billing::admission::FundingActivity;
use ic_blob_storage_contracts::dto::operator::OperatorScope;

pub(crate) fn read(
    execution: UploadContext,
    input: OperatorScope,
) -> Result<FundingJournalSummary, Failure> {
    let scope = FundingJournalScope {
        service: input.service,
        cashier: input.cashier,
        account: input.payment_account,
        namespace: std::num::NonZeroU128::new(input.namespace).ok_or(Failure::Invalid)?,
    };
    STATE
        .with_borrow(|state| state.as_ref().unwrap().funding.summary(execution, scope))
        .map_err(failure)
}
pub(crate) fn present(view: FundingJournalSummary, activity: FundingActivity) -> Summary {
    Summary {
        scope: OperatorScope {
            service: view.scope.service,
            cashier: view.scope.cashier,
            payment_account: view.scope.account,
            namespace: view.scope.namespace.get(),
        },
        allocation: Allocation {
            available: view.allocation.available(),
            accepted: view.allocation.accepted(),
            refunded: view.allocation.refunded(),
            not_enqueued: view.allocation.not_enqueued(),
            uncertain: view.allocation.reserved_or_uncertain(),
            fenced: view.fenced,
        },
        retained_intents: view.retained_intents,
        intent_capacity: view.intent_capacity,
        last_operation: view.last_operation.map(std::num::NonZeroU128::get),
        local_activity: match activity {
            FundingActivity::Clear => FundingActivityView::Clear,
            FundingActivity::InProgress => FundingActivityView::InProgress,
            FundingActivity::Uncertain => FundingActivityView::Uncertain,
        },
    }
}
