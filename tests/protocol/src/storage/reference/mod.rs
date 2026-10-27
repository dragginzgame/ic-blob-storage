//! Private controls for exercising the maintained reference receipt client.
use ic_platform::Principal;
use serde::Deserialize;
/// Explicit local proxy controls; not a production service request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct ReferenceClientInput {
    /// Exact shared service request.
    pub request: ic_blob_storage_contracts::dto::reference::ReferenceCommand,
    /// Expected executing tenant.
    pub tenant: Principal,
    /// Encoded response budget.
    pub max_reply_bytes: u32,
}
/// Typed observation of the maintained client.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub enum ReferenceProbeFailure {
    /// Proxy operator check failed.
    Denied,
    /// Invalid client settings or response.
    Invalid,
    /// Actual identity or exact response differs.
    Binding,
    /// Call attempted outside replicated execution.
    Execution,
    /// Platform reject code.
    Rejected(u32),
    /// Call not enqueued.
    NotEnqueued,
    /// Reply budget exceeded.
    Limit,
    /// Authenticated lookup refusal.
    Remote(ic_blob_storage_contracts::dto::reference::ReferenceFailure),
}

/// Private trap injection on the same shared mutation handler as the canonical update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct ReferenceFaultInput {
    /// Exact maintained operation.
    pub request: ic_blob_storage_contracts::dto::reference::ReferenceCommand,
    /// Stable write at which the fixture traps.
    pub fault: super::WriteFault,
}

/// Explicit local consumer of passive exact-reference state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct ReferenceStatusClientInput {
    /// Exact maintained lookup, with no mutation operation or action.
    pub request: ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest,
    /// Expected executing tenant canister.
    pub tenant: Principal,
    /// Maximum encoded reply bytes.
    pub max_reply_bytes: u32,
}
