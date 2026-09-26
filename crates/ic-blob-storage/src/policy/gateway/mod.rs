//! Pure gateway membership and object-scope checks over trusted local values.

use candid::Principal;
use thiserror::Error;

use crate::model::{gateway::registry::GatewayRegistry, lifecycle::binding::ObjectBinding};

/// Actual execution context supplied by the authenticated endpoint adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GatewayCallbackContext {
    /// Running service instance, not the request's claimed service.
    pub service: Principal,
    /// Authenticated caller, not a principal from callback payload data.
    pub actor: Principal,
}

/// A callback fails local service, namespace or current membership checks.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GatewayAccessError {
    /// Registry, object or actual execution context belongs to a different service.
    #[error("wrong callback service")]
    WrongService,
    /// Registry does not own the object's configured provider namespace.
    #[error("wrong callback namespace")]
    WrongNamespace,
    /// Caller is absent from the current gateway membership.
    #[error("caller is not a current gateway")]
    NotGateway,
}

/// Check the current registry for a callback targeting a trusted object binding.
///
/// Obtain the registry and binding from authoritative state. Assess this again
/// at mutation time after any await; cached membership is not revocation-safe.
/// Success does not establish provider deletion, exact operation/root association,
/// billing settlement or restore safety. Those remain separate workflow checks.
/// Controller status and tenant identity provide no override.
/// # Errors
/// Rejects mismatched service/namespace and callers outside current membership.
pub fn assess_gateway_callback(
    object: ObjectBinding,
    registry: &GatewayRegistry,
    context: GatewayCallbackContext,
) -> Result<(), GatewayAccessError> {
    if context.service != object.service() {
        return Err(GatewayAccessError::WrongService);
    }
    if registry.scope().service() != object.service() {
        return Err(GatewayAccessError::WrongService);
    }
    if registry.scope().namespace() != object.identity().namespace {
        return Err(GatewayAccessError::WrongNamespace);
    }
    if !registry.gateways().contains(context.actor) {
        return Err(GatewayAccessError::NotGateway);
    }
    Ok(())
}
