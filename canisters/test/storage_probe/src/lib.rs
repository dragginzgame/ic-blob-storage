//! Local IC evidence for durable service components and a Cashier substitute.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Candid endpoints own decoded inputs; the macro duplicates function-level expectations"
)]
mod ops;
mod workflow;
use blob_test_protocol::{
    admission::Request,
    storage::{Failure, FaultAdmission, FaultPreparation, Observation, Status},
};
use candid::Principal;
use ic_blob_storage::dto::operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope};
use ic_blob_storage::dto::tenant::{
    TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest,
};
use ic_blob_storage::dto::upload::admission::{
    UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
    UploadAdmissionResponse,
};
use ic_blob_storage::dto::upload::manifest::{
    UploadManifestFailure, UploadManifestMutation, UploadManifestRequest, UploadManifestResponse,
};
use ic_blob_storage::model::service::upload::UploadContext;
fn context() -> UploadContext {
    UploadContext {
        service: ic_cdk::api::canister_self(),
        actor: ic_cdk::api::msg_caller(),
    }
}
#[ic_cdk::init(decode_with = "ops::decode")]
fn init(operator: Principal) {
    workflow::initialize(operator, false);
}
#[ic_cdk::post_upgrade(decode_with = "ops::decode")]
fn post_upgrade(operator: Principal) {
    workflow::initialize(operator, true);
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn blob_admit_upload(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    workflow::admit(context(), input, None)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn admit_with_write_trap(
    input: FaultAdmission,
) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    workflow::admit(context(), input.permission, Some(input.fault))
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn blob_upload_admission(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
    workflow::uploads::admission(context(), input)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn blob_prepare_upload(
    input: UploadManifestRequest,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    workflow::prepare(context(), &input, None)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn prepare_with_write_trap(
    input: FaultPreparation,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    workflow::prepare(context(), &input.preparation, Some(input.fault))
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn blob_upload_manifest(
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    workflow::uploads::manifest(context(), input)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn expose(
    input: blob_test_protocol::storage::exposure::ExposureInput,
) -> Result<
    blob_test_protocol::storage::exposure::ExposureOutcome,
    ic_blob_storage::dto::upload::exposure::UploadExposureFailure,
> {
    workflow::expose(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn exposure_preview(
    input: blob_test_protocol::storage::exposure::ExposureInput,
) -> Result<
    Vec<blob_test_protocol::storage::exposure::ExposureBlocker>,
    ic_blob_storage::dto::upload::exposure::UploadExposureFailure,
> {
    workflow::exposure_preview(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn exposure_status(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, ic_blob_storage::dto::upload::exposure::UploadExposureFailure>
{
    workflow::exposure_status(context(), input)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn configure_certificate_fixture(input: blob_test_protocol::storage::exposure::ExposureInput) {
    workflow::configure_certificate(context(), input);
}
#[ic_cdk::update(
    name = "_immutableObjectStorageCreateCertificate",
    decode_with = "ops::decode"
)]
fn caffeine_upload_certificate(
    root: String,
) -> ic_blob_storage::dto::upload::certificate::CaffeineUploadCertificateResponse {
    workflow::certificate(context(), &root)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn blob_revoke_upload(
    input: UploadAdmissionRequest,
) -> Result<ic_blob_storage::dto::upload::admission::UploadRevocationResponse, UploadAdmissionFailure>
{
    workflow::revoke(context(), input, None)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn revoke_with_usage_write_trap(
    input: UploadAdmissionRequest,
) -> Result<ic_blob_storage::dto::upload::admission::UploadRevocationResponse, UploadAdmissionFailure>
{
    workflow::revoke(
        context(),
        input,
        Some(blob_test_protocol::storage::WriteFault::Usage),
    )
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn lookup(input: Request) -> Result<Observation, Failure> {
    workflow::lookup(context(), input)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn blob_update_tenant(
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    workflow::update_tenant(context(), input, None)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn fixture_update_tenant_with_write_trap(
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    workflow::update_tenant(
        context(),
        input,
        Some(blob_test_protocol::storage::WriteFault::Tenants),
    )
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn blob_tenant(scope: TenantScope) -> Result<TenantEnrollmentResponse, TenantFailure> {
    workflow::tenant(context(), scope)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn blob_local_status(input: OperatorScope) -> Result<LocalServiceStatus, LocalStatusFailure> {
    workflow::local_status(context(), input)
}
#[ic_cdk::query]
fn status() -> Result<Status, Failure> {
    workflow::status(context())
}

#[ic_cdk::update(decode_with = "ops::decode")]
fn fixture_provider_fact(input: blob_test_protocol::storage::FactInput) -> Result<bool, Failure> {
    workflow::fact(context(), input)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn blob_apply_reference(
    input: ic_blob_storage::dto::reference::ReferenceCommand,
) -> Result<
    ic_blob_storage::dto::reference::ReferenceMutationResponse,
    ic_blob_storage::dto::reference::ReferenceFailure,
> {
    workflow::references::apply(context(), input, None)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn fixture_apply_reference_with_write_trap(
    input: blob_test_protocol::storage::reference::ReferenceFaultInput,
) -> Result<
    ic_blob_storage::dto::reference::ReferenceMutationResponse,
    ic_blob_storage::dto::reference::ReferenceFailure,
> {
    workflow::references::apply(context(), input.request, Some(input.fault))
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn blob_reference_receipt(
    input: ic_blob_storage::dto::reference::ReferenceCommand,
) -> Result<
    ic_blob_storage::dto::reference::ReferenceReceiptLookup,
    ic_blob_storage::dto::reference::ReferenceFailure,
> {
    workflow::references::receipt(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn reference_is_live(
    input: blob_test_protocol::admission::input::ReferenceInput,
) -> Result<bool, Failure> {
    workflow::live(context(), input)
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn lookup_content(
    input: blob_test_protocol::admission::ContentLookup,
) -> Result<Option<blob_test_protocol::admission::ContentObservation>, Failure> {
    workflow::content(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn content_descriptor(
    input: blob_test_protocol::admission::ContentLookup,
) -> Result<Option<blob_test_protocol::admission::ContentDescriptor>, Failure> {
    workflow::declaration(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn retained_content_descriptor(
    input: blob_test_protocol::admission::input::RetainedDescriptorInput,
) -> Result<Option<blob_test_protocol::admission::input::RetainedDescriptor>, Failure> {
    workflow::retained(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn blob_upload_history(
    input: ic_blob_storage::dto::upload::history::UploadHistoryRequest,
) -> Result<
    ic_blob_storage::dto::upload::history::UploadHistoryPage,
    ic_blob_storage::dto::upload::history::UploadHistoryFailure,
> {
    workflow::scan(context(), input)
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn admission_capacity(
    input: blob_test_protocol::admission::planning::AdmissionCapacityInput,
) -> Result<blob_test_protocol::admission::planning::AdmissionCapacity, Failure> {
    workflow::admission_capacity(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn reference_capacity(
    input: blob_test_protocol::admission::ContentLookup,
) -> Result<Option<blob_test_protocol::admission::release::ReferenceCapacity>, Failure> {
    workflow::reference_capacity(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn observe_roots(
    input: blob_test_protocol::storage::read::RootBatchInput,
) -> Result<Vec<blob_test_protocol::storage::read::RootObservation>, Failure> {
    workflow::observe_roots(context(), &input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
fn fixture_funding(input: blob_test_protocol::storage::funding::Command) -> Result<bool, Failure> {
    workflow::funding(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn funding_lookup(
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<Option<blob_test_protocol::storage::funding::Phase>, Failure> {
    workflow::funding_lookup(context(), input)
}
#[ic_cdk::query]
fn funding_allocation() -> Result<blob_test_protocol::storage::funding::Allocation, Failure> {
    workflow::funding_allocation(context())
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn funding_request(
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<blob_test_protocol::storage::funding::Request, Failure> {
    workflow::funding_request(context(), input)
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn funding_history(
    input: blob_test_protocol::storage::funding::history::Input,
) -> Result<blob_test_protocol::storage::funding::history::Page, Failure> {
    workflow::funding_history(context(), input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_funding_transport(
    input: blob_test_protocol::storage::funding::transport::Input,
) -> Result<blob_test_protocol::storage::funding::transport::Observation, Failure> {
    workflow::funding_transport(context(), input).await
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_guarded_funding_dispatch(
    input: blob_test_protocol::storage::funding::transport::DispatchInput,
) -> Result<blob_test_protocol::storage::funding::transport::DispatchResult, Failure> {
    workflow::funding::dispatch::run(context(), input).await
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn funding_outcome(
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<Option<blob_test_protocol::storage::funding::outcome::Outcome>, Failure> {
    workflow::funding_outcome(context(), input)
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn funding_summary(
    input: blob_test_protocol::storage::funding::history::Scope,
) -> Result<blob_test_protocol::storage::funding::summary::Summary, Failure> {
    workflow::funding_summary(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn funding_preparation(
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<blob_test_protocol::storage::funding::admission::View, Failure> {
    workflow::funding::inspect(context(), input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
fn prepare_funding(
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<blob_test_protocol::storage::funding::admission::Preparation, Failure> {
    workflow::funding::prepare(context(), input)
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn funding_attempt(
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<blob_test_protocol::storage::funding::admission::View, Failure> {
    workflow::funding::inspect_attempt(context(), input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
fn mark_funding_attempt(
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<blob_test_protocol::storage::funding::admission::Attempt, Failure> {
    workflow::funding::mark_attempt(context(), input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
fn fixture_gateways(
    input: blob_test_protocol::storage::gateways::Command,
) -> Result<blob_test_protocol::storage::gateways::Outcome, Failure> {
    workflow::gateways(context(), input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_gateway_transport(
    input: blob_test_protocol::storage::gateways::TransportInput,
) -> Result<(), blob_test_protocol::storage::Failure> {
    workflow::gateways::transport::run(context(), input).await
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn fixture_gateway_roots(
    input: blob_test_protocol::storage::gateways::RootsInput,
) -> Result<
    Vec<blob_test_protocol::storage::gateways::RootView>,
    blob_test_protocol::storage::Failure,
> {
    workflow::gateways::callbacks::roots(context(), &input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_read_chunk(
    input: blob_test_protocol::storage::gateways::ReadSessionInput,
) -> Result<
    blob_test_protocol::journey::readback::JourneyReadChunk,
    blob_test_protocol::storage::Failure,
> {
    workflow::reads::run(context(), input).await
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn read_sessions() -> Result<
    blob_test_protocol::storage::gateways::ReadSessionsView,
    blob_test_protocol::storage::Failure,
> {
    ops::read::sessions::inspect(context())
}

#[ic_cdk::update(decode_with = "ops::decode")]
fn blob_download_descriptor(
    input: ic_blob_storage::dto::download::DownloadRequest,
) -> Result<
    ic_blob_storage::dto::download::DownloadResponse,
    ic_blob_storage::dto::download::DownloadFailure,
> {
    workflow::reads::download::describe(context(), input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_fetch_descriptor(
    input: blob_test_protocol::storage::read::DownloadClientInput,
) -> Result<
    ic_blob_storage::dto::download::DownloadResponse,
    blob_test_protocol::storage::read::DownloadProbeFailure,
> {
    workflow::reads::download::fetch(context(), input).await
}
#[ic_cdk::query(decode_with = "ops::decode")]
async fn fixture_nonreplicated_descriptor(
    input: blob_test_protocol::storage::read::DownloadClientInput,
) -> Result<
    ic_blob_storage::dto::download::DownloadResponse,
    blob_test_protocol::storage::read::DownloadProbeFailure,
> {
    workflow::reads::download::fetch(context(), input).await
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_replicated_gateway_transport(
    input: blob_test_protocol::storage::gateways::ReplicatedInput,
) -> Result<(), blob_test_protocol::storage::Failure> {
    workflow::gateways::replicated::run(context(), input).await
}

// Deliberately wrong execution context: the transport must refuse before calling.
#[ic_cdk::query(decode_with = "ops::decode")]
async fn fixture_nonreplicated_gateway_transport(
    input: blob_test_protocol::storage::gateways::ReplicatedInput,
) -> Result<(), blob_test_protocol::storage::Failure> {
    workflow::gateways::replicated::run(context(), input).await
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn gateway_registry(
    input: blob_test_protocol::storage::gateways::Scope,
) -> Result<blob_test_protocol::storage::gateways::View, Failure> {
    workflow::gateway_registry(context(), input)
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_fetch_reference_receipt(
    input: Box<blob_test_protocol::storage::reference::ReferenceClientInput>,
) -> Result<
    ic_blob_storage::dto::reference::ReferenceReceiptLookup,
    blob_test_protocol::storage::reference::ReferenceProbeFailure,
> {
    workflow::references::fetch(context(), &input).await
}
#[ic_cdk::query(decode_with = "ops::decode")]
async fn fixture_nonreplicated_reference_receipt(
    input: Box<blob_test_protocol::storage::reference::ReferenceClientInput>,
) -> Result<
    ic_blob_storage::dto::reference::ReferenceReceiptLookup,
    blob_test_protocol::storage::reference::ReferenceProbeFailure,
> {
    workflow::references::fetch(context(), &input).await
}

#[ic_cdk::update(decode_with = "ops::decode")]
async fn fixture_mutate_reference(
    input: Box<blob_test_protocol::storage::reference::ReferenceClientInput>,
) -> Result<
    ic_blob_storage::dto::reference::ReferenceMutationResponse,
    blob_test_protocol::storage::reference::ReferenceProbeFailure,
> {
    workflow::references::mutate(context(), &input).await
}
#[ic_cdk::query(decode_with = "ops::decode")]
async fn fixture_nonreplicated_reference_mutation(
    input: Box<blob_test_protocol::storage::reference::ReferenceClientInput>,
) -> Result<
    ic_blob_storage::dto::reference::ReferenceMutationResponse,
    blob_test_protocol::storage::reference::ReferenceProbeFailure,
> {
    workflow::references::mutate(context(), &input).await
}

#[ic_cdk::query(decode_with = "ops::decode")]
fn blob_upload_status(
    input: ic_blob_storage::dto::reference::ReferenceUpload,
) -> Result<
    ic_blob_storage::dto::upload::UploadStatusResponse,
    ic_blob_storage::dto::upload::UploadStatusFailure,
> {
    workflow::uploads::inspect(context(), input)
}
