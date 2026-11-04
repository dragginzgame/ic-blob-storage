use ic_platform::candid::contract::Int;
use std::num::NonZeroUsize;

use super::*;
use crate::ops::billing::balance::BalanceField;
use crate::ops::caffeine::wire::AccountCycleBalances;
use crate::ops::caffeine::wire::DebtTarget;

fn n(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive bound")
}

fn limits() -> CaffeineReplyLimits {
    CaffeineReplyLimits {
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
    hex::decode(hex.trim()).expect("fixture hex")
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
                account: account().into(),
                account_cycle_balances: balance,
            });
            let bytes = ic_platform::candid::contract::encode_one(reply).expect("wire balance");
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
        ic_platform::candid::contract::encode_args(()).expect("absent result"),
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
            CaffeineReplyLimits {
                max_bytes: n(good.len() - 1),
                ..limits()
            },
            BalanceReplyError::ReplyTooLarge,
        ),
        (
            CaffeineReplyLimits {
                decoding_quota: n(1),
                ..limits()
            },
            BalanceReplyError::InvalidReply,
        ),
        (
            CaffeineReplyLimits {
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
            CaffeineReplyLimits {
                max_bytes: n(good.len()),
                ..limits()
            }
        ),
        Ok(BalanceReply::ReportedBalance {
            account: account(),
            balance: BalanceAmounts::new(100, 80, 10, 0)
        })
    );
    let reply: wire::AccountBalanceGetResult = Err(wire::AccountBalanceGetError::AccountNotFound);
    let extra = ic_platform::candid::contract::encode_args((reply, "diagnostic".repeat(10)))
        .expect("extra argument");
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
            CaffeineReplyLimits {
                skipping_quota: n(1),
                ..limits()
            }
        ),
        Err(BalanceReplyError::InvalidReply)
    );
}
