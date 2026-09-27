//! Attachment accounting for a bounded, sequential funding journal.
//!
//! A local allocation is separate from platform liquidity, execution costs and
//! provider credit. This module neither persists intents nor authorizes effects.

use std::num::{NonZeroU128, NonZeroUsize};
use thiserror::Error;

use super::transfer::FundingTransfer;

/// Installed attachment allocation and lifetime journal bound, without defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingAllocation {
    allocated: u128,
    reserve: NonZeroU128,
    max_attempts: NonZeroUsize,
}

impl FundingAllocation {
    /// Validate an allocation that must retain its positive reserve.
    /// # Errors
    /// Rejects a reserve larger than the entire allocation.
    pub const fn new(
        allocated: u128,
        reserve: NonZeroU128,
        max_attempts: NonZeroUsize,
    ) -> Result<Self, FundingAllocationError> {
        if reserve.get() > allocated {
            return Err(FundingAllocationError::ReserveExceedsAllocation);
        }
        Ok(Self {
            allocated,
            reserve,
            max_attempts,
        })
    }

    /// Reconstruct allocation use from the complete, ordered, sequential journal.
    ///
    /// Every original offer must fit before its own refund or proven unsent result
    /// is applied. The next intent must have been admitted after the preceding
    /// transport completed. Unknown transport can therefore appear only last.
    /// This is not a replay algorithm for overlapping calls.
    ///
    /// The caller must validate unique identities, bind every transfer to the same
    /// allocation and preserve all lifetime entries. These arithmetic checks cannot
    /// detect omitted history, establish freshness or release a recovery fence.
    /// Incoming receipts, gross cycle top-ups and provider balance reports are not
    /// inputs and cannot replenish the installed allocation.
    /// # Errors
    /// Rejects excess history, invalid sequencing, an original reserve violation
    /// or overflowing lifetime return totals. No partial projection is returned.
    pub fn reconstruct(
        self,
        attempts: &[FundingTransfer],
    ) -> Result<FundingAllocationView, FundingAllocationError> {
        if attempts.len() > self.max_attempts.get() {
            return Err(FundingAllocationError::Capacity);
        }
        let mut view = FundingAllocationView {
            reserve: self.reserve.get(),
            available: self.allocated,
            accepted: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
        };
        for (index, transfer) in attempts.iter().enumerate() {
            let offered = transfer.offered().get();
            if offered > view.transferable() {
                return Err(FundingAllocationError::ReserveWouldBeViolated {
                    attempt_index: index,
                });
            }
            view.available -= offered;
            if let Some(accepted) = transfer.accepted() {
                // Known accepted amounts are bounded by the original allocation.
                view.accepted += accepted;
                let returned = offered - accepted;
                let total = if transfer.refunded().is_some() {
                    &mut view.refunded
                } else {
                    &mut view.not_enqueued
                };
                // Lifetime returns can exceed the allocation after repeated
                // full refunds/unsent attempts, so they need checked addition.
                *total = total
                    .checked_add(returned)
                    .ok_or(FundingAllocationError::TotalsOverflow)?;
                view.available += returned;
            } else {
                if index + 1 != attempts.len() {
                    return Err(FundingAllocationError::PendingNotLast);
                }
                view.reserved_or_uncertain = offered;
            }
        }
        Ok(view)
    }
}

/// Complete attachment projection; never evidence of credit or spendable cycles.
/// Available + accepted + reserved/uncertain equals the original allocation.
/// Historical refunded/unsent totals are separate and must not be added again.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingAllocationView {
    reserve: u128,
    available: u128,
    accepted: u128,
    refunded: u128,
    not_enqueued: u128,
    reserved_or_uncertain: u128,
}

impl FundingAllocationView {
    /// Remaining allocation, including the retained reserve.
    #[must_use]
    pub const fn available(self) -> u128 {
        self.available
    }

    /// Remaining attachment allowance after the reserve, not dispatch authority.
    #[must_use]
    pub const fn transferable(self) -> u128 {
        self.available - self.reserve
    }

    /// Transport-accepted attachments, still charged regardless of provider replies.
    #[must_use]
    pub const fn accepted(self) -> u128 {
        self.accepted
    }

    /// Lifetime exact callback refunds; already returned to available allocation.
    #[must_use]
    pub const fn refunded(self) -> u128 {
        self.refunded
    }

    /// Lifetime proven unsent attachments; these have no callback refund.
    #[must_use]
    pub const fn not_enqueued(self) -> u128 {
        self.not_enqueued
    }

    /// Whole attachment of the last attempt with missing transport evidence.
    #[must_use]
    pub const fn reserved_or_uncertain(self) -> u128 {
        self.reserved_or_uncertain
    }
}

/// A journal cannot supply a complete, valid attachment projection.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FundingAllocationError {
    /// The positive reserve exceeds the original allocation.
    #[error("reserve exceeds attachment allocation")]
    ReserveExceedsAllocation,
    /// Lifetime history exceeds the explicitly installed bound.
    #[error("attachment journal capacity exceeded")]
    Capacity,
    /// An original full offer could not have preserved the reserve.
    #[error("attachment at index {attempt_index} violates the reserve")]
    ReserveWouldBeViolated {
        /// Zero-based index in the supplied journal, not an operation identity.
        attempt_index: usize,
    },
    /// A later attempt follows missing transport evidence in a sequential journal.
    #[error("unknown transport must be the final sequential attempt")]
    PendingNotLast,
    /// Exact lifetime totals do not fit; never wrap or saturate the projection.
    #[error("attachment return totals overflow")]
    TotalsOverflow,
}

#[cfg(test)]
mod tests;
