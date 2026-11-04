//! Retain structured Cashier outcomes instead of decoding a reply as `()`.
//!
//! Schema: `docs/evidence/caffeine-cashier.did`, SHA-256
//! `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
//! Decoding alone performs no effect or settlement. The explicit [`transport`]
//! primitive captures exact IC attachment refunds; it never retries automatically.

use crate::ops::caffeine::decoding::CaffeineReplyLimits;
pub mod request;
pub mod transport;

use ic_platform::Principal;
use ic_platform::candid::contract::CandidType;
use serde::Deserialize;
use thiserror::Error;

use crate::{
    model::billing::balance::BalanceAmounts,
    ops::billing::balance::{BalanceAmountsInput, BalanceInputError, balance_amounts_from_candid},
};

/// A decoded provider report; neither variant authorizes another payment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopUpReply {
    /// Cashier returned `Ok` and all reported balance amounts passed validation.
    ///
    /// This is not proof of the credited/accepted amount or an operation receipt.
    /// The response has neither field; do not infer either from balance differences.
    ReportedSuccess {
        /// Validated snapshot, without account freshness or settlement guarantees.
        balance: BalanceAmounts,
    },
    /// Cashier returned a structured failure, even though the IC call replied.
    ///
    /// A failure category alone does not prove zero cycles were accepted/refunded.
    ProviderFailure(TopUpProviderError),
}

/// The advertised top-up error categories, without treating diagnostic text as data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopUpProviderError {
    /// Provider rejected the identified principal's authority.
    NotAuthorized(Principal),
    /// Provider reported an account balance overflow.
    AccountBalanceOverflow,
    /// Provider reported an internal failure; diagnostic text is not exposed.
    InternalError,
    /// Provider reported that the call supplied no cycles.
    TopUpWithoutCycles,
}

/// Decode the full selected top-up result, retaining every supported error kind.
///
/// The transport/workflow must establish the expected Cashier, account and exact
/// in-flight operation. Missing responses and any decode failure remain unresolved;
/// none may be converted to success, zero accepted cycles or automatic retry.
/// # Errors
/// Returns a typed error for oversized, malformed, incompatible or over-budget
/// replies, or a reported success containing an invalid balance amount.
pub fn decode_top_up_reply(
    bytes: &[u8],
    limits: CaffeineReplyLimits,
) -> Result<TopUpReply, TopUpReplyError> {
    let reply: wire::AccountTopUpResult = limits.decode(
        bytes,
        TopUpReplyError::ReplyTooLarge,
        TopUpReplyError::InvalidReply,
    )?;
    match reply {
        Ok(reply) => {
            let balance = balance_amounts_from_candid(BalanceAmountsInput {
                total: &reply.balance.total,
                prepaid: &reply.balance.cycles_prepaid,
                promotional: &reply.balance.cycles_promo,
                ledger: &reply.balance.cycles_ledger,
            })?;
            Ok(TopUpReply::ReportedSuccess { balance })
        }
        Err(error) => Ok(TopUpReply::ProviderFailure(match error {
            wire::AccountTopUpError::NotAuthorized(principal) => {
                TopUpProviderError::NotAuthorized(principal.into())
            }
            wire::AccountTopUpError::AccountBalanceOverflow => {
                TopUpProviderError::AccountBalanceOverflow
            }
            wire::AccountTopUpError::InternalError(_) => TopUpProviderError::InternalError,
            wire::AccountTopUpError::TopUpWithoutCycles => TopUpProviderError::TopUpWithoutCycles,
        })),
    }
}

/// A reply that cannot be used to resolve a funding operation.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TopUpReplyError {
    /// Byte limit exceeded before Candid decoding.
    #[error("top-up reply exceeds byte limit")]
    ReplyTooLarge,
    /// Malformed, incompatible or above the configured decoder work/type bound.
    #[error("invalid or over-budget top-up reply")]
    InvalidReply,
    /// A reported success contains a negative or out-of-range balance component.
    #[error(transparent)]
    InvalidBalance(#[from] BalanceInputError),
}

// Provider DTOs stay private and passive. Both adapters must use this decoder;
// exposing these types would allow another owner to silently discard outcomes.
mod wire {
    use super::super::wire::AccountCycleBalances;
    use super::CandidType;
    use super::Deserialize;
    use ic_platform::candid::contract::Principal;

    pub(super) type AccountTopUpResult = Result<AccountTopUpResponse, AccountTopUpError>;

    #[derive(CandidType, Deserialize)]
    #[candid_path("ic_platform::candid::contract")]
    pub(super) struct AccountTopUpResponse {
        pub balance: AccountCycleBalances,
        pub message: String,
    }

    #[derive(CandidType, Deserialize)]
    #[candid_path("ic_platform::candid::contract")]
    pub(super) enum AccountTopUpError {
        NotAuthorized(Principal),
        AccountBalanceOverflow,
        InternalError(String),
        TopUpWithoutCycles,
    }
}

#[cfg(test)]
mod tests;
