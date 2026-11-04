//! Capacity facts and failures shared by the durable upload owner.
pub mod admission;
use std::num::NonZeroU128;
use thiserror::Error;

fn check_bytes(
    current: u128,
    additional: u64,
    maximum: NonZeroU128,
    capacity: CatalogCapacity,
) -> Result<(), CatalogError> {
    if current
        .checked_add(u128::from(additional))
        .is_none_or(|total| total > maximum.get())
    {
        return Err(CatalogError::Capacity(capacity));
    }
    Ok(())
}

/// Exhausted catalog resource; no partial object admission occurs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CatalogCapacity {
    /// Lifetime object/root history capacity.
    Objects,
    /// Per-tenant lifetime object/root history capacity, independent of byte usage.
    TenantObjects,
    /// Global physical byte capacity.
    PhysicalBytes,
    /// Global unresolved billing-byte capacity.
    LiabilityBytes,
    /// Per-tenant logical byte capacity across namespaces.
    TenantLogicalBytes,
}

/// Rejected catalog operation; lifecycle failures inside recorded receipts are separate.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CatalogError {
    /// A configured capacity does not admit a fresh confirmed object.
    #[error("catalog capacity exhausted: {0:?}")]
    Capacity(CatalogCapacity),
}
