//! Persisted allocation counters; updates share reconstruction arithmetic.
use super::{FundingAllocation, FundingAllocationError, FundingAllocationView, FundingTransfer};
use candid::{CandidType, Deserialize};
use std::num::NonZeroU128;

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct FundingAllocationRecord {
    allocated: u128,
    reserve: u128,
    max_attempts: u64,
    available: u128,
    accepted: u128,
    refunded: u128,
    not_enqueued: u128,
    reserved_or_uncertain: u128,
}
impl FundingAllocationRecord {
    pub(crate) fn new(config: FundingAllocation) -> Self {
        Self {
            allocated: config.allocated,
            reserve: config.reserve.get(),
            max_attempts: config.max_attempts.get() as u64,
            available: config.allocated,
            accepted: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
        }
    }
    pub(crate) fn matches(self, config: FundingAllocation) -> bool {
        self.allocated == config.allocated
            && self.reserve == config.reserve.get()
            && self.max_attempts == config.max_attempts.get() as u64
    }
    pub(crate) const fn max_attempts(self) -> u64 {
        self.max_attempts
    }
    pub(crate) const fn view(self) -> FundingAllocationView {
        FundingAllocationView {
            reserve: self.reserve,
            available: self.available,
            accepted: self.accepted,
            refunded: self.refunded,
            not_enqueued: self.not_enqueued,
            reserved_or_uncertain: self.reserved_or_uncertain,
        }
    }
    fn with_view(mut self, view: FundingAllocationView) -> Self {
        self.available = view.available;
        self.accepted = view.accepted;
        self.refunded = view.refunded;
        self.not_enqueued = view.not_enqueued;
        self.reserved_or_uncertain = view.reserved_or_uncertain;
        self
    }
    pub(crate) fn reserve(
        self,
        amount: NonZeroU128,
        index: usize,
    ) -> Result<Self, FundingAllocationError> {
        Ok(self.with_view(self.view().reserve(amount, index)?))
    }
    pub(crate) fn resolve(self, transfer: FundingTransfer) -> Result<Self, FundingAllocationError> {
        Ok(self.with_view(self.view().resolve(transfer)?))
    }
}
