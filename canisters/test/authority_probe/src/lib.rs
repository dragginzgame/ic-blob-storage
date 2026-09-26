//! Test-only IC authority probe. Sample objects are supplied facts, not uploads.
//! No stable state, deployed provider transport, credentials or production blob API.

mod ops;
mod workflow;

use blob_test_protocol::SyncFailure;
use blob_test_protocol::content::{ContentProbeCase, ContentProbeFailure, ContentProbeReport};
use candid::Principal;
use ic_blob_storage::policy::tenant::TenantAccessContext;

fn context() -> TenantAccessContext {
    TenantAccessContext {
        service: ic_cdk::api::canister_self(),
        actor: ic_cdk::api::msg_caller(),
    }
}

#[ic_cdk::init]
fn init(first: Principal, second: Principal, gateway: Principal, operator: Principal) {
    workflow::initialize(
        ic_cdk::api::canister_self(),
        first,
        second,
        gateway,
        operator,
    );
}

#[ic_cdk::query]
fn usage() -> Option<u128> {
    workflow::usage(context())
}

#[ic_cdk::query]
fn upload_usage() -> Option<u128> {
    workflow::upload_usage(context())
}

#[ic_cdk::query]
fn active_uploads() -> Option<Vec<u8>> {
    workflow::active_uploads(context())
}

#[ic_cdk::query]
fn upload_roots() -> Option<Vec<blob_test_protocol::uploads::UploadProbeState>> {
    workflow::upload_roots(context())
}

#[ic_cdk::update]
fn cancel_upload(root: u8) -> bool {
    workflow::cancel_upload(context(), root)
}

#[ic_cdk::query]
fn reference_live(root: u8, claimed_service: Principal, claimed_tenant: Principal) -> Option<bool> {
    workflow::reference_live(context(), root, claimed_service, claimed_tenant)
}

#[ic_cdk::update]
fn release(root: u8) -> bool {
    workflow::release(context(), root)
}

#[ic_cdk::query]
fn pending() -> Option<Vec<u8>> {
    workflow::pending(context())
}

#[ic_cdk::update]
fn revoke_gateway() -> bool {
    workflow::revoke_gateway(context())
}

#[ic_cdk::update]
async fn sync_gateway() -> Result<(), SyncFailure> {
    workflow::sync_gateway(context()).await
}

#[ic_cdk::update]
fn probe_content(case: ContentProbeCase) -> Result<ContentProbeReport, ContentProbeFailure> {
    workflow::probe_content(context(), case)
}
