//! Decode Cashier ledger-deposit notifications, never issue or retry a notification.
//!
//! `cycles_ledger_deposit_notify_v1` is an update, not a completion lookup for
//! `account_top_up_v1`. Its response does not echo an account, ledger principal or
//! caller operation identity. The reported block must not be assumed to identify
//! the caller's original deposit. No report here settles another funding attempt.
//! Schema: `docs/evidence/caffeine-cashier.did`, SHA-256
//! `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.

use crate::{
    model::billing::balance::BalanceAmounts,
    ops::billing::balance::{BalanceAmountsInput, BalanceInputError, balance_amounts_from_candid},
};
use candid::{CandidType, Nat, de::DecoderConfig, decode_one_with_config};
use serde::Deserialize;
use std::num::NonZeroUsize;
use thiserror::Error;

/// Explicit per-reply resource limits; transport buffering needs its own bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LedgerDepositReplyLimits {
    /// Maximum supplied bytes, checked before decoding.
    pub max_bytes: NonZeroUsize,
    /// Maximum Candid decoding work.
    pub decoding_quota: NonZeroUsize,
    /// Maximum work skipping additional fields or arguments.
    pub skipping_quota: NonZeroUsize,
    /// Maximum entries in the Candid type table.
    pub max_type_entries: NonZeroUsize,
}

/// Provider-reported figures, not an authenticated or operation-bound receipt.
/// Transport must be bound to the persisted service/Cashier/account request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerDepositReportView {
    balance: BalanceAmounts,
    ledger_block_index: Nat,
    credited_cycles: u128,
}

impl LedgerDepositReportView {
    /// Validated snapshot, independent of the reported credited amount.
    #[must_use]
    pub const fn balance(&self) -> BalanceAmounts {
        self.balance
    }

    /// Exact arbitrary-precision index, bounded by the reply/decoder limits.
    /// Neither its ledger identity nor its relationship to a deposit is established.
    #[must_use]
    pub const fn ledger_block_index(&self) -> &Nat {
        &self.ledger_block_index
    }

    /// Provider-reported credit, including zero; never inferred from balance changes.
    #[must_use]
    pub const fn credited_cycles(&self) -> u128 {
        self.credited_cycles
    }
}

/// A notification response; no variant grants retry or settlement authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerDepositReply {
    /// The complete advertised success result decoded with valid numeric amounts.
    ReportedCredit(LedgerDepositReportView),
    /// A structured failure; this alone does not prove no earlier deposit/credit.
    ProviderFailure(LedgerDepositProviderError),
}

/// All advertised error categories, without exposing provider diagnostic text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LedgerDepositProviderError {
    /// Provider reported a sweep failure; its effects remain unspecified.
    SweepFailed,
    /// Provider reported nothing to deposit, not proof that a prior credit completed.
    NothingToDeposit,
    /// Provider reported an internal failure.
    InternalError,
    /// Reported balance was too small; both amounts remain diagnostic observations.
    DepositTooSmall {
        /// Reported fee; not a configured or current fee schedule.
        fee_cycles: u128,
        /// Reported deposit balance; not authoritative service spendability.
        balance_cycles: u128,
    },
}

/// Numeric field outside the service's unsigned 128-bit cycle range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LedgerDepositAmountField {
    /// Reported credited amount.
    Credited,
    /// Reported deposit fee.
    DepositFee,
    /// Reported deposit balance.
    DepositBalance,
}

/// Unusable notification bytes; retain the original unresolved operation.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum LedgerDepositReplyError {
    /// Size limit exceeded before decoding.
    #[error("ledger notification reply exceeds byte limit")]
    ReplyTooLarge,
    /// Malformed, incompatible or above configured decoder work/type bounds.
    #[error("invalid or over-budget ledger notification reply")]
    InvalidReply,
    /// A balance component is negative or too large.
    #[error(transparent)]
    InvalidBalance(#[from] BalanceInputError),
    /// A cycle amount is too large; it is never truncated or clamped.
    #[error("{field:?} is outside the unsigned 128-bit cycle range")]
    InvalidAmount {
        /// Rejected field.
        field: LedgerDepositAmountField,
    },
}

/// Decode a supplied response without notification, pagination or credit matching.
/// Unknown/missing responses and decode failures never become zero-credit success.
///
/// # Errors
/// Rejects oversized, malformed, incompatible, over-budget or invalid-amount replies.
pub fn decode_ledger_deposit_reply(
    bytes: &[u8],
    limits: LedgerDepositReplyLimits,
) -> Result<LedgerDepositReply, LedgerDepositReplyError> {
    if bytes.len() > limits.max_bytes.get() {
        return Err(LedgerDepositReplyError::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(limits.decoding_quota.get())
        .set_skipping_quota(limits.skipping_quota.get())
        .set_max_type_len(limits.max_type_entries.get())
        .set_full_error_message(false);
    let reply: wire::NotifyResult = decode_one_with_config(bytes, &config)
        .map_err(|_| LedgerDepositReplyError::InvalidReply)?;
    match reply {
        Ok(value) => {
            let balance = balance_amounts_from_candid(BalanceAmountsInput {
                total: &value.balance.total,
                prepaid: &value.balance.cycles_prepaid,
                promotional: &value.balance.cycles_promo,
                ledger: &value.balance.cycles_ledger,
            })?;
            Ok(LedgerDepositReply::ReportedCredit(
                LedgerDepositReportView {
                    balance,
                    ledger_block_index: value.ledger_block_index,
                    credited_cycles: amount(&value.credited, LedgerDepositAmountField::Credited)?,
                },
            ))
        }
        Err(value) => Ok(LedgerDepositReply::ProviderFailure(match value {
            wire::NotifyError::SweepFailed { .. } => LedgerDepositProviderError::SweepFailed,
            wire::NotifyError::NothingToDeposit => LedgerDepositProviderError::NothingToDeposit,
            wire::NotifyError::InternalError { .. } => LedgerDepositProviderError::InternalError,
            wire::NotifyError::DepositTooSmall { fee, balance } => {
                LedgerDepositProviderError::DepositTooSmall {
                    fee_cycles: amount(&fee, LedgerDepositAmountField::DepositFee)?,
                    balance_cycles: amount(&balance, LedgerDepositAmountField::DepositBalance)?,
                }
            }
        })),
    }
}

fn amount(value: &Nat, field: LedgerDepositAmountField) -> Result<u128, LedgerDepositReplyError> {
    u128::try_from(&value.0).map_err(|_| LedgerDepositReplyError::InvalidAmount { field })
}

// Private provider schema with the shared balance DTO; clients cannot become
// alternate owners of this notification contract.
mod wire {
    use super::super::wire::AccountCycleBalances;
    use super::{CandidType, Deserialize, Nat};

    pub(super) type NotifyResult = Result<NotifyResponse, NotifyError>;
    #[derive(CandidType, Deserialize)]
    pub(super) struct NotifyResponse {
        pub balance: AccountCycleBalances,
        pub ledger_block_index: Nat,
        pub credited: Nat,
    }
    #[derive(CandidType, Deserialize)]
    pub(super) enum NotifyError {
        SweepFailed { message: String },
        NothingToDeposit,
        InternalError { message: String },
        DepositTooSmall { fee: Nat, balance: Nat },
    }
}

#[cfg(test)]
mod tests;
