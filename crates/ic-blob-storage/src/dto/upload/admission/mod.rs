//! Exact tenant permission admission and recovery, without provider dispatch.
use super::{ReferenceUpload, UploadState};
use candid::{CandidType, Deserialize, Principal};
/// Immutable permission. Persist every field before dispatch; IDs are caller allocated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadAdmissionRequest {
    /// Complete original upload and first-reference identity.
    pub upload: ReferenceUpload,
    /// Explicit uploader, distinct from tenant admission authority.
    pub uploader: Principal,
    /// Exclusive issuance deadline in IC nanoseconds; retries never extend it.
    pub expires_at_ns: u64,
}
/// Exact retained permission and historical phase, not issuance or serving authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadAdmissionResponse {
    /// Complete original permission, checked on both admission and inspection.
    pub permission: UploadAdmissionRequest,
    /// Current retained phase; confirmation remains historical after release.
    pub state: UploadState,
    /// Local permission was withdrawn; escaped effects may remain uncertain.
    pub revoked: bool,
}
/// Result of a synchronous local reservation, with no certificate or provider call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadAdmissionMutation {
    /// Exact retained permission and phase.
    pub admission: UploadAdmissionResponse,
    /// Existing permission was observed without renewing it or reserving more capacity.
    pub replayed: bool,
}
/// Admission/inspection refusal; missing history never proves safe provider retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadAdmissionFailure {
    /// Malformed identity, uploader or object length.
    Invalid,
    /// Caller is not the named tenant.
    Denied,
    /// Wrong installed service or namespace.
    Binding,
    /// Exact operation has no retained permission.
    Unknown,
    /// Existing operation, root or object has conflicting arguments.
    Conflict,
    /// Tenant is not enrolled or is suspended for fresh admission.
    Inactive,
    /// Fresh permission's exclusive deadline has passed.
    Expired,
    /// Configured object, history, reservation or byte capacity is exhausted.
    Capacity,
    /// Restored owner permits inspection only.
    Fenced,
    /// Inconsistent storage or an unexpected internal result.
    Internal,
}
