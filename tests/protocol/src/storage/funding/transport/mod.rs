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
