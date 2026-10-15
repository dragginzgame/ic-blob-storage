//! Passive tenant admission headroom from the same accounting used by mutations.

use crate::model::catalog::admission::UploadUsage;
use candid::Principal;
use ic_blob_storage_contracts::configuration::service::ServiceLimits;
use ic_blob_storage_contracts::tenant::TenantEnrollmentView;
use std::num::NonZeroU128;

/// Explicit tenant and provider namespace for a capacity observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdmissionCapacityLookup {
    /// Must equal the independently authenticated caller.
    pub tenant: Principal,
    /// Must equal the installed provider namespace.
    pub namespace: NonZeroU128,
}

/// Independent capacity dimensions observed synchronously, without reserving any.
///
/// Each remaining count is the tighter global/tenant bound. No other tenant's
/// identity or objects are disclosed, but shared contention affects the result.
/// All dimensions, metadata rules and identity checks must fit together at actual
/// admission. A positive result never proves a root is unclaimed, an operation is
/// fresh, funding is available or a provider effect is safe. Exact operation
/// retries and existing-object retains use their own maintained rules.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdmissionCapacityView {
    /// Suspension preserves observations but disallows fresh admission.
    pub enrollment: TenantEnrollmentView,
    /// Configured per-object ceiling, not remaining byte headroom.
    pub max_object_bytes: u64,
    /// Configured per-object metadata entry bound.
    pub max_headers: usize,
    /// Configured per-object framed metadata byte bound.
    pub max_header_bytes: usize,
    /// Remaining lifetime operation/root slots, never refunded on cleanup.
    pub remaining_objects: usize,
    /// Remaining concurrent reserved/possibly-exposed upload slots.
    /// Confirmation or pre-exposure cancellation can restore these slots.
    pub remaining_active_uploads: usize,
    /// Remaining lifetime manifest leaves; sum ceil(bytes / chunk size) per object.
    /// Cancellation, physical deletion and settlement never refund these leaves.
    pub remaining_manifest_chunks: u64,
    /// Remaining tenant logical bytes, including this tenant's reservations.
    pub remaining_logical_bytes: u128,
    /// Remaining global physical bytes, including every tenant's reservations.
    /// Logical release does not restore this capacity.
    pub remaining_physical_bytes: u128,
    /// Remaining global billing-liability bytes, including every reservation.
    /// Physical deletion does not establish billing cessation.
    pub remaining_liability_bytes: u128,
    /// Minimum remaining tenant logical, global physical and global liability bytes.
    /// Includes pending reservations. Logical release cannot clear physical/billing
    /// obligations; deletion alone cannot clear continuing billing.
    pub remaining_bytes: u128,
}

/// Shared arithmetic over validated maintained totals, never an admission promise.
pub(crate) fn headroom(
    limits: ServiceLimits,
    enrollment: TenantEnrollmentView,
    global: UploadUsage,
    tenant: UploadUsage,
    remaining_manifest_chunks: u64,
) -> AdmissionCapacityView {
    let remaining_logical_bytes =
        limits.catalog.max_tenant_logical_bytes.get() - tenant.logical_bytes;
    let remaining_physical_bytes = limits.catalog.max_physical_bytes.get() - global.physical_bytes;
    let remaining_liability_bytes =
        limits.catalog.max_liability_bytes.get() - global.liability_bytes;
    AdmissionCapacityView {
        enrollment,
        max_object_bytes: limits.max_object_bytes.get(),
        max_headers: limits.max_headers.get(),
        max_header_bytes: limits.max_header_bytes.get(),
        remaining_objects: (limits.catalog.max_objects.get() - global.operations)
            .min(limits.catalog.max_tenant_objects.get() - tenant.operations),
        remaining_active_uploads: (limits.uploads.max_active.get() - global.active_reservations)
            .min(limits.uploads.max_tenant_active.get() - tenant.active_reservations),
        remaining_manifest_chunks,
        remaining_logical_bytes,
        remaining_physical_bytes,
        remaining_liability_bytes,
        remaining_bytes: remaining_logical_bytes
            .min(remaining_physical_bytes)
            .min(remaining_liability_bytes),
    }
}
