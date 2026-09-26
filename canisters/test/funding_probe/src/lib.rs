//! Local IC cycle-transfer experiment. Never deploy as a service or Cashier.

mod model;
mod ops;
mod workflow;

use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingFailure, FundingObservation, FundingReceiptRecord, FundingRequest,
    FundingUpgradeArgs,
};
use candid::Principal;

#[ic_cdk::init]
fn init(peer: Principal, driver: Principal) {
    ops::initialize(peer, driver);
}

#[ic_cdk::post_upgrade]
fn post_upgrade(args: FundingUpgradeArgs) {
    // Restore before exposing endpoints; no deferred task can reset unresolved work.
    ops::restore();
    if args.trap_after_restore {
        ic_cdk::trap("deliberate fixture restore failure");
    }
}

#[ic_cdk::update]
async fn fund(request: FundingRequest) -> Result<FundingObservation, FundingFailure> {
    workflow::fund(ic_cdk::api::msg_caller(), request).await
}

#[ic_cdk::update(manual_reply = true)]
fn receive(request: FundingRequest) {
    workflow::receive(ic_cdk::api::msg_caller(), request);
}

#[ic_cdk::query]
fn attempts() -> Option<Vec<FundingAttemptRecord>> {
    ops::attempts(ic_cdk::api::msg_caller())
}

#[ic_cdk::query]
fn receipts() -> Option<Vec<FundingReceiptRecord>> {
    ops::receipts(ic_cdk::api::msg_caller())
}
