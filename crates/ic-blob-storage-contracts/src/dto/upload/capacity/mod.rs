//! Passive admission headroom; independent dimensions never reserve capacity.
use crate::dto::tenant::TenantEnrollment;
use crate::dto::tenant::TenantScope;
use candid::CandidType;
use candid::Deserialize;

/// Current tighter global/tenant headroom, including retained provider liabilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadCapacityResponse {
    /// Exact authenticated service, namespace and tenant.
    pub scope: TenantScope,
    /// Suspension preserves inspection while preventing fresh admission.
    pub enrollment: TenantEnrollment,
    /// Configured per-object byte ceiling, independent of remaining bytes.
    pub max_object_bytes: u64,
    /// Configured per-object metadata entry limit.
    pub max_headers: u64,
    /// Configured per-object framed metadata byte limit.
    pub max_header_bytes: u64,
    /// Remaining lifetime operation/root slots; cleanup never refunds history.
    pub remaining_objects: u64,
    /// Remaining concurrent reservations, including possibly exposed uploads.
    pub remaining_active_uploads: u64,
    /// Remaining lifetime retained leaf slots.
    pub remaining_manifest_chunks: u64,
    /// Minimum logical, physical and continuing-billing byte headroom.
    pub remaining_bytes: u128,
    /// Restored owner permits inspection only, regardless of positive headroom.
    pub fenced: bool,
}

/// Refusal to inspect capacity, never an implicit zero or permission to upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadCapacityFailure {
    /// Invalid tenant principal or zero namespace.
    Invalid,
    /// Caller differs from the explicitly named tenant.
    Denied,
    /// Service or namespace differs from the installed owner.
    Binding,
    /// No retained enrollment exists for this tenant.
    NotEnrolled,
    /// Retained state or counter conversion is inconsistent.
    Internal,
}
