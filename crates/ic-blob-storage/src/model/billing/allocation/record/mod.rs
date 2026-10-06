//! Persisted allocation counters; updates share reconstruction arithmetic.
use super::{FundingAllocation, FundingAllocationError, FundingAllocationView, FundingTransfer};
use candid::{CandidType, Deserialize};
use std::num::NonZeroU128;

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct FundingAllocationRecord {
    initial_allocated: u128,
    allocated: u128,
    renewal_ceiling: u128,
    reserve: u128,
    max_attempts: u64,
    available: u128,
    accepted: u128,
    credit_confirmed: u128,
    refunded: u128,
    not_enqueued: u128,
    reserved_or_uncertain: u128,
}
impl FundingAllocationRecord {
    pub(crate) fn new(config: FundingAllocation) -> Self {
        Self {
            initial_allocated: config.allocated,
            allocated: config.allocated,
            renewal_ceiling: config.renewal_ceiling,
            reserve: config.reserve.get(),
            max_attempts: config.max_attempts.get() as u64,
            available: config.allocated,
            accepted: 0,
            credit_confirmed: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
        }
    }
    pub(crate) fn matches(self, config: FundingAllocation) -> bool {
        self.initial_allocated == config.allocated
            && self.renewal_ceiling == config.renewal_ceiling
            && self.reserve == config.reserve.get()
            && self.max_attempts == config.max_attempts.get() as u64
    }
    pub(crate) const fn max_attempts(self) -> u64 {
        self.max_attempts
    }
    pub(crate) const fn view(self) -> FundingAllocationView {
        FundingAllocationView {
            allocated: self.allocated,
            renewal_ceiling: self.renewal_ceiling,
            reserve: self.reserve,
            available: self.available,
            accepted: self.accepted,
            credit_confirmed: self.credit_confirmed,
            refunded: self.refunded,
            not_enqueued: self.not_enqueued,
            reserved_or_uncertain: self.reserved_or_uncertain,
        }
    }
    fn with_view(mut self, view: FundingAllocationView) -> Self {
        self.allocated = view.allocated;
        self.available = view.available;
        self.accepted = view.accepted;
        self.credit_confirmed = view.credit_confirmed;
        self.refunded = view.refunded;
        self.not_enqueued = view.not_enqueued;
        self.reserved_or_uncertain = view.reserved_or_uncertain;
        self
    }
    pub(crate) fn renew(self, additional: NonZeroU128) -> Result<Self, FundingAllocationError> {
        Ok(self.with_view(self.view().renew(additional)?))
    }
    pub(crate) fn reserve(
        self,
        amount: NonZeroU128,
        index: usize,
    ) -> Result<Self, FundingAllocationError> {
        Ok(self.with_view(self.view().reserve(amount, index)?))
    }
    pub(crate) fn confirm_credit(
        self,
        accepted: NonZeroU128,
    ) -> Result<Self, FundingAllocationError> {
        Ok(self.with_view(self.view().confirm_credit(accepted)?))
    }
    pub(crate) fn resolve(self, transfer: FundingTransfer) -> Result<Self, FundingAllocationError> {
        Ok(self.with_view(self.view().resolve(transfer)?))
    }
}
