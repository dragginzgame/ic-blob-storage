//! Authorized read-only catalog views over trusted transient values.
//!
//! No endpoint, state loading or provider protocol mapping occurs here. The
//! caller must fence stale/restored state and supply actual execution context.

pub mod tenant;
pub mod upload;

use candid::Principal;
use thiserror::Error;

use crate::model::{
    catalog::{
        BlobCatalog, CatalogUsage,
        pending::{
            PendingCursorError, PendingDeletionCursor, PendingDeletionPage, PendingPageLimits,
        },
    },
    gateway::registry::GatewayRegistry,
    identity::{HashParseError, batch::ProviderRootBatch},
    lifecycle::LifecyclePhase,
};

use super::{
    gateway::{GatewayAccessError, GatewayCallbackContext},
    tenant::{TenantAccessContext, TenantAccessError},
};

/// Local root observation, deliberately not the provider's deletion/liveness boolean.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CatalogRootStatus {
    /// Phase of an object in the authenticated gateway's namespace.
    Known(LifecyclePhase),
    /// No confirmed object is known. This is not evidence that deletion is safe.
    Unknown,
    /// The root belongs to a different configured namespace; no phase is disclosed.
    WrongNamespace,
    /// Invalid input retained at its original batch position.
    Malformed(HashParseError),
}

/// Inspect a bounded root batch after checking current gateway membership.
///
/// Order, duplicates and invalid positions are retained. Unknown/malformed roots
/// remain explicit; they are never silently interpreted as deletion permission.
/// A known phase is local bookkeeping, not provider availability or settlement proof.
/// # Errors
/// Rejects wrong service or a revoked/unknown gateway before inspecting roots.
pub fn assess_gateway_roots(
    catalog: &BlobCatalog,
    registry: &GatewayRegistry,
    context: GatewayCallbackContext,
    batch: &ProviderRootBatch,
) -> Result<Vec<CatalogRootStatus>, GatewayAccessError> {
    check_gateway(catalog, registry, context)?;
    Ok(batch
        .entries()
        .iter()
        .map(|entry| {
            let root = match entry {
                Ok(root) => *root,
                Err(error) => return CatalogRootStatus::Malformed(*error),
            };
            let Some(journal) = catalog.get(root) else {
                return CatalogRootStatus::Unknown;
            };
            let lifecycle = journal.lifecycle();
            if lifecycle.binding().identity().namespace != registry.scope().namespace() {
                return CatalogRootStatus::WrongNamespace;
            }
            CatalogRootStatus::Known(lifecycle.phase())
        })
        .collect())
}

/// List a bounded page only for the gateway's current service/namespace.
///
/// Membership is checked on every page; a continuation grants no authority.
/// Limits must come from trusted configuration, not unchecked request values.
/// Pages are not snapshots; restart each new sweep to find newly pending earlier roots.
/// # Errors
/// Rejects wrong service, revoked callers or a continuation from another scope.
pub fn assess_gateway_pending(
    catalog: &BlobCatalog,
    registry: &GatewayRegistry,
    context: GatewayCallbackContext,
    cursor: Option<PendingDeletionCursor>,
    limits: PendingPageLimits,
) -> Result<PendingDeletionPage, CatalogGatewayReadError> {
    check_gateway(catalog, registry, context)?;
    Ok(catalog.pending_deletions(registry.scope().namespace(), cursor, limits)?)
}

/// Read only the authenticated direct tenant's usage across its namespaces.
///
/// No requested tenant field or controller override exists. A concrete caller
/// with no entries receives zero local usage, not another tenant's totals.
/// # Errors
/// Rejects wrong running service and anonymous/management actors.
pub fn assess_tenant_usage(
    catalog: &BlobCatalog,
    context: TenantAccessContext,
) -> Result<CatalogUsage, TenantAccessError> {
    check_tenant(catalog, context)?;
    Ok(catalog.tenant_usage(context.actor))
}

fn check_tenant(
    catalog: &BlobCatalog,
    context: TenantAccessContext,
) -> Result<(), TenantAccessError> {
    if context.service != catalog.service() {
        return Err(TenantAccessError::WrongService);
    }
    if context.actor == Principal::anonymous() || context.actor == Principal::management_canister()
    {
        return Err(TenantAccessError::NotTenant);
    }
    Ok(())
}

fn check_gateway(
    catalog: &BlobCatalog,
    registry: &GatewayRegistry,
    context: GatewayCallbackContext,
) -> Result<(), GatewayAccessError> {
    if context.service != catalog.service() || registry.scope().service() != catalog.service() {
        return Err(GatewayAccessError::WrongService);
    }
    if !registry.gateways().contains(context.actor) {
        return Err(GatewayAccessError::NotGateway);
    }
    Ok(())
}

/// A pending-page read rejected before returning object identities.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CatalogGatewayReadError {
    /// Current caller/service policy failed.
    #[error(transparent)]
    Access(#[from] GatewayAccessError),
    /// Cursor service or namespace differs from the authorized registry scope.
    #[error(transparent)]
    Cursor(#[from] PendingCursorError),
}
