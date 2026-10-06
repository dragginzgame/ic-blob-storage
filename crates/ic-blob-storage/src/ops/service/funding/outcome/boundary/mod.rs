//! Passive exact lookup and DTO projection. Reconciliation policy lives in workflow.
use crate::{
    dto::funding::{
        FundingPhase,
        outcome::{
            FundingBalanceField, FundingOutcomeFailure, FundingOutcomeRequest,
            FundingOutcomeResponse, FundingReconciliation as Reconciliation,
            FundingReportedBalance, FundingResponse,
        },
    },
    model::{
        billing::journal::{FundingIntent, FundingIntentError, FundingIntentState},
        service::upload::UploadContext,
    },
    ops::{
        billing::balance::BalanceField,
        caffeine::funding::{
            TopUpProviderError, TopUpReply, TopUpReplyError, transport::CashierTopUpStatus,
        },
        service::funding::{
            FundingJournalError, StableFundingJournal, outcome::FundingOutcomeView,
        },
    },
    policy::billing::reconciliation::FundingReconciliation,
};
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU128;

/// Canonical passive exact-outcome query. Linking exports no endpoint.
pub const FUNDING_OUTCOME_METHOD: &str = "blob_funding_outcome";
fn number(value: u128) -> Result<NonZeroU128, FundingOutcomeFailure> {
    NonZeroU128::new(value).ok_or(FundingOutcomeFailure::Invalid)
}
pub(crate) fn read<M: Memory>(
    journal: &StableFundingJournal<M>,
    context: UploadContext,
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeView>, FundingOutcomeFailure> {
    let intent = FundingIntent {
        service: input.scope.service,
        cashier: input.scope.cashier,
        account: input.scope.payment_account,
        namespace: number(input.scope.namespace)?,
        operation: number(input.operation)?,
        offered: number(input.offered)?,
        target_balance: input.target_balance.map(number).transpose()?,
    };
    journal
        .outcome(context, intent)
        .map_err(|error| match error {
            FundingJournalError::Binding => FundingOutcomeFailure::Binding,
            FundingJournalError::NotOperator => FundingOutcomeFailure::Denied,
            FundingJournalError::Intent(FundingIntentError::Conflict) => {
                FundingOutcomeFailure::Conflict
            }
            _ => FundingOutcomeFailure::Internal,
        })
}
pub(crate) fn present(
    input: FundingOutcomeRequest,
    view: &FundingOutcomeView,
    reconciliation: FundingReconciliation,
    fenced: bool,
) -> FundingOutcomeResponse {
    FundingOutcomeResponse {
        request: input,
        phase: match view.state {
            FundingIntentState::Prepared => FundingPhase::Prepared,
            FundingIntentState::Uncertain => FundingPhase::Uncertain,
            FundingIntentState::NotEnqueued => FundingPhase::NotEnqueued,
            FundingIntentState::Callback { refunded } => FundingPhase::Callback { refunded },
        },
        response: view.response.map(response),
        reconciliation: match reconciliation {
            FundingReconciliation::CreditConfirmed {
                accepted_cycles,
                receipt_digest,
            } => Reconciliation::CreditConfirmed {
                accepted_cycles: accepted_cycles.get(),
                receipt_digest,
            },
            FundingReconciliation::NoTransfer => Reconciliation::NoTransfer,
            FundingReconciliation::CreditRequired { accepted_cycles } => {
                Reconciliation::CreditRequired(accepted_cycles.get())
            }
            FundingReconciliation::TransferUnknown { offered_cycles } => {
                Reconciliation::TransferUnknown(offered_cycles.get())
            }
        },
        renewed_allocation: view.renewed_allocation.map_or(0, NonZeroU128::get),
        fenced,
    }
}
fn response(status: CashierTopUpStatus) -> FundingResponse {
    match status {
        CashierTopUpStatus::NotDispatched => FundingResponse::NotDispatched,
        CashierTopUpStatus::NotEnqueued => FundingResponse::NotEnqueued,
        CashierTopUpStatus::Rejected(code) => FundingResponse::Rejected(code),
        CashierTopUpStatus::Replied(Ok(TopUpReply::ReportedSuccess { balance })) => {
            FundingResponse::ReportedSuccess(FundingReportedBalance {
                total: balance.total(),
                prepaid: balance.prepaid(),
                promotional: balance.promotional(),
                ledger: balance.ledger(),
            })
        }
        CashierTopUpStatus::Replied(Ok(TopUpReply::ProviderFailure(error))) => match error {
            TopUpProviderError::NotAuthorized(p) => FundingResponse::NotAuthorized(p),
            TopUpProviderError::AccountBalanceOverflow => FundingResponse::AccountBalanceOverflow,
            TopUpProviderError::InternalError => FundingResponse::InternalError,
            TopUpProviderError::TopUpWithoutCycles => FundingResponse::TopUpWithoutCycles,
        },
        CashierTopUpStatus::Replied(Err(error)) => match error {
            TopUpReplyError::ReplyTooLarge => FundingResponse::ReplyTooLarge,
            TopUpReplyError::InvalidReply => FundingResponse::InvalidReply,
            TopUpReplyError::InvalidBalance(error) => {
                FundingResponse::InvalidBalance(match error.field {
                    BalanceField::Total => FundingBalanceField::Total,
                    BalanceField::Prepaid => FundingBalanceField::Prepaid,
                    BalanceField::Promotional => FundingBalanceField::Promotional,
                    BalanceField::Ledger => FundingBalanceField::Ledger,
                })
            }
        },
    }
}
