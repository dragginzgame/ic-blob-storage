use super::*;
use crate::ops::caffeine::decoding::CaffeineReplyLimits;
use ic_platform::candid::decoder;
use std::num::NonZeroUsize;

fn binding() -> PaymentRelationshipBinding {
    PaymentRelationshipBinding {
        paid_canister: Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("owner"),
        payment_account: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").expect("payer"),
    }
}

fn limits() -> CaffeineReplyLimits {
    CaffeineReplyLimits {
        max_bytes: NonZeroUsize::new(4096).expect("positive"),
        decoding_quota: NonZeroUsize::new(100_000).expect("positive"),
        skipping_quota: NonZeroUsize::new(1000).expect("positive"),
        max_type_entries: NonZeroUsize::new(32).expect("positive"),
    }
}

fn fixture(hex: &str) -> Vec<u8> {
    hex::decode(hex.trim()).expect("fixture hex")
}

const LINKED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/caffeine-relationship/linked.hex"
));
const WIDE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/caffeine-relationship/wide-signed.hex"
));

fn report(bytes: &[u8]) -> PaymentRelationshipView {
    let PaymentRelationshipReply::ReportedRelationship(value) =
        decode_payment_relationship_reply(bytes, binding(), limits()).expect("valid reply")
    else {
        panic!("expected a present relationship")
    };
    value
}

#[test]
fn independent_linked_payer_report_preserves_fields_without_spendability_arithmetic() {
    let value = report(&fixture(LINKED));
    assert_eq!(value.binding(), binding());
    assert_eq!(value.spending_limit_per_day(), &Int::from(100));
    assert_eq!(value.current_period_spent(), &Int::from(120));
    assert_eq!(value.current_period_start(), 2);
    assert_eq!(value.added_timestamp(), 1);
    assert_eq!(value.expiration_timestamp(), Some(3));
    assert_eq!(value.bandwidth_baseline_uploaded(), &Nat::from(4_u8));
    assert_eq!(value.bandwidth_baseline_downloaded(), &Nat::from(5_u8));
    assert_eq!(value.bandwidth_baseline_ts_ns(), 6);

    let value = report(&fixture(WIDE));
    let huge = b"340282366920938463463374607431768211456";
    assert_eq!(
        value.spending_limit_per_day(),
        &Int::parse(b"-340282366920938463463374607431768211456").expect("signed")
    );
    assert_eq!(
        value.current_period_spent(),
        &Int::parse(huge).expect("wide")
    );
    assert_eq!(
        value.bandwidth_baseline_uploaded(),
        &Nat::parse(huge).expect("wide")
    );
    assert_eq!(value.bandwidth_baseline_downloaded(), &Nat::from(0_u8));
    assert_eq!(value.current_period_start(), u64::MAX);
    assert_eq!(value.added_timestamp(), 0);
    assert_eq!(value.bandwidth_baseline_ts_ns(), u64::MAX);
    assert_eq!(value.expiration_timestamp(), None);
}

#[test]
fn absence_and_all_advertised_errors_remain_distinct() {
    use PaymentRelationshipProviderError as Provider;
    let cases = [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-relationship/absent.hex"
            )),
            PaymentRelationshipReply::NoRelationshipReported,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-relationship/not-found.hex"
            )),
            PaymentRelationshipReply::ProviderFailure(Provider::RelationshipNotFound(
                binding().paid_canister,
            )),
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-relationship/denied.hex"
            )),
            PaymentRelationshipReply::ProviderFailure(Provider::NotAuthorized(
                binding().payment_account,
            )),
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-relationship/invalid.hex"
            )),
            PaymentRelationshipReply::ProviderFailure(Provider::InvalidRequest),
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-relationship/internal.hex"
            )),
            PaymentRelationshipReply::ProviderFailure(Provider::InternalError),
        ),
    ];
    for (hex, expected) in cases {
        assert_eq!(
            decode_payment_relationship_reply(&fixture(hex), binding(), limits()),
            Ok(expected)
        );
    }
}

#[test]
fn an_explicit_self_payer_is_supported_without_an_absence_fallback() {
    let mut reply: wire::GetResult =
        decoder::decode_one_with_config(&fixture(LINKED), &decoder::DecoderConfig::new())
            .expect("fixture");
    reply
        .as_mut()
        .ok()
        .expect("success")
        .relationship
        .as_mut()
        .expect("present")
        .payment_account = binding().paid_canister;
    let expected = PaymentRelationshipBinding {
        payment_account: binding().paid_canister,
        ..binding()
    };
    let bytes = ic_platform::candid::contract::encode_one(reply).expect("self relationship");
    let PaymentRelationshipReply::ReportedRelationship(value) =
        decode_payment_relationship_reply(&bytes, expected, limits()).expect("exact self payer")
    else {
        panic!("expected present self relationship")
    };
    assert_eq!(value.binding(), expected);
}

