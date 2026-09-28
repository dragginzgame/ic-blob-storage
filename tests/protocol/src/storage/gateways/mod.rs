//! Labelled local registry controls; these do not authenticate a provider reply.
use candid::{CandidType, Deserialize, Principal};
/// Local transport control for a previously reserved fixture handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct TransportInput {
    /// Exact installed scope.
    pub scope: Scope,
    /// Existing pending attempt; never provider authority.
    pub token: u64,
    /// Trap the callback's registry write.
    pub callback_fault: bool,
}
/// Explicit configured provider scope for every request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Scope {
    /// Actual service.
    pub service: Principal,
    /// Configured Cashier candidate.
    pub cashier: Principal,
    /// Local namespace.
    pub namespace: u128,
}
/// Test-only transitions. Tokens are bounded fixture handles, never ingress authority.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Action {
    /// Operator add.
    Add(Principal),
    /// Operator removal, including the final member.
    Remove(Principal),
    /// Reserve a read-only sync token.
    Begin,
    /// Apply a labelled source observation, never a real provider callback.
    Apply {
        /// Handle issued by this fixture.
        token: u64,
        /// Synthetic transport source scope.
        source: Scope,
        /// Encoded Cashier gateway-list reply, bounded by host decoder limits.
        reply: Vec<u8>,
    },
    /// Cancel the exact read-only sync.
    Cancel(u64),
}
/// Bounded fixture command, separate from production callback endpoints.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Command {
    /// Explicit installed scope.
    pub scope: Scope,
    /// Registry transition.
    pub action: Action,
    /// Trap on its stable write.
    pub fault: bool,
}
/// Observable fixture result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Outcome {
    /// Bounded local token handle.
    Begun(u64),
    /// Add/remove membership change.
    Changed(bool),
    /// Complete apply/cancel succeeded.
    Applied,
}
/// Passive operator view, not callback or recovery authority.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct View {
    /// Retained members in order.
    pub members: Vec<Principal>,
    /// Last allocated sync identity.
    pub last_sequence: u64,
    /// Retained outstanding identity.
    pub pending_sequence: Option<u64>,
    /// All restored mutation is fenced.
    pub fenced: bool,
}
