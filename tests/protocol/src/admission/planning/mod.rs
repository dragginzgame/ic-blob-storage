//! Passive local fixture capacity, not a production endpoint contract.

use super::Enrollment;
use candid::{CandidType, Principal};
use serde::Deserialize;

/// Explicit tenant scope; the actual caller must match independently.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct AdmissionCapacityInput {
    /// Expected running service.
    pub service: Principal,
    /// Tenant whose headroom is requested.
    pub tenant: Principal,
    /// Expected installed provider namespace.
    pub namespace: u128,
}

/// Tighter global/tenant headroom, including pending obligations and lifetime history.
/// Counts are independent dimensions, never reservations or permission to upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct AdmissionCapacity {
    /// Checked scope, allowing clients to reject a mismatched response.
    pub scope: AdmissionCapacityInput,
    /// Current enrollment; suspension prevents fresh uploads despite headroom.
    pub enrollment: Enrollment,
    /// Configured per-object byte ceiling.
    pub max_object_bytes: u64,
    /// Configured per-object metadata entry ceiling.
    pub max_headers: u64,
    /// Configured per-object framed metadata byte ceiling.
    pub max_header_bytes: u64,
    /// Remaining lifetime operation/root slots.
    pub remaining_objects: u64,
    /// Remaining concurrent reservations/exposures.
    pub remaining_active_uploads: u64,
    /// Remaining lifetime leaf slots.
    pub remaining_manifest_chunks: u64,
    /// Tighter logical, physical and continuing-billing byte headroom.
    pub remaining_bytes: u128,
}