#[test]
fn both_expected_principals_are_required_even_for_absent_or_failed_replies() {
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        for expected in [
            PaymentRelationshipBinding {
                paid_canister: invalid,
                ..binding()
            },
            PaymentRelationshipBinding {
                payment_account: invalid,
                ..binding()
            },
        ] {
            assert_eq!(
                decode_payment_relationship_reply(&[], expected, limits()),
                Err(PaymentRelationshipReplyError::InvalidBinding)
            );
        }
    }
    let other = Principal::from_slice(&[42]);
    for (expected, error) in [
        (
            PaymentRelationshipBinding {
                paid_canister: other,
                ..binding()
            },
            PaymentRelationshipReplyError::PaidCanisterMismatch,
        ),
        (
            PaymentRelationshipBinding {
                payment_account: other,
                ..binding()
            },
            PaymentRelationshipReplyError::PaymentAccountMismatch,
        ),
        (
            PaymentRelationshipBinding {
                payment_account: binding().paid_canister,
                ..binding()
            },
            PaymentRelationshipReplyError::PaymentAccountMismatch,
        ),
    ] {
        assert_eq!(
            decode_payment_relationship_reply(&fixture(LINKED), expected, limits()),
            Err(error)
        );
    }
}

#[test]
fn malformed_unknown_and_resource_exhaustion_never_report_a_relationship() {
    #[derive(ic_platform::candid::contract::CandidType)]
    #[candid_path("ic_platform::candid::contract")]
    enum Unknown {
        NewError,
    }
    let unknown: Result<wire::GetResponse, Unknown> = Err(Unknown::NewError);
    for bytes in [
        Vec::new(),
        b"DIDL".to_vec(),
        ic_platform::candid::contract::encode_one(unknown).expect("unknown error"),
        fixture(LINKED)[..20].to_vec(),
    ] {
        assert_eq!(
            decode_payment_relationship_reply(&bytes, binding(), limits()),
            Err(PaymentRelationshipReplyError::InvalidReply)
        );
    }
    let bytes = fixture(LINKED);
    for (bounded, error) in [
        (
            CaffeineReplyLimits {
                max_bytes: NonZeroUsize::new(bytes.len() - 1).expect("positive"),
                ..limits()
            },
            PaymentRelationshipReplyError::ReplyTooLarge,
        ),
        (
            CaffeineReplyLimits {
                decoding_quota: NonZeroUsize::MIN,
                ..limits()
            },
            PaymentRelationshipReplyError::InvalidReply,
        ),
        (
            CaffeineReplyLimits {
                max_type_entries: NonZeroUsize::MIN,
                ..limits()
            },
            PaymentRelationshipReplyError::InvalidReply,
        ),
    ] {
        assert_eq!(
            decode_payment_relationship_reply(&bytes, binding(), bounded),
            Err(error)
        );
    }
    assert!(matches!(
        decode_payment_relationship_reply(
            &bytes,
            binding(),
            CaffeineReplyLimits {
                max_bytes: NonZeroUsize::new(bytes.len()).expect("positive"),
                ..limits()
            }
        ),
        Ok(PaymentRelationshipReply::ReportedRelationship(_))
    ));
}

#[test]
fn optional_subtyping_is_not_evidence_of_self_payment_and_skipping_is_bounded() {
    #[derive(ic_platform::candid::contract::CandidType)]
    #[candid_path("ic_platform::candid::contract")]
    struct Extended {
        extension: Vec<u64>,
    }
    #[derive(ic_platform::candid::contract::CandidType)]
    #[candid_path("ic_platform::candid::contract")]
    struct Incompatible {
        relationship: Option<bool>,
    }
    // Candid permits omitted optional fields. NoRelationshipReported deliberately
    // means less than proven absence; method/source binding is a transport duty.
    let reply: Result<Extended, wire::GetError> = Ok(Extended {
        extension: vec![1; 64],
    });
    let bytes = ic_platform::candid::contract::encode_one(reply).expect("extended response");
    assert_eq!(
        decode_payment_relationship_reply(&bytes, binding(), limits()),
        Ok(PaymentRelationshipReply::NoRelationshipReported)
    );
    assert_eq!(
        decode_payment_relationship_reply(
            &bytes,
            binding(),
            CaffeineReplyLimits {
                skipping_quota: NonZeroUsize::MIN,
                ..limits()
            }
        ),
        Err(PaymentRelationshipReplyError::InvalidReply)
    );
    let reply: Result<Incompatible, wire::GetError> = Ok(Incompatible {
        relationship: Some(true),
    });
    let bytes =
        ic_platform::candid::contract::encode_one(reply).expect("incompatible optional field");
    assert_eq!(
        decode_payment_relationship_reply(&bytes, binding(), limits()),
        Ok(PaymentRelationshipReply::NoRelationshipReported)
    );
}
