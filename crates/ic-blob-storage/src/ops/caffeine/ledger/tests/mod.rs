use super::*;
use crate::ops::billing::balance::BalanceField;

fn number(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).unwrap()
}
fn limits() -> LedgerDepositReplyLimits {
    LedgerDepositReplyLimits {
        max_bytes: number(4096),
        decoding_quota: number(100_000),
        skipping_quota: number(1000),
        max_type_entries: number(32),
    }
}
fn fixture(hex: &str) -> Vec<u8> {
    let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());
    pairs
        .iter()
        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
        .collect()
}
macro_rules! bytes {
    ($name:literal) => {
        fixture(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/caffeine-ledger/",
            $name,
            ".hex"
        )))
    };
}

#[test]
fn credit_and_block_index_are_preserved_independently_of_balance() {
    let LedgerDepositReply::ReportedCredit(report) =
        decode_ledger_deposit_reply(&bytes!("credit"), limits()).unwrap()
    else {
        panic!("expected credit report")
    };
    assert_eq!(report.credited_cycles(), u128::MAX);
    assert_eq!(report.balance().total(), 7);
    assert_eq!(
        report.ledger_block_index(),
        &Nat::parse(b"340282366920938463463374607431768211456").unwrap()
    );
    let LedgerDepositReply::ReportedCredit(zero) =
        decode_ledger_deposit_reply(&bytes!("zero"), limits()).unwrap()
    else {
        panic!("expected zero report")
    };
    assert_eq!(zero.credited_cycles(), 0);
    assert_eq!(zero.ledger_block_index(), &Nat::from(0_u8));
    assert_eq!(zero.balance(), report.balance());
}

#[test]
fn all_provider_errors_are_distinct_and_diagnostic_text_is_not_exposed() {
    for (bytes, error) in [
        (
            bytes!("nothing"),
            LedgerDepositProviderError::NothingToDeposit,
        ),
        (bytes!("sweep"), LedgerDepositProviderError::SweepFailed),
        (
            bytes!("internal"),
            LedgerDepositProviderError::InternalError,
        ),
        (
            bytes!("small"),
            LedgerDepositProviderError::DepositTooSmall {
                fee_cycles: u128::MAX,
                balance_cycles: 0,
            },
        ),
    ] {
        assert_eq!(
            decode_ledger_deposit_reply(&bytes, limits()),
            Ok(LedgerDepositReply::ProviderFailure(error))
        );
    }
}

#[test]
fn no_numeric_failure_becomes_a_partial_credit_report() {
    for (bytes, field) in [
        (
            bytes!("credit-overflow"),
            LedgerDepositAmountField::Credited,
        ),
        (bytes!("fee-overflow"), LedgerDepositAmountField::DepositFee),
        (
            bytes!("balance-overflow"),
            LedgerDepositAmountField::DepositBalance,
        ),
    ] {
        assert_eq!(
            decode_ledger_deposit_reply(&bytes, limits()),
            Err(LedgerDepositReplyError::InvalidAmount { field })
        );
    }
    for (bytes, field) in [
        (bytes!("negative-total"), BalanceField::Total),
        (bytes!("negative-cycles_prepaid"), BalanceField::Prepaid),
        (bytes!("negative-cycles_promo"), BalanceField::Promotional),
        (bytes!("negative-cycles_ledger"), BalanceField::Ledger),
    ] {
        assert_eq!(
            decode_ledger_deposit_reply(&bytes, limits()),
            Err(LedgerDepositReplyError::InvalidBalance(BalanceInputError {
                field
            }))
        );
    }
}

#[test]
fn malformed_oversized_and_over_budget_replies_are_not_results() {
    let encoded = bytes!("credit");
    assert_eq!(
        decode_ledger_deposit_reply(
            &encoded,
            LedgerDepositReplyLimits {
                max_bytes: number(encoded.len() - 1),
                ..limits()
            }
        ),
        Err(LedgerDepositReplyError::ReplyTooLarge)
    );
    assert!(matches!(
        decode_ledger_deposit_reply(
            &encoded,
            LedgerDepositReplyLimits {
                max_bytes: number(encoded.len()),
                ..limits()
            }
        ),
        Ok(LedgerDepositReply::ReportedCredit(_))
    ));
    for bytes in [&b"not candid"[..], &encoded[..encoded.len() - 1], &[]] {
        assert_eq!(
            decode_ledger_deposit_reply(bytes, limits()),
            Err(LedgerDepositReplyError::InvalidReply)
        );
    }
    for constrained in [
        LedgerDepositReplyLimits {
            decoding_quota: number(1),
            ..limits()
        },
        LedgerDepositReplyLimits {
            max_type_entries: number(1),
            ..limits()
        },
    ] {
        assert_eq!(
            decode_ledger_deposit_reply(&encoded, constrained),
            Err(LedgerDepositReplyError::InvalidReply)
        );
    }
    // A well-formed direct top-up response cannot impersonate a ledger credit.
    let direct = fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-top-up/success.hex"
    )));
    assert_eq!(
        decode_ledger_deposit_reply(&direct, limits()),
        Err(LedgerDepositReplyError::InvalidReply)
    );
}

#[test]
fn extensions_are_bounded_and_unknown_failures_remain_unusable() {
    let extended = bytes!("extended");
    assert_eq!(
        decode_ledger_deposit_reply(
            &extended,
            LedgerDepositReplyLimits {
                skipping_quota: number(100_000),
                ..limits()
            }
        ),
        decode_ledger_deposit_reply(&bytes!("zero"), limits())
    );
    assert_eq!(
        decode_ledger_deposit_reply(
            &extended,
            LedgerDepositReplyLimits {
                skipping_quota: number(1),
                ..limits()
            }
        ),
        Err(LedgerDepositReplyError::InvalidReply)
    );
    assert_eq!(
        decode_ledger_deposit_reply(&bytes!("unknown"), limits()),
        Err(LedgerDepositReplyError::InvalidReply)
    );
}
