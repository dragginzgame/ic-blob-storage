//! Passive local accounting observations; no readiness or provider credit assertion.
use candid::{CandidType, Deserialize, Principal};

/// Complete installed service/provider/account scope for explicit operator requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct OperatorScope {
    /// Actual storage service.
    pub service: Principal,
    /// Positive installed local namespace.
    pub namespace: u128,
    /// Installed Cashier, not a caller-selected transport destination.
    pub cashier: Principal,
    /// Installed payer account.
    pub payment_account: Principal,
}
/// One synchronous local snapshot. No provider call or history traversal occurs.
/// Separate fences remain visible; none of these observations authorize dispatch.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct LocalServiceStatus {
    /// Independently checked full request scope.
    pub scope: OperatorScope,
    /// Upload reservations and continuing object obligations.
    pub uploads: LocalUploadStatus,
    /// Local attachment allocation; not platform liquidity or provider credit.
    pub funding: LocalFundingStatus,
    /// Bounded installed membership and local sync state.
    pub gateways: LocalGatewayStatus,
    /// Retained concurrent read occupancy.
    pub reads: LocalReadStatus,
}
/// Maintained upload counters, including cancelled/settled lifetime history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct LocalUploadStatus {
    /// Lifetime operation slots; cancellation does not free these.
    pub operations: u64,
    /// Reserved or possibly exposed uploads.
    pub active_reservations: u64,
    /// Bytes reserved by these active uploads.
    pub reserved_bytes: u128,
    /// Logical bytes, including reservations.
    pub logical_bytes: u128,
    /// Physical bytes, including reservations.
    pub physical_bytes: u128,
    /// Continuing billing-liability bytes, including reservations.
    pub liability_bytes: u128,
    /// Upload owner refuses mutations after restoration.
    pub fenced: bool,
}
/// Complete maintained totals for this local funding journal only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct LocalFundingStatus {
    /// Remaining attachment allocation including its reserve.
    pub available_allocation: u128,
    /// Remaining attachment allowance after reserve; never dispatch authority.
    pub attachment_allowance: u128,
    /// Transport-accepted attachments; not necessarily credited by the provider.
    pub transport_accepted: u128,
    /// Lifetime exact refunds, already included in available allocation.
    pub refunded: u128,
    /// Lifetime proven unsent attachments, not callback refunds.
    pub not_enqueued: u128,
    /// Attachment whose transport outcome remains absent.
    pub reserved_or_uncertain: u128,
    /// Retained lifetime intents, including terminal outcomes.
    pub retained_intents: u64,
    /// Installed lifetime intent capacity.
    pub intent_capacity: u64,
    /// Highest retained identity; not a restore-safe allocator.
    pub last_operation: Option<u128>,
    /// Funding owner refuses mutations after restoration.
    pub fenced: bool,
}
/// One bounded membership row, not evidence of deployed provider authorization.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct LocalGatewayStatus {
    /// Installed principals in retained order, at most 1024.
    pub members: Vec<Principal>,
    /// Last allocated local sync identity, including invalidated attempts.
    pub last_sequence: u64,
    /// Outstanding local read-only sync.
    pub pending_sequence: Option<u64>,
    /// Gateway owner refuses mutations after restoration.
    pub fenced: bool,
}
/// Occupied read slots survive interruption; inspection does not release them.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct LocalReadStatus {
    /// Highest locally allocated session identity.
    pub last_sequence: u64,
    /// Occupied sessions, including abandoned callbacks.
    pub sessions: u32,
    /// Reserved transport response buffers.
    pub reserved_bytes: u64,
    /// Read owner refuses admission and completion after restoration.
    pub fenced: bool,
}
/// Local inspection refusal; failures never become empty or zero observations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum LocalStatusFailure {
    /// Actual/requested scope or assembled owners differ.
    Binding,
    /// Caller is not the configured operator.
    Denied,
    /// Retained accounting or registry state cannot be read consistently.
    Internal,
}
