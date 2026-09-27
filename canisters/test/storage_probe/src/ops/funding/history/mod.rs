//! Passive history conversion; the shared journal owns authority and traversal.
use super::{Failure, STATE, UploadContext, failure, phase};
use blob_test_protocol::storage::funding::{
    Intent,
    history::{Cursor, Entry, Input, Page, Scope},
};
use ic_blob_storage::model::billing::journal::FundingJournalScope;
use ic_blob_storage::ops::service::funding::history::{FundingHistoryCursor, FundingHistoryError};
use std::num::{NonZeroU128, NonZeroUsize};

pub(super) fn scope(input: Scope) -> Result<FundingJournalScope, Failure> {
    Ok(FundingJournalScope {
        service: input.service,
        cashier: input.cashier,
        account: input.account,
        namespace: NonZeroU128::new(input.namespace).ok_or(Failure::Invalid)?,
    })
}
pub(super) fn wire_scope(input: FundingJournalScope) -> Scope {
    Scope {
        service: input.service,
        cashier: input.cashier,
        account: input.account,
        namespace: input.namespace.get(),
    }
}
fn cursor(input: Cursor) -> Result<FundingHistoryCursor, Failure> {
    Ok(FundingHistoryCursor {
        scope: scope(input.scope)?,
        before_operation: NonZeroU128::new(input.before_operation).ok_or(Failure::Invalid)?,
    })
}
pub(crate) fn read(execution: UploadContext, input: Input) -> Result<Page, Failure> {
    let scope = scope(input.scope)?;
    let cursor = input.cursor.map(cursor).transpose()?;
    STATE
        .with_borrow(|state| {
            state.as_ref().unwrap().funding.history(
                execution,
                scope,
                cursor,
                NonZeroUsize::new(2).unwrap(),
            )
        })
        .map(|page| Page {
            entries: page
                .entries
                .into_iter()
                .map(|view| Entry {
                    intent: Intent {
                        service: view.intent.service,
                        cashier: view.intent.cashier,
                        account: view.intent.account,
                        namespace: view.intent.namespace.get(),
                        operation: view.intent.operation.get(),
                        offered: view.intent.offered.get(),
                        target_balance: view.intent.target_balance.map(NonZeroU128::get),
                    },
                    phase: phase(view.state),
                })
                .collect(),
            next: page.next.map(|value| Cursor {
                scope: wire_scope(value.scope),
                before_operation: value.before_operation.get(),
            }),
        })
        .map_err(|error| match error {
            FundingHistoryError::CursorScope => Failure::CursorScope,
            FundingHistoryError::Journal(error) => failure(error),
        })
}
