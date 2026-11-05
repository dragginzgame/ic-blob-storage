//! Test-only IC authority probe. Sample objects are supplied facts, not uploads.
//! Same-release archive recovery always fences service and provider authority.

#![expect(
    clippy::needless_pass_by_value,
    reason = "Entrypoints own decoded inputs; the macro duplicates function-level expectations"
)]

use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
mod model;
mod ops;
mod workflow;

use blob_test_protocol::SyncFailure;
use blob_test_protocol::content::{ContentProbeCase, ContentProbeFailure, ContentProbeReport};
use blob_test_protocol::journey::{
    JourneyFailure, JourneyProgress, JourneyReservation, JourneyUpload, JourneyUsage,
};
use blob_test_protocol::obligations::{ObligationProbeFact, ObligationProbeView};
use ic_blob_storage::policy::tenant::TenantAccessContext;
use ic_platform::principal::ExternalPrincipal;

fn context() -> TenantAccessContext {
    ops::archive::recovery::require_active();
    inspection_context()
}

fn inspection_context() -> TenantAccessContext {
    TenantAccessContext {
        service: ic_platform::cdk::api::canister_self(),
        actor: ic_platform::cdk::api::msg_caller(),
    }
}

#[ic_platform::cdk::init(decode_with = "blob_test_protocol::fixture_wire::hooks::init")]
fn init(
    first: ExternalPrincipal,
    second: ExternalPrincipal,
    gateway: ExternalPrincipal,
    operator: ExternalPrincipal,
) {
    workflow::initialize(
        ic_platform::cdk::api::canister_self(),
        first.into(),
        second.into(),
        gateway.into(),
        operator.into(),
    );
}

#[ic_platform::cdk::pre_upgrade]
fn pre_upgrade() {
    ops::archive::recovery::prepare();
}

#[ic_platform::cdk::post_upgrade(
    decode_with = "blob_test_protocol::fixture_wire::hooks::upgrade_unit"
)]
fn post_upgrade() {
    ops::archive::recovery::restore(ic_platform::cdk::api::canister_self());
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"usage\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"usage\") }>"
)]
fn usage() -> Option<u128> {
    workflow::usage(context())
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"unsettled_objects\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"unsettled_objects\") }>"
)]
fn unsettled_objects() -> Option<Vec<ObligationProbeView>> {
    workflow::unsettled_objects(context())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::args::<_, { blob_test_protocol::fixture_wire::identity(\"confirm_obligation\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"confirm_obligation\") }>"
)]
fn confirm_obligation(root: u8, fact: ObligationProbeFact) -> bool {
    workflow::confirm_obligation(context(), root, fact)
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"upload_usage\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"upload_usage\") }>"
)]
fn upload_usage() -> Option<u128> {
    workflow::upload_usage(context())
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"active_uploads\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"active_uploads\") }>"
)]
fn active_uploads() -> Option<Vec<u8>> {
    workflow::active_uploads(context())
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"upload_roots\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"upload_roots\") }>"
)]
fn upload_roots() -> Option<Vec<blob_test_protocol::uploads::UploadProbeState>> {
    workflow::upload_roots(context())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"cancel_upload\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"cancel_upload\") }>"
)]
fn cancel_upload(root: u8) -> bool {
    workflow::cancel_upload(context(), root)
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::args::<_, { blob_test_protocol::fixture_wire::identity(\"reference_live\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"reference_live\") }>"
)]
fn reference_live(
    root: u8,
    claimed_service: ExternalPrincipal,
    claimed_tenant: ExternalPrincipal,
) -> Option<bool> {
    workflow::reference_live(
        context(),
        root,
        claimed_service.into(),
        claimed_tenant.into(),
    )
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"release\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"release\") }>"
)]
fn release(root: u8) -> bool {
    workflow::release(context(), root)
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"pending\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"pending\") }>"
)]
fn pending() -> Option<Vec<u8>> {
    workflow::pending(context())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"revoke_gateway\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"revoke_gateway\") }>"
)]
fn revoke_gateway() -> bool {
    workflow::revoke_gateway(context())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"sync_gateway\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"sync_gateway\") }>"
)]
async fn sync_gateway(input: blob_test_protocol::GatewaySyncRequest) -> Result<(), SyncFailure> {
    workflow::sync_gateway(inspection_context(), input).await
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"preview_gateway_sync\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"preview_gateway_sync\") }>"
)]
fn preview_gateway_sync(input: blob_test_protocol::GatewaySyncRequest) -> Result<(), SyncFailure> {
    workflow::preview_gateway_sync(inspection_context(), input)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"probe_content\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"probe_content\") }>"
)]
fn probe_content(case: ContentProbeCase) -> Result<ContentProbeReport, ContentProbeFailure> {
    workflow::probe_content(context(), case)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"journey_reserve\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_reserve\") }>"
)]
fn journey_reserve(input: JourneyReservation) -> Result<(), JourneyFailure> {
    workflow::journey::reserve(context(), &input)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::args::<_, { blob_test_protocol::fixture_wire::identity(\"journey_append\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_append\") }>"
)]
fn journey_append(
    root: Vec<u8>,
    index: u64,
    bytes: ic_wire::ByteBuf,
) -> Result<(), JourneyFailure> {
    workflow::journey::append(context(), &root, index, &bytes)
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"journey_progress\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_progress\") }>"
)]
fn journey_progress(root: Vec<u8>) -> Result<JourneyProgress, JourneyFailure> {
    workflow::journey::progress(context(), &root)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::args::<_, { blob_test_protocol::fixture_wire::identity(\"journey_read\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_read\") }>"
)]
async fn journey_read(
    root: Vec<u8>,
    index: u64,
) -> Result<blob_test_protocol::journey::readback::JourneyReadChunk, JourneyFailure> {
    workflow::journey::readback::read(context(), &root, index).await
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"journey_read_resources\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_read_resources\") }>"
)]
fn journey_read_resources()
-> Result<Option<blob_test_protocol::journey::readback::ReadExecutionProfile>, JourneyFailure> {
    workflow::journey::readback::resources(inspection_context())
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"journey_arm_read_callback_trap\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_arm_read_callback_trap\") }>"
)]
fn journey_arm_read_callback_trap(root: Vec<u8>) -> bool {
    workflow::journey::readback::arm_callback_trap(context(), &root)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"journey_cancel\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_cancel\") }>"
)]
fn journey_cancel(input: JourneyUpload) -> Result<(), JourneyFailure> {
    workflow::journey::cancel(context(), input)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"journey_release\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_release\") }>"
)]
fn journey_release(root: Vec<u8>) -> Result<(), JourneyFailure> {
    workflow::journey::release(context(), &root)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"journey_settle\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_settle\") }>"
)]
fn journey_settle(root: Vec<u8>) -> Result<(), JourneyFailure> {
    workflow::journey::settled(context(), &root)
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"journey_usage\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"journey_usage\") }>"
)]
fn journey_usage() -> Result<JourneyUsage, JourneyFailure> {
    workflow::journey::usage(context())
}

