//! Thin dispatch to the single shared owner, without a parallel permission model.

use crate::ops;
use blob_test_protocol::admission::{
    Command, ContentLookup, ExecutionProfile, Failure, Installation, Observation, Outcome, Request,
};
use candid::Principal;
use ic_blob_storage_contracts::upload::binding::UploadContext;

pub(crate) fn initialize(service: Principal, installation: Installation) {
    ops::initialize(service, installation);
}

pub(crate) fn execute(
    context: UploadContext,
    command: Command,
    now: u64,
) -> Result<Outcome, Failure> {
    let before = ops::resources::instructions();
    let result = match command {
        Command::FixtureLifecycle(command) => ops::release::execute(context, command),
        Command::Enroll {
            tenant,
            expected,
            active,
        } => ops::enroll(context, tenant, expected, active),
        Command::Admit(input) => ops::admit(context, input, now),
        Command::Prepare(request, manifest) => ops::prepare(context, request, &manifest, now),
        Command::Expose(root) => ops::expose(context, root, now),
        Command::Revoke(request) => ops::revoke(context, request),
    };
    ops::resources::record(context, before);
    result
}

pub(crate) fn inspect(context: UploadContext, request: Request) -> Result<Observation, Failure> {
    ops::inspect(context, request)
}

pub(crate) fn discover(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest,
) -> Result<
    ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse,
    ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure,
> {
    ops::content::discover(context, input)
}

pub(crate) fn resources(
    context: UploadContext,
    limit: u8,
) -> Result<Vec<ExecutionProfile>, Failure> {
    ops::resources::inspect(context, limit)
}

pub(crate) fn admission_capacity(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::tenant::TenantScope,
) -> Result<
    ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse,
    ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure,
> {
    ops::planning::capacity(context, input)
}

pub(crate) fn content_descriptor(
    context: UploadContext,
    input: ContentLookup,
) -> Result<Option<blob_test_protocol::admission::ContentDescriptor>, Failure> {
    ops::content::descriptor(context, input)
}

pub(crate) fn reference_capacity(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest,
) -> Result<
    ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse,
    ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure,
> {
    ops::release::capacity(context, input)
}

pub(crate) fn retained_content_descriptor(
    context: UploadContext,
    input: blob_test_protocol::admission::input::RetainedDescriptorInput,
) -> Result<Option<blob_test_protocol::admission::input::RetainedDescriptor>, Failure> {
    ops::content::retained_descriptor(context, input)
}
