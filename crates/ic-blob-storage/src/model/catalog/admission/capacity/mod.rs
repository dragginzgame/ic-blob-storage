//! Shared reservation admission against already observed owner totals.
use super::CatalogCapacity;
use super::CatalogError;
use super::CatalogLimits;
use super::UploadError;
use super::UploadUsage;
use super::check_bytes;
use ic_blob_storage_contracts::configuration::limits::UploadLimits;

pub(crate) fn check(
    usage: UploadUsage,
    tenant: UploadUsage,
    bounds: CatalogLimits,
    limits: UploadLimits,
    bytes: u64,
) -> Result<(), UploadError> {
    if usage.operations >= bounds.max_objects.get() {
        return Err(CatalogError::Capacity(CatalogCapacity::Objects).into());
    }
    if tenant.operations >= bounds.max_tenant_objects.get() {
        return Err(CatalogError::Capacity(CatalogCapacity::TenantObjects).into());
    }
    if usage.active_reservations >= limits.max_active.get() {
        return Err(UploadError::ActiveLimit);
    }
    if tenant.active_reservations >= limits.max_tenant_active.get() {
        return Err(UploadError::TenantActiveLimit);
    }
    check_bytes(
        usage.physical_bytes,
        bytes,
        bounds.max_physical_bytes,
        CatalogCapacity::PhysicalBytes,
    )?;
    check_bytes(
        usage.liability_bytes,
        bytes,
        bounds.max_liability_bytes,
        CatalogCapacity::LiabilityBytes,
    )?;
    check_bytes(
        tenant.logical_bytes,
        bytes,
        bounds.max_tenant_logical_bytes,
        CatalogCapacity::TenantLogicalBytes,
    )?;
    Ok(())
}
