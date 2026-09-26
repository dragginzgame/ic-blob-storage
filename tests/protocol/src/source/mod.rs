//! Inspection of the local source journal; never provider reconciliation evidence.
use crate::{SourceMode, journey::readback::ReadSourceConfig};
use candid::{CandidType, Principal};
use serde::Deserialize;

/// Local service call whose intent was committed before dispatch.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum SourceAction {
    /// Ask the bound service to refresh its gateway list.
    Sync,
    /// Ask the bound service to revoke its gateway.
    Revoke,
    /// Submit exactly these deletion confirmations to the bound service.
    Delete(Vec<[u8; 32]>),
}

/// Read-only local call history; vector position identifies the lifetime attempt.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct SourceEffectView {
    /// Exact action and payload; service binding is retained by the journal.
    pub action: SourceAction,
    /// Whether a successful local method result was observed; absent is uncertain.
    /// Neither value proves provider deletion or billing cessation.
    pub succeeded: Option<bool>,
}

/// Driver-only retained state. Restored instances remain permanently fenced.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct SourceRecoveryView {
    /// Restoration blocks all operational endpoints, with no reset control.
    pub fenced: bool,
    /// Service targeted by every recorded action.
    pub service: Principal,
    /// Gateway currently advertised by the fixture.
    pub gateway: Principal,
    /// Explicit inspection/control authority, independent of controllers.
    pub driver: Principal,
    /// Retained gateway-list response mode.
    pub mode: SourceMode,
    /// At most one retained 1 MiB leaf, not a provider object archive.
    pub read: Option<ReadSourceConfig>,
    /// Whether an admitted read had not finished at the recorded boundary.
    pub read_pending: bool,
    /// Whether the driver had released that read's hold.
    pub read_ready: bool,
    /// Bounded lifetime history, including unresolved actions.
    pub effects: Vec<SourceEffectView>,
}
