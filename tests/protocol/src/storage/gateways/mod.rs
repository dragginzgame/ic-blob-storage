//! Labelled local registry controls; these do not authenticate a provider reply.
use ic_platform::Principal;
use serde::Deserialize;
/// Bounded local gateway root observation; not the provider's liveness API.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct RootsInput {
    /// Exact service/Cashier/namespace; actor comes from actual IC context.
    pub scope: Scope,
    /// Ordered raw roots, including duplicates and malformed positions.
    pub roots: Vec<Vec<u8>>,
}
/// Minimal phase-only local view, never a deletion boolean or effect receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub enum RootView {
    /// Local upload/lifecycle phase; no tenant or operation identity is disclosed.
    Known(crate::admission::ContentState),
    /// No local claim; no provider deletion authority is implied.
    Unknown,
    /// Root could not be parsed.
    Malformed {
        /// Original supplied byte length.
        bytes: u64,
    },
}
/// Adversarial local controls for the explicit replicated transport primitive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct ReplicatedInput {
    /// Previously persisted sync and callback fault selection.
    pub attempt: TransportInput,
    /// Expected executing canister; independently checked against the platform.
    pub service: Principal,
    /// Expected Cashier; independently checked against the canonical request.
    pub cashier: Principal,
    /// Test-only application response byte bound.
    pub max_reply_bytes: u32,
}
/// Local transport control for a retained fixture attempt's durable sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct TransportInput {
    /// Exact installed scope.
    pub scope: Scope,
    /// Existing pending attempt; never provider authority.
    pub token: u64,
    /// Trap the callback's registry write.
    pub callback_fault: bool,
}
/// Explicit configured provider scope for every request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct Scope {
    /// Actual service.
    pub service: Principal,
    /// Configured Cashier candidate.
    pub cashier: Principal,
    /// Local namespace.
    pub namespace: u128,
}
/// Test-only transitions. Sequences identify retained attempts, never ingress authority.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub enum Action {
    /// Operator add.
    Add(Principal),
    /// Reserve a read-only sync token.
    Begin,
    /// Apply a labelled source observation, never a real provider callback.
    Apply {
        /// Durable sequence of an attempt retained by this fixture.
        token: u64,
        /// Synthetic transport source scope.
        source: Scope,
        /// Encoded Cashier gateway-list reply, bounded by host decoder limits.
        reply: Vec<u8>,
    },
}
/// Bounded fixture command, separate from production callback endpoints.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct Command {
    /// Explicit installed scope.
    pub scope: Scope,
    /// Registry transition.
    pub action: Action,
    /// Trap on its stable write.
    pub fault: bool,
}
/// Observable fixture result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub enum Outcome {
    /// Durable local sequence of the newly retained attempt.
    Begun(u64),
    /// Addition membership change.
    Changed(bool),
    /// Complete apply succeeded.
    Applied,
}
/// Passive operator view, not callback or recovery authority.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
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

/// Local transaction fault control around the maintained revocation handler.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct FaultRevocation {
    /// Exact shared operator request.
    pub request: ic_blob_storage_contracts::dto::gateway::GatewayRevocationRequest,
    /// Trap on the registry write; never part of production ingress.
    pub fault: bool,
}

/// Local stable-write interruption around exact shared cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct FaultCancellation {
    /// Exact maintained request.
    pub request: ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncCancellation,
    /// Trap the cancellation write.
    pub fault: bool,
}
