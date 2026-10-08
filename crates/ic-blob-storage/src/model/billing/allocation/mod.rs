//! Attachment accounting for a bounded, sequential funding journal.
//!
//! A local allocation is separate from platform liquidity, execution costs and
//! provider credit. This module neither persists intents nor authorizes effects.

use std::num::{NonZeroU128, NonZeroUsize};
use thiserror::Error;

use ic_blob_storage_contracts::funding::transfer::FundingTransfer;
pub(crate) mod record;

/// Installed attachment allocation and lifetime journal bound, without defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingAllocation {
    allocated: u128,
    renewal_ceiling: u128,
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
        if ic_blob_storage_contracts::configuration::envelope::validate_allocation(
            allocated,
            reserve.get(),
            allocated,
        )
        .is_err()
        {
            return Err(FundingAllocationError::ReserveExceedsAllocation);
        }
        Ok(Self {
            allocated,
            renewal_ceiling: allocated,
            reserve,
            max_attempts,
        })
    }

    /// Set the immutable ceiling for cumulative host-authorized allocation.
    /// Equal to initial allocation disables increases. This supplies no cycles.
    /// # Errors
    /// Rejects a ceiling below the initial allocation.
    pub const fn with_renewal_ceiling(
        mut self,
        ceiling: u128,
    ) -> Result<Self, FundingAllocationError> {
        if ic_blob_storage_contracts::configuration::envelope::validate_allocation(
            self.allocated,
            self.reserve.get(),
            ceiling,
        )
        .is_err()
        {
            return Err(FundingAllocationError::RenewalCeiling);
        }
        self.renewal_ceiling = ceiling;
        Ok(self)
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
    /// This transport-only projection has no confirmed credits. The durable
    /// journal additionally reconstructs its validated immutable credit receipts.
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
        let mut view = self.empty();
        for (index, transfer) in attempts.iter().enumerate() {
            view = view.reserve(transfer.offered(), index)?;
            view = view.resolve(*transfer)?;
        }
        Ok(view)
    }

    pub(crate) const fn empty(self) -> FundingAllocationView {
        FundingAllocationView {
            allocated: self.allocated,
            renewal_ceiling: self.renewal_ceiling,
            reserve: self.reserve.get(),
            available: self.allocated,
            accepted: 0,
            credit_confirmed: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
        }
    }
}

/// Complete attachment projection; never evidence of credit or spendable cycles.
/// Available + accepted + reserved/uncertain equals the cumulative authorized allocation.
/// Historical refunded/unsent totals are separate and must not be added again.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingAllocationView {
    allocated: u128,
    renewal_ceiling: u128,
    reserve: u128,
    available: u128,
    accepted: u128,
    credit_confirmed: u128,
    refunded: u128,
    not_enqueued: u128,
    reserved_or_uncertain: u128,
}

impl FundingAllocationView {
    /// Cumulative authorized allocation, including retained budget increases.
    #[must_use]
    pub const fn allocated(self) -> u128 {
        self.allocated
    }
    /// Immutable installed ceiling, not liquidity or provider credit.
    #[must_use]
    pub const fn renewal_ceiling(self) -> u128 {
        self.renewal_ceiling
    }
    pub(crate) fn renew(mut self, additional: NonZeroU128) -> Result<Self, FundingAllocationError> {
        if additional.get() > self.renewal_ceiling - self.allocated {
            return Err(FundingAllocationError::RenewalCeiling);
        }
        self.allocated += additional.get();
        self.available += additional.get();
        Ok(self)
    }
    pub(crate) fn reserve(
        mut self,
        offered: NonZeroU128,
        index: usize,
    ) -> Result<Self, FundingAllocationError> {
        if self.reserved_or_uncertain != 0 {
            return Err(FundingAllocationError::PendingNotLast);
        }
        if offered.get() > self.transferable() {
            return Err(FundingAllocationError::ReserveWouldBeViolated {
                attempt_index: index,
            });
        }
        self.available -= offered.get();
        self.reserved_or_uncertain = offered.get();
        Ok(self)
    }
    pub(crate) fn resolve(
        mut self,
        transfer: FundingTransfer,
    ) -> Result<Self, FundingAllocationError> {
        assert_eq!(
            self.reserved_or_uncertain,
            transfer.offered().get(),
            "exact reserved offer"
        );
        if let Some(accepted) = transfer.accepted() {
            self.accepted += accepted;
            let returned = transfer.offered().get() - accepted;
            let total = if transfer.refunded().is_some() {
                &mut self.refunded
            } else {
                &mut self.not_enqueued
            };
            *total = total
                .checked_add(returned)
                .ok_or(FundingAllocationError::TotalsOverflow)?;
            self.available += returned;
            self.reserved_or_uncertain = 0;
        }
        Ok(self)
    }
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

    /// Transport-accepted attachments with exact retained host credit evidence.
    /// Confirmation never replenishes available allocation or erases acceptance.
    #[must_use]
    pub const fn credit_confirmed(self) -> u128 {
        self.credit_confirmed
    }

    /// Accepted attachments still awaiting independent credit reconciliation.
    #[must_use]
    pub const fn uncredited(self) -> u128 {
        self.accepted - self.credit_confirmed
    }

    pub(crate) fn confirm_credit(
        mut self,
        accepted: NonZeroU128,
    ) -> Result<Self, FundingAllocationError> {
        if accepted.get() > self.uncredited() {
            return Err(FundingAllocationError::CreditExceedsAcceptance);
        }
        self.credit_confirmed += accepted.get();
        Ok(self)
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
    /// A budget increase cannot exceed the immutable installed ceiling.
    #[error("funding renewal ceiling exceeded")]
    RenewalCeiling,
    /// Confirmations cannot exceed retained accepted attachments.
    #[error("confirmed credit exceeds accepted attachments")]
    CreditExceedsAcceptance,
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
