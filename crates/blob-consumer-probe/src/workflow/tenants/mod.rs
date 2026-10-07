//! Fixture intent commit precedes the shared one-call operator client.
use crate::ops;
use blob_test_protocol::consumer::{Failure, TenantDispatch};
use candid::Principal;
use ic_blob_storage::dto::tenant::{TenantEnrollmentResponse, TenantScope, TenantUpdateRequest};

pub(crate) async fn update(
    actor: Principal,
    input: TenantDispatch,
) -> Result<TenantEnrollmentResponse, Failure> {
    let max = ops::tenants::start(actor, input.command, input.max_reply_bytes)?;
    let result = ops::tenants::update(input.command, max).await?;
    if input.trap_after_reply {
        ic_cdk::trap("fixture tenant callback interruption");
    }
    Ok(result)
}
pub(crate) async fn inspect(
    actor: Principal,
    scope: TenantScope,
) -> Result<TenantEnrollmentResponse, Failure> {
    ops::tenants::inspect(actor, scope).await
}
pub(crate) fn saved(actor: Principal) -> Result<Option<TenantUpdateRequest>, Failure> {
    ops::tenants::saved(actor)
}
