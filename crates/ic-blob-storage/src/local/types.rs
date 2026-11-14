//! Inputs and observations of the experimental local control model.

use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};

use ic_platform_types::Principal;

use crate::caffeine::{ManifestLimits, VerificationError};
use crate::references::{ReferenceError, ReferenceId, RetentionState};

/// Application-assigned upload identity with one retained lifetime.
pub type UploadId = NonZeroU128;

/// Local content key within this model's single, hypothetical provider binding.
/// This is not a qualified provider deletion key or a wire representation.
pub type Root = [u8; 32];

/// Fixed bounds for one local model. Released/retired identities occupy slots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Per-declaration hashing and content limits.
    pub manifest: ManifestLimits,
    /// Maximum historical upload identities, including cancelled uploads.
    pub uploads: NonZeroUsize,
    /// Maximum historical reference identities across all objects.
    pub references: NonZeroUsize,
    /// Maximum reserved plus live declared bytes.
    pub logical_bytes: NonZeroU64,
    /// Maximum reserved plus unresolved declared bytes, not a physical/cost cap.
    pub exposure_bytes: NonZeroU64,
}

/// Immutable application authorization. Knowledge of its fields conveys no rights.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadRequest {
    /// One-use upload identity.
    pub id: UploadId,
    /// Caller allowed to perform the first local registration.
    pub uploader: Principal,
    /// Candidate root, including original metadata commitments.
    pub root: Root,
    /// Declared byte count, checked for consistency but not authenticated by hashes.
    pub bytes: u64,
    /// Application-selected reference, known before any registration.
    pub first_reference: ReferenceId,
    /// Exclusive deadline in the host-supplied clock's units.
    pub expires_at: u64,
}

/// Current disposition of a historical upload identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadState {
    /// Capacity and the first reference name are reserved; no registration occurred.
    Reserved,
    /// The original uploader consumed the grant in the local model.
    Registered,
    /// Application authorization attached a reference to an already Open object.
    /// It does not permit another registration or provider upload.
    Attached,
    /// Application cancellation retired an unconsumed grant.
    Cancelled,
    /// Explicit cleanup retired an expired, unconsumed grant.
    Expired,
}

/// Authorized inspection or historical authorization replay; never a certificate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadView {
    /// Exact original authorization.
    pub request: UploadRequest,
    /// Current disposition; Registered/Attached does not assert a live reference.
    pub state: UploadState,
}

/// Current application reference disposition, including pre-registration names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceStatus {
    /// Reserved by an unconsumed grant.
    Reserved,
    /// Registered and currently retained.
    Live,
    /// Released after registration; historical retain cannot resurrect it.
    Released,
    /// Its grant was cancelled or expired; the name cannot be reused.
    Retired,
}

/// One reference's immutable target and current local disposition.
/// This observation grants no authority and proves no provider availability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceView {
    /// Original root, preserved even when the reference is released or retired.
    pub root: Root,
    /// Current local disposition, not a guarantee about a later operation.
    pub status: ReferenceStatus,
}

/// Read-only projection of a root's original grant and local retention.
/// This is not provider presence, deletion permission, settlement or upload authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectView {
    /// Original upload lifetime; additional uses never replace it.
    pub original_upload: UploadView,
    /// None before registration, including cancelled/expired grants. Closing
    /// means local uses ended, while the original registration remains recorded.
    pub retention: Option<RetentionState>,
    /// Currently live references, zero before registration or after final release.
    pub live_references: usize,
}

/// Separate declared-byte counters; never add them as total physical storage.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Usage {
    /// Bytes reserved by unconsumed grants.
    pub reserved_bytes: u64,
    /// Bytes counted once per object with live references.
    pub live_bytes: u64,
    /// Bytes counted once per locally registered object, including Closing ones.
    /// No settlement operation exists, so this counter never decreases.
    pub unresolved_bytes: u64,
    /// Lifetime upload slots occupied, including retired identities.
    pub uploads: usize,
    /// Lifetime reference slots occupied, including released/retired identities.
    pub references: usize,
}

/// A refused local operation. Ordinary errors leave all state unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// The captured caller is not permitted to perform this operation.
    Denied,
    /// Anonymous or management principal cannot be an application/uploader.
    InvalidPrincipal,
    /// The upload identity was never admitted.
    UnknownUpload,
    /// This root has never been admitted.
    UnknownObject,
    /// The reference identity was never admitted.
    UnknownReference,
    /// An immutable request, declaration or reference target conflicts.
    Conflict,
    /// Another grant already reserves this root.
    ObjectReserved,
    /// The root's original grant was cancelled/expired; no new lifetime is allowed.
    RootRetired,
    /// A reserved/retired name cannot be used as a new retained reference.
    ReferenceUnavailable,
    /// Registration requires an unconsumed grant; replay cannot issue new authority.
    NotReserved,
    /// The deadline has passed; explicit cleanup can release its reservation.
    Expired,
    /// Explicit expiry was requested before the deadline.
    NotExpired,
    /// The supplied clock precedes the grant's admission.
    ClockReversed,
    /// A byte or lifetime record bound would be exceeded.
    Capacity,
    /// The proposed manifest is invalid or exceeds its work bounds.
    Manifest(VerificationError),
    /// The registered reference owner refused the operation.
    Reference(ReferenceError),
}
