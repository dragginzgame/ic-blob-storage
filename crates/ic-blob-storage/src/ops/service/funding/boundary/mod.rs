//! Shared passive funding conversions; callers retain authority and refusal mapping.
use crate::model::billing::journal::{FundingIntent, FundingIntentState};
use ic_blob_storage_contracts::dto::{funding::FundingPhase, operator::OperatorScope};
use std::num::NonZeroU128;

pub(super) fn intent(
    scope: OperatorScope,
    operation: u128,
    offered: u128,
    target_balance: Option<u128>,
) -> Option<FundingIntent> {
    Some(FundingIntent {
        service: scope.service,
        cashier: scope.cashier,
        account: scope.payment_account,
        namespace: NonZeroU128::new(scope.namespace)?,
        operation: NonZeroU128::new(operation)?,
        offered: NonZeroU128::new(offered)?,
        target_balance: match target_balance {
            Some(value) => Some(NonZeroU128::new(value)?),
            None => None,
        },
    })
}

pub(crate) const fn phase(state: FundingIntentState) -> FundingPhase {
    match state {
        FundingIntentState::Prepared => FundingPhase::Prepared,
        FundingIntentState::Uncertain => FundingPhase::Uncertain,
        FundingIntentState::NotEnqueued => FundingPhase::NotEnqueued,
        FundingIntentState::Callback { refunded } => FundingPhase::Callback { refunded },
    }
}
