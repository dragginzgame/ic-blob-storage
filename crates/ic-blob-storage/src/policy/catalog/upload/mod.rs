//! Authorized current views including pending upload capacity and root history.
//! No provider wire mapping, effect scheduling, persistence or recovery occurs here.

use crate::{
    model::{
        catalog::admission::{
            UploadCatalog, UploadUsage,
            read::{
                ActiveUploadCursor, ActiveUploadPage, UploadCursorError, UploadPageLimits,
                UploadRootState,
            },
        },
        gateway::registry::GatewayRegistry,
        identity::{HashParseError, batch::ProviderRootBatch},
    },
    policy::{
        gateway::{GatewayAccessError, GatewayCallbackContext},
        tenant::{TenantAccessContext, TenantAccessError},
    },
};
use thiserror::Error;

/// Read charged capacity for the authenticated direct tenant, including reservations.
/// # Errors
/// Rejects wrong service and anonymous/management actors, including empty histories.
pub fn assess_tenant_upload_usage(
    catalog: &UploadCatalog,
    context: TenantAccessContext,
) -> Result<UploadUsage, TenantAccessError> {
    super::check_tenant(catalog.confirmed(), context)?;
    Ok(catalog.tenant_usage(context.actor))
}

/// List current active uploads for the actual tenant caller across its namespaces.
///
/// Recheck execution context for every page. A cursor selects position only; no
/// request field can select another tenant. Limits must be trusted configuration.
/// The returned requests are observations, not permission to repeat provider work.
/// # Errors
/// Rejects caller/service context before checking cursor service/tenant scope.
pub fn assess_tenant_active_uploads(
    catalog: &UploadCatalog,
    context: TenantAccessContext,
    cursor: Option<ActiveUploadCursor>,
    limits: UploadPageLimits,
) -> Result<ActiveUploadPage, UploadReadError> {
    super::check_tenant(catalog.confirmed(), context)?;
    Ok(catalog.active_uploads(context.actor, cursor, limits)?)
}

/// Gateway root observation, never a provider liveness/deletion boolean.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadRootStatus {
    /// Current local state in the authenticated gateway's namespace.
    Known(UploadRootState),
    /// No local claim exists; this is not permission to delete a provider object.
    Unknown,
    /// Root belongs to another namespace; no phase or binding is disclosed.
    WrongNamespace,
    /// Malformed input preserved in its original batch position.
    Malformed(HashParseError),
}

/// Inspect roots across both pending operations and confirmed lifecycle history.
///
/// Current membership/service checks precede all root reads, even for empty batches.
/// Input order, duplicates and malformed positions are retained. In particular,
/// Reserved and `ExposurePossible` are known local states rather than absent objects.
/// A provider adapter must separately qualify how to protect these states in its
/// protocol. Cancelled, unknown and settled observations confer no deletion authority.
/// Unique roots share at most one bounded history scan, and duplicates reuse their
/// observation. Temporary memory is bounded by raw batch length. Batch/history
/// limits still need production sizing before an endpoint exposes the view.
/// # Errors
/// Rejects wrong service or unknown/revoked gateways before returning any statuses.
pub fn assess_gateway_upload_roots(
    catalog: &UploadCatalog,
    registry: &GatewayRegistry,
    context: GatewayCallbackContext,
    batch: &ProviderRootBatch,
) -> Result<Vec<UploadRootStatus>, GatewayAccessError> {
    super::check_gateway(catalog.confirmed(), registry, context)?;
    Ok(catalog
        .root_views(batch)
        .entries
        .into_iter()
        .map(|entry| {
            let view = match entry {
                Ok(Some(view)) => view,
                Ok(None) => return UploadRootStatus::Unknown,
                Err(error) => return UploadRootStatus::Malformed(error),
            };
            if view.object.identity().namespace != registry.scope().namespace() {
                return UploadRootStatus::WrongNamespace;
            }
            UploadRootStatus::Known(view.state)
        })
        .collect())
}

/// Authorized upload-page read failed before exposing operation data.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UploadReadError {
    /// Actual execution service/caller is not an eligible direct tenant.
    #[error(transparent)]
    Access(#[from] TenantAccessError),
    /// Cursor scope does not match the current service and tenant caller.
    #[error(transparent)]
    Cursor(#[from] UploadCursorError),
}
