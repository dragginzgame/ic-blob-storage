//! Controlled local source for real inter-canister scheduling experiments.
//! Not a Cashier implementation; no deployed provider or monetary effects.

mod model;
mod ops;
mod workflow;

use std::marker::PhantomData;

use blob_test_protocol::{SourceMode, SourceObservation};
use ic_platform::principal::ExternalPrincipal;

#[ic_platform::cdk::init(decode_with = "blob_test_protocol::fixture_wire::hooks::init")]
fn init(service: ExternalPrincipal, gateway: ExternalPrincipal, driver: ExternalPrincipal) {
    workflow::initialize(service.into(), gateway.into(), driver.into());
}

#[ic_platform::cdk::pre_upgrade]
fn pre_upgrade() {
    workflow::prepare_upgrade();
}

#[ic_platform::cdk::post_upgrade(
    decode_with = "blob_test_protocol::fixture_wire::hooks::upgrade_unit"
)]
fn post_upgrade() {
    workflow::restore();
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"configure\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"configure\") }>"
)]
fn configure(mode: SourceMode) -> bool {
    workflow::configure(ic_platform::cdk::api::msg_caller(), mode)
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"observation\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"observation\") }>"
)]
fn observation() -> Option<SourceObservation> {
    workflow::observation(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"run_deletion\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"run_deletion\") }>"
)]
async fn run_deletion(roots: Vec<Vec<u8>>) -> Result<(), u32> {
    workflow::run_deletion(ic_platform::cdk::api::msg_caller(), roots).await
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"resume_sync\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"resume_sync\") }>"
)]
fn resume_sync() -> bool {
    workflow::resume_sync(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"recovery_observation\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"recovery_observation\") }>"
)]
fn recovery_observation() -> Option<blob_test_protocol::source::SourceRecoveryView> {
    workflow::recovery(ic_platform::cdk::api::msg_caller())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"configure_balance\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"configure_balance\") }>"
)]
fn configure_balance(config: blob_test_protocol::balance::BalanceSourceConfig) -> bool {
    workflow::configure_balance(ic_platform::cdk::api::msg_caller(), config)
}
#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"resume_balance\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"resume_balance\") }>"
)]
fn resume_balance() -> bool {
    workflow::resume_balance(ic_platform::cdk::api::msg_caller())
}
#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"balance_observation\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"balance_observation\") }>"
)]
fn balance_observation() -> Option<blob_test_protocol::balance::BalanceSourceView> {
    workflow::balance_observation(ic_platform::cdk::api::msg_caller())
}
#[ic_platform::cdk::update(
    manual_reply = true,
    decode_with = "blob_test_protocol::provider_candid::decode_one"
)]
async fn fixture_balance(
    input: blob_test_protocol::balance::BalanceSourceRequest,
) -> PhantomData<()> {
    workflow::balance_reply(ic_platform::cdk::api::msg_caller(), input.account.into()).await;
    PhantomData
}

// Maintained method/argument shape over driver-controlled bytes, never Cashier semantics.
#[ic_platform::cdk::query(
    manual_reply = true,
    decode_with = "blob_test_protocol::provider_candid::decode_one"
)]
fn account_balance_get_v1(
    input: blob_test_protocol::balance::BalanceSourceRequest,
) -> PhantomData<()> {
    workflow::inspect_relationship(ic_platform::cdk::api::msg_caller(), input.account.into());
    PhantomData
}

// Both account query probes use the same passive driver-controlled reply source.
#[ic_platform::cdk::query(
    manual_reply = true,
    decode_with = "blob_test_protocol::provider_candid::decode_one"
)]
fn payment_account_canister_get_v1(
    input: blob_test_protocol::balance::RelationshipSourceRequest,
) -> PhantomData<()> {
    workflow::inspect_relationship(ic_platform::cdk::api::msg_caller(), input.canister.into());
    PhantomData
}

// Explicit local scheduling substitute; not the provider query endpoint.
#[ic_platform::cdk::update(
    manual_reply = true,
    decode_with = "blob_test_protocol::provider_candid::ignore_arguments"
)]
async fn fixture_gateway_query() -> PhantomData<Vec<ExternalPrincipal>> {
    workflow::gateway_query(ic_platform::cdk::api::msg_caller()).await;
    PhantomData
}

// Query-only wire probe; fixture scheduling/effect modes cannot execute here.
#[ic_platform::cdk::query(
    manual_reply = true,
    decode_with = "blob_test_protocol::provider_candid::ignore_arguments"
)]
fn storage_gateway_list_v1() -> PhantomData<Vec<ExternalPrincipal>> {
    workflow::inspect_gateways(ic_platform::cdk::api::msg_caller());
    PhantomData
}
