//! Controlled local source for real inter-canister scheduling experiments.
//! Not a Cashier implementation; no deployed provider or monetary effects.

mod model;
mod ops;
mod workflow;

use std::marker::PhantomData;

use blob_test_protocol::journey::readback::{ReadSourceConfig, ReadSourceObservation};
use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use candid::Principal;

#[ic_cdk::init]
fn init(service: Principal, gateway: Principal, driver: Principal) {
    workflow::initialize(service, gateway, driver);
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    workflow::prepare_upgrade();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    workflow::restore();
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

#[ic_cdk::update]
async fn run_deletion(roots: Vec<Vec<u8>>) -> Result<(), u32> {
    workflow::run_deletion(ic_cdk::api::msg_caller(), roots).await
}

#[ic_cdk::update]
fn configure_read(config: ReadSourceConfig) -> bool {
    workflow::readback::configure(ic_cdk::api::msg_caller(), config)
}

#[ic_cdk::query]
fn read_observation() -> Option<ReadSourceObservation> {
    workflow::readback::observation(ic_cdk::api::msg_caller())
}

#[ic_cdk::update]
fn resume_read() -> bool {
    workflow::readback::resume(ic_cdk::api::msg_caller())
}

#[ic_cdk::update(manual_reply = true)]
async fn fixture_chunk(root: Vec<u8>, index: u64) -> PhantomData<Vec<u8>> {
    workflow::readback::reply(ic_cdk::api::msg_caller(), &root, index).await;
    PhantomData
}

#[ic_cdk::update(manual_reply = true)]
async fn fixture_gateways() -> PhantomData<Vec<Principal>> {
    workflow::reply(ic_cdk::api::msg_caller()).await;
    PhantomData
}

#[ic_cdk::query]
fn recovery_observation() -> Option<blob_test_protocol::source::SourceRecoveryView> {
    workflow::recovery(ic_cdk::api::msg_caller())
}
