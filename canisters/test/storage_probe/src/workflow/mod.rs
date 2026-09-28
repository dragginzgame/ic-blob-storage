//! Delegate to shared-owner operations without a second upload workflow.
pub(crate) mod funding;
pub(crate) mod gateways;
pub(crate) mod references;
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
use blob_test_protocol::{
    admission::{
        Enrollment, Request,
        input::{EnrollmentInput, PreparationInput},
    },
    storage::{Failure, Observation, Status, WriteFault},
};
use candid::Principal;
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
    input: PreparationInput,
    fault: Option<WriteFault>,
) -> Result<bool, Failure> {
    ops::prepare(context, input, fault)
}
pub(crate) fn expose(context: UploadContext, input: Request) -> Result<(), Failure> {
    ops::expose(context, input)
}
pub(crate) fn revoke(
    context: UploadContext,
    input: Request,
    fault: Option<WriteFault>,
) -> Result<bool, Failure> {
    ops::revoke(context, input, fault)
}
pub(crate) fn lookup(context: UploadContext, input: Request) -> Result<Observation, Failure> {
    ops::lookup(context, input)
}
pub(crate) fn enroll(
    context: UploadContext,
    input: EnrollmentInput,
) -> Result<Enrollment, Failure> {
    ops::enroll(context, input)
}
pub(crate) fn tenant(
    context: UploadContext,
    tenant: Principal,
) -> Result<Option<Enrollment>, Failure> {
    ops::tenant(context, tenant)
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
pub(crate) fn live(
    context: UploadContext,
    input: blob_test_protocol::admission::input::ReferenceInput,
) -> Result<bool, Failure> {
    ops::lifecycle::live(context, input)
}

pub(crate) fn content(
    context: UploadContext,
    input: blob_test_protocol::admission::ContentLookup,
) -> Result<Option<blob_test_protocol::admission::ContentObservation>, Failure> {
    ops::read::content(context, input)
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
    input: blob_test_protocol::storage::read::ScanInput,
) -> Result<blob_test_protocol::storage::read::Page, Failure> {
    ops::read::scan(context, input)
}

pub(crate) fn admission_capacity(
    context: UploadContext,
    input: blob_test_protocol::admission::planning::AdmissionCapacityInput,
) -> Result<blob_test_protocol::admission::planning::AdmissionCapacity, Failure> {
    ops::planning::admission(context, input)
}
pub(crate) fn reference_capacity(
    context: UploadContext,
    input: blob_test_protocol::admission::ContentLookup,
) -> Result<Option<blob_test_protocol::admission::release::ReferenceCapacity>, Failure> {
    ops::planning::reference(context, input)
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
    input: blob_test_protocol::storage::funding::history::Input,
) -> Result<blob_test_protocol::storage::funding::history::Page, Failure> {
    ops::funding::history::read(execution, input)
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
    input: blob_test_protocol::storage::funding::Intent,
) -> Result<Option<blob_test_protocol::storage::funding::outcome::Outcome>, Failure> {
    let view = ops::funding::outcome::read(execution, input)?;
    Ok(view.map(|view| {
        let reconciliation =
            ic_blob_storage::policy::billing::reconciliation::assess_funding_reconciliation(
                view.transfer,
            );
        ops::funding::outcome::present(input, &view, reconciliation)
    }))
}

pub(crate) fn funding_summary(
    execution: UploadContext,
    input: blob_test_protocol::storage::funding::history::Scope,
) -> Result<blob_test_protocol::storage::funding::summary::Summary, Failure> {
    let view = ops::funding::summary::read(execution, input)?;
    let activity = ic_blob_storage::policy::billing::reconciliation::assess_uncredited_allocation(
        view.allocation,
    );
    Ok(ops::funding::summary::present(view, activity))
}
pub(crate) mod reads;
pub(crate) mod uploads;
