//! Thin dispatch to the single shared owner, without a parallel permission model.

use crate::ops::qualification as ops;
use blob_test_protocol::admission::{
    Command, ExecutionProfile, Failure, Observation, Outcome, Request,
};
use ic_blob_storage_contracts::upload::binding::UploadContext;

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

pub(crate) fn resources(
    context: UploadContext,
    limit: u8,
) -> Result<Vec<ExecutionProfile>, Failure> {
    ops::resources::inspect(context, limit)
}
