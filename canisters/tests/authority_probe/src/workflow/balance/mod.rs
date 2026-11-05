//! Operator-authorized, explicitly requested local balance observation.
use crate::ops;
use blob_test_protocol::balance::{BalanceFailure, BalanceRefreshRequest, BalanceScope};
use ic_blob_storage::policy::tenant::TenantAccessContext;

pub(crate) fn configure(
    context: TenantAccessContext,
    scope: BalanceScope,
) -> Result<(), BalanceFailure> {
    ops::balance::authorize(context.service, context.actor)?;
    ops::balance::configure(scope)
}

pub(crate) fn preview(
    context: TenantAccessContext,
    input: BalanceRefreshRequest,
) -> Result<(), BalanceFailure> {
    ops::balance::authorize(context.service, context.actor)?;
    ops::balance::preview(context.service, context.actor, input)
}

pub(crate) async fn refresh(
    context: TenantAccessContext,
    input: BalanceRefreshRequest,
) -> Result<(), BalanceFailure> {
    ops::balance::authorize(context.service, context.actor)?;
    let (id, scope) = ops::balance::begin(input)?;
    let response = ops::balance::fetch(scope).await;
    ops::balance::complete(id, scope, response)
}

pub(crate) fn configure_limits(
    context: TenantAccessContext,
    input: blob_test_protocol::billing::BillingLimitsInput,
) -> Result<(), BalanceFailure> {
    ops::balance::authorize(context.service, context.actor)?;
    ops::balance::configure_limits(input)
}
