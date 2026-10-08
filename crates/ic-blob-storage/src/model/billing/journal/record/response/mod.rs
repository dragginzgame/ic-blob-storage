//! Bounded local response schema, distinct from the provider's wire contract.
use super::FundingPhaseRecord;
use candid::{CandidType, Deserialize, Principal};

/// Fixed-size diagnostic facts. No text, wire buffers or inferred credit is stored.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) enum FundingResponseRecord {
    /// No structured observation was retained; transport may still be known.
    Missing,
    NotDispatched,
    NotEnqueued,
    Rejected(u32),
    Balance {
        total: u128,
        prepaid: u128,
        promotional: u128,
        ledger: u128,
    },
    NotAuthorized(Principal),
    AccountBalanceOverflow,
    InternalError,
    TopUpWithoutCycles,
    ReplyTooLarge,
    InvalidReply,
    InvalidBalance(BalanceFieldRecord),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) enum BalanceFieldRecord {
    Total,
    Prepaid,
    Promotional,
    Ledger,
}

impl FundingResponseRecord {
    pub(crate) const fn matches(self, phase: FundingPhaseRecord) -> bool {
        match self {
            Self::Missing => true,
            Self::NotDispatched | Self::NotEnqueued => {
                matches!(phase, FundingPhaseRecord::NotEnqueued)
            }
            _ => matches!(phase, FundingPhaseRecord::Callback { .. }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::billing::journal::FundingIntent;
    use crate::model::billing::journal::FundingIntentError;
    use crate::model::billing::journal::record::FundingIntentRecord;
    use std::num::NonZeroU128;

    #[test]
    fn response_phase_mismatch_is_rejected_on_transition_and_inspection() {
        let n = NonZeroU128::MIN;
        let intent = FundingIntent {
            service: Principal::self_authenticating(b"service"),
            cashier: Principal::self_authenticating(b"cashier"),
            account: Principal::self_authenticating(b"account"),
            namespace: n,
            operation: n,
            offered: n,
            target_balance: None,
        };
        for (phase, response) in [
            (
                FundingPhaseRecord::Prepared,
                FundingResponseRecord::NotDispatched,
            ),
            (
                FundingPhaseRecord::Uncertain,
                FundingResponseRecord::InternalError,
            ),
            (
                FundingPhaseRecord::NotEnqueued,
                FundingResponseRecord::InvalidReply,
            ),
            (
                FundingPhaseRecord::Callback { refunded: 1 },
                FundingResponseRecord::NotEnqueued,
            ),
        ] {
            let mut row = FundingIntentRecord::new(intent);
            row.phase = phase;
            assert_eq!(
                row.with_response(response),
                Err(FundingIntentError::OutcomeConflict)
            );
            row.response = response; // Corrupted current schema, not a transition API.
            assert_eq!(row.view(), None);
            assert_eq!(row.transfer(), None);
        }
    }
}
