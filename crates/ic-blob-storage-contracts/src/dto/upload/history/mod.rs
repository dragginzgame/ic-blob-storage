//! Bounded retained operation discovery. A page never grants effect authority.
use crate::dto::reference::ReferenceUpload;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;

/// Independently authenticated observer scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadHistoryScope {
    /// Caller must be this tenant, including while suspended.
    Tenant(Principal),
    /// Caller must be the installed operator.
    Service,
}
/// Current local states to select, not a provider reconciliation result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadHistoryFilter {
    /// Every retained operation, including cancelled and settled history.
    All,
    /// Reserved or possibly exposed uploads.
    Active,
    /// Last reference released, physical deletion unresolved.
    DeletionPending,
    /// Active uploads or confirmed objects with unresolved obligations.
    Outstanding,
}
/// Untrusted forward position. Changes behind it require a fresh sweep.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadHistoryCursor {
    /// Original service.
    pub service: Principal,
    /// Original local namespace.
    pub namespace: u128,
    /// Original observer scope.
    pub scope: UploadHistoryScope,
    /// Original filter.
    pub filter: UploadHistoryFilter,
    /// Tenant of the last inspected row.
    pub after_tenant: Principal,
    /// Last inspected operation; not necessarily a matching result.
    pub after_request: u128,
}
/// One scan; work and result limits are host-owned, never supplied by ingress.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadHistoryRequest {
    /// Actual intended storage service.
    pub service: Principal,
    /// Installed local namespace.
    pub namespace: u128,
    /// Authority checked independently of the cursor.
    pub scope: UploadHistoryScope,
    /// Current-state predicate.
    pub filter: UploadHistoryFilter,
    /// Optional continuation, not a snapshot or completed sweep proof.
    pub cursor: Option<UploadHistoryCursor>,
}
/// Current local reservation or confirmed lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadContentState {
    /// Unexposed local reservation.
    Reserved,
    /// Escaped upload authority may have caused an effect.
    ExposurePossible,
    /// Cancelled before exposure; identity remains retained.
    Cancelled,
    /// Confirmed content with live references.
    Live,
    /// No references remain; provider deletion unresolved.
    DeletionPending,
    /// Physical deletion established; billing cessation unresolved.
    ProviderDeleted,
    /// Billing cessation established; history remains retained.
    Settled,
}
/// Complete original upload identity with its current local lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadHistoryEntry {
    /// Operation, object and first-reference IDs are independent.
    pub request: ReferenceUpload,
    /// Current observation; not publication or retry authority.
    pub state: UploadContentState,
}
/// One bounded synchronous observation, available during inspection-only restore.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadHistoryPage {
    /// Full original request echoed for correlation.
    pub request: UploadHistoryRequest,
    /// Matching entries only; no manifests, credentials or file bytes.
    pub entries: Vec<UploadHistoryEntry>,
    /// Advances past inspected history even when the result is empty.
    pub next: Option<UploadHistoryCursor>,
    /// Operation rows inspected, including filtered-out rows.
    pub scanned: u64,
    /// Restored upload owner permits inspection only.
    pub fenced: bool,
}
/// Refusal never means empty history or safe retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadHistoryFailure {
    /// Zero or malformed identity.
    Invalid,
    /// Actual caller does not own the requested observer scope.
    Denied,
    /// Wrong actual or installed service/namespace.
    Binding,
    /// Cursor does not match the requested scan.
    CursorScope,
    /// Retained state could not be read consistently.
    Internal,
}
