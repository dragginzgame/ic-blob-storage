//! Delegate to shared-owner operations without a second upload workflow.
pub(crate) mod funding;
pub(crate) mod gateways;
pub(crate) mod references;
pub(crate) fn configure_certificate(
    context: UploadContext,
    input: blob_test_protocol::storage::exposure::ExposureInput,
) {
    ops::exposure::configure_certificate(context, input);
}
pub(crate) fn certificate(
    context: UploadContext,
    root: &str,
) -> ic_blob_storage::dto::upload::certificate::CaffeineUploadCertificateResponse {
    ops::exposure::certificate(context, root)
}
pub(crate) fn gateways(
    context: UploadContext,
    input: blob_test_protocol::storage::gateways::Command,
) -> Result<blob_test_protocol::storage::gateways::Outcome, Failure> {
    gateways::apply(context, input)
}
pub(crate) fn gateway_registry(
    context: UploadContext,
    input: blob_test_protocol::storage::gateways::Scope,
) -> Result<blob_test_protocol::storage::gateways::View, Failure> {
    ops::gateways::inspect(context, input)
}
use crate::ops;
pub(crate) fn local_status(
    context: UploadContext,
    input: ic_blob_storage::dto::operator::OperatorScope,
) -> Result<
    ic_blob_storage::dto::operator::LocalServiceStatus,
    ic_blob_storage::dto::operator::LocalStatusFailure,
> {
    ops::with_operator_stores(|stores| {
        ic_blob_storage::workflow::operator::inspect(stores, context, input)
    })
}
use blob_test_protocol::{
    admission::Request,
    storage::{Failure, Observation, Status, WriteFault},
};
use candid::Principal;
use ic_blob_storage::dto::tenant::{
    TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest,
};
use ic_blob_storage::dto::upload::manifest::{
    UploadManifestFailure, UploadManifestMutation, UploadManifestRequest,
};
use ic_blob_storage::model::service::upload::UploadContext;
pub(crate) fn initialize(operator: Principal, restored: bool) {
    ops::initialize(operator, restored);
}
pub(crate) fn admit(
    context: UploadContext,
    input: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
    fault: Option<WriteFault>,
) -> Result<
    ic_blob_storage::dto::upload::admission::UploadAdmissionMutation,
    ic_blob_storage::dto::upload::admission::UploadAdmissionFailure,
> {
    ops::admit(context, input, fault)
}
pub(crate) fn prepare(
    context: UploadContext,
    input: &UploadManifestRequest,
    fault: Option<WriteFault>,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    ops::prepare(context, input, fault)
}
pub(crate) fn expose(
    context: UploadContext,
    input: blob_test_protocol::storage::exposure::ExposureInput,
) -> Result<
    blob_test_protocol::storage::exposure::ExposureOutcome,
    ic_blob_storage::dto::upload::exposure::UploadExposureFailure,
> {
    ops::exposure::commit(context, input)
}
pub(crate) fn exposure_preview(
    context: UploadContext,
    input: blob_test_protocol::storage::exposure::ExposureInput,
) -> Result<
    Vec<blob_test_protocol::storage::exposure::ExposureBlocker>,
    ic_blob_storage::dto::upload::exposure::UploadExposureFailure,
> {
    ops::exposure::preview(context, input)
}
pub(crate) fn exposure_status(
    context: UploadContext,
    input: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
) -> Result<
    ic_blob_storage::dto::upload::admission::UploadAdmissionResponse,
    ic_blob_storage::dto::upload::exposure::UploadExposureFailure,
> {
    ops::exposure::inspect(context, input)
}
pub(crate) fn revoke(
    context: UploadContext,
    input: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
    fault: Option<WriteFault>,
) -> Result<
    ic_blob_storage::dto::upload::admission::UploadRevocationResponse,
    ic_blob_storage::dto::upload::admission::UploadAdmissionFailure,