#[ic_platform::cdk::update(
    name = "_immutableObjectStorageCreateCertificate",
    decode_with = "blob_test_protocol::provider_candid::decode_one",
    encode_with = "blob_test_protocol::provider_candid::encode_one"
)]
fn create_certificate(root: String) -> CaffeineUploadCertificateResponse {
    workflow::journey::certificate(context(), &root).expect("certificate admission")
}

#[ic_platform::cdk::query(
    name = "_immutableObjectStorageBlobsAreLive",
    decode_with = "blob_test_protocol::provider_candid::decode_one",
    encode_with = "blob_test_protocol::provider_candid::encode_one"
)]
fn blobs_live(roots: Vec<Vec<u8>>) -> Vec<bool> {
    workflow::journey::live(context(), &roots).expect("gateway root observation")
}

#[ic_platform::cdk::query(
    name = "_immutableObjectStorageBlobsToDelete",
    decode_with = "blob_test_protocol::provider_candid::ignore_arguments",
    encode_with = "blob_test_protocol::provider_candid::encode_one"
)]
fn blobs_to_delete() -> Vec<Vec<u8>> {
    workflow::journey::pending(context()).expect("gateway deletion list")
}

#[ic_platform::cdk::update(
    name = "_immutableObjectStorageConfirmBlobDeletion",
    decode_with = "blob_test_protocol::provider_candid::decode_one",
    encode_with = "blob_test_protocol::provider_candid::encode_unit"
)]
fn confirm_deletion(roots: Vec<Vec<u8>>) {
    workflow::journey::deleted(context(), roots).expect("gateway deletion confirmation");
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"authority_archive\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"authority_archive\") }>"
)]
fn authority_archive() -> Option<blob_test_protocol::authority::AuthorityArchiveView> {
    workflow::archive(inspection_context())
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"operator_status\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"operator_status\") }>"
)]
fn operator_status() -> Option<blob_test_protocol::status::OperatorStatusView> {
    workflow::operator_status(inspection_context())
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"probe_checkpoint\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"probe_checkpoint\") }>"
)]
fn probe_checkpoint(
    input: blob_test_protocol::authority::CheckpointProbeInput,
) -> Result<JourneyProgress, blob_test_protocol::authority::CheckpointProbeFailure> {
    workflow::probe_checkpoint(inspection_context(), &input)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"configure_balance\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"configure_balance\") }>"
)]
fn configure_balance(
    scope: blob_test_protocol::balance::BalanceScope,
) -> Result<(), blob_test_protocol::balance::BalanceFailure> {
    workflow::balance::configure(inspection_context(), scope)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"refresh_balance\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"refresh_balance\") }>"
)]
async fn refresh_balance(
    input: blob_test_protocol::balance::BalanceRefreshRequest,
) -> Result<(), blob_test_protocol::balance::BalanceFailure> {
    workflow::balance::refresh(inspection_context(), input).await
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"preview_balance_refresh\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"preview_balance_refresh\") }>"
)]
fn preview_balance_refresh(
    input: blob_test_protocol::balance::BalanceRefreshRequest,
) -> Result<(), blob_test_protocol::balance::BalanceFailure> {
    workflow::balance::preview(inspection_context(), input)
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"configure_billing_limits\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"configure_billing_limits\") }>"
)]
fn configure_billing_limits(
    input: blob_test_protocol::billing::BillingLimitsInput,
) -> Result<(), blob_test_protocol::balance::BalanceFailure> {
    workflow::balance::configure_limits(inspection_context(), input)
}
