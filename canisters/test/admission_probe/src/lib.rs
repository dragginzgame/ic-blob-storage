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

#[ic_platform::cdk::update(
    decode_with = "ops::decode::mutation::<_, { blob_test_protocol::fixture_wire::identity(\"enroll\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"enroll\") }>"
)]
fn enroll(input: EnrollmentInput) -> Result<Outcome, Failure> {
    dispatch(Command::Enroll {
        tenant: input.tenant.into(),
        expected: input.expected,
        active: input.active,
    })
}

#[ic_platform::cdk::update(
    decode_with = "ops::decode::mutation::<_, { blob_test_protocol::fixture_wire::identity(\"admit\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"admit\") }>"
)]
fn admit(input: Permission) -> Result<Outcome, Failure> {
    dispatch(Command::Admit(input))
}

#[ic_platform::cdk::update(
    decode_with = "ops::decode::mutation::<_, { blob_test_protocol::fixture_wire::identity(\"prepare\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"prepare\") }>"
)]
fn prepare(input: PreparationInput) -> Result<Outcome, Failure> {
    dispatch(Command::Prepare(input.request, input.manifest))
}

#[ic_platform::cdk::update(
    decode_with = "ops::decode::mutation::<_, { blob_test_protocol::fixture_wire::identity(\"expose\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"expose\") }>"
)]
fn expose(root: [u8; 32]) -> Result<Outcome, Failure> {
    dispatch(Command::Expose(root))
}

#[ic_platform::cdk::update(
    decode_with = "ops::decode::mutation::<_, { blob_test_protocol::fixture_wire::identity(\"revoke\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"revoke\") }>"
)]
fn revoke(input: Request) -> Result<Outcome, Failure> {
    dispatch(Command::Revoke(input))
}

#[ic_platform::cdk::update(
    decode_with = "ops::decode::mutation::<_, { blob_test_protocol::fixture_wire::identity(\"substitute_deletion\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"substitute_deletion\") }>"
)]
fn substitute_deletion(input: Request) -> Result<Outcome, Failure> {
    dispatch(Command::FixtureLifecycle(
        LifecycleCommand::SubstituteDeletion(input),
    ))
}

#[ic_platform::cdk::update(
    decode_with = "ops::decode::mutation::<_, { blob_test_protocol::fixture_wire::identity(\"substitute_settlement\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"substitute_settlement\") }>"
)]
fn substitute_settlement(input: Request) -> Result<Outcome, Failure> {
    dispatch(Command::FixtureLifecycle(
        LifecycleCommand::SubstituteSettlement(input),
    ))
}

#[ic_platform::cdk::update(
    decode_with = "ops::decode::mutation::<_, { blob_test_protocol::fixture_wire::identity(\"reference\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"reference\") }>"
)]
fn reference(input: ReferenceInput) -> Result<Outcome, Failure> {
    dispatch(Command::FixtureLifecycle(LifecycleCommand::Reference {
        object: input.object,
        reference: input.reference,
        operation: input.operation,
        retain: input.retain,
    }))
}

#[ic_platform::cdk::query(
    decode_with = "ops::decode::request::<{ blob_test_protocol::fixture_wire::identity(\"inspect\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"inspect\") }>"
)]
fn inspect(request: Request) -> Result<Observation, Failure> {
    workflow::inspect(context(), request)
}

#[ic_platform::cdk::query(
    manual_reply = true,
    guard = "ops::decode::admit_discovery",
    decode_with = "std::convert::identity"
)]
fn blob_lookup_content(bytes: Vec<u8>) {
    let method = "blob_lookup_content";
    let input = ops::decode::blob(method, &bytes);
    drop(bytes);
    ops::reply_blob(method, workflow::discover(context(), input));
}

#[ic_platform::cdk::query(
    decode_with = "ops::decode::content::<{ blob_test_protocol::fixture_wire::identity(\"content_descriptor\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"content_descriptor\") }>"
)]
fn content_descriptor(
    input: ContentLookup,
) -> Result<Option<blob_test_protocol::admission::ContentDescriptor>, Failure> {
    workflow::content_descriptor(context(), input)
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"resources\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"resources\") }>"
)]
fn resources(limit: u8) -> Result<Vec<ExecutionProfile>, Failure> {
    workflow::resources(context(), limit)
}

#[ic_platform::cdk::query(
    manual_reply = true,
    guard = "ops::decode::admit_upload_capacity",
    decode_with = "std::convert::identity"
)]
fn blob_upload_capacity(bytes: Vec<u8>) {
    let method = "blob_upload_capacity";
    let input = ops::decode::blob(method, &bytes);
    drop(bytes);
    ops::reply_blob(method, workflow::admission_capacity(context(), input));
}

#[ic_platform::cdk::query(
    decode_with = "ops::decode::retained_descriptor::<{ blob_test_protocol::fixture_wire::identity(\"retained_content_descriptor\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"retained_content_descriptor\") }>"
)]
fn retained_content_descriptor(
    input: blob_test_protocol::admission::input::RetainedDescriptorInput,
) -> Result<Option<blob_test_protocol::admission::input::RetainedDescriptor>, Failure> {
    workflow::retained_content_descriptor(context(), input)
}

#[ic_platform::cdk::query(
    manual_reply = true,
    guard = "ops::decode::admit_reference_capacity",
    decode_with = "std::convert::identity"
)]
fn blob_reference_capacity(bytes: Vec<u8>) {
    let method = "blob_reference_capacity";
    let input = ops::decode::blob(method, &bytes);
    drop(bytes);
    ops::reply_blob(method, workflow::reference_capacity(context(), input));
}

// This disposable probe has no stable schema. Both hooks reject so neither a
// normal upgrade nor skipping the outgoing hook can silently erase its owner.
// Controller-driven reinstall/snapshot replacement remains outside its contract.
#[ic_platform::cdk::pre_upgrade]
fn pre_upgrade() {
    ic_platform::cdk::api::trap("transient admission probe cannot upgrade");
}

#[ic_platform::cdk::post_upgrade(
    decode_with = "blob_test_protocol::fixture_wire::hooks::upgrade_unit"
)]
fn post_upgrade() {
    ic_platform::cdk::api::trap("transient admission probe cannot restore");
}
