//! Fixture orchestration. No payment retry or production provider contract.

use blob_test_protocol::funding::{
    FundingFailure, FundingObservation, FundingReconciliationView, FundingRequest,
};
use candid::Principal;
use ic_blob_storage::policy::billing::reconciliation::{
    FundingReconciliation, assess_funding_reconciliation,
};

use crate::ops;

pub(crate) async fn fund(
    caller: Principal,
    request: FundingRequest,
) -> Result<FundingObservation, FundingFailure> {
    let peer = ops::admit(caller, request)?;
    let call = ops::transfer(peer, request).await;
    let reconciliation = match assess_funding_reconciliation(call.transfer) {
        FundingReconciliation::NoTransfer => FundingReconciliationView::NoTransfer,
        FundingReconciliation::CreditRequired { accepted_cycles } => {
            FundingReconciliationView::CreditRequired(accepted_cycles.get())
        }
        FundingReconciliation::TransferUnknown { offered_cycles } => {
            FundingReconciliationView::TransferUnknown(offered_cycles.get())
        }
    };
    let observation = FundingObservation {
        refunded: call.transfer.refunded(),
        transport_accepted: call.transfer.accepted(),
        outcome: call.outcome,
        reconciliation,
    };
    ops::complete(request.id, observation);
    if request.trap_callback {
        // The actual IC rolls back complete() while the receiver's acceptance survives.
        ic_cdk::trap("deliberate fixture callback failure");
    }
    Ok(observation)
}

pub(crate) fn receive(caller: Principal, request: FundingRequest) {
    ops::accept(caller, request);
    ops::reply(request.reply);
}
