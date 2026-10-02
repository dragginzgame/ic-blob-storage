//! Exposure boundary failures; no certificate or provider response schema.
use super::admission::UploadAdmissionFailure;
use candid::{CandidType, Deserialize};

/// A missing independent prerequisite for exposure or certificate issuance.
/// A successful inspection with blockers is not upload authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadExposureBlocker {
    /// Host facts were not established in the current execution.
    StaleObservation,
    /// Explicit installed local namespace does not match the permission.
    NamespaceBinding,
    /// Original uploader lacks explicit installed trust.
    TrustedUploader,
    /// Owner is inspection-only after restoration; no backup freshness claim.
    CurrentOwner,
    /// Atomic intent-before-effect durability is not established.
    Durability,
}
/// Refused local exposure or historical permission inspection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadExposureFailure {
    /// Full permission, authority, activation, deadline or restore fence failed.
    Permission(UploadAdmissionFailure),
    /// No validated manifest is bound to the original permission.
    Unprepared,
    /// Local issuance permission has been withdrawn.
    Revoked,
    /// Exposure already happened or the operation/clock no longer allows it.
    Phase,
    /// Host evidence does not bind the entire original permission.
    EvidenceBinding,
}
