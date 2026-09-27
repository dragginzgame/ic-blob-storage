//! Explicit conversion between validated decoder values and the bounded local schema.
use crate::{
    model::billing::{
        balance::BalanceAmounts,
        journal::record::response::{BalanceFieldRecord, FundingResponseRecord},
    },
    ops::{
        billing::balance::{BalanceField, BalanceInputError},
        caffeine::funding::{
            TopUpProviderError, TopUpReply, TopUpReplyError, transport::CashierTopUpStatus,
        },
    },
};
pub(super) const fn record(status: CashierTopUpStatus) -> FundingResponseRecord {
    match status {
        CashierTopUpStatus::NotDispatched => FundingResponseRecord::NotDispatched,
        CashierTopUpStatus::NotEnqueued => FundingResponseRecord::NotEnqueued,
        CashierTopUpStatus::Rejected(code) => FundingResponseRecord::Rejected(code),
        CashierTopUpStatus::Replied(Ok(TopUpReply::ReportedSuccess { balance })) => {
            FundingResponseRecord::Balance {
                total: balance.total(),
                prepaid: balance.prepaid(),
                promotional: balance.promotional(),
                ledger: balance.ledger(),
            }
        }
        CashierTopUpStatus::Replied(Ok(TopUpReply::ProviderFailure(error))) => match error {
            TopUpProviderError::NotAuthorized(p) => FundingResponseRecord::NotAuthorized(p),
            TopUpProviderError::AccountBalanceOverflow => {
                FundingResponseRecord::AccountBalanceOverflow
            }
            TopUpProviderError::InternalError => FundingResponseRecord::InternalError,
            TopUpProviderError::TopUpWithoutCycles => FundingResponseRecord::TopUpWithoutCycles,
        },
        CashierTopUpStatus::Replied(Err(error)) => match error {
            TopUpReplyError::ReplyTooLarge => FundingResponseRecord::ReplyTooLarge,
            TopUpReplyError::InvalidReply => FundingResponseRecord::InvalidReply,
            TopUpReplyError::InvalidBalance(error) => {
                FundingResponseRecord::InvalidBalance(match error.field {
                    BalanceField::Total => BalanceFieldRecord::Total,
                    BalanceField::Prepaid => BalanceFieldRecord::Prepaid,
                    BalanceField::Promotional => BalanceFieldRecord::Promotional,
                    BalanceField::Ledger => BalanceFieldRecord::Ledger,
                })
            }
        },
    }
}
pub(super) const fn view(record: FundingResponseRecord) -> Option<CashierTopUpStatus> {
    let reply = match record {
        FundingResponseRecord::Missing => return None,
        FundingResponseRecord::NotDispatched => return Some(CashierTopUpStatus::NotDispatched),
        FundingResponseRecord::NotEnqueued => return Some(CashierTopUpStatus::NotEnqueued),
        FundingResponseRecord::Rejected(code) => return Some(CashierTopUpStatus::Rejected(code)),
        FundingResponseRecord::Balance {
            total,
            prepaid,
            promotional,
            ledger,
        } => Ok(TopUpReply::ReportedSuccess {
            balance: BalanceAmounts::new(total, prepaid, promotional, ledger),
        }),
        FundingResponseRecord::NotAuthorized(p) => Ok(TopUpReply::ProviderFailure(
            TopUpProviderError::NotAuthorized(p),
        )),
        FundingResponseRecord::AccountBalanceOverflow => Ok(TopUpReply::ProviderFailure(
            TopUpProviderError::AccountBalanceOverflow,
        )),
        FundingResponseRecord::InternalError => Ok(TopUpReply::ProviderFailure(
            TopUpProviderError::InternalError,
        )),
        FundingResponseRecord::TopUpWithoutCycles => Ok(TopUpReply::ProviderFailure(
            TopUpProviderError::TopUpWithoutCycles,
        )),
        FundingResponseRecord::ReplyTooLarge => Err(TopUpReplyError::ReplyTooLarge),
        FundingResponseRecord::InvalidReply => Err(TopUpReplyError::InvalidReply),
        FundingResponseRecord::InvalidBalance(field) => {
            Err(TopUpReplyError::InvalidBalance(BalanceInputError {
                field: match field {
                    BalanceFieldRecord::Total => BalanceField::Total,
                    BalanceFieldRecord::Prepaid => BalanceField::Prepaid,
                    BalanceFieldRecord::Promotional => BalanceField::Promotional,
                    BalanceFieldRecord::Ledger => BalanceField::Ledger,
                },
            }))
        }
    };
    Some(CashierTopUpStatus::Replied(reply))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_diagnostics_preserve_every_decoder_category_and_component() {
        let principal = candid::Principal::self_authenticating(b"reported unauthorized principal");
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
            assert_eq!(view(record(status)), Some(status));
        }
        assert_eq!(view(FundingResponseRecord::Missing), None);
    }
}
