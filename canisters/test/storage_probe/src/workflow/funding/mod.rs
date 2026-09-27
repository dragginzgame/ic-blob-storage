//! Delegate current-state admission to the shared handler, not a saved preview.
pub(crate) mod dispatch;
use crate::ops::funding::{self, admission};
use blob_test_protocol::storage::{
    Failure,
    funding::{
        Intent,
        admission::{Preparation, View},
    },
};
use ic_blob_storage::{
    model::{
        billing::journal::{FundingIntent, FundingJournalScope},
        service::upload::UploadContext,
    },
    workflow::funding::{FundingPreparationResult, inspect_preparation, prepare_new},
};
const fn scope(intent: FundingIntent) -> FundingJournalScope {
    FundingJournalScope {
        service: intent.service,
        cashier: intent.cashier,
        account: intent.account,
        namespace: intent.namespace,
    }
}
pub(crate) fn inspect(execution: UploadContext, input: Intent) -> Result<View, Failure> {
    let intent = funding::intent(input)?;
    let evidence = admission::unknown_evidence(scope(intent));
    let view = admission::with_journal(|journal| {
        inspect_preparation(journal, execution, intent, evidence)
    })
    .map_err(funding::failure)?;
    Ok(View {
        intent: input,
        blockers: admission::blockers(view.assessment.blockers()),
    })
}
pub(crate) fn prepare(execution: UploadContext, input: Intent) -> Result<Preparation, Failure> {
    let intent = funding::intent(input)?;
    let evidence = admission::unknown_evidence(scope(intent));
    let result =
        admission::with_journal_mut(|journal| prepare_new(journal, execution, intent, evidence))
            .map_err(funding::failure)?;
    Ok(match result {
        FundingPreparationResult::Blocked(assessment) => {
            Preparation::Blocked(admission::blockers(assessment.blockers()))
        }
        FundingPreparationResult::Prepared(_) => Preparation::Prepared,
    })
}
pub(crate) fn inspect_attempt(execution: UploadContext, input: Intent) -> Result<View, Failure> {
    let intent = funding::intent(input)?;
    let evidence = admission::unknown_attempt_evidence(intent);
    let view = admission::with_journal(|journal| {
        ic_blob_storage::workflow::funding::attempt::inspect_attempt(
            journal, execution, intent, evidence,
        )
    })
    .map_err(funding::failure)?;
    Ok(View {
        intent: input,
        blockers: admission::attempt_blockers(view.blockers()),
    })
}
pub(crate) fn mark_attempt(
    execution: UploadContext,
    input: Intent,
) -> Result<blob_test_protocol::storage::funding::admission::Attempt, Failure> {
    use blob_test_protocol::storage::funding::admission::Attempt;
    use ic_blob_storage::workflow::funding::attempt::{FundingAttemptResult, mark_first_attempt};
    let intent = funding::intent(input)?;
    let evidence = admission::unknown_attempt_evidence(intent);
    let result = admission::with_journal_mut(|journal| {
        mark_first_attempt(journal, execution, intent, evidence)
    })
    .map_err(funding::failure)?;
    Ok(match result {
        FundingAttemptResult::Blocked(assessment) => {
            Attempt::Blocked(admission::attempt_blockers(assessment.blockers()))
        }
        FundingAttemptResult::Marked(_) => Attempt::Marked,
    })
}
