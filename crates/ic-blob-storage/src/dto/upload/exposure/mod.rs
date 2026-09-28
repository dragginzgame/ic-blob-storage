//! Exposure boundary failures; no certificate or provider response schema.
use super::admission::UploadAdmissionFailure;
use candid::{CandidType, Deserialize};
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
