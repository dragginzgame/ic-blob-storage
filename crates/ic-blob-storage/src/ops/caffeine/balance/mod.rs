//! Bounded, account-checked Cashier balance reply decoding without provider calls.
//!
//! Selected method: `account_balance_get_v1` in the retained Cashier Candid,
//! SHA-256 `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
//! Reports do not prove freshness, spendability or the outcome of a payment.

use std::num::NonZeroUsize;

use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use thiserror::Error;

use crate::{
    model::billing::balance::BalanceAmounts,
    ops::billing::balance::{BalanceAmountsInput, BalanceInputError, balance_amounts_from_candid},
};

/// Explicit resource bounds for a balance reply; no production budget is inferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BalanceReplyLimits {
    /// Bound supplied bytes before decoding; transport buffering needs its own bound.
    pub max_bytes: NonZeroUsize,
    /// Maximum Candid decoding work under the decoder's cost model.
    pub decoding_quota: NonZeroUsize,
    /// Maximum work spent skipping additional fields or arguments.
    pub skipping_quota: NonZeroUsize,
    /// Maximum entries in the Candid type table.
    pub max_type_entries: NonZeroUsize,
}

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
    limits: BalanceReplyLimits,
) -> Result<BalanceReply, BalanceReplyError> {
    if expected_account == Principal::anonymous()
        || expected_account == Principal::management_canister()
    {
        return Err(BalanceReplyError::InvalidAccount);
    }
    if bytes.len() > limits.max_bytes.get() {
        return Err(BalanceReplyError::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(limits.decoding_quota.get())
        .set_skipping_quota(limits.skipping_quota.get())
        .set_max_type_len(limits.max_type_entries.get())
        .set_full_error_message(false);
    let reply: wire::AccountBalanceGetResult =
        decode_one_with_config(bytes, &config).map_err(|_| BalanceReplyError::InvalidReply)?;
    match reply {
        Ok(reply) => {
            if reply.account != expected_account {
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
                account: reply.account,
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
    use candid::{CandidType, Principal};
    use serde::Deserialize;

    use super::super::wire::AccountCycleBalances;

    pub(super) type AccountBalanceGetResult =
        Result<AccountBalanceGetResponse, AccountBalanceGetError>;

    #[derive(CandidType, Deserialize)]
    pub(super) struct AccountBalanceGetResponse {
        pub account_cycle_balances: AccountCycleBalances,
        pub account: Principal,
    }

    #[derive(CandidType, Deserialize)]
    pub(super) enum AccountBalanceGetError {
        AccountNotFound,
        InternalError(String),
    }
}

#[cfg(test)]
mod tests {
    use candid::Int;

    use super::*;
    use crate::ops::{
        billing::balance::BalanceField,
        caffeine::wire::{AccountCycleBalances, DebtTarget},
    };

    fn n(value: usize) -> NonZeroUsize {
        NonZeroUsize::new(value).expect("positive bound")
    }

    fn limits() -> BalanceReplyLimits {
        BalanceReplyLimits {
            max_bytes: n(4096),
            decoding_quota: n(100_000),
            skipping_quota: n(1000),
            max_type_entries: n(32),
        }
    }

    fn account() -> Principal {
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("fixture account")
    }

    fn fixture(hex: &str) -> Vec<u8> {
        let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
        assert!(remainder.is_empty(), "complete fixture hex bytes");
        pairs
            .iter()
            .map(|pair| {
                u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16)
                    .expect("fixture hex byte")
            })
            .collect()
    }

    const SUCCESS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-balance/success.hex"
    ));
    const NOT_FOUND: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-balance/not-found.hex"
    ));

    #[test]
    fn independent_fixtures_keep_reports_and_provider_failures_distinct() {
        assert_eq!(
            decode_balance_reply(&fixture(SUCCESS), account(), limits()),
            Ok(BalanceReply::ReportedBalance {
                account: account(),
                balance: BalanceAmounts::new(100, 80, 10, 0)
            })
        );
        for (hex, error) in [
            (NOT_FOUND, BalanceProviderError::AccountNotFound),
            (
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/caffeine-balance/internal.hex"
                )),
                BalanceProviderError::InternalError,
            ),
        ] {
            assert_eq!(
                decode_balance_reply(&fixture(hex), account(), limits()),
                Ok(BalanceReply::ProviderFailure(error))
            );
        }
        assert_eq!(
            decode_balance_reply(
                &fixture(include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/caffeine-balance/negative-ledger.hex"
                ))),
                account(),
                limits()
            ),
            Err(BalanceReplyError::InvalidBalance(BalanceInputError {
                field: BalanceField::Ledger
            }))
        );
    }

    #[test]
    fn account_binding_rejects_before_exposing_any_reported_amounts() {
        for invalid in [Principal::anonymous(), Principal::management_canister()] {
            assert_eq!(
                decode_balance_reply(&[], invalid, limits()),
                Err(BalanceReplyError::InvalidAccount)
            );
        }
        let other = Principal::from_slice(&[9, 1]);
        for hex in [
            SUCCESS,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-balance/negative-ledger.hex"
            )),
        ] {
            assert_eq!(
                decode_balance_reply(&fixture(hex), other, limits()),
                Err(BalanceReplyError::AccountMismatch)
            );
        }
    }

    #[test]
    fn every_wire_amount_must_fit_before_the_report_is_usable() {
        for invalid in [
            Int::from(-1),
            Int::parse(b"340282366920938463463374607431768211456").expect("oversized"),
        ] {
            for field in [
                BalanceField::Total,
                BalanceField::Prepaid,
                BalanceField::Promotional,
                BalanceField::Ledger,
            ] {
                let mut balance = AccountCycleBalances {
                    total: Int::from(u128::MAX),
                    cycles_prepaid: Int::from(u128::MAX),
                    cycles_promo: Int::from(u128::MAX),
                    cycles_ledger: Int::from(u128::MAX),
                    debt_target: DebtTarget::Ledger,
                };
                match field {
                    BalanceField::Total => balance.total = invalid.clone(),
                    BalanceField::Prepaid => balance.cycles_prepaid = invalid.clone(),
                    BalanceField::Promotional => balance.cycles_promo = invalid.clone(),
                    BalanceField::Ledger => balance.cycles_ledger = invalid.clone(),
                }
                let reply: wire::AccountBalanceGetResult = Ok(wire::AccountBalanceGetResponse {
                    account: account(),
                    account_cycle_balances: balance,
                });
                let bytes = candid::encode_one(reply).expect("wire balance");
                assert_eq!(
                    decode_balance_reply(&bytes, account(), limits()),
                    Err(BalanceReplyError::InvalidBalance(BalanceInputError {
                        field
                    }))
                );
            }
        }
    }

    #[test]
    fn malformed_and_over_budget_replies_never_become_a_balance() {
        let good = fixture(SUCCESS);
        let mut trailing = good.clone();
        trailing.push(0);
        for bytes in [
            Vec::new(),
            candid::encode_args(()).expect("absent result"),
            good[..good.len() - 1].to_vec(),
            trailing,
            fixture(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-balance/unknown-error.hex"
            ))),
        ] {
            assert_eq!(
                decode_balance_reply(&bytes, account(), limits()),
                Err(BalanceReplyError::InvalidReply)
            );
        }
        for (bounded, error) in [
            (
                BalanceReplyLimits {
                    max_bytes: n(good.len() - 1),
                    ..limits()
                },
                BalanceReplyError::ReplyTooLarge,
            ),
            (
                BalanceReplyLimits {
                    decoding_quota: n(1),
                    ..limits()
                },
                BalanceReplyError::InvalidReply,
            ),
            (
                BalanceReplyLimits {
                    max_type_entries: n(1),
                    ..limits()
                },
                BalanceReplyError::InvalidReply,
            ),
        ] {
            assert_eq!(decode_balance_reply(&good, account(), bounded), Err(error));
        }
        assert_eq!(
            decode_balance_reply(
                &good,
                account(),
                BalanceReplyLimits {
                    max_bytes: n(good.len()),
                    ..limits()
                }
            ),
            Ok(BalanceReply::ReportedBalance {
                account: account(),
                balance: BalanceAmounts::new(100, 80, 10, 0)
            })
        );
        let reply: wire::AccountBalanceGetResult =
            Err(wire::AccountBalanceGetError::AccountNotFound);
        let extra = candid::encode_args((reply, "diagnostic".repeat(10))).expect("extra argument");
        assert_eq!(
            decode_balance_reply(&extra, account(), limits()),
            Ok(BalanceReply::ProviderFailure(
                BalanceProviderError::AccountNotFound
            ))
        );
        assert_eq!(
            decode_balance_reply(
                &extra,
                account(),
                BalanceReplyLimits {
                    skipping_quota: n(1),
                    ..limits()
                }
            ),
            Err(BalanceReplyError::InvalidReply)
        );
    }
}
