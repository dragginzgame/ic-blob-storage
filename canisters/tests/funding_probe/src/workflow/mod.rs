//! Fixture orchestration. No payment retry or production provider contract.

use blob_test_protocol::funding::{
    FundingFailure, FundingObservation, FundingOperatorStatusView, FundingRequest,
};
use ic_blob_storage::policy::{
    billing::{
        liquidity::{FundingLiquidityDecision, assess_funding_liquidity},
        reconciliation::{assess_funding_reconciliation, assess_uncredited_activity},
    },
    diagnostics::assess_operator,
};
use ic_platform::Principal;

use crate::ops;
pub(crate) mod preview;

pub(crate) fn lookup(
    caller: Principal,
    request: blob_test_protocol::funding::lookup::FundingLookupRequest,
) -> Result<
    blob_test_protocol::funding::lookup::FundingLookupView,
    blob_test_protocol::funding::lookup::FundingLookupFailure,
> {
    ops::lookup::lookup(caller, request)
}

pub(crate) async fn fund(
    caller: Principal,
    request: FundingRequest,
) -> Result<FundingObservation, FundingFailure> {
    let peer = ops::admit(caller, request)?;
    // Persist the attachment intent before observing costs: stable-memory growth
    // changes the platform's liquid balance. Never reuse the query's observation.
    let prepared = ops::liquidity::prepare(peer, request);
    let call = match assess_funding_liquidity(prepared.offered, prepared.liquidity) {
        FundingLiquidityDecision::Fits => ops::transfer(prepared).await,
        FundingLiquidityDecision::Insufficient { .. } => ops::liquidity::blocked(prepared),
    };
    let reconciliation = ops::status::reconciliation(assess_funding_reconciliation(call.transfer));
    let observation = FundingObservation {
        refunded: call.transfer.refunded(),
        transport_accepted: call.transfer.accepted(),
        outcome: call.outcome,
        reconciliation,
    };
    ops::complete(request, observation);
    if request.trap_callback && observation.refunded.is_some() {
        // The actual IC rolls back complete() while the receiver's acceptance survives.
        // An unsent call has no callback: its refusal must commit in this message.
        ic_platform::cdk::api::trap("deliberate fixture callback failure");
    }
    Ok(observation)
}

pub(crate) async fn receive(caller: Principal, request: FundingRequest) {
    ops::accept(caller, request);
    ops::delay_reply(request.reply).await;
    ops::reply(request.reply);
}

pub(crate) fn configure_cashier(
    caller: Principal,
    input: blob_test_protocol::storage::funding::transport::Substitute,
) -> Result<(), FundingFailure> {
    ops::cashier::configure(caller, input)
}
pub(crate) async fn receive_top_up(caller: Principal, arguments: Vec<u8>) {
    let mode = ops::cashier::accept(caller, &arguments);
    ops::delay_reply(mode).await;
    ops::reply(mode);
}

pub(crate) fn operator_status(
    service: Principal,
    caller: Principal,
) -> Option<FundingOperatorStatusView> {
    let snapshot = ops::status::snapshot(caller, assess_funding_reconciliation)?;
    let activity = assess_uncredited_activity(ops::status::reconciliations(&snapshot));
    let diagnosis = assess_operator(ops::status::observation(&snapshot, activity));
    Some(ops::status::view(service, snapshot, activity, &diagnosis))
}

pub(crate) fn restore() {
    let journal = ops::load();
    for (transfer, retained) in ops::restored_reconciliations(&journal) {
        assert_eq!(
            ops::status::reconciliation(assess_funding_reconciliation(transfer)),
            retained,
            "retained reconciliation matches shared policy"
        );
    }
    // No pre-upgrade snapshot is needed: every mutation is already durable.
    // Unknown intents survive without reconstituting an executable continuation.
    ops::restore_fenced(journal);
}
