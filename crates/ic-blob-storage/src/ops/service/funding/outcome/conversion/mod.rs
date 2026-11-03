//! Explicit conversion between validated decoder values and the bounded local schema.
use crate::model::billing::journal::{
    FundingIntentError, FundingTransportOutcome,
    record::progress::{FundingProgressRecord, UnsentResponseRecord},
};
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
fn callback(status: CashierTopUpStatus) -> Result<FundingResponseRecord, FundingIntentError> {
    Ok(match status {
        CashierTopUpStatus::NotDispatched | CashierTopUpStatus::NotEnqueued => {
            return Err(FundingIntentError::OutcomeConflict);
        }
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
            TopUpProviderError::NotAuthorized(p) => FundingResponseRecord::NotAuthorized(
                ic_platform::principal::ExternalPrincipal::from_slice(p.as_slice()),
            ),
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
    })
}
fn callback_view(record: FundingResponseRecord) -> CashierTopUpStatus {
    let reply = match record {
        FundingResponseRecord::Rejected(code) => return CashierTopUpStatus::Rejected(code),
        FundingResponseRecord::Balance {
            total,
            prepaid,
            promotional,
            ledger,
        } => Ok(TopUpReply::ReportedSuccess {
            balance: BalanceAmounts::new(total, prepaid, promotional, ledger),
        }),
        FundingResponseRecord::NotAuthorized(p) => Ok(TopUpReply::ProviderFailure(
            TopUpProviderError::NotAuthorized(ic_platform::Principal::from_slice(p.as_slice())),
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
    CashierTopUpStatus::Replied(reply)
}

/// Correlate transport and diagnostics before constructing a persisted terminal state.
pub(super) fn record(
    outcome: FundingTransportOutcome,
    status: Option<CashierTopUpStatus>,
) -> Result<FundingProgressRecord, FundingIntentError> {
    Ok(match outcome {
        FundingTransportOutcome::NotEnqueued => FundingProgressRecord::NotEnqueued(match status {
            None => None,
            Some(CashierTopUpStatus::NotDispatched) => Some(UnsentResponseRecord::NotDispatched),
            Some(CashierTopUpStatus::NotEnqueued) => Some(UnsentResponseRecord::NotEnqueued),
            _ => return Err(FundingIntentError::OutcomeConflict),
        }),
        FundingTransportOutcome::Callback { refunded } => FundingProgressRecord::Callback {
            refunded,
            response: status.map(callback).transpose()?,
        },
    })
}
pub(super) fn view(progress: FundingProgressRecord) -> Option<CashierTopUpStatus> {
    match progress {
        FundingProgressRecord::Prepared
        | FundingProgressRecord::Uncertain
        | FundingProgressRecord::NotEnqueued(None) => None,
        FundingProgressRecord::NotEnqueued(Some(UnsentResponseRecord::NotDispatched)) => {
            Some(CashierTopUpStatus::NotDispatched)
        }
        FundingProgressRecord::NotEnqueued(Some(UnsentResponseRecord::NotEnqueued)) => {
            Some(CashierTopUpStatus::NotEnqueued)
        }
        FundingProgressRecord::Callback { response, .. } => response.map(callback_view),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_diagnostics_preserve_every_decoder_category_and_component() {
        let principal =
            ic_platform::Principal::self_authenticating(b"reported unauthorized principal");
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
}
