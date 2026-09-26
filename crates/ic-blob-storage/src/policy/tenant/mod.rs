//! Pure direct-tenant access checks, independent of endpoint transport.
//!
//! The expected binding must come from trusted object/configuration state. The
//! service and actor context must come from the running instance and authenticated
//! caller, never fields copied from an untrusted request. Delegations, per-user
//! roles and controller privileges are not interpreted by this rule.

use candid::Principal;
use thiserror::Error;

use crate::model::lifecycle::binding::ObjectBinding;

/// Independently supplied execution context, not a wire request or authority token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TenantAccessContext {
    /// Actual service instance handling the operation.
    pub service: Principal,
    /// Authenticated caller, rather than a claimed tenant in request data.
    pub actor: Principal,
}

/// A direct-tenant request does not match its trusted object binding.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TenantAccessError {
    /// The request is being evaluated in another service instance.
    #[error("wrong service instance")]
    WrongService,
    /// The authenticated caller is not the bound tenant.
    #[error("caller is not the tenant")]
    NotTenant,
}

/// Assess a call made directly by the object's tenant principal.
///
/// Success is only this actor/binding predicate, not upload or effect admission.
/// Recovery fences, quota, reference/payload binding and effect evidence remain
/// separate requirements. No controller or hash-based bypass is provided.
/// # Errors
/// Rejects wrong-service and non-tenant actors, including anonymous callers.
pub fn assess_tenant_access(
    object: ObjectBinding,
    context: TenantAccessContext,
) -> Result<(), TenantAccessError> {
    if context.service != object.service() {
        return Err(TenantAccessError::WrongService);
    }
    if context.actor != object.tenant() {
        return Err(TenantAccessError::NotTenant);
    }
    Ok(())
}
