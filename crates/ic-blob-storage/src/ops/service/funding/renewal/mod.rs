//! One synchronous budget grant; original spent and receipt records remain intact.
use super::{FundingJournalError, Memory, StableFundingJournal, UploadContext};
use crate::model::billing::journal::renewal::{FundingBudgetRenewal, FundingRenewalError};

impl<M: Memory> StableFundingJournal<M> {
    /// Increase authorization within the immutable installed cumulative ceiling.
    /// Host/operator policy owns the grant; this is not an ingress fact setter.
    /// No cycles are received/refunded, no provider call occurs, and lifetime
    /// capacity/operation identity never reset. All dispatch gates still apply.
    /// Exact replay returns false, even after later intents; conflicts refuse.
    /// # Errors
    /// Rejects caller, binding, fence, absent credit, excess/conflicting amount,
    /// unreconciled history, a new grant on an older intent or exhausted ceiling.
    /// # Panics
    /// Propagate stable traps for whole-message IC rollback of both writes.
    pub fn renew_budget(
        &mut self,
        execution: UploadContext,
        input: FundingBudgetRenewal,
    ) -> Result<bool, FundingJournalError> {
        self.authorize(execution)?;
        self.mutable()?;
        let totals = self.totals()?;
        Self::scope(&totals, input.intent)?;
        let prior = self.required(input.intent)?;
        let next = prior.renew(input)?;
        if next == prior {
            return Ok(false);
        }
        if totals.last() != input.intent.operation.get() {
            return Err(FundingRenewalError::NotLatest.into());
        }
        if totals.view().uncredited() != 0 || totals.view().reserved_or_uncertain() != 0 {
            return Err(FundingRenewalError::Unreconciled.into());
        }
        let totals = totals.renew(input.additional)?;
        self.intents.insert(input.intent.operation.get(), next);
        self.write_totals(&totals);
        Ok(true)
    }
}
