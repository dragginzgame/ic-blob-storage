//! Controlled local source for real inter-canister scheduling experiments.
//! Not a Cashier implementation; no deployed provider or monetary effects.

mod model;
mod ops;
mod workflow;

use std::marker::PhantomData;

use blob_test_protocol::journey::readback::{ReadSourceConfig, ReadSourceObservation};
use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use ic_platform::Principal;

#[ic_platform::cdk::init(decode_with = "blob_test_protocol::fixture_candid::decode_args")]
fn init(service: Principal, gateway: Principal, driver: Principal) {
    workflow::initialize(service, gateway, driver);
}

#[ic_platform::cdk::pre_upgrade]
fn pre_upgrade() {
    workflow::prepare_upgrade();
}

#[ic_platform::cdk::post_upgrade]
fn post_upgrade() {
    workflow::restore();
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_candid::decode_one",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn configure(mode: SourceMode) -> bool {
    workflow::configure(ic_platform::cdk::api::msg_caller(), mode)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_candid::decode_one",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
async fn run_sync(input: blob_test_protocol::GatewaySyncRequest) -> Result<(), SyncFailure> {
    workflow::run_sync(ic_platform::cdk::api::msg_caller(), input).await
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn observation() -> Option<SourceObservation> {
    workflow::observation(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_candid::decode_one",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
async fn run_deletion(roots: Vec<Vec<u8>>) -> Result<(), u32> {
    workflow::run_deletion(ic_platform::cdk::api::msg_caller(), roots).await
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_candid::decode_one",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn configure_read(config: ReadSourceConfig) -> bool {
    workflow::readback::configure(ic_platform::cdk::api::msg_caller(), config)
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn read_observation() -> Option<ReadSourceObservation> {
    workflow::readback::observation(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn resume_read() -> bool {
    workflow::readback::resume(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn resume_sync() -> bool {
    workflow::resume_sync(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::update(
    manual_reply = true,
    decode_with = "blob_test_protocol::fixture_candid::decode_args"
)]
async fn fixture_chunk(root: Vec<u8>, index: u64) -> PhantomData<Vec<u8>> {
    workflow::readback::reply(ic_platform::cdk::api::msg_caller(), &root, index).await;
    PhantomData
}

#[ic_platform::cdk::update(
    manual_reply = true,
    decode_with = "blob_test_protocol::fixture_candid::decode_one"
)]
async fn fixture_gateways(
    input: blob_test_protocol::GatewaySyncRequest,
) -> PhantomData<Vec<Principal>> {
    workflow::reply(ic_platform::cdk::api::msg_caller(), input).await;
    PhantomData
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn recovery_observation() -> Option<blob_test_protocol::source::SourceRecoveryView> {
    workflow::recovery(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_candid::decode_one",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn configure_balance(config: blob_test_protocol::balance::BalanceSourceConfig) -> bool {
    workflow::configure_balance(ic_platform::cdk::api::msg_caller(), config)
}
#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn resume_balance() -> bool {
    workflow::resume_balance(ic_platform::cdk::api::msg_caller())
}
#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments",
    encode_with = "blob_test_protocol::fixture_candid::encode_one"
)]
fn balance_observation() -> Option<blob_test_protocol::balance::BalanceSourceView> {
    workflow::balance_observation(ic_platform::cdk::api::msg_caller())
}
#[ic_platform::cdk::update(
    manual_reply = true,
    decode_with = "blob_test_protocol::fixture_candid::decode_one"
)]
async fn fixture_balance(
    input: blob_test_protocol::balance::BalanceSourceRequest,
) -> PhantomData<()> {
    workflow::balance_reply(ic_platform::cdk::api::msg_caller(), input.account).await;
    PhantomData
}

// Maintained method/argument shape over driver-controlled bytes, never Cashier semantics.
#[ic_platform::cdk::query(
    manual_reply = true,
    decode_with = "blob_test_protocol::fixture_candid::decode_one"
)]
fn account_balance_get_v1(
    input: blob_test_protocol::balance::BalanceSourceRequest,
) -> PhantomData<()> {
    workflow::inspect_relationship(ic_platform::cdk::api::msg_caller(), input.account);
    PhantomData
}

// Both account query probes use the same passive driver-controlled reply source.
#[ic_platform::cdk::query(
    manual_reply = true,
    decode_with = "blob_test_protocol::fixture_candid::decode_one"
)]
fn payment_account_canister_get_v1(
    input: blob_test_protocol::balance::RelationshipSourceRequest,
) -> PhantomData<()> {
    workflow::inspect_relationship(ic_platform::cdk::api::msg_caller(), input.canister);
    PhantomData
}

// Explicit local scheduling substitute; not the provider query endpoint.
#[ic_platform::cdk::update(
    manual_reply = true,
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments"
)]
async fn fixture_gateway_query() -> PhantomData<Vec<Principal>> {
    workflow::gateway_query(ic_platform::cdk::api::msg_caller()).await;
    PhantomData
}

// Query-only wire probe; fixture scheduling/effect modes cannot execute here.
#[ic_platform::cdk::query(
    manual_reply = true,
    decode_with = "blob_test_protocol::fixture_candid::ignore_arguments"
)]
fn storage_gateway_list_v1() -> PhantomData<Vec<Principal>> {
    workflow::inspect_gateways(ic_platform::cdk::api::msg_caller());
    PhantomData
}
