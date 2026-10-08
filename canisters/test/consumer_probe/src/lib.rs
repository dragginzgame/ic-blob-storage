//! Local application transaction/outbox substitute. Never deploy as a product or Toko adapter.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Candid endpoints own decoded inputs"
)]
mod model;
mod ops;
mod workflow;
use blob_test_protocol::consumer::manifests::{
    ManifestDispatch, ManifestIntent, ManifestIntentView,
};
use blob_test_protocol::consumer::{AssetView, Failure, Recovery, Release, Run, Use};
use candid::Principal;

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_funding_history(
    input: Box<blob_test_protocol::consumer::funding::FundingHistoryInspection>,
) -> Result<ic_blob_storage_contracts::dto::funding::FundingHistoryPage, Failure> {
    workflow::funding::history(
        ic_cdk::api::msg_caller(),
        input.request,
        input.max_reply_bytes,
    )
    .await
}
#[ic_cdk::query(composite = true, decode_with = "ops::decode")]
async fn fixture_query_funding(
    input: Box<blob_test_protocol::consumer::funding::FundingHistoryInspection>,
) -> Result<ic_blob_storage_contracts::dto::funding::FundingHistoryPage, Failure> {
    workflow::funding::history(
        ic_cdk::api::msg_caller(),
        input.request,
        input.max_reply_bytes,
    )
    .await
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_funding_outcome(
    input: ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest,
) -> Result<Option<ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeResponse>, Failure>
{
    workflow::funding::outcome(ic_cdk::api::msg_caller(), input).await
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_update_tenant(
    input: blob_test_protocol::consumer::TenantDispatch,
) -> Result<ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse, Failure> {
    workflow::tenants::update(ic_cdk::api::msg_caller(), input).await
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_inspect_tenant(
    scope: ic_blob_storage_contracts::dto::tenant::TenantScope,
) -> Result<ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse, Failure> {
    workflow::tenants::inspect(ic_cdk::api::msg_caller(), scope).await
}
#[ic_cdk::query(composite = true, decode_with = "ops::decode")]
async fn fixture_query_tenant(
    scope: ic_blob_storage_contracts::dto::tenant::TenantScope,
) -> Result<ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse, Failure> {
    workflow::tenants::inspect(ic_cdk::api::msg_caller(), scope).await
}
#[ic_cdk::query]
fn fixture_tenant_command()
-> Result<Option<ic_blob_storage_contracts::dto::tenant::TenantUpdateRequest>, Failure> {
    workflow::tenants::saved(ic_cdk::api::msg_caller())
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn save_manifest(input: Box<ManifestIntent>) -> Result<ManifestIntentView, Failure> {
    workflow::manifests::save(ic_cdk::api::msg_caller(), &input)
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn dispatch_manifest(input: ManifestDispatch) -> Result<ManifestIntentView, Failure> {
    workflow::manifests::dispatch(ic_cdk::api::msg_caller(), input).await
}
#[ic_cdk::update(decode_with = "ops::decode")]
async fn recover_manifest(id: u128) -> Result<ManifestIntentView, Failure> {
    workflow::manifests::recover(ic_cdk::api::msg_caller(), id).await
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn cancel_manifest(id: u128) -> Result<ManifestIntentView, Failure> {
    workflow::manifests::cancel(ic_cdk::api::msg_caller(), id)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn manifest_intent(id: u128) -> Result<ManifestIntentView, Failure> {
    workflow::manifests::view(ic_cdk::api::msg_caller(), id)
}
#[ic_cdk::query(composite = true, decode_with = "ops::decode")]
async fn query_manifest(
    id: u128,
) -> Result<ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse, Failure> {
    workflow::manifests::inspect(ic_cdk::api::msg_caller(), id).await
}
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
