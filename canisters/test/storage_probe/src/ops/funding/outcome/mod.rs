//! Read-only projection of the exact durable funding observation.
use super::{Failure, STATE, UploadContext, failure, intent, phase, transport};
use blob_test_protocol::{
    funding::FundingReconciliationView,
    storage::funding::{Intent, outcome::Outcome},
};
use ic_blob_storage::{
    ops::caffeine::funding::{TopUpReply, transport::CashierTopUpStatus},
    ops::service::funding::outcome::FundingOutcomeView,
    policy::billing::reconciliation::FundingReconciliation,
};

pub(crate) fn read(
    execution: UploadContext,
    input: Intent,
) -> Result<Option<FundingOutcomeView>, Failure> {
    let original = intent(input)?;
    STATE
        .with_borrow(|state| state.as_ref().unwrap().funding.outcome(execution, original))
        .map_err(failure)
}
pub(crate) fn present(
    input: Intent,
    view: &FundingOutcomeView,
    reconciliation: FundingReconciliation,
) -> Outcome {
    let reported_balance = match view.response {
        Some(CashierTopUpStatus::Replied(Ok(TopUpReply::ReportedSuccess { balance }))) => Some([
            balance.total(),
            balance.prepaid(),
            balance.promotional(),
            balance.ledger(),
        ]),
        _ => None,
    };
    Outcome {
        intent: input,
        phase: phase(view.state),
        response: view.response.map(transport::outcome),
        reported_balance,
        reconciliation: match reconciliation {
            FundingReconciliation::NoTransfer => FundingReconciliationView::NoTransfer,
            FundingReconciliation::CreditRequired { accepted_cycles } => {
                FundingReconciliationView::CreditRequired(accepted_cycles.get())
            }
            FundingReconciliation::TransferUnknown { offered_cycles } => {
                FundingReconciliationView::TransferUnknown(offered_cycles.get())
            }
        },
    }
}
