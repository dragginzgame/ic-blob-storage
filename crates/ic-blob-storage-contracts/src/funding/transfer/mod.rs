//! Cycle transfer facts, separate from provider credit and execution fees.

use std::num::NonZeroU128;

use thiserror::Error;

/// Validated arithmetic over supplied evidence for one exact funding attempt.
///
/// The caller must bind evidence to the original service, provider, account and
/// operation, and capture refunds in the matching callback before another await.
/// This value proves neither those bindings nor provider credit. It is not a
/// persisted record, retry permit or estimate of total execution costs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingTransfer {
    offered: NonZeroU128,
    evidence: TransferEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TransferEvidence {
    NotEnqueued,
    UnboundedCallback { refunded: u128 },
    Unknown,
}

impl FundingTransfer {
    /// Record positive proof that this attempt never entered the outgoing queue.
    ///
    /// Missing replies, cancellation and timeouts are not such proof. No refund
    /// callback exists in this case, but zero attached cycles were transferred.
    #[must_use]
    pub const fn not_enqueued(offered: NonZeroU128) -> Self {
        Self {
            offered,
            evidence: TransferEvidence::NotEnqueued,
        }
    }

    /// Validate a refund captured from this unbounded call's reply or rejection.
    ///
    /// This constructor must not receive a bounded-wait `SYS_UNKNOWN` observation.
    /// Refund arithmetic remains independent of reply decoding and account credit.
    /// # Errors
    /// Rejects a refund greater than the original attachment; never clamps it.
    pub const fn unbounded_callback(
        offered: NonZeroU128,
        refunded: u128,
    ) -> Result<Self, FundingTransferError> {
        if refunded > offered.get() {
            return Err(FundingTransferError::RefundExceedsOffer);
        }
        Ok(Self {
            offered,
            evidence: TransferEvidence::UnboundedCallback { refunded },
        })
    }

    /// Retain uncertainty after missing/lost evidence, including `SYS_UNKNOWN`.
    ///
    /// The full attachment remains potentially spent. No ambient refund, elapsed
    /// time or later balance can turn this observation into a known transfer.
    #[must_use]
    pub const fn unknown(offered: NonZeroU128) -> Self {
        Self {
            offered,
            evidence: TransferEvidence::Unknown,
        }
    }

    /// Original positive attachment, not the amount credited by the provider.
    #[must_use]
    pub const fn offered(self) -> NonZeroU128 {
        self.offered
    }

    /// Exact callback refund, absent for unknown and never-enqueued attempts.
    #[must_use]
    pub const fn refunded(self) -> Option<u128> {
        match self.evidence {
            TransferEvidence::UnboundedCallback { refunded } => Some(refunded),
            TransferEvidence::NotEnqueued | TransferEvidence::Unknown => None,
        }
    }

    /// Known transport acceptance, never provider credit; `None` means unknown.
    #[must_use]
    pub const fn accepted(self) -> Option<u128> {
        match self.evidence {
            TransferEvidence::NotEnqueued => Some(0),
            TransferEvidence::UnboundedCallback { refunded } => Some(self.offered.get() - refunded),
            TransferEvidence::Unknown => None,
        }
    }
}

/// Inconsistent supplied transport evidence must remain unusable.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FundingTransferError {
    /// A callback cannot refund more than this attempt offered.
    #[error("funding refund exceeds the original attachment")]
    RefundExceedsOffer,
}

#[cfg(test)]
mod tests;
