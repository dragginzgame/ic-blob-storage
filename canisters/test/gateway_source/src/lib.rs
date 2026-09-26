//! Controlled local source for real inter-canister scheduling experiments.
//! Not a Cashier implementation; no deployed provider or monetary effects.

mod ops;
mod workflow;

use std::marker::PhantomData;

use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use candid::Principal;

#[ic_cdk::init]
fn init(service: Principal, gateway: Principal, driver: Principal) {
    workflow::initialize(service, gateway, driver);
}

#[ic_cdk::update]
fn configure(mode: SourceMode) -> bool {
    workflow::configure(ic_cdk::api::msg_caller(), mode)
}

#[ic_cdk::update]
async fn run_sync() -> Result<(), SyncFailure> {
    workflow::run_sync(ic_cdk::api::msg_caller()).await
}

#[ic_cdk::query]
fn observation() -> Option<SourceObservation> {
    workflow::observation(ic_cdk::api::msg_caller())
}

#[ic_cdk::update(manual_reply = true)]
async fn fixture_gateways() -> PhantomData<Vec<Principal>> {
    workflow::reply(ic_cdk::api::msg_caller()).await;
    PhantomData
}
