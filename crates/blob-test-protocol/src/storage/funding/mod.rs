//! Local funding bookkeeping and explicitly labelled substitute transport.
pub mod admission;
pub mod summary;
pub mod transport;
use super::WriteFault;
use candid::{CandidType, Deserialize, Principal};

/// Private synthetic host-credit fixture input; never a production endpoint DTO.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct CreditCommand {
    /// Original exact funding scope and identity.
    pub intent: Intent,
    /// Full accepted attachment covered by the synthetic independent receipt.
    pub accepted: u128,
    /// Fingerprint retained to test replay and reuse refusal.
    pub receipt_digest: [u8; 32],
    /// Simulate whether the trusted host established independent credit.
    pub established: bool,
    /// Optional stable write trap for whole-message rollback evidence.
    pub fault: Option<super::WriteFault>,
}
/// Complete exact local identity, without inferred payer/provider scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Intent {
    /// Actual service.
    pub service: Principal,
    /// Configured Cashier candidate.
    pub cashier: Principal,
    /// Configured payment account.
    pub account: Principal,
    /// Installed namespace.
    pub namespace: u128,
    /// Explicit lifetime operation identity.
    pub operation: u128,
    /// Full positive attachment.
    pub offered: u128,
    /// Optional exact Cashier target balance.
    pub target_balance: Option<u128>,
}
/// Local progress only; no provider credit or effect authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Phase {
    /// Reserved before an attempt marker.
    Prepared,
    /// Possibly dispatched, full attachment charged.
    Uncertain,
    /// Labelled proof of enqueue failure.
    NotEnqueued,
    /// Labelled exact callback refund.
    Callback(u128),
}
/// Operator-only local bookkeeping transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Action {
    /// Reserve attachment.
    Prepare,
    /// Persist first possible dispatch; never actually sends a call.
    Attempt,
    /// Supply labelled enqueue-failure evidence.
    NotEnqueued,
    /// Supply labelled exact callback refund.
    Callback(u128),
}
/// Bounded test command, not a production payment endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Command {
    /// Exact immutable identity.
    pub intent: Intent,
    /// Bookkeeping transition only.
    pub action: Action,
    /// Optional fixture-only write trap.
    pub fault: Option<WriteFault>,
    /// Labelled transport source override for correlation rejection tests only.
    pub source: Option<Principal>,
}
/// Canonical shared request inspection; no dispatch capability.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Request {
    /// Explicit Cashier target.
    pub cashier: Principal,
    /// Maintained provider method.
    pub method: String,
    /// Exact canonical Candid arguments.
    pub arguments: Vec<u8>,
    /// Exact cycle attachment, independent of reported credit.
    pub offered: u128,
}
/// Local allocation observation, never provider credit/spendability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Allocation {
    /// Available installed attachment allocation, including reserve.
    pub available: u128,
    /// Lifetime transport acceptance, still charged after host credit confirmation.
    pub accepted: u128,
    /// Exact lifetime callback refunds.
    pub refunded: u128,
    /// Lifetime proven unsent amounts.
    pub not_enqueued: u128,
    /// Entire reserved/uncertain attachment.
    pub uncertain: u128,
    /// Permanent restore fence.
    pub fenced: bool,
}

/// Private fixture request for one local host-authorized budget increase.
#[derive(Clone, Copy, Debug, candid::CandidType, candid::Deserialize)]
pub struct RenewalCommand {
    /// Exact original credited intent; not a new payment.
    pub intent: Intent,
    /// Additional allowance, not received/refunded cycles.
    pub additional: u128,
    /// Explicit atomic-write fault, absent in ordinary execution.
    pub fault: Option<super::WriteFault>,
}
