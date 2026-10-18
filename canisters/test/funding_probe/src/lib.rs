//! Local IC cycle-transfer experiment. Never deploy as a service or Cashier.

mod model;
mod ops;
mod workflow;

use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingFailure, FundingObservation, FundingOperatorStatusView,
    FundingReceiptRecord, FundingRequest, FundingUpgradeArgs,
};
use ic_platform::Principal;

#[ic_platform::cdk::init]
fn init(
    peer: Principal,
    driver: Principal,
    budget: blob_test_protocol::funding::budget::FundingBudgetInput,
) {
    ops::initialize(peer, driver, budget);
}

#[ic_platform::cdk::post_upgrade]
fn post_upgrade(args: FundingUpgradeArgs) {
    // Restore before exposing endpoints; no deferred task can reset unresolved work.
    workflow::restore();
    if args.trap_after_restore {
        ic_platform::cdk::api::trap("deliberate fixture restore failure");
    }
}

#[ic_platform::cdk::update]
async fn fund(request: FundingRequest) -> Result<FundingObservation, FundingFailure> {
    workflow::fund(ic_platform::cdk::api::msg_caller(), request).await
}

#[ic_platform::cdk::update(manual_reply = true)]
async fn receive(request: FundingRequest) {
    workflow::receive(ic_platform::cdk::api::msg_caller(), request).await;
}

#[ic_platform::cdk::update]
fn configure_cashier(
    input: blob_test_protocol::storage::funding::transport::Substitute,
) -> Result<(), FundingFailure> {
    workflow::configure_cashier(ic_platform::cdk::api::msg_caller(), input)
}

#[ic_platform::cdk::update(manual_reply = true, decode_with = "ops::cashier::raw_arguments")]
async fn account_top_up_v1(arguments: Vec<u8>) {
    workflow::receive_top_up(ic_platform::cdk::api::msg_caller(), arguments).await;
}

#[ic_platform::cdk::query]
fn attempts() -> Option<Vec<FundingAttemptRecord>> {
    ops::attempts(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::query]
fn receipts() -> Option<Vec<FundingReceiptRecord>> {
    ops::receipts(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::query]
fn lookup_funding(
    request: blob_test_protocol::funding::lookup::FundingLookupRequest,
) -> Result<
    blob_test_protocol::funding::lookup::FundingLookupView,
    blob_test_protocol::funding::lookup::FundingLookupFailure,
> {
    workflow::lookup(ic_platform::cdk::api::msg_caller(), request)
}

#[ic_platform::cdk::query]
fn operator_status() -> Option<FundingOperatorStatusView> {
    workflow::operator_status(
        ic_platform::cdk::api::canister_self(),
        ic_platform::cdk::api::msg_caller(),
    )
}

#[ic_platform::cdk::query]
fn preview_funding(
    request: blob_test_protocol::funding::preview::FundingPreviewRequest,
) -> Result<
    blob_test_protocol::funding::preview::FundingPreviewView,
    blob_test_protocol::funding::preview::FundingPreviewFailure,
> {
    workflow::preview::preview(
        ic_platform::cdk::api::canister_self(),
        ic_platform::cdk::api::msg_caller(),
        request,
    )
}
