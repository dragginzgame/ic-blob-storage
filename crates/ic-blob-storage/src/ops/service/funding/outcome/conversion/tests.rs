use super::*;
#[test]
fn bounded_diagnostics_preserve_every_decoder_category_and_component() {
    let principal = ic_platform::Principal::self_authenticating(b"reported unauthorized principal");
    let mut statuses = vec![
        CashierTopUpStatus::NotDispatched,
        CashierTopUpStatus::NotEnqueued,
        CashierTopUpStatus::Rejected(u32::MAX),
    ];
    statuses.push(CashierTopUpStatus::Replied(Ok(
        TopUpReply::ReportedSuccess {
            balance: BalanceAmounts::new(u128::MAX, 7, 0, 19),
        },
    )));
    for error in [
        TopUpProviderError::NotAuthorized(principal),
        TopUpProviderError::AccountBalanceOverflow,
        TopUpProviderError::InternalError,
        TopUpProviderError::TopUpWithoutCycles,
    ] {
        statuses.push(CashierTopUpStatus::Replied(Ok(
            TopUpReply::ProviderFailure(error),
        )));
    }
    for error in [
        TopUpReplyError::ReplyTooLarge,
        TopUpReplyError::InvalidReply,
    ] {
        statuses.push(CashierTopUpStatus::Replied(Err(error)));
    }
    for field in [
        BalanceField::Total,
        BalanceField::Prepaid,
        BalanceField::Promotional,
        BalanceField::Ledger,
    ] {
        statuses.push(CashierTopUpStatus::Replied(Err(
            TopUpReplyError::InvalidBalance(BalanceInputError { field }),
        )));
    }
    for status in statuses {
        let transport = if matches!(
            status,
            CashierTopUpStatus::NotDispatched | CashierTopUpStatus::NotEnqueued
        ) {
            FundingTransportOutcome::NotEnqueued
        } else {
            FundingTransportOutcome::Callback { refunded: 0 }
        };
        assert_eq!(view(record(transport, Some(status)).unwrap()), Some(status));
    }
    assert_eq!(view(FundingProgressRecord::Uncertain), None);
}