> {
    ops::revoke(context, input, fault)
}
pub(crate) fn lookup(context: UploadContext, input: Request) -> Result<Observation, Failure> {
    ops::lookup(context, input)
}
pub(crate) fn update_tenant(
    context: UploadContext,
    input: TenantUpdateRequest,
    fault: Option<WriteFault>,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    ops::update_tenant(context, input, fault)
}
pub(crate) fn tenant(
    context: UploadContext,
    scope: TenantScope,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    ops::tenant(context, scope)
}
pub(crate) fn status(context: UploadContext) -> Result<Status, Failure> {
    ops::status(context)
}

pub(crate) fn fact(
    context: UploadContext,
    input: blob_test_protocol::storage::FactInput,
) -> Result<bool, Failure> {
    ops::lifecycle::fact(context, input)
}
pub(crate) fn reference_status(
    context: UploadContext,
    input: ic_blob_storage::dto::reference::status::ReferenceStatusRequest,
) -> Result<
    ic_blob_storage::dto::reference::status::ReferenceStatusResponse,
    ic_blob_storage::dto::reference::ReferenceFailure,
> {
    ops::read::download::with_uploads(|uploads| {
        ic_blob_storage::workflow::references::status::inspect(uploads, context, input)
    })
}

pub(crate) fn discover(
    context: UploadContext,
    input: ic_blob_storage::dto::upload::discovery::UploadDiscoveryRequest,
) -> Result<
    ic_blob_storage::dto::upload::discovery::UploadDiscoveryResponse,
    ic_blob_storage::dto::upload::discovery::UploadDiscoveryFailure,
> {
    ops::read::download::with_uploads(|uploads| {
        ic_blob_storage::workflow::uploads::discovery::inspect(uploads, context, input)
    })
}
pub(crate) fn declaration(
    context: UploadContext,
    input: blob_test_protocol::admission::ContentLookup,
) -> Result<Option<blob_test_protocol::admission::ContentDescriptor>, Failure> {
    ops::read::declaration(context, input)
}
pub(crate) fn retained(
    context: UploadContext,
    input: blob_test_protocol::admission::input::RetainedDescriptorInput,
) -> Result<Option<blob_test_protocol::admission::input::RetainedDescriptor>, Failure> {
    ops::read::retained(context, input)
}
pub(crate) fn scan(
    context: UploadContext,
    input: ic_blob_storage::dto::upload::history::UploadHistoryRequest,
) -> Result<
    ic_blob_storage::dto::upload::history::UploadHistoryPage,
    ic_blob_storage::dto::upload::history::UploadHistoryFailure,
> {
    use ic_blob_storage::model::catalog::admission::read::UploadPageLimits;
    use std::num::NonZeroUsize;
    ops::with_operator_stores(|stores| {
        ic_blob_storage::workflow::uploads::history::inspect(
            stores.uploads,
            context,
            input,
            UploadPageLimits {
                max_scan: NonZeroUsize::MIN,
                max_results: NonZeroUsize::MIN,
            },
        )
    })
}

pub(crate) fn admission_capacity(
    context: UploadContext,
    input: ic_blob_storage::dto::tenant::TenantScope,
) -> Result<
    ic_blob_storage::dto::upload::capacity::UploadCapacityResponse,
    ic_blob_storage::dto::upload::capacity::UploadCapacityFailure,
> {
    ops::read::download::with_uploads(|uploads| {
        ic_blob_storage::workflow::uploads::capacity::inspect(uploads, context, input)
    })
}
pub(crate) fn reference_capacity(
    context: UploadContext,
    input: ic_blob_storage::dto::reference::capacity::ReferenceCapacityRequest,
) -> Result<
    ic_blob_storage::dto::reference::capacity::ReferenceCapacityResponse,
    ic_blob_storage::dto::reference::capacity::ReferenceCapacityFailure,
> {
    ops::read::download::with_uploads(|uploads| {
        ic_blob_storage::workflow::references::capacity::inspect(uploads, context, input)
    })
}
pub(crate) fn observe_roots(
    context: UploadContext,
    input: &blob_test_protocol::storage::read::RootBatchInput,
) -> Result<Vec<blob_test_protocol::storage::read::RootObservation>, Failure> {
    ops::planning::roots(context, input)
}

