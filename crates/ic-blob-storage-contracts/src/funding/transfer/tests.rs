use super::*;

#[test]
fn callback_arithmetic_conserves_the_attachment_across_the_full_cycle_range() {
    for offered in [1, 2, u128::from(u64::MAX), u128::MAX] {
        let amount = NonZeroU128::new(offered).expect("positive amount");
        for refunded in [0, offered / 2, offered] {
            let transfer =
                FundingTransfer::unbounded_callback(amount, refunded).expect("bounded refund");
            assert_eq!(transfer.offered(), amount);
            assert_eq!(transfer.refunded(), Some(refunded));
            let accepted = transfer.accepted().expect("known callback");
            assert_eq!(accepted, offered - refunded);
            assert_eq!(accepted.checked_add(refunded), Some(offered));
        }
    }
}

#[test]
fn impossible_refunds_are_rejected_instead_of_becoming_zero_acceptance() {
    for offered in [1, u128::from(u64::MAX), u128::MAX - 1] {
        assert_eq!(
            FundingTransfer::unbounded_callback(
                NonZeroU128::new(offered).expect("positive amount"),
                offered + 1,
            ),
            Err(FundingTransferError::RefundExceedsOffer),
        );
    }
}

#[test]
fn absent_refund_distinguishes_unsent_from_unknown() {
    let offered = NonZeroU128::MAX;
    let unsent = FundingTransfer::not_enqueued(offered);
    let unknown = FundingTransfer::unknown(offered);
    assert_eq!(unsent.offered(), offered);
    assert_eq!(unknown.offered(), offered);
    assert_eq!(unsent.refunded(), None);
    assert_eq!(unknown.refunded(), None);
    assert_eq!(unsent.accepted(), Some(0));
    assert_eq!(unknown.accepted(), None);
    assert_ne!(unsent, unknown);
}
