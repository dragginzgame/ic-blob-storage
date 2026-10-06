//! One exact credit commit in the existing journal; no provider effect or endpoint.
use super::outcome::FundingOutcomeView;
use super::{FundingJournalError, Memory, StableFundingJournal, UploadContext};
use crate::model::billing::journal::{
    FundingIntent,
    credit::{FundingCreditConfirmation, FundingCreditError},
};

impl<M: Memory> StableFundingJournal<M> {
    pub(crate) fn credit_candidate(
        &self,
        execution: UploadContext,
        input: FundingIntent,
    ) -> Result<FundingOutcomeView, FundingJournalError> {
        self.authorize(execution)?;
        self.mutable()?;
        let view = self
            .outcome(execution, input)?
            .ok_or(FundingJournalError::Unknown)?;
        if view.transfer.accepted().is_none_or(|amount| amount == 0) {
            return Err(FundingCreditError::AcceptanceRequired.into());
        }
        Ok(view)
    }

    /// Persist independently established host credit evidence for one exact intent.
    /// Never expose this as an ingress fact setter. The host must authenticate the
    /// provider evidence and its unique account/operation correlation first.
    /// Receipt reuse uses the bounded index in the existing accounting memory;
    /// there is no external journal, provider call or new memory grant.
    /// Exact replay returns false. Spent allocation, transport/refunds and fences
    /// remain unchanged. Writes must execute synchronously in an IC update.
    /// # Errors
    /// Rejects authority/scope/fence, absent or unaccepted intent, wrong amount,
    /// conflicting receipt and reuse across intents before any write.
    /// # Panics
    /// Stable traps must propagate for whole-message rollback of intent, receipt index and accounting writes.
    pub fn record_credit(
        &mut self,
        execution: UploadContext,
        confirmation: FundingCreditConfirmation,
    ) -> Result<bool, FundingJournalError> {
        let input = confirmation.intent;
        self.credit_candidate(execution, input)?;
        let prior = self.required(input)?;
        let next = prior.confirm_credit(confirmation)?;
        if prior == next {
            return Ok(false);
        }
        if self.accounting.contains_key(&confirmation.receipt_digest) {
            return Err(FundingCreditError::ReceiptReused.into());
        }
        let totals = self
            .totals()?
            .confirm_credit(confirmation.accepted_cycles)?;
        self.intents.insert(input.operation.get(), next);
        self.accounting.insert(
            confirmation.receipt_digest,
            super::FundingAccountingRecord::Receipt {
                operation: input.operation.get(),
            },
        );
        self.write_totals(&totals);
        Ok(true)
    }
}
