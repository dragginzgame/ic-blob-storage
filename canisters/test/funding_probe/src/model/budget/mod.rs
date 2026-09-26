//! Bounded local attachment accounting derived from retained exact attempts.
use super::{FundingAttemptRecord, FundingOutcome, JournalFailure, checked_transfer};
use candid::CandidType;
use serde::Deserialize;

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
        if reserve == 0 || allocated < reserve || operating_reserve == 0 {
            return Err(JournalFailure::Budget);
        }
        Ok(Self {
            allocated,
            reserve,
            operating_reserve,
            other_liabilities,
        })
    }

    pub fn snapshot(
        &self,
        attempts: &[FundingAttemptRecord],
    ) -> Result<FundingBudgetSnapshot, JournalFailure> {
        if attempts.len() > super::MAX_ATTEMPTS {
            return Err(JournalFailure::Capacity);
        }
        Self::new(
            self.allocated,
            self.reserve,
            self.operating_reserve,
            self.other_liabilities,
        )?;
        let mut view = FundingBudgetSnapshot {
            operating_reserve: self.operating_reserve,
            other_liabilities: self.other_liabilities,
            allocated: self.allocated,
            reserve: self.reserve,
            revision: 0,
            available: self.allocated,
            accepted: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
        };
        for entry in attempts {
            // Replay the accounting facts, not effects. Every original full
            // attachment must have fitted before any callback refund existed.
            let offered = entry.request.offered;
            if offered > view.transferable() {
                return Err(JournalFailure::Budget);
            }
            view.revision += 1;
            view.available -= offered;
            let transfer = checked_transfer(entry)?;
            if let Some(observation) = entry.observation {
                view.revision += 1;
                let accepted = transfer.accepted().ok_or(JournalFailure::Budget)?;
                view.accepted = view
                    .accepted
                    .checked_add(accepted)
                    .ok_or(JournalFailure::Budget)?;
                let returned = offered - accepted;
                if matches!(
                    observation.outcome,
                    FundingOutcome::NotEnqueued | FundingOutcome::LiquidityBlocked
                ) {
                    view.not_enqueued = view
                        .not_enqueued
                        .checked_add(returned)
                        .ok_or(JournalFailure::Budget)?;
                } else {
                    view.refunded = view
                        .refunded
                        .checked_add(returned)
                        .ok_or(JournalFailure::Budget)?;
                }
                view.available = view
                    .available
                    .checked_add(returned)
                    .ok_or(JournalFailure::Budget)?;
            } else {
                view.reserved_or_uncertain = view
                    .reserved_or_uncertain
                    .checked_add(offered)
                    .ok_or(JournalFailure::Budget)?;
            }
        }
        Ok(view)
    }
}

#[cfg(test)]
mod tests;
