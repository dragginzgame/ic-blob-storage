//! Local IC boundary for the transient shared admission owner. No provider effects.

mod ops;
mod workflow;

use blob_test_protocol::admission::{
    Command, ContentLookup, ExecutionProfile, Failure, Installation, Observation, Outcome,
    Permission, Request,
    input::{EnrollmentInput, PreparationInput, ReferenceInput},
    release::LifecycleCommand,
};
use ic_blob_storage_contracts::upload::binding::UploadContext;

fn context() -> UploadContext {
    UploadContext {
        service: ic_platform::cdk::api::canister_self(),
        actor: ic_platform::cdk::api::msg_caller(),
    }
}

#[ic_platform::cdk::init(decode_with = "ops::decode::installation")]
fn init(installation: Installation) {
    workflow::initialize(ic_platform::cdk::api::canister_self(), installation);
}

fn dispatch(command: Command) -> Result<Outcome, Failure> {
    workflow::execute(context(), command, ic_platform::cdk::api::time())
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn enroll(input: EnrollmentInput) -> Result<Outcome, Failure> {
    dispatch(Command::Enroll {
        tenant: input.tenant,
        expected: input.expected,
        active: input.active,
    })
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn admit(input: Permission) -> Result<Outcome, Failure> {
    dispatch(Command::Admit(input))
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn prepare(input: PreparationInput) -> Result<Outcome, Failure> {
    dispatch(Command::Prepare(input.request, input.manifest))
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn expose(root: [u8; 32]) -> Result<Outcome, Failure> {
    dispatch(Command::Expose(root))
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn revoke(input: Request) -> Result<Outcome, Failure> {
    dispatch(Command::Revoke(input))
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn substitute_completion(input: Request) -> Result<Outcome, Failure> {
    dispatch(Command::FixtureLifecycle(
        LifecycleCommand::SubstituteCompletion(input),
    ))
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn substitute_deletion(input: Request) -> Result<Outcome, Failure> {
    dispatch(Command::FixtureLifecycle(
        LifecycleCommand::SubstituteDeletion(input),
    ))
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn substitute_settlement(input: Request) -> Result<Outcome, Failure> {
    dispatch(Command::FixtureLifecycle(
        LifecycleCommand::SubstituteSettlement(input),
    ))
}

#[ic_platform::cdk::update(decode_with = "ops::decode::mutation")]
fn reference(input: ReferenceInput) -> Result<Outcome, Failure> {
    dispatch(Command::FixtureLifecycle(LifecycleCommand::Reference {
        object: input.object,
        reference: input.reference,
        operation: input.operation,
        retain: input.retain,
    }))
}

#[ic_platform::cdk::query(decode_with = "ops::decode::request")]
fn inspect(request: Request) -> Result<Observation, Failure> {
    workflow::inspect(context(), request)
}

#[ic_platform::cdk::query(manual_reply = true, hidden = true)]
fn blob_lookup_content() {
    let method = "blob_lookup_content";
    let input = ops::decode::blob(method);
    ops::reply_blob(method, workflow::discover(context(), input));
}

#[ic_platform::cdk::query(decode_with = "ops::decode::content")]
fn content_descriptor(
    input: ContentLookup,
) -> Result<Option<blob_test_protocol::admission::ContentDescriptor>, Failure> {
    workflow::content_descriptor(context(), input)
}

#[ic_platform::cdk::query]
fn resources(limit: u8) -> Result<Vec<ExecutionProfile>, Failure> {
    workflow::resources(context(), limit)
}

#[ic_platform::cdk::query(manual_reply = true, hidden = true)]
fn blob_upload_capacity() {
    let method = "blob_upload_capacity";
    let input = ops::decode::blob(method);
    ops::reply_blob(method, workflow::admission_capacity(context(), input));
}

#[ic_platform::cdk::query(decode_with = "ops::decode::retained_descriptor")]
fn retained_content_descriptor(
    input: blob_test_protocol::admission::input::RetainedDescriptorInput,
) -> Result<Option<blob_test_protocol::admission::input::RetainedDescriptor>, Failure> {
    workflow::retained_content_descriptor(context(), input)
}

#[ic_platform::cdk::query(manual_reply = true, hidden = true)]
fn blob_reference_capacity() {
    let method = "blob_reference_capacity";
    let input = ops::decode::blob(method);
    ops::reply_blob(method, workflow::reference_capacity(context(), input));
}

// This disposable probe has no stable schema. Both hooks reject so neither a
// normal upgrade nor skipping the outgoing hook can silently erase its owner.
// Controller-driven reinstall/snapshot replacement remains outside its contract.
#[ic_platform::cdk::pre_upgrade]
fn pre_upgrade() {
    ic_platform::cdk::api::trap("transient admission probe cannot upgrade");
}

#[ic_platform::cdk::post_upgrade]
fn post_upgrade() {
    ic_platform::cdk::api::trap("transient admission probe cannot restore");
}
