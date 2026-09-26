//! Test-only IC authority probe. Sample objects are supplied facts, not uploads.
//! Same-release archive recovery always fences service and provider authority.

#![expect(
    clippy::needless_pass_by_value,
    reason = "Candid endpoints own their decoded inputs; the macro duplicates function-level expectations"
)]

mod model;
mod ops;
mod workflow;

use blob_test_protocol::SyncFailure;
use blob_test_protocol::content::{ContentProbeCase, ContentProbeFailure, ContentProbeReport};
use blob_test_protocol::journey::{
    JourneyCertificate, JourneyFailure, JourneyProgress, JourneyReservation, JourneyUpload,
    JourneyUsage,
};
use blob_test_protocol::obligations::{ObligationProbeFact, ObligationProbeView};
use candid::Principal;
use ic_blob_storage::policy::tenant::TenantAccessContext;

fn context() -> TenantAccessContext {
    ops::archive::recovery::require_active();
    inspection_context()
}

fn inspection_context() -> TenantAccessContext {
    TenantAccessContext {
        service: ic_cdk::api::canister_self(),
        actor: ic_cdk::api::msg_caller(),
    }
}

#[ic_cdk::init]
fn init(first: Principal, second: Principal, gateway: Principal, operator: Principal) {
    workflow::initialize(
        ic_cdk::api::canister_self(),
        first,
        second,
        gateway,
        operator,
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    ops::archive::recovery::prepare();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ops::archive::recovery::restore(ic_cdk::api::canister_self());
}

#[ic_cdk::query]
fn usage() -> Option<u128> {
    workflow::usage(context())
}

#[ic_cdk::query]
fn unsettled_objects() -> Option<Vec<ObligationProbeView>> {
    workflow::unsettled_objects(context())
}

#[ic_cdk::update]
fn confirm_obligation(root: u8, fact: ObligationProbeFact) -> bool {
    workflow::confirm_obligation(context(), root, fact)
}

#[ic_cdk::query]
fn upload_usage() -> Option<u128> {
    workflow::upload_usage(context())
}

#[ic_cdk::query]
fn active_uploads() -> Option<Vec<u8>> {
    workflow::active_uploads(context())
}

#[ic_cdk::query]
fn upload_roots() -> Option<Vec<blob_test_protocol::uploads::UploadProbeState>> {
    workflow::upload_roots(context())
}

#[ic_cdk::update]
fn cancel_upload(root: u8) -> bool {
    workflow::cancel_upload(context(), root)
}

#[ic_cdk::query]
fn reference_live(root: u8, claimed_service: Principal, claimed_tenant: Principal) -> Option<bool> {
    workflow::reference_live(context(), root, claimed_service, claimed_tenant)
}

#[ic_cdk::update]
fn release(root: u8) -> bool {
    workflow::release(context(), root)
}

#[ic_cdk::query]
fn pending() -> Option<Vec<u8>> {
    workflow::pending(context())
}

#[ic_cdk::update]
fn revoke_gateway() -> bool {
    workflow::revoke_gateway(context())
}

#[ic_cdk::update]
async fn sync_gateway(input: blob_test_protocol::GatewaySyncRequest) -> Result<(), SyncFailure> {
    workflow::sync_gateway(inspection_context(), input).await
}

#[ic_cdk::query]
fn preview_gateway_sync(input: blob_test_protocol::GatewaySyncRequest) -> Result<(), SyncFailure> {
    workflow::preview_gateway_sync(inspection_context(), input)
}

#[ic_cdk::update]
fn probe_content(case: ContentProbeCase) -> Result<ContentProbeReport, ContentProbeFailure> {
    workflow::probe_content(context(), case)
}

#[ic_cdk::update]
fn journey_reserve(input: JourneyReservation) -> Result<(), JourneyFailure> {
    workflow::journey::reserve(context(), &input)
}

#[ic_cdk::update]
fn journey_append(root: Vec<u8>, index: u64, bytes: Vec<u8>) -> Result<(), JourneyFailure> {
    workflow::journey::append(context(), &root, index, &bytes)
}

#[ic_cdk::query]
fn journey_progress(root: Vec<u8>) -> Result<JourneyProgress, JourneyFailure> {
    workflow::journey::progress(context(), &root)
}

#[ic_cdk::update]
async fn journey_read(
    root: Vec<u8>,
    index: u64,
) -> Result<blob_test_protocol::journey::readback::JourneyReadChunk, JourneyFailure> {
    workflow::journey::readback::read(context(), &root, index).await
}

#[ic_cdk::update]
fn journey_arm_read_callback_trap(root: Vec<u8>) -> bool {
    workflow::journey::readback::arm_callback_trap(context(), &root)
}

#[ic_cdk::update]
fn journey_cancel(input: JourneyUpload) -> Result<(), JourneyFailure> {
    workflow::journey::cancel(context(), input)
}

#[ic_cdk::update]
fn journey_complete(owner: Principal, input: JourneyUpload) -> Result<(), JourneyFailure> {
    workflow::journey::complete(context(), owner, input)
}

#[ic_cdk::update]
fn journey_release(root: Vec<u8>) -> Result<(), JourneyFailure> {
    workflow::journey::release(context(), &root)
}

#[ic_cdk::update]
fn journey_settle(root: Vec<u8>) -> Result<(), JourneyFailure> {
    workflow::journey::settled(context(), &root)
}

#[ic_cdk::query]
fn journey_usage() -> Result<JourneyUsage, JourneyFailure> {
    workflow::journey::usage(context())
}

#[ic_cdk::update(name = "_immutableObjectStorageCreateCertificate")]
fn create_certificate(root: String) -> JourneyCertificate {
    workflow::journey::certificate(context(), &root).expect("certificate admission")
}

#[ic_cdk::query(name = "_immutableObjectStorageBlobsAreLive")]
fn blobs_live(roots: Vec<Vec<u8>>) -> Vec<bool> {
    workflow::journey::live(context(), &roots).expect("gateway root observation")
}

#[ic_cdk::query(name = "_immutableObjectStorageBlobsToDelete")]
fn blobs_to_delete() -> Vec<Vec<u8>> {
    workflow::journey::pending(context()).expect("gateway deletion list")
}

#[ic_cdk::update(name = "_immutableObjectStorageConfirmBlobDeletion")]
fn confirm_deletion(roots: Vec<Vec<u8>>) {
    workflow::journey::deleted(context(), roots).expect("gateway deletion confirmation");
}

#[ic_cdk::query]
fn authority_archive() -> Option<blob_test_protocol::authority::AuthorityArchiveView> {
    workflow::archive(inspection_context())
}

#[ic_cdk::query]
fn operator_status() -> Option<blob_test_protocol::status::OperatorStatusView> {
    workflow::operator_status(inspection_context())
}

#[ic_cdk::query]
fn probe_checkpoint(
    input: blob_test_protocol::authority::CheckpointProbeInput,
) -> Result<JourneyProgress, blob_test_protocol::authority::CheckpointProbeFailure> {
    workflow::probe_checkpoint(inspection_context(), &input)
}

#[ic_cdk::update]
fn configure_balance(
    scope: blob_test_protocol::balance::BalanceScope,
) -> Result<(), blob_test_protocol::balance::BalanceFailure> {
    workflow::balance::configure(inspection_context(), scope)
}

#[ic_cdk::update]
async fn refresh_balance(
    input: blob_test_protocol::balance::BalanceRefreshRequest,
) -> Result<(), blob_test_protocol::balance::BalanceFailure> {
    workflow::balance::refresh(inspection_context(), input).await
}

#[ic_cdk::query]
fn preview_balance_refresh(
    input: blob_test_protocol::balance::BalanceRefreshRequest,
) -> Result<(), blob_test_protocol::balance::BalanceFailure> {
    workflow::balance::preview(inspection_context(), input)
}

#[ic_cdk::update]
fn configure_billing_limits(
    input: blob_test_protocol::billing::BillingLimitsInput,
) -> Result<(), blob_test_protocol::balance::BalanceFailure> {
    workflow::balance::configure_limits(inspection_context(), input)
}
