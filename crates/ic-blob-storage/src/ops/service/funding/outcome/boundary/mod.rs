//! Passive exact lookup and DTO projection. Reconciliation policy lives in workflow.
use crate::model::billing::journal::FundingIntentError;
use crate::ops::billing::balance::BalanceField;
use crate::ops::caffeine::funding::TopUpProviderError;
use crate::ops::caffeine::funding::TopUpReply;
use crate::ops::caffeine::funding::TopUpReplyError;
use crate::ops::caffeine::funding::transport::CashierTopUpStatus;
use crate::ops::service::funding::FundingJournalError;
use crate::ops::service::funding::StableFundingJournal;
use crate::ops::service::funding::outcome::FundingOutcomeView;
use crate::policy::billing::reconciliation::FundingReconciliation;
use ic_blob_storage_contracts::dto::funding::outcome::FundingBalanceField;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeFailure;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeResponse;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReconciliation as Reconciliation;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReportedBalance;
use ic_blob_storage_contracts::dto::funding::outcome::FundingResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::Memory;
use std::num::NonZeroU128;

pub(crate) fn read<M: Memory>(
    journal: &StableFundingJournal<M>,
    context: UploadContext,
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeView>, FundingOutcomeFailure> {
    let intent = super::super::boundary::intent(
        input.scope,
        input.operation,
        input.offered,
        input.target_balance,
    )
    .ok_or(FundingOutcomeFailure::Invalid)?;
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
        phase: super::super::boundary::phase(view.state),
        response: view.response.map(response),
        reconciliation: match reconciliation {
            FundingReconciliation::ProviderAcknowledged { accepted_cycles } => {
                Reconciliation::ProviderAcknowledged(accepted_cycles.get())
            }
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
