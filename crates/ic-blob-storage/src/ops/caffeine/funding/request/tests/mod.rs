use super::*;
fn fixture(hex: &str) -> Vec<u8> {
    let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());
    pairs
        .iter()
        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
        .collect()
}
#[test]
fn exact_provider_schema_vectors_preserve_explicit_account_and_optional_target() {
    let account = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let cashier = Principal::from_text("72ch2-fiaaa-aaaar-qbsvq-cai").unwrap();
    for (target, hex) in [
        (
            None,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-top-up/request-explicit-account.hex"
            )),
        ),
        (
            Some(NonZeroU128::MAX),
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-top-up/request-target-balance.hex"
            )),
        ),
    ] {
        let request = CashierTopUpRequest::new(cashier, account, NonZeroU128::MIN, target).unwrap();
        assert_eq!(request.arguments(), fixture(hex));
        assert_eq!(request.method_name(), "account_top_up_v1");
        assert_eq!(request.cashier(), cashier);
        assert_eq!(request.account(), account);
        assert_eq!(request.target_balance(), target);
        assert_eq!(request.offered(), NonZeroU128::MIN);
        let other_offer =
            CashierTopUpRequest::new(cashier, account, NonZeroU128::MAX, target).unwrap();
        assert_eq!(request.arguments(), other_offer.arguments());
        assert_ne!(request.offered(), other_offer.offered());
    }
}
#[test]
fn target_and_payer_must_be_explicit_eligible_principals() {
    let principal = Principal::self_authenticating(b"account");
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        for (cashier, account) in [(invalid, principal), (principal, invalid)] {
            assert_eq!(
                CashierTopUpRequest::new(cashier, account, NonZeroU128::MIN, None),
                Err(TopUpRequestError::InvalidPrincipal)
            );
        }
    }
}
