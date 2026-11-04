//! Bounded, account-checked Cashier balance reply decoding without provider calls.
//!
//! Selected method: `account_balance_get_v1` in the retained Cashier Candid,
//! SHA-256 `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
//! Reports do not prove freshness, spendability or the outcome of a payment.

use crate::ops::caffeine::decoding::CaffeineReplyLimits;

use ic_platform::Principal;
use thiserror::Error;

use crate::{
    model::billing::balance::BalanceAmounts,
    ops::billing::balance::{BalanceAmountsInput, BalanceInputError, balance_amounts_from_candid},
};

/// A decoded account balance report, never evidence for repeating a payment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BalanceReply {
    /// All amounts passed validation and the response names the requested account.
    ReportedBalance {
        /// Account matched against trusted request context, not authenticated here.
        account: Principal,
        /// Independently reported amounts; total is not recomputed from components.
        balance: BalanceAmounts,
    },
    /// A structured provider failure; neither category means a zero balance.
    ProviderFailure(BalanceProviderError),
}

/// Advertised provider failure categories, without exposing diagnostic text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BalanceProviderError {
    /// Cashier reports no account; this does not authorize creating or funding one.
    AccountNotFound,
    /// Cashier reports an internal error.
    InternalError,
}

/// Decode one balance result and bind its successful report to the requested account.
///
/// `expected_account` must come from trusted request context. The future workflow
/// must separately bind service, namespace, Cashier and current attempt to the
/// transport response. Account equality alone proves neither source nor freshness.
/// No missing/error reply is converted to zero, and no state or policy is mutated.
/// # Errors
/// Rejects special account principals, oversized/malformed/over-budget replies,
/// a different returned account and any amount outside the unsigned cycle range.
pub fn decode_balance_reply(
    bytes: &[u8],
    expected_account: Principal,
    limits: CaffeineReplyLimits,
) -> Result<BalanceReply, BalanceReplyError> {
    if expected_account == Principal::anonymous()
        || expected_account == Principal::management_canister()
    {
        return Err(BalanceReplyError::InvalidAccount);
    }
    let reply: wire::AccountBalanceGetResult = limits.decode(
        bytes,
        BalanceReplyError::ReplyTooLarge,
        BalanceReplyError::InvalidReply,
    )?;
    match reply {
        Ok(reply) => {
            if reply.account != expected_account.into() {
                return Err(BalanceReplyError::AccountMismatch);
            }
            let amounts = reply.account_cycle_balances;
            let balance = balance_amounts_from_candid(BalanceAmountsInput {
                total: &amounts.total,
                prepaid: &amounts.cycles_prepaid,
                promotional: &amounts.cycles_promo,
                ledger: &amounts.cycles_ledger,
            })?;
            Ok(BalanceReply::ReportedBalance {
                account: reply.account.into(),
                balance,
            })
        }
        Err(error) => Ok(BalanceReply::ProviderFailure(match error {
            wire::AccountBalanceGetError::AccountNotFound => BalanceProviderError::AccountNotFound,
            wire::AccountBalanceGetError::InternalError(_) => BalanceProviderError::InternalError,
        })),
    }
}

/// Unusable balance reply; no variant supplies a substitute balance.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BalanceReplyError {
    /// Anonymous/management cannot identify the requested service account.
    #[error("invalid balance account")]
    InvalidAccount,
    /// Byte limit exceeded before decoding.
    #[error("balance reply exceeds byte limit")]
    ReplyTooLarge,
    /// Malformed, incompatible or above configured decoder work/type bounds.
    #[error("invalid or over-budget balance reply")]
    InvalidReply,
    /// Provider response names an account different from trusted request context.
    #[error("balance account mismatch")]
    AccountMismatch,
    /// At least one reported amount cannot be represented as unsigned cycles.
    #[error(transparent)]
    InvalidBalance(#[from] BalanceInputError),
}

mod wire {
    use ic_platform::candid::contract::CandidType;
    use ic_platform::candid::contract::Principal;
    use serde::Deserialize;

    use super::super::wire::AccountCycleBalances;

    pub(super) type AccountBalanceGetResult =
        Result<AccountBalanceGetResponse, AccountBalanceGetError>;

    #[derive(CandidType, Deserialize)]
    #[candid_path("ic_platform::candid::contract")]
    pub(super) struct AccountBalanceGetResponse {
        pub account_cycle_balances: AccountCycleBalances,
        pub account: Principal,
    }

    #[derive(CandidType, Deserialize)]
    #[candid_path("ic_platform::candid::contract")]
    pub(super) enum AccountBalanceGetError {
        AccountNotFound,
        InternalError(String),
    }
}

#[cfg(test)]
mod tests;
