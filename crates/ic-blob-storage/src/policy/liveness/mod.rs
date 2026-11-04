//! Read-only direct-tenant reference liveness over a supplied lifecycle value.
//!
//! The eventual workflow must supply current trusted state and authenticated
//! context, and fence unreconciled restoration. These consumer reads are not
//! gateway root-liveness responses or authority to delete a provider object.

use std::num::NonZeroUsize;

use thiserror::Error;

use crate::model::lifecycle::BlobLifecycle;
use ic_blob_storage_contracts::binding::ObjectBindingMismatch;
use ic_blob_storage_contracts::binding::ReferenceKey;

use super::tenant::{TenantAccessContext, TenantAccessError, assess_tenant_access};

/// Read one reference only after checking the caller against the trusted owner.
///
/// Unknown/released references return false; another live reference does not
/// grant liveness to this key. A matching key alone grants no caller authority.
/// # Errors
/// Rejects unauthorized context before checking the supplied reference binding.
pub fn assess_reference_liveness(
    lifecycle: &BlobLifecycle,
    key: ReferenceKey,
    context: TenantAccessContext,
) -> Result<bool, ReferenceLivenessError> {
    assess_tenant_access(lifecycle.binding(), context)?;
    Ok(lifecycle.reference_is_live(key)?)
}

/// Read a bounded batch of references to one object in exact input order.
///
/// Duplicates count toward `max_entries` and retain their result positions.
/// Empty batches still require tenant authorization. Every binding is validated
/// before allocating results; any mismatch rejects the entire batch. Limits
/// cover supplied keys/results, not a future transport decoder's allocation.
/// # Errors
/// Rejects unauthorized context, excessive raw entries or any mismatched binding,
/// in that order. Rejection never returns partial reference status.
pub fn assess_reference_liveness_batch(
    lifecycle: &BlobLifecycle,
    keys: &[ReferenceKey],
    context: TenantAccessContext,
    max_entries: NonZeroUsize,
) -> Result<Vec<bool>, ReferenceLivenessError> {
    assess_tenant_access(lifecycle.binding(), context)?;
    if keys.len() > max_entries.get() {
        return Err(ReferenceLivenessError::TooManyEntries {
            actual: keys.len(),
            maximum: max_entries.get(),
        });
    }
    for key in keys {
        lifecycle.binding().check(key.object())?;
    }
    keys.iter()
        .map(|key| {
            lifecycle
                .reference_is_live(*key)
                .map_err(ReferenceLivenessError::from)
        })
        .collect()
}

/// A consumer liveness read rejected without returning reference status.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ReferenceLivenessError {
    /// Reader is not the direct tenant in the running service.
    #[error(transparent)]
    Access(#[from] TenantAccessError),
    /// A supplied key is not bound to the expected object incarnation.
    #[error(transparent)]
    Binding(#[from] ObjectBindingMismatch),
    /// Raw input count exceeds the configured processing/result budget.
    #[error("reference batch has {actual} entries, maximum is {maximum}")]
    TooManyEntries {
        /// Supplied reference count, including duplicates.
        actual: usize,
        /// Configured maximum count.
        maximum: usize,
    },
}

#[cfg(test)]
mod tests;
