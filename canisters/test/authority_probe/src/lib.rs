//! Test-only IC authority probe. Sample objects are supplied facts, not uploads.
//! No stable state, deployed provider transport, credentials or production blob API.

mod ops;
mod workflow;

use blob_test_protocol::SyncFailure;
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
