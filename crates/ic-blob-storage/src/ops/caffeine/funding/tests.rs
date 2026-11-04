use super::*;
use crate::ops::billing::balance::BalanceField;
use std::num::NonZeroUsize;

fn number(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive decoding budget")
}

fn limits() -> CaffeineReplyLimits {
    CaffeineReplyLimits {
        max_bytes: number(4096),
        decoding_quota: number(100_000),
        skipping_quota: number(1000),
        max_type_entries: number(32),
    }
}

fn fixture(hex: &str) -> Vec<u8> {
    hex::decode(hex.trim()).expect("fixture hex")
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
        ic_platform::candid::contract::encode_args(()).expect("empty reply"),
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
    let bytes =
        ic_platform::candid::contract::encode_args((reply, "unconsumed diagnostic".repeat(4)))
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