pub(crate) fn funding(
    execution: UploadContext,
    input: blob_test_protocol::storage::funding::Command,
) -> Result<bool, Failure> {
    ops::funding::apply(execution, input)
}
pub(crate) fn funding_lookup(
    execution: UploadContext,
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<Option<blob_test_protocol::storage::funding::Phase>, Failure> {
    ops::funding::lookup(execution, input)
}
pub(crate) fn funding_allocation(
    execution: UploadContext,
) -> Result<blob_test_protocol::storage::funding::Allocation, Failure> {
    ops::funding::allocation(execution)
}

pub(crate) fn funding_request(
    execution: UploadContext,
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<blob_test_protocol::storage::funding::Request, Failure> {
    ops::funding::request(execution, input)
}

pub(crate) fn funding_history(
    execution: UploadContext,
    input: ic_blob_storage::dto::funding::FundingHistoryRequest,
) -> Result<
    ic_blob_storage::dto::funding::FundingHistoryPage,
    ic_blob_storage::dto::funding::FundingHistoryFailure,
> {
    ops::with_operator_stores(|stores| {
        ic_blob_storage::workflow::funding::history::inspect(
            stores.funding,
            execution,
            input,
            std::num::NonZeroUsize::new(2).expect("fixture page bound"),
        )
    })
}

pub(crate) async fn funding_transport(
    execution: UploadContext,
    input: blob_test_protocol::storage::funding::transport::Input,
) -> Result<blob_test_protocol::storage::funding::transport::Observation, Failure> {
    use ic_blob_storage::policy::billing::liquidity::{
        FundingLiquidityDecision, assess_funding_liquidity,
    };
    let reserve = std::num::NonZeroU128::new(input.operating_reserve).ok_or(Failure::Invalid)?;
    let (original, prepared) = ops::funding::transport::begin(execution, input)?;
    let liquidity = prepared.liquidity(reserve, input.other_liabilities);
    let call = match assess_funding_liquidity(prepared.offered(), liquidity) {
        FundingLiquidityDecision::Fits => prepared.execute(ops::funding::transport::limits()).await,
        FundingLiquidityDecision::Insufficient { .. } => prepared.cancel(),
    };
    ops::funding::transport::complete(execution, original, call, input, liquidity.call_cost)
}

pub(crate) fn funding_outcome(
    execution: UploadContext,
    input: ic_blob_storage::dto::funding::outcome::FundingOutcomeRequest,
) -> Result<
    Option<ic_blob_storage::dto::funding::outcome::FundingOutcomeResponse>,
    ic_blob_storage::dto::funding::outcome::FundingOutcomeFailure,
> {
    ops::with_operator_stores(|stores| {
        ic_blob_storage::workflow::funding::outcome::inspect(stores.funding, execution, input)
    })
}

pub(crate) fn funding_summary(
    execution: UploadContext,
    input: ic_blob_storage::dto::operator::OperatorScope,
) -> Result<blob_test_protocol::storage::funding::summary::Summary, Failure> {
    let view = ops::funding::summary::read(execution, input)?;
    let activity = ic_blob_storage::policy::billing::reconciliation::assess_uncredited_allocation(
        view.allocation,
    );
    Ok(ops::funding::summary::present(view, activity))
}
pub(crate) mod reads;
pub(crate) mod uploads;

pub(crate) fn attest(
    context: ic_blob_storage::model::service::upload::UploadContext,
    input: &ic_blob_storage::dto::upload::completion::UploadAttestationRequest,
    fault: Option<blob_test_protocol::storage::WriteFault>,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadAttestationMutation,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    ops::lifecycle::attest(context, input, fault)
}
pub(crate) fn attestation(
    context: ic_blob_storage::model::service::upload::UploadContext,
    input: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadAttestationResponse,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    ops::lifecycle::attestation(context, input)
}
pub(crate) fn verification_plan(
    context: ic_blob_storage::model::service::upload::UploadContext,
    input: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadVerificationPlan,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    ops::lifecycle::verification_plan(context, input)
}
