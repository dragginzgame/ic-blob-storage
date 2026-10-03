//! Explicit standalone host, including IC-proven current-instance recovery.
#![expect(
    clippy::needless_pass_by_value,
    clippy::large_types_passed_by_value,
    reason = "Candid endpoint macros own decoded inputs"
)]
use crate::{ops, workflow};
use ic_blob_storage::dto::funding::assessment::{
    FundingPreparationFailure, FundingPreparationRequest, FundingPreparationResponse,
};
use ic_blob_storage::dto::funding::outcome::{
    FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse,
};
use ic_blob_storage::dto::funding::{
    FundingHistoryFailure, FundingHistoryPage, FundingHistoryRequest,
};
use ic_blob_storage::dto::operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope};
use ic_blob_storage::dto::upload::history::{
    UploadHistoryFailure, UploadHistoryPage, UploadHistoryRequest,
};
use ic_blob_storage::{
    dto::{
        configuration::{HostConfigurationView, HostFailure, ServiceInstallationInput},
        reference::{
            ReferenceCommand, ReferenceFailure, ReferenceMutationResponse, ReferenceReceiptLookup,
        },
        tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
        upload::{
            admission::{
                UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
                UploadAdmissionResponse, UploadRevocationResponse,
            },
            manifest::{
                UploadManifestFailure, UploadManifestMutation, UploadManifestRequest,
                UploadManifestResponse,
            },
        },
    },
    model::service::upload::UploadContext,
};
fn context() -> UploadContext {
    UploadContext {
        service: ic_cdk::api::canister_self(),
        actor: ic_cdk::api::msg_caller(),
    }
}
#[ic_cdk::init(hidden = true, decode_with = "ops::decode_configuration")]
// Custom decoding bounds raw Candid; explicitly retain its typed wire contract.
#[candid::candid_method(init)]
fn init(input: ServiceInstallationInput) {
    workflow::install(&input);
}
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    workflow::restore();
}
#[ic_cdk::update]
async fn blob_resume_current_instance()
-> Result<(), ic_blob_storage::dto::recovery::CurrentInstanceRecoveryFailure> {
    let outcome = workflow::resume_current_instance(context()).await;
    ops::observe_version();
    outcome
}
#[ic_cdk::query]
fn blob_configuration() -> Result<HostConfigurationView, HostFailure> {
    workflow::configuration(ic_cdk::api::msg_caller())
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_local_status(input: OperatorScope) -> Result<LocalServiceStatus, LocalStatusFailure> {
    workflow::local_status(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_update_tenant(
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    let call_context = context();
    workflow::before_update().await;
    workflow::update_tenant(call_context, input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_tenant(input: TenantScope) -> Result<TenantEnrollmentResponse, TenantFailure> {
    workflow::tenant(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_admit_upload(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    let call_context = context();
    workflow::before_update().await;
    workflow::admit(call_context, input, ic_cdk::api::time())
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_admission(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
    workflow::admission(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_status(
    input: ic_blob_storage::dto::reference::ReferenceUpload,
) -> Result<
    ic_blob_storage::dto::upload::UploadStatusResponse,
    ic_blob_storage::dto::upload::UploadStatusFailure,
> {
    workflow::upload_status(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_revoke_upload(
    input: UploadAdmissionRequest,
) -> Result<UploadRevocationResponse, UploadAdmissionFailure> {
    let call_context = context();
    workflow::before_update().await;
    workflow::revoke(call_context, input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode_manifest")]
#[candid::candid_method(update)]
async fn blob_prepare_upload(
    input: UploadManifestRequest,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    let call_context = context();
    workflow::before_update().await;
    workflow::prepare(call_context, &input, ic_cdk::api::time())
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_certificate_assessment(
    root: String,
) -> Result<
    ic_blob_storage::dto::upload::certificate::UploadCertificateAssessmentResponse,
    ic_blob_storage::dto::upload::exposure::UploadExposureFailure,
> {
    workflow::certificate_assessment(context(), &root, ic_cdk::api::time())
}
#[ic_cdk::update(
    name = "_immutableObjectStorageCreateCertificate",
    hidden = true,
    decode_with = "ops::decode"
)]
#[candid::candid_method(update, rename = "_immutableObjectStorageCreateCertificate")]
async fn caffeine_upload_certificate(
    root: String,
) -> ic_blob_storage::dto::upload::certificate::CaffeineUploadCertificateResponse {
    let call_context = context();
    workflow::before_update().await;
    // Never encode an error as a successful provider reply. Shared workflow
    // rechecks current authority/evidence and commits exposure synchronously.
    workflow::certificate(call_context, &root, ic_cdk::api::time())
        .unwrap_or_else(|_| ic_cdk::trap("certificate issuance refused"))
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_manifest(
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    workflow::manifest(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_apply_reference(
    input: ReferenceCommand,
) -> Result<ReferenceMutationResponse, ReferenceFailure> {
    let call_context = context();
    workflow::before_update().await;
    workflow::reference(call_context, input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_reference_receipt(
    input: ReferenceCommand,
) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
    workflow::receipt(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_history(
    input: UploadHistoryRequest,
) -> Result<UploadHistoryPage, UploadHistoryFailure> {
    workflow::history(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_funding_history(
    input: FundingHistoryRequest,
) -> Result<FundingHistoryPage, FundingHistoryFailure> {
    workflow::funding_history(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_funding_outcome(
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
    workflow::funding_outcome(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_funding_preparation_assessment(
    input: FundingPreparationRequest,
) -> Result<FundingPreparationResponse, FundingPreparationFailure> {
    workflow::funding_assessment(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_revoke_gateway(
    input: ic_blob_storage::dto::gateway::GatewayRevocationRequest,
) -> Result<
    ic_blob_storage::dto::gateway::GatewayRevocationResponse,
    ic_blob_storage::dto::gateway::GatewayRevocationFailure,
> {
    let call_context = context();
    workflow::before_update().await;
    workflow::revoke_gateway(call_context, input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_sync_gateways(
    input: OperatorScope,
) -> Result<
    ic_blob_storage::dto::gateway::sync::GatewaySyncResponse,
    ic_blob_storage::dto::gateway::sync::GatewaySyncFailure,
> {
    let call_context = context();
    workflow::before_update().await;
    let outcome = workflow::sync_gateways(call_context, input).await;
    ops::observe_version();
    outcome
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_cancel_gateway_sync(
    input: ic_blob_storage::dto::gateway::sync::GatewaySyncCancellation,
) -> Result<(), ic_blob_storage::dto::gateway::sync::GatewaySyncFailure> {
    let call_context = context();
    workflow::before_update().await;
    workflow::cancel_gateway_sync(call_context, input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_capacity(
    input: ic_blob_storage::dto::tenant::TenantScope,
) -> Result<
    ic_blob_storage::dto::upload::capacity::UploadCapacityResponse,
    ic_blob_storage::dto::upload::capacity::UploadCapacityFailure,
> {
    workflow::upload_capacity(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_reference_capacity(
    input: ic_blob_storage::dto::reference::capacity::ReferenceCapacityRequest,
) -> Result<
    ic_blob_storage::dto::reference::capacity::ReferenceCapacityResponse,
    ic_blob_storage::dto::reference::capacity::ReferenceCapacityFailure,
> {
    workflow::reference_capacity(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_lookup_content(
    input: ic_blob_storage::dto::upload::discovery::UploadDiscoveryRequest,
) -> Result<
    ic_blob_storage::dto::upload::discovery::UploadDiscoveryResponse,
    ic_blob_storage::dto::upload::discovery::UploadDiscoveryFailure,
> {
    workflow::discover(context(), input)
}

#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_download_descriptor(
    input: ic_blob_storage::dto::download::DownloadRequest,
) -> Result<
    ic_blob_storage::dto::download::DownloadResponse,
    ic_blob_storage::dto::download::DownloadFailure,
> {
    let call_context = context();
    workflow::before_update().await;
    workflow::download(call_context, input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_reference_status(
    input: ic_blob_storage::dto::reference::status::ReferenceStatusRequest,
) -> Result<
    ic_blob_storage::dto::reference::status::ReferenceStatusResponse,
    ic_blob_storage::dto::reference::ReferenceFailure,
> {
    workflow::reference_status(context(), input)
}

#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_inspect_account(
    input: ic_blob_storage::dto::account::AccountInspectionRequest,
) -> Result<
    ic_blob_storage::dto::account::AccountInspectionResponse,
    ic_blob_storage::dto::account::AccountInspectionFailure,
> {
    let call_context = context();
    workflow::before_update().await;
    let outcome = workflow::inspect_account(call_context, input).await;
    ops::observe_version();
    outcome
}

#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_attest_upload(
    input: ic_blob_storage::dto::upload::completion::UploadAttestationRequest,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadAttestationMutation,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    let call_context = context();
    workflow::before_update().await;
    workflow::attest(call_context, &input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_attestation(
    input: UploadAdmissionRequest,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadAttestationResponse,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    workflow::attestation(context(), input)
}

#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_verification_manifest(
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    workflow::verification_manifest(context(), input)
}

#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_verification_plan(
    input: UploadAdmissionRequest,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadVerificationPlan,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    workflow::verification_plan(context(), input)
}

ic_cdk::export_candid!();
pub(crate) fn interface() -> String {
    __export_service()
}
