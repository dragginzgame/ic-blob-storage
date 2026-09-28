//! Local application transaction/outbox substitute. Never deploy as a product or Toko adapter.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Candid endpoints own decoded inputs"
)]
mod model;
mod ops;
mod workflow;
use blob_test_protocol::consumer::{AssetView, Failure, Recovery, Release, Run, Use};
use candid::Principal;
#[ic_cdk::init]
fn init(operator: Principal, service: Principal) {
    ops::initialize(Some((operator, service)));
}
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ops::initialize(None);
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn register(input: Box<Run>) -> Result<AssetView, Failure> {
    workflow::register(ic_cdk::api::msg_caller(), &input).await
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn cancel(asset: u128) -> Result<AssetView, Failure> {
    let actor = ic_cdk::api::msg_caller();
    ops::mutate(actor, |r| {
        r.cancel(asset)?;
        r.view(asset)
    })
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn release(input: Release) -> Result<AssetView, Failure> {
    workflow::release(ic_cdk::api::msg_caller(), input.asset, input.fault).await
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn recover(input: Recovery) -> Result<AssetView, Failure> {
    workflow::recover(ic_cdk::api::msg_caller(), input.asset, input.release).await
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn use_asset(input: Use) -> Result<AssetView, Failure> {
    let actor = ic_cdk::api::msg_caller();
    ops::mutate(actor, |r| {
        r.use_asset(input.asset, input.attach)?;
        r.view(input.asset)
    })
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn asset(asset: u128) -> Result<AssetView, Failure> {
    ops::view(ic_cdk::api::msg_caller(), asset)
}
#[ic_cdk::query]
fn waiting() -> Result<bool, Failure> {
    ops::waiting(ic_cdk::api::msg_caller())
}
#[ic_cdk::query]
fn fenced() -> Result<bool, Failure> {
    ops::read(ic_cdk::api::msg_caller(), |r| Ok(r.fenced))
}
#[ic_cdk::update]
fn resume() -> Result<(), Failure> {
    ops::resume(ic_cdk::api::msg_caller())
}

// Save intent without dispatch; admit also persists intent before its first call.
#[ic_cdk::update(decode_with = "ops::decode")]
fn prepare(input: Box<blob_test_protocol::consumer::Registration>) -> Result<AssetView, Failure> {
    workflow::prepare(ic_cdk::api::msg_caller(), &input)
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn admit(input: Box<Run>) -> Result<AssetView, Failure> {
    workflow::admit(ic_cdk::api::msg_caller(), &input).await
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn revoke(input: blob_test_protocol::consumer::Revocation) -> Result<AssetView, Failure> {
    workflow::revoke(ic_cdk::api::msg_caller(), input).await
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn recover_revocation(asset: u128) -> Result<AssetView, Failure> {
    workflow::recover_revocation(ic_cdk::api::msg_caller(), asset).await
}
