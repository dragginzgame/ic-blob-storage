//! Explicit operator removal of local gateway membership, not provider revocation.
pub mod sync;
use crate::dto::operator::OperatorScope;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;

/// One current operator decision to revoke a gateway from this installation.
/// Repeating it is another revocation, not replay of a historical receipt; it can
/// remove membership added since an earlier call. Clients must not retry silently.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct GatewayRevocationRequest {
    /// Complete installed service/namespace/Cashier/payer scope.
    pub scope: OperatorScope,
    /// Concrete gateway principal to revoke, including an already absent member.
    pub gateway: Principal,
}
/// Synchronous acknowledgment, not a retained receipt or provider-side settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct GatewayRevocationResponse {
    /// Exact operator decision applied by the service.
    pub request: GatewayRevocationRequest,
    /// Whether the principal was a member before this decision. Pending sync and
    /// read observations are invalidated even when this is false.
    pub removed: bool,
}
/// Rejected revocations leave membership and pending work unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum GatewayRevocationFailure {
    /// Zero namespace or anonymous/management gateway principal.
    Invalid,
    /// Wrong actual service or installed service/provider/account scope.
    Binding,
    /// Actual caller is not the configured operator.
    Denied,
    /// Restored registry remains inspection-only.
    Fenced,
    /// Retained registry cannot be read consistently.
    Internal,
}
