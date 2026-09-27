//! Fixed-size inspection of complete local funding accounting, without dispatch.
use super::{
    FundingAllocationView, FundingJournalError, Memory, StableFundingJournal, UploadContext,
};
use crate::model::billing::journal::FundingJournalScope;
use std::num::NonZeroU128;

/// One journal's maintained lifetime facts. These do not establish completeness
/// across other installations, linked payers or direct provider-account activity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingJournalSummary {
    /// Independently checked service/provider/account/namespace binding.
    pub scope: FundingJournalScope,
    /// Complete local attachment totals, never provider credit or spendable funds.
    pub allocation: FundingAllocationView,
    /// Retained lifetime intent count, including refunded and proven unsent work.
    pub retained_intents: u64,
    /// Installed lifetime intent capacity; no eviction or reuse is permitted.
    pub intent_capacity: u64,
    /// Highest retained identity, not a freshness or allocation authority.
    pub last_operation: Option<NonZeroU128>,
    /// Restored journals remain inspection-only even with zero obligations.
    pub fenced: bool,
}
impl<M: Memory> StableFundingJournal<M> {
    /// Read one maintained accounting row, without scanning or decoding intent rows.
    /// Authenticate and check complete scope even for an empty journal. The host
    /// must establish account-wide completeness separately before payment admission.
    /// # Errors
    /// Rejects unauthorized callers, changed scope or inconsistent metadata counts.
    pub fn summary(
        &self,
        execution: UploadContext,
        scope: FundingJournalScope,
    ) -> Result<FundingJournalSummary, FundingJournalError> {
        self.authorize_scope(execution, scope)?;
        let totals = self.totals()?;
        if totals.count() != self.intents.len()
            || totals.count() > totals.limit()
            || (totals.count() == 0) != (totals.last() == 0)
        {
            return Err(FundingJournalError::InvalidRecord);
        }
        Ok(FundingJournalSummary {
            scope,
            allocation: totals.view(),
            retained_intents: totals.count(),
            intent_capacity: totals.limit(),
            last_operation: NonZeroU128::new(totals.last()),
            fenced: self.fenced,
        })
    }
    pub(super) fn authorize_scope(
        &self,
        execution: UploadContext,
        scope: FundingJournalScope,
    ) -> Result<(), FundingJournalError> {
        self.authorize(execution)?;
        let bindings = self.config.bindings();
        let expected = FundingJournalScope {
            service: bindings.service,
            cashier: self.config.billing().cashier(),
            account: bindings.payment_account,
            namespace: bindings.namespace,
        };
        if scope != expected {
            return Err(FundingJournalError::Binding);
        }
        Ok(())
    }
}
