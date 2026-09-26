//! Direct-tenant catalog reads over current supplied state, without effects.
//!
//! Unknown roots and roots owned by another tenant have the same typed rejection.
//! These predicates do not authenticate IC calls, confer download authority or
//! reconcile stale/restored state. Supply actual execution context and fence
//! recovery before using these results.

use std::num::NonZeroUsize;

use thiserror::Error;

use crate::{
    model::{
        catalog::{BlobCatalog, CatalogReferenceKey},
        identity::ProviderRootHash,
        lifecycle::{
            binding::ObjectBindingMismatch,
            requests::{
                ReferenceReceiptView, ReferenceRequest, ReferenceRequestError, ReferenceRequests,
            },
        },
    },
    policy::tenant::{TenantAccessContext, TenantAccessError},
};

/// Read references across the caller's objects and namespaces in input order.
///
/// Duplicates consume the raw entry budget and preserve their positions. Unknown
/// references in an owned object return false, like released references. Unknown
/// or foreign objects reject the entire batch, without partial statuses. Empty
/// batches still require a concrete caller in the catalog's service.
///
/// Ownership of every root is checked before supplied bindings, and every binding
/// is checked before result allocation. The caller supplies a trusted bound;
/// transport decoding needs its own allocation budget. Cost is bounded by the
/// raw key count and logarithmic catalog/reference lookups, with no full scan.
/// # Errors
/// Rejects execution context, raw count, unavailable objects or wrong bindings,
/// in that order. No reference, receipt or accounting state changes.
pub fn assess_tenant_references(
    catalog: &BlobCatalog,
    keys: &[CatalogReferenceKey],
    context: TenantAccessContext,
    max_entries: NonZeroUsize,
) -> Result<Vec<bool>, CatalogTenantReadError> {
    super::check_tenant(catalog, context)?;
    if keys.len() > max_entries.get() {
        return Err(CatalogTenantReadError::TooManyEntries {
            actual: keys.len(),
            maximum: max_entries.get(),
        });
    }
    for key in keys {
        owned_journal(catalog, key.root, context)?;
    }
    for key in keys {
        owned_journal(catalog, key.root, context)?
            .lifecycle()
            .binding()
            .check(key.reference.object())?;
    }
    keys.iter()
        .map(|key| {
            Ok(owned_journal(catalog, key.root, context)?
                .lifecycle()
                .reference_is_live(key.reference)?)
        })
        .collect()
}

/// Read an exact reference request's original result from an owned object.
///
/// This never reapplies the operation or consumes receipt capacity. A recorded
/// retain can have succeeded while that reference is now released; a recorded
/// rejection remains a rejection after conditions change. `None` means absent
/// from the supplied local journal, not a safe-to-retry provider outcome.
/// # Errors
/// Rejects context or unknown/foreign roots before inspecting request details.
/// Wrong object bindings and conflicting ID payloads return no receipt.
pub fn assess_tenant_receipt(
    catalog: &BlobCatalog,
    root: ProviderRootHash,
    request: ReferenceRequest,
    context: TenantAccessContext,
) -> Result<Option<ReferenceReceiptView>, CatalogTenantReadError> {
    super::check_tenant(catalog, context)?;
    Ok(owned_journal(catalog, root, context)?.receipt(context.actor, request)?)
}

fn owned_journal(
    catalog: &BlobCatalog,
    root: ProviderRootHash,
    context: TenantAccessContext,
) -> Result<&ReferenceRequests, CatalogTenantReadError> {
    catalog
        .get(root)
        .filter(|journal| journal.lifecycle().binding().tenant() == context.actor)
        .ok_or(CatalogTenantReadError::Unavailable)
}

/// Tenant catalog read rejection, with no partial values or state changes.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CatalogTenantReadError {
    /// Actual service/caller context failed the direct-tenant predicate.
    #[error(transparent)]
    Access(#[from] TenantAccessError),
    /// Root is unknown or not owned by the caller; existence is not disclosed.
    #[error("catalog object is unavailable")]
    Unavailable,
    /// Reference binding differs from the caller's trusted object binding.
    #[error(transparent)]
    Binding(#[from] ObjectBindingMismatch),
    /// Exact receipt lookup rejected request scope or conflicting ID reuse.
    #[error(transparent)]
    Request(#[from] ReferenceRequestError),
    /// Raw reference count, including duplicates, exceeds the processing budget.
    #[error("catalog reference batch has {actual} entries, maximum is {maximum}")]
    TooManyEntries {
        /// Supplied count, including repeated keys.
        actual: usize,
        /// Maximum from trusted configuration.
        maximum: usize,
    },
}
