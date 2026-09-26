//! Retain structured Cashier outcomes instead of decoding a reply as `()`.
//!
//! Schema: `docs/evidence/caffeine-cashier.did`, SHA-256
//! `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
//! No call, automatic retry, accepted-cycle accounting or settlement happens here.

use std::num::NonZeroUsize;

use candid::{CandidType, Principal, de::DecoderConfig, decode_one_with_config};
use serde::Deserialize;
use thiserror::Error;

use crate::{
    model::billing::balance::BalanceAmounts,
    ops::billing::balance::{BalanceAmountsInput, BalanceInputError, balance_amounts_from_candid},
};

/// Explicit Candid resource bounds; no production budget is inferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopUpReplyLimits {
    /// Bound the supplied reply before decoding; transport buffering needs its own bound.
    pub max_bytes: NonZeroUsize,
    /// Maximum Candid decoding work under the decoder's cost model.
    pub decoding_quota: NonZeroUsize,
    /// Maximum work spent skipping fields or arguments not consumed by this schema.
    pub skipping_quota: NonZeroUsize,
    /// Maximum entries in the Candid type table.
    pub max_type_entries: NonZeroUsize,
}

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
    limits: TopUpReplyLimits,
) -> Result<TopUpReply, TopUpReplyError> {
    if bytes.len() > limits.max_bytes.get() {
        return Err(TopUpReplyError::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(limits.decoding_quota.get())
        .set_skipping_quota(limits.skipping_quota.get())
        .set_max_type_len(limits.max_type_entries.get())
        .set_full_error_message(false);
    let reply: wire::AccountTopUpResult =
        decode_one_with_config(bytes, &config).map_err(|_| TopUpReplyError::InvalidReply)?;
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
                TopUpProviderError::NotAuthorized(principal)
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
    use super::{CandidType, Deserialize, Principal};

    pub(super) type AccountTopUpResult = Result<AccountTopUpResponse, AccountTopUpError>;

    #[derive(CandidType, Deserialize)]
    pub(super) struct AccountTopUpResponse {
        pub balance: AccountCycleBalances,
        pub message: String,
    }

    #[derive(CandidType, Deserialize)]
    pub(super) enum AccountTopUpError {
        NotAuthorized(Principal),
        AccountBalanceOverflow,
        InternalError(String),
        TopUpWithoutCycles,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::billing::balance::BalanceField;

    fn number(value: usize) -> NonZeroUsize {
        NonZeroUsize::new(value).expect("positive decoding budget")
    }

    fn limits() -> TopUpReplyLimits {
        TopUpReplyLimits {
            max_bytes: number(4096),
            decoding_quota: number(100_000),
            skipping_quota: number(1000),
            max_type_entries: number(32),
        }
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

    const WITHOUT_CYCLES: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-top-up/without-cycles.hex"
    ));
    const SUCCESS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-top-up/success.hex"
    ));

    #[test]
    fn independent_candid_fixtures_preserve_all_provider_error_categories() {
        for (hex, expected) in [
            (WITHOUT_CYCLES, TopUpProviderError::TopUpWithoutCycles),
            (
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/caffeine-top-up/overflow.hex"
                )),
                TopUpProviderError::AccountBalanceOverflow,
            ),
            (
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/caffeine-top-up/internal.hex"
                )),
                TopUpProviderError::InternalError,
            ),
            (
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/caffeine-top-up/unauthorized.hex"
                )),
                TopUpProviderError::NotAuthorized(Principal::anonymous()),
            ),
        ] {
            let bytes = fixture(hex);
            assert_eq!(
                decode_top_up_reply(&bytes, limits()),
                Ok(TopUpReply::ProviderFailure(expected))
            );
        }
    }

    #[test]
    fn success_is_a_balance_report_and_invalid_components_never_become_success() {
        assert_eq!(
            decode_top_up_reply(&fixture(SUCCESS), limits()),
            Ok(TopUpReply::ReportedSuccess {
                balance: BalanceAmounts::new(100, 80, 10, 10)
            })
        );
        let negative = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/caffeine-top-up/negative-prepaid.hex"
        ));
        assert_eq!(
            decode_top_up_reply(&fixture(negative), limits()),
            Err(TopUpReplyError::InvalidBalance(BalanceInputError {
                field: BalanceField::Prepaid
            }))
        );
    }

    #[test]
    fn absent_malformed_unknown_and_over_budget_results_remain_unusable() {
        let known = fixture(WITHOUT_CYCLES);
        for bytes in [
            Vec::new(),
            candid::encode_args(()).expect("empty reply"),
            known[..known.len() - 1].to_vec(),
            fixture(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-top-up/unknown-error.hex"
            ))),
        ] {
            assert_eq!(
                decode_top_up_reply(&bytes, limits()),
                Err(TopUpReplyError::InvalidReply)
            );
        }
        let mut bounded = limits();
        bounded.max_bytes = number(known.len());
        assert!(decode_top_up_reply(&known, bounded).is_ok());
        bounded.max_bytes = number(known.len() - 1);
        assert_eq!(
            decode_top_up_reply(&known, bounded),
            Err(TopUpReplyError::ReplyTooLarge)
        );
        bounded = limits();
        bounded.decoding_quota = number(1);
        assert_eq!(
            decode_top_up_reply(&known, bounded),
            Err(TopUpReplyError::InvalidReply)
        );
        bounded = limits();
        bounded.max_type_entries = number(1);
        assert_eq!(
            decode_top_up_reply(&known, bounded),
            Err(TopUpReplyError::InvalidReply)
        );
    }

    #[test]
    fn additional_reply_data_is_subject_to_the_skipping_budget() {
        let reply: wire::AccountTopUpResult = Err(wire::AccountTopUpError::TopUpWithoutCycles);
        let bytes = candid::encode_args((reply, "unconsumed diagnostic".repeat(4)))
            .expect("reply with an additional argument");
        assert_eq!(
            decode_top_up_reply(&bytes, limits()),
            Ok(TopUpReply::ProviderFailure(
                TopUpProviderError::TopUpWithoutCycles
            ))
        );
        let mut bounded = limits();
        bounded.skipping_quota = number(1);
        assert_eq!(
            decode_top_up_reply(&bytes, bounded),
            Err(TopUpReplyError::InvalidReply)
        );
    }
}
