//! Callback-only diagnostics; unsent outcomes cannot inhabit a callback record.
use ic_platform::principal::ExternalPrincipal;
use serde::Deserialize;
use serde::Serialize;

/// Fixed-size diagnostic facts. No text, wire buffers or inferred credit is stored.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum FundingResponseRecord {
    Rejected(u32),
    Balance {
        total: u128,
        prepaid: u128,
        promotional: u128,
        ledger: u128,
    },
    NotAuthorized(ExternalPrincipal),
    AccountBalanceOverflow,
    InternalError,
    TopUpWithoutCycles,
    ReplyTooLarge,
    InvalidReply,
    InvalidBalance(BalanceFieldRecord),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum BalanceFieldRecord {
    Total,
    Prepaid,
    Promotional,
    Ledger,
}
