//! Transport progress owns the responses permitted at each terminal state.
use super::response::FundingResponseRecord;
use crate::model::billing::journal::{FundingIntentError, FundingIntentState};
use serde::{Deserialize, Serialize};

/// Bounded evidence that no provider call was enqueued.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum UnsentResponseRecord {
    NotDispatched,
    NotEnqueued,
}
/// Transport progress owns its diagnostics; credit attribution remains independent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum FundingProgressRecord {
    Prepared,
    Uncertain,
    NotEnqueued(Option<UnsentResponseRecord>),
    Callback {
        refunded: u128,
        response: Option<FundingResponseRecord>,
    },
}
impl FundingProgressRecord {
    pub(crate) const fn state(self) -> FundingIntentState {
        match self {
            Self::Prepared => FundingIntentState::Prepared,
            Self::Uncertain => FundingIntentState::Uncertain,
            Self::NotEnqueued(_) => FundingIntentState::NotEnqueued,
            Self::Callback { refunded, .. } => FundingIntentState::Callback { refunded },
        }
    }
    /// Missing diagnostic evidence cannot erase an earlier observation.
    pub(super) fn complete(self, next: Self) -> Result<Self, FundingIntentError> {
        match (self, next) {
            (Self::Prepared, _) => Err(FundingIntentError::NotAttempted),
            (_, Self::Prepared | Self::Uncertain) => Err(FundingIntentError::OutcomeConflict),
            (Self::Uncertain, next) => Ok(next),
            (Self::NotEnqueued(prior), Self::NotEnqueued(next)) => {
                Ok(Self::NotEnqueued(enrich(prior, next)?))
            }
            (
                Self::Callback {
                    refunded,
                    response: prior,
                },
                Self::Callback {
                    refunded: next,
                    response,
                },
            ) if refunded == next => Ok(Self::Callback {
                refunded,
                response: enrich(prior, response)?,
            }),
            _ => Err(FundingIntentError::OutcomeConflict),
        }
    }
}
fn enrich<T: Copy + Eq>(
    prior: Option<T>,
    next: Option<T>,
) -> Result<Option<T>, FundingIntentError> {
    match (prior, next) {
        (Some(prior), Some(next)) if prior != next => Err(FundingIntentError::OutcomeConflict),
        (prior, next) => Ok(prior.or(next)),
    }
}
