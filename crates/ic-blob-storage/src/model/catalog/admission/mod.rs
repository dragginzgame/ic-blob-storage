//! Admission facts for the single durable upload/lifecycle owner.
pub(crate) mod capacity;
pub mod read;
use super::{CatalogCapacity, CatalogError, check_bytes};
use crate::model::lifecycle::roots::RootClaimError;
use ic_blob_storage_contracts::configuration::limits::CatalogLimits;
use thiserror::Error;

/// Local operation state, not a provider observation or permission to send work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadPhase {
    /// Capacity reserved; no upload authority may have escaped yet.
    Reserved,
    /// Retention intent and its first live reference were registered.
    Registered,
    /// Cancelled before exposure; byte capacity freed but identity history retained.
    Cancelled,
}

/// Admission result. Replays report current local state, never new upload authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadAdmission {
    /// A fresh reservation and immutable root claim were recorded together.
    Reserved,
    /// Exact operation already exists; no state or accounting changed.
    Existing(UploadPhase),
}

/// Read-only capacity charged to pending operations plus confirmed lifecycles.
/// Byte liabilities are capacity units, not measured currency or a spending cap.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UploadUsage {
    /// Lifetime operation/root slots, including cancelled and settled entries.
    pub operations: usize,
    /// Reserved or possibly exposed operations, including zero-byte uploads.
    pub active_reservations: usize,
    /// Bytes reserved by active operations, counted once in each total below.
    pub reserved_bytes: u128,
    /// Tenant logical bytes plus active reservations.
    pub logical_bytes: u128,
    /// Confirmed physical bytes plus active reservations.
    pub physical_bytes: u128,
    /// Unsettled confirmed bytes plus active reservations.
    pub liability_bytes: u128,
}

/// Rejected local admission/transition. No error authorizes repeating an effect.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UploadError {
    /// The supplied authenticated actor is not the tenant owner.
    #[error("upload actor is not tenant owner")]
    Denied,
    /// No operation was admitted with this tenant/ID.
    #[error("unknown upload request")]
    UnknownRequest,
    /// The operation ID was already used with different exact arguments.
    #[error("upload request conflicts with original input")]
    RequestConflict,
    /// Global active reservation limit reached.
    #[error("active upload limit reached")]
    ActiveLimit,
    /// Per-tenant active reservation limit reached.
    #[error("tenant active upload limit reached")]
    TenantActiveLimit,
    /// The current phase forbids this transition.
    #[error("upload transition rejected in {0:?}")]
    InvalidPhase(UploadPhase),
    /// Immutable root/object/service history rejects admission.
    #[error(transparent)]
    Root(#[from] RootClaimError),
    /// Shared catalog capacity or lifecycle admission failed.
    #[error(transparent)]
    Catalog(#[from] CatalogError),
}
