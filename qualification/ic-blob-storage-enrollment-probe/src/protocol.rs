//! Private qualification interface. A received reply is not provider enrollment.

use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};

/// Bounds ingress, inspection and the one durable record.
pub const LIMITS: ic_wire::Limits = ic_wire::Limits {
    // Bulk byte fields still consume Wire's conservative traversal allowance.
    // Include the complete 8 KiB reply plus immutable intent in every budget.
    bytes: 16_384,
    allocation: 262_144,
    nodes: 2048,
    depth: 16,
};

/// Maximum retained provider reply; larger replies leave the attempt unresolved.
pub const REPLY_BYTES: usize = 8192;

/// Immutable intent for one owner-origin, zero-attachment Cashier call.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Installation {
    /// Operator must also remain a controller to invoke or inspect the probe.
    pub operator: Principal,
    /// Explicit target; never discovered or replaced on failure.
    pub cashier: Principal,
    /// Positive requested account balance, at most the pilot's 9.9B credit.
    /// This is not a provider-enforced debit or liability cap.
    pub target_balance: u64,
}

/// Transport disposition only. No variant authorizes retry or establishes credit.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Outcome {
    /// Exact raw reply retained for bounded external decoding, including malformed replies.
    Reply(ic_wire::Bytes<REPLY_BYTES>),
    /// Platform declined submission; this deliberately single-use probe still cannot retry.
    NotEnqueued,
    /// IC rejection or bounded-wait expiry; provider effects may already exist.
    Rejected(u32),
    /// Reply could not be retained within the bound; provider effects remain unresolved.
    ReplyTooLarge(u64),
}

/// Exact installation, request bytes and retained attempt, recoverable after a lost reply.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct View {
    /// Installation-bound provider account; always this canister, never caller input.
    pub owner: Principal,
    /// Immutable operation configuration.
    pub configuration: Installation,
    /// Original Candid arguments to `account_top_up_v1`, prepared before exposure.
    pub arguments: ic_wire::Bytes<1024>,
    /// Committed before dispatch; absence is the only state permitting a first call.
    pub attempted_at: Option<u64>,
    /// Absent after an attempt means pending or interrupted, never safe to repeat.
    pub outcome: Option<Outcome>,
}

/// Authorization failure; a callback still records the result if the operator is revoked.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Error {
    /// Caller is not the configured operator and a current controller.
    Denied,
}
