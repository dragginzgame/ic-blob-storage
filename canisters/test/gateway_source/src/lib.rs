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
async fn run_sync(input: blob_test_protocol::GatewaySyncRequest) -> Result<(), SyncFailure> {
    workflow::run_sync(ic_cdk::api::msg_caller(), input).await
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

#[ic_cdk::update]
fn resume_sync() -> bool {
    workflow::resume_sync(ic_cdk::api::msg_caller())
}

#[ic_cdk::update(manual_reply = true)]
async fn fixture_chunk(root: Vec<u8>, index: u64) -> PhantomData<Vec<u8>> {
    workflow::readback::reply(ic_cdk::api::msg_caller(), &root, index).await;
    PhantomData
}

#[ic_cdk::update(manual_reply = true)]
async fn fixture_gateways(
    input: blob_test_protocol::GatewaySyncRequest,
) -> PhantomData<Vec<Principal>> {
    workflow::reply(ic_cdk::api::msg_caller(), input).await;
    PhantomData
}

#[ic_cdk::query]
fn recovery_observation() -> Option<blob_test_protocol::source::SourceRecoveryView> {
    workflow::recovery(ic_cdk::api::msg_caller())
}

#[ic_cdk::update]
fn configure_balance(config: blob_test_protocol::balance::BalanceSourceConfig) -> bool {
    workflow::configure_balance(ic_cdk::api::msg_caller(), config)
}
#[ic_cdk::update]
fn resume_balance() -> bool {
    workflow::resume_balance(ic_cdk::api::msg_caller())
}
#[ic_cdk::query]
fn balance_observation() -> Option<blob_test_protocol::balance::BalanceSourceView> {
    workflow::balance_observation(ic_cdk::api::msg_caller())
}
#[ic_cdk::update(manual_reply = true)]
async fn fixture_balance(
    input: blob_test_protocol::balance::BalanceSourceRequest,
) -> PhantomData<()> {
    workflow::balance_reply(ic_cdk::api::msg_caller(), input.account).await;
    PhantomData
}

// Maintained method/argument shape over driver-controlled bytes, never Cashier semantics.
#[ic_cdk::query(manual_reply = true)]
fn account_balance_get_v1(
    input: blob_test_protocol::balance::BalanceSourceRequest,
) -> PhantomData<()> {
    workflow::inspect_relationship(ic_cdk::api::msg_caller(), input.account);
    PhantomData
}

// Both account query probes use the same passive driver-controlled reply source.
#[ic_cdk::query(manual_reply = true)]
fn payment_account_canister_get_v1(
    input: blob_test_protocol::balance::RelationshipSourceRequest,
) -> PhantomData<()> {
    workflow::inspect_relationship(ic_cdk::api::msg_caller(), input.canister);
    PhantomData
}

// Explicit local scheduling substitute; not the provider query endpoint.
#[ic_cdk::update(manual_reply = true)]
async fn fixture_gateway_query() -> PhantomData<Vec<Principal>> {
    workflow::gateway_query(ic_cdk::api::msg_caller()).await;
    PhantomData
}

// Query-only wire probe; fixture scheduling/effect modes cannot execute here.
#[ic_cdk::query(manual_reply = true)]
fn storage_gateway_list_v1() -> PhantomData<Vec<Principal>> {
    workflow::inspect_gateways(ic_cdk::api::msg_caller());
    PhantomData
}
