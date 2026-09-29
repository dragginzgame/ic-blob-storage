//! Tenant-scoped indexed content discovery, without allocation or serving authority.
use crate::dto::{tenant::TenantScope, upload::history::UploadHistoryEntry};
use candid::{CandidType, Deserialize};

/// Discover retained content without already knowing its operation identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadDiscoveryRequest {
    /// Actual tenant/service and installed namespace.
    pub scope: TenantScope,
    /// Provider root; a digest alone supplies no tenant authority.
    pub root: [u8; 32],
}
/// One synchronous local observation. Absence is not upload or retry permission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadDiscoveryResponse {
    /// Exact request echoed even for absent or foreign content.
    pub request: UploadDiscoveryRequest,
    /// Complete original identity and current local state, visible only to its tenant.
    pub content: Option<UploadHistoryEntry>,
    /// The owner's restore fence, independent of content visibility.
    pub fenced: bool,
}
/// Refusal must never be interpreted as an absent root.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadDiscoveryFailure {
    /// Invalid tenant or zero namespace.
    Invalid,
    /// Actual caller is not the named tenant.
    Denied,
    /// Wrong actual or installed service/namespace.
    Binding,
    /// Retained state could not be read consistently.
    Internal,
}
