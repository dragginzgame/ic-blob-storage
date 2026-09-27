//! Bounded fixture configuration projected through shared attachment accounting.
use super::{FundingAttemptRecord, JournalFailure, checked_transfer};
use candid::CandidType;
use ic_blob_storage::model::billing::allocation::FundingAllocation;
use serde::Deserialize;
use std::num::{NonZeroU128, NonZeroUsize};

#[derive(CandidType, Deserialize)]
pub(crate) struct FundingBudgetRecord {
    allocated: u128,
    reserve: u128,
    operating_reserve: u128,
    other_liabilities: u128,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct FundingBudgetSnapshot {
    pub operating_reserve: u128,
    pub other_liabilities: u128,
    pub allocated: u128,
    pub reserve: u128,
    pub revision: u64,
    pub available: u128,
    pub accepted: u128,
    pub refunded: u128,
    pub not_enqueued: u128,
    pub reserved_or_uncertain: u128,
}

impl FundingBudgetSnapshot {
    pub fn transferable(&self) -> u128 {
        self.available - self.reserve
    }
}

impl FundingBudgetRecord {
    pub fn new(
        allocated: u128,
        reserve: u128,
        operating_reserve: u128,
        other_liabilities: u128,
    ) -> Result<Self, JournalFailure> {
        if operating_reserve == 0 {
            return Err(JournalFailure::Budget);
        }
        let value = Self {
            allocated,
            reserve,
            operating_reserve,
            other_liabilities,
        };
        value.allocation()?;
        Ok(value)
    }

    fn allocation(&self) -> Result<FundingAllocation, JournalFailure> {
        FundingAllocation::new(
            self.allocated,
            NonZeroU128::new(self.reserve).ok_or(JournalFailure::Budget)?,
            NonZeroUsize::new(super::MAX_ATTEMPTS).expect("positive fixture capacity"),
        )
        .map_err(|_| JournalFailure::Budget)
    }

    pub fn snapshot(
        &self,
        attempts: &[FundingAttemptRecord],
    ) -> Result<FundingBudgetSnapshot, JournalFailure> {
        if attempts.len() > super::MAX_ATTEMPTS {
            return Err(JournalFailure::Capacity);
        }
        if self.operating_reserve == 0 {
            return Err(JournalFailure::Budget);
        }
        let transfers = attempts
            .iter()
            .map(checked_transfer)
            .collect::<Result<Vec<_>, _>>()?;
        let view = self
            .allocation()?
            .reconstruct(&transfers)
            .map_err(|_| JournalFailure::Budget)?;
        Ok(FundingBudgetSnapshot {
            operating_reserve: self.operating_reserve,
            other_liabilities: self.other_liabilities,
            allocated: self.allocated,
            reserve: self.reserve,
            // Bounded to twice MAX_ATTEMPTS; identity/versioning belongs to this
            // journal rather than the shared amount-only projection.
            revision: attempts
                .iter()
                .map(|entry| 1 + u64::from(entry.observation.is_some()))
                .sum(),
            available: view.available(),
            accepted: view.accepted(),
            refunded: view.refunded(),
            not_enqueued: view.not_enqueued(),
            reserved_or_uncertain: view.reserved_or_uncertain(),
        })
    }
}

#[cfg(test)]
mod tests;
