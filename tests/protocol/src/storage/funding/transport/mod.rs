//! Explicit local-only transport experiment, not a production payment endpoint.
use super::Intent;
use crate::{
    funding::{FundingOutcome, FundingRequest},
    storage::WriteFault,
};
use candid::{CandidType, Deserialize};

/// Operator control for one journal-bound call to the local Cashier substitute.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Input {
    /// Complete retained funding identity.
    pub intent: Intent,
    /// Explicit positive operating slack for the local liquidity check.
    pub operating_reserve: u128,
    /// Complete additional fixture liabilities, excluding the requested attachment.
    pub other_liabilities: u128,
    /// Optional callback-only write trap to verify retained uncertainty.
    pub callback_fault: Option<WriteFault>,
    /// Fixture-only wrong callback identity to test shared correlation checks.
    pub callback_identity_fault: Option<CallbackIdentityFault>,
}
/// Deliberate fixture corruption after a real call; never production input.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum CallbackIdentityFault {
    /// Decrement the retained attachment.
    Offered,
    /// Substitute the anonymous account.
    Account,
    /// Remove a present target, or supply one when it was absent.
    TargetBalance,
}
/// Immediate local call observation; stored journal accounting remains authoritative.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Observation {
    /// Actual refunded attachment, absent for calls never enqueued.
    pub refunded: Option<u128>,
    /// Accepted attachment, never provider credit.
    pub accepted: u128,
    /// Separate transport/provider interpretation.
    pub outcome: FundingOutcome,
    /// Platform call-cost bound for the exact wire request, excluding attachment.
    pub call_cost: u128,
}
/// Local receiver controls; none of these fields are Cashier wire arguments.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Substitute {
    /// Exact expected canonical bytes; bounded by the fixture before storage.
    pub expected_arguments: Vec<u8>,
    /// Labelled cycle acceptance/reply behavior from the existing test receiver.
    pub behavior: FundingRequest,
}

/// Fixed host-evidence scenarios for the labelled guarded-dispatch experiment.
/// These choices are not accepted by any production adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum EvidenceScenario {
    /// Synthetic complete test facts; no deployed-provider qualification implied.
    Complete,
    /// Every external fact, including liquidity holds, remains unknown.
    Unknown,
    /// First-attempt facts supplied, but complete liquidity holds absent.
    MissingHolds,
}
/// Local experiment control; no arbitrary provider method or evidence payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct DispatchInput {
    /// Exact reserved intent.
    pub intent: Intent,
    /// Additional operating slack for post-write platform liquidity.
    pub operating_reserve: u128,
    /// Other fixture liabilities, excluding this exact offer.
    pub other_liabilities: u128,
    /// Explicit labelled substitute observations.
    pub evidence: EvidenceScenario,
    /// First-marker write fault to verify rollback before any remote effect.
    pub attempt_fault: Option<WriteFault>,
    /// Callback-only storage fault; never production input.
    pub callback_fault: Option<WriteFault>,
}
/// Shared dispatcher result, after durable settlement or refusal before mutation.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum DispatchResult {
    /// First-attempt blockers and independent unknown liquidity holds.
    Blocked {
        /// Current journal and host-evidence blockers.
        blockers: Vec<super::admission::Blocker>,
        /// No complete host liquidity holds were supplied.
        holds_unknown: bool,
    },
    /// Call observation already committed with journal accounting.
    Settled(Observation),
}
