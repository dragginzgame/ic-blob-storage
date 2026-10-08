//! Explicit gateway refresh/cancellation; no payment or provider qualification.
use crate::dto::operator::OperatorScope;
use candid::CandidType;
use candid::Deserialize;

/// Acknowledgment of one completed local membership replacement.
/// Inspect local status after an uncertain reply; do not automatically start again.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct GatewaySyncResponse {
    /// Complete original installed scope.
    pub scope: OperatorScope,
    /// Local sync identity, never independent freshness or provider authority.
    pub sequence: u64,
}
/// Cancel exactly one observed pending read-only sync, never a paid operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct GatewaySyncCancellation {
    /// Complete installed service/provider/account scope.
    pub scope: OperatorScope,
    /// Positive pending identity obtained from local status.
    pub sequence: u64,
}
/// Typed failure; transport/decoding failures preserve pending work for inspection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum GatewaySyncFailure {
    /// Zero namespace/cancellation identity or invalid host transport configuration.
    Invalid,
    /// Scope, actual service or authenticated transport source differs.
    Binding,
    /// Actual caller is not the configured operator.
    Denied,
    /// Restored registry remains inspection-only.
    Fenced,
    /// Another sync is already pending.
    Busy,
    /// Local sync sequence cannot advance.
    Exhausted,
    /// The original pending sync was cancelled, revoked, completed or replaced.
    Conflict,
    /// Reply exceeds the host's transport/decoder byte budget.
    ReplyTooLarge,
    /// Malformed, over-budget, empty or invalid membership reply.
    InvalidReply,
    /// Platform refused to enqueue the provider query; pending sync is retained.
    NotEnqueued,
    /// Raw platform rejection, including bounded-wait uncertainty.
    Rejected(u32),
    /// Retained state could not be read consistently.
    Internal,
}
