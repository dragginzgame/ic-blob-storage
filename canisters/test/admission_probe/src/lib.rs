//! Local IC boundary for the transient shared admission owner. No provider effects.

mod ops;
mod workflow;

use blob_test_protocol::admission::{
    Command, ContentLookup, ContentObservation, ExecutionProfile, Failure, Installation,
    Observation, Outcome, Request,
};
use ic_blob_storage::model::service::upload::UploadContext;

fn context() -> UploadContext {
    UploadContext {
        service: ic_cdk::api::canister_self(),
        actor: ic_cdk::api::msg_caller(),
    }
}

#[ic_cdk::init(decode_with = "ops::decode::installation")]
fn init(installation: Installation) {
    workflow::initialize(ic_cdk::api::canister_self(), installation);
}

#[ic_cdk::update(decode_with = "ops::decode::command")]
fn execute(command: Command) -> Result<Outcome, Failure> {
    workflow::execute(context(), command, ic_cdk::api::time())
}

#[ic_cdk::query(decode_with = "ops::decode::request")]
fn inspect(request: Request) -> Result<Observation, Failure> {
    workflow::inspect(context(), request)
}

#[ic_cdk::query(decode_with = "ops::decode::content")]
fn lookup_content(input: ContentLookup) -> Result<Option<ContentObservation>, Failure> {
    workflow::lookup_content(context(), input)
}

#[ic_cdk::query]
fn resources() -> Result<Option<ExecutionProfile>, Failure> {
    workflow::resources(context())
}

#[ic_cdk::query(decode_with = "ops::decode::content")]
fn reference_capacity(
    input: ContentLookup,
) -> Result<Option<blob_test_protocol::admission::release::ReferenceCapacity>, Failure> {
    workflow::reference_capacity(context(), input)
}

// This disposable probe has no stable schema. Both hooks reject so neither a
// normal upgrade nor skipping the outgoing hook can silently erase its owner.
// Controller-driven reinstall/snapshot replacement remains outside its contract.
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::trap("transient admission probe cannot upgrade");
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_cdk::trap("transient admission probe cannot restore");
}
