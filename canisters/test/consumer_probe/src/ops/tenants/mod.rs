//! Actual replicated transport using the shared tenant client.
use blob_test_protocol::consumer::Failure;
use candid::Principal;
use ic_blob_storage::{
    dto::tenant::{TenantEnrollmentResponse, TenantScope, TenantUpdateRequest},
    ops::service::uploads::tenants::{client::ReplicatedTenantClient, reply},
};
use std::num::NonZeroUsize;

pub(crate) fn start(
    actor: Principal,
    input: TenantUpdateRequest,
    max: u32,
) -> Result<NonZeroUsize, Failure> {
    let max = NonZeroUsize::new(max as usize).ok_or(Failure::Invalid)?;
    super::read(actor, |r| r.check_tenant_scope(input.scope))?;
    reply::validate_update(input).map_err(|_| Failure::Invalid)?;
    super::mutate(actor, |r| r.start_tenant_command(input))?;
    Ok(max)
}
pub(crate) fn saved(actor: Principal) -> Result<Option<TenantUpdateRequest>, Failure> {
    super::read(actor, |r| Ok(r.tenant_command()))
}
fn client(scope: TenantScope) -> ReplicatedTenantClient {
    ReplicatedTenantClient::new(ic_cdk::api::canister_self(), scope, 30.try_into().unwrap())
        .unwrap()
}
pub(crate) async fn update(
    input: TenantUpdateRequest,
    max: NonZeroUsize,
) -> Result<TenantEnrollmentResponse, Failure> {
    client(input.scope)
        .update(input, max)
        .await
        .map_err(|_| Failure::Transport)
}
pub(crate) async fn inspect(
    actor: Principal,
    scope: TenantScope,
) -> Result<TenantEnrollmentResponse, Failure> {
    super::read(actor, |r| r.check_tenant_scope(scope))?;
    client(scope)
        .inspect(4096.try_into().unwrap())
        .await
        .map_err(|_| Failure::Transport)
}
