//! Private bounded uploader intent controls; distinct from tenant registration.
use candid::{CandidType, Deserialize};
use ic_blob_storage::dto::upload::manifest::{
    UploadManifestDeclaration, UploadManifestFailure, UploadManifestRequest,
};
/// Exact locally assigned identity and immutable declaration intent.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ManifestIntent {
    /// Explicit lifetime journal identity; never allocated by the fixture.
    pub id: u128,
    /// Full original permission and declaration.
    pub request: UploadManifestRequest,
}
/// Read-only retained intent and dispatch result.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ManifestIntentView {
    /// Immutable original request.
    pub intent: ManifestIntent,
    /// Local tombstone blocks further dispatch; it does not revoke tenant permission.
    pub cancelled: bool,
    /// Dispatch may have happened; absent result requires inspection.
    pub started: bool,
    /// First declaration accepted by the service, or retained typed refusal.
    pub result: Option<Result<UploadManifestDeclaration, UploadManifestFailure>>,
}
/// Scheduling controls do not change saved intent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ManifestDispatch {
    /// Previously saved local intent.
    pub id: u128,
    /// Encoded reply budget.
    pub max_reply_bytes: u32,
    /// Local IC interruption control.
    pub fault: ManifestFault,
    /// Pause after the remote response, before recording its outcome.
    pub hold: bool,
}
/// Fixture-only IC transaction interruption.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ManifestFault {
    /// No interruption.
    None,
    /// Trap after local start writes, before sending.
    BeforeDispatch,
    /// Trap after remote preparation, before acknowledgment writes.
    AfterPreparation,
}
