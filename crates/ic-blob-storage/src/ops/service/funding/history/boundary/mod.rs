//! Conversion around the existing bounded journal traversal.
use crate::{
    dto::{
        funding::{
            FundingHistoryCursor, FundingHistoryEntry, FundingHistoryFailure, FundingHistoryPage,
            FundingHistoryRequest, FundingPhase,
        },
        operator::OperatorScope,
    },
    model::{
        billing::journal::{FundingIntentState, FundingJournalScope},
        service::upload::UploadContext,
    },
    ops::service::funding::{
        FundingJournalError, StableFundingJournal,
        history::{FundingHistoryCursor as Cursor, FundingHistoryError},
    },
};
use ic_memory::ic_stable_structures::Memory;
use std::num::{NonZeroU128, NonZeroUsize};

/// Canonical passive funding history query. Linking exports no endpoint.
pub const FUNDING_HISTORY_METHOD: &str = "blob_funding_history";
fn scope(input: OperatorScope) -> Result<FundingJournalScope, FundingHistoryFailure> {
    Ok(FundingJournalScope {
        service: input.service,
        cashier: input.cashier,
        account: input.payment_account,
        namespace: NonZeroU128::new(input.namespace).ok_or(FundingHistoryFailure::Invalid)?,
    })
}
fn failure(error: FundingHistoryError) -> FundingHistoryFailure {
    match error {
        FundingHistoryError::CursorScope => FundingHistoryFailure::CursorScope,
        FundingHistoryError::Journal(FundingJournalError::Binding) => {
            FundingHistoryFailure::Binding
        }
        FundingHistoryError::Journal(FundingJournalError::NotOperator) => {
            FundingHistoryFailure::Denied
        }
        FundingHistoryError::Journal(_) => FundingHistoryFailure::Internal,
    }
}
pub(crate) fn inspect<M: Memory>(
    journal: &StableFundingJournal<M>,
    context: UploadContext,
    input: FundingHistoryRequest,
    limit: NonZeroUsize,
) -> Result<FundingHistoryPage, FundingHistoryFailure> {
    let cursor = input
        .cursor
        .map(|value| {
            Ok(Cursor {
                scope: scope(value.scope)?,
                before_operation: NonZeroU128::new(value.before_operation)
                    .ok_or(FundingHistoryFailure::Invalid)?,
            })
        })
        .transpose()?;
    let page = journal
        .history(context, scope(input.scope)?, cursor, limit)
        .map_err(failure)?;
    Ok(FundingHistoryPage {
        request: input,
        entries: page
            .entries
            .into_iter()
            .map(|view| FundingHistoryEntry {
                scope: OperatorScope {
                    service: view.intent.service,
                    cashier: view.intent.cashier,
                    payment_account: view.intent.account,
                    namespace: view.intent.namespace.get(),
                },
                operation: view.intent.operation.get(),
                offered: view.intent.offered.get(),
                target_balance: view.intent.target_balance.map(NonZeroU128::get),
                phase: match view.state {
                    FundingIntentState::Prepared => FundingPhase::Prepared,
                    FundingIntentState::Uncertain => FundingPhase::Uncertain,
                    FundingIntentState::NotEnqueued => FundingPhase::NotEnqueued,
                    FundingIntentState::Callback { refunded } => {
                        FundingPhase::Callback { refunded }
                    }
                },
            })
            .collect(),
        next: page.next.map(|position| FundingHistoryCursor {
            scope: input.scope,
            before_operation: position.before_operation.get(),
        }),
        fenced: journal.is_fenced(),
    })
}
