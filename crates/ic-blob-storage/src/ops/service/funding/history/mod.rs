//! Bounded operator discovery of retained funding intents, including after restore.
use super::{FundingIntentView, FundingJournalError, Memory, StableFundingJournal, UploadContext};
use crate::model::billing::journal::FundingJournalScope;
use std::{
    num::{NonZeroU128, NonZeroUsize},
    ops::Bound::{Excluded, Unbounded},
};
use thiserror::Error;

/// Untrusted position in descending operation-ID order, not a snapshot or authority.
/// Start a fresh sweep to see new intents or changed states behind this position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingHistoryCursor {
    /// Complete scope of the original page.
    pub scope: FundingJournalScope,
    /// Exclusive upper bound; need not identify an existing row.
    pub before_operation: NonZeroU128,
}

/// Bounded current observations. No result authorizes retry, payment or unfencing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FundingHistoryPage {
    /// Exact intents and current local states, newest operation first.
    /// Since admission is sequential, any prepared/uncertain intent is first in a
    /// fresh sweep; terminal transport outcomes remain retained as well.
    pub entries: Vec<FundingIntentView>,
    /// Continue with the same scope. Absence only marks this local range's end.
    pub next: Option<FundingHistoryCursor>,
}

impl<M: Memory> StableFundingJournal<M> {
    /// Discover original identities/amounts without requiring a saved request.
    ///
    /// Authenticates even empty ranges. Reads at most `limit` intent values plus
    /// one index lookahead; no history reconstruction, mutation or provider call.
    /// `limit` must be bounded by trusted host policy, not unrestricted ingress.
    /// Fencing preserves these reads, but neither cursor nor operation ordering
    /// establishes freshness, provider credit or safe operational restoration.
    /// # Errors
    /// Rejects caller/service/scope, changed cursor scope or invalid retained rows.
    pub fn history(
        &self,
        execution: UploadContext,
        scope: FundingJournalScope,
        cursor: Option<FundingHistoryCursor>,
        limit: NonZeroUsize,
    ) -> Result<FundingHistoryPage, FundingHistoryError> {
        self.authorize_scope(execution, scope)?;
        if cursor.is_some_and(|value| value.scope != scope) {
            return Err(FundingHistoryError::CursorScope);
        }
        let totals = self.totals()?;
        let end = cursor.map_or(Unbounded, |value| Excluded(value.before_operation.get()));
        let mut rows = self.intents.range((Unbounded, end)).rev().peekable();
        let mut entries = Vec::new();
        for entry in rows.by_ref().take(limit.get()) {
            let view = entry
                .value()
                .view()
                .ok_or(FundingJournalError::InvalidRecord)?;
            if view.intent.operation.get() != *entry.key() || !totals.scope_matches(view.intent) {
                return Err(FundingJournalError::InvalidRecord.into());
            }
            entries.push(view);
        }
        let next = if rows.peek().is_some() {
            entries.last().map(|view| FundingHistoryCursor {
                scope,
                before_operation: view.intent.operation,
            })
        } else {
            None
        };
        Ok(FundingHistoryPage { entries, next })
    }
}

/// Discovery errors do not modify records or release any reservation.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FundingHistoryError {
    /// Journal authority, installation binding or retained-state error.
    #[error(transparent)]
    Journal(#[from] FundingJournalError),
    /// A cursor from a different service/provider/account/namespace was supplied.
    #[error("funding history cursor scope mismatch")]
    CursorScope,
}
