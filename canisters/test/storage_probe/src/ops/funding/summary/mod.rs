//! Scope conversion and read-only status projection; policy belongs to workflow.
use super::{Failure, STATE, UploadContext, failure, history};
use blob_test_protocol::{
    status::FundingActivityView,
    storage::funding::{Allocation, history::Scope, summary::Summary},
};
use ic_blob_storage::{
    ops::service::funding::summary::FundingJournalSummary,
    policy::billing::admission::FundingActivity,
};

pub(crate) fn read(
    execution: UploadContext,
    input: Scope,
) -> Result<FundingJournalSummary, Failure> {
    let scope = history::scope(input)?;
    STATE
        .with_borrow(|state| state.as_ref().unwrap().funding.summary(execution, scope))
        .map_err(failure)
}
pub(crate) fn present(view: FundingJournalSummary, activity: FundingActivity) -> Summary {
    Summary {
        scope: history::wire_scope(view.scope),
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
