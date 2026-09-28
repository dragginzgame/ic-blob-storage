//! Local IC evidence for durable service components and a Cashier substitute.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Candid endpoints own decoded inputs; the macro duplicates function-level expectations"
)]
mod ops;
mod workflow;
use blob_test_protocol::{
    admission::{
        Enrollment, Permission, Request,
        input::{EnrollmentInput, PreparationInput},
    },
    storage::{Failure, FaultAdmission, FaultPreparation, Observation, Status},
};
use candid::Principal;
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
fn admit(input: Permission) -> Result<bool, Failure> {
    workflow::admit(context(), input, None)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn admit_with_write_trap(input: FaultAdmission) -> Result<bool, Failure> {
    workflow::admit(context(), input.permission, Some(input.fault))
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn prepare(input: PreparationInput) -> Result<bool, Failure> {
    workflow::prepare(context(), input, None)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn prepare_with_write_trap(input: FaultPreparation) -> Result<bool, Failure> {
    workflow::prepare(context(), input.preparation, Some(input.fault))
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn expose(input: Request) -> Result<(), Failure> {
    workflow::expose(context(), input)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn revoke(input: Request) -> Result<bool, Failure> {
    workflow::revoke(context(), input, None)
}
#[ic_cdk::update(decode_with = "ops::decode")]
fn revoke_with_usage_write_trap(input: Request) -> Result<bool, Failure> {
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
fn enroll(input: EnrollmentInput) -> Result<Enrollment, Failure> {
    workflow::enroll(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn tenant(tenant: Principal) -> Result<Option<Enrollment>, Failure> {
    workflow::tenant(context(), tenant)
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
fn apply_reference(
    input: blob_test_protocol::storage::ReferenceMutationInput,
) -> Result<blob_test_protocol::storage::ReferenceOutcome, Failure> {
    workflow::reference(context(), input)
}
#[ic_cdk::query(decode_with = "ops::decode")]
fn reference_receipt(
    input: blob_test_protocol::admission::input::ReferenceInput,
) -> Result<Option<blob_test_protocol::storage::ReferenceResult>, Failure> {
    workflow::receipt(context(), input)
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
fn scan_uploads(
    input: blob_test_protocol::storage::read::ScanInput,
) -> Result<blob_test_protocol::storage::read::Page, Failure> {
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
async fn fixture_read_authority(
    input: blob_test_protocol::storage::gateways::ReadAuthorityInput,
) -> Result<(), blob_test_protocol::storage::Failure> {
    workflow::reads::run(context(), input).await
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
