//! Shared scoped enrollment handlers; hosts supply actual service and caller identities.
use crate::{
    dto::tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
    model::service::upload::UploadContext,
    ops::service::uploads::{StableUploads, tenants},
};
use ic_memory::ic_stable_structures::Memory;

/// Apply one exact operator precondition synchronously; suspension retains obligations.
/// Lost replies require inspection: the command is not an idempotent operation receipt.
/// Hosts must propagate stable write traps for IC transaction rollback.
/// # Errors
/// Rejects scope, authority, invalid preconditions, capacity, stale state or restoration.
pub fn update<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    tenants::authorize(store, context, input.scope, true)?;
    let command = tenants::parse(input)?;
    let view = store
        .update_tenant(context, command)
        .map_err(tenants::failure)?;
    Ok(tenants::present(input.scope, Some(view), store.is_fenced()))
}

/// Inspect as the tenant or configured operator, including during suspension/restore.
/// Absence does not grant tenant authority or prove safe retry of another operation.
/// # Errors
/// Rejects wrong scope, unrelated observers, invalid tenants or inconsistent state.
pub fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    scope: TenantScope,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    tenants::authorize(store, context, scope, false)?;
    let view = store
        .tenant(context, scope.tenant)
        .map_err(tenants::failure)?;
    Ok(tenants::present(scope, view, store.is_fenced()))
}
