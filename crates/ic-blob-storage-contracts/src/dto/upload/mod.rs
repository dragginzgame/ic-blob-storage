//! Exact historical upload observation, distinct from current serving authority.
pub mod admission;
pub mod capacity;
pub mod certificate;
pub mod discovery;
pub mod exposure;
pub mod history;
pub mod manifest;
use crate::dto::reference::ReferenceUpload;
use serde::Deserialize;
/// Retained local upload phase; neither provider dispatch nor publication authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum UploadState {
    /// Reserved locally; upload authority has not escaped.
    Reserved,
    /// Retention intent was registered and the first reference created.
    /// This does not prove that the provider received or still stores the bytes.
    Registered,
    /// Cancelled before exposure; retained identity is not reusable.
    Cancelled,
}
/// Full original binding and current retained local state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UploadStatusResponse {
    /// Exact original upload, including its independent first reference identity.
    pub upload: ReferenceUpload,
    /// Local lifecycle observation, not current reference liveness.
    pub state: UploadState,
    /// Local issuance permission has been withdrawn; escaped effects remain possible.
    pub revoked: bool,
}
/// Failed exact upload inspection. Absence does not authorize a fresh upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum UploadStatusFailure {
    /// Invalid original identity or length.
    Invalid,
    /// Actual caller differs from the named tenant.
    Denied,
    /// Service/namespace differs from installed configuration.
    Binding,
    /// No retained upload under that operation identity.
    Unknown,
    /// Retained operation has different original arguments.
    Conflict,
    /// Inconsistent internal storage.
    Internal,
}
