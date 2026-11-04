//! Pure funding diagnosis from transport facts and retained host confirmations.

use std::num::NonZeroU128;

use super::admission::FundingActivity;
use crate::model::billing::allocation::FundingAllocationView;
use crate::model::billing::journal::credit::FundingCreditReceipt;
use ic_blob_storage_contracts::funding::transfer::FundingTransfer;

/// Diagnose the complete maintained totals of one uncredited local journal.
/// Includes successful callback acknowledgments and independent credit confirmations.
/// Prepared/uncertain reservations conservatively block alongside accepted amounts.
/// A local `Clear` establishes neither account-wide completeness nor freshness,
/// provider credit, spendability or permission to release a restoration fence.
#[must_use]
pub const fn assess_uncredited_allocation(allocation: FundingAllocationView) -> FundingActivity {
    if allocation.unresolved() != 0 || allocation.reserved_or_uncertain() != 0 {
        FundingActivity::Uncertain
    } else {
        FundingActivity::Clear
    }
}

/// Diagnose retained attempts for which no independent credit settlement exists.
///
/// Supply the complete journal for one bound account, including older completed
/// callbacks. A later no-transfer attempt cannot clear an earlier unknown transfer
/// or unverified credit. Empty input is clear only within this observation; it
/// proves neither journal freshness nor safe recovery and never permits a retry.
#[must_use]
pub fn assess_uncredited_activity(
    attempts: impl IntoIterator<Item = FundingReconciliation>,
) -> FundingActivity {
    if attempts.into_iter().any(|attempt| {
        !matches!(
            attempt,
            FundingReconciliation::NoTransfer
                | FundingReconciliation::CreditConfirmed { .. }
                | FundingReconciliation::ProviderAcknowledged { .. }
        )
    }) {
        FundingActivity::Uncertain
    } else {
        FundingActivity::Clear
    }
}

/// Work remaining for one exactly bound attempt; acknowledgment is not credit proof.
///
/// No variant clears account-wide activity or authorizes a replacement payment.
/// Identity/recovery fences and other liabilities remain the workflow's concern.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingReconciliation {
    /// Exact accepted attachment acknowledged by the matching successful callback.
    /// This is operational provider acknowledgment, not independently verified credit.
    ProviderAcknowledged {
        /// Full positive accepted attachment from the original callback.
        accepted_cycles: NonZeroU128,
    },
    /// Exact immutable credit receipt was established by the trusted host.
    /// Spent attachments remain charged and other activity is independent.
    CreditConfirmed {
        /// Full transport-accepted attachment covered by the receipt.
        accepted_cycles: NonZeroU128,
        /// Preserved evidence fingerprint, not a self-authenticating proof.
        receipt_digest: [u8; 32],
    },
    /// This attempt transferred no attached cycles. Execution fees are separate.
    NoTransfer,
    /// Transport acceptance is exact, but provider credit is still unestablished.
    CreditRequired {
        /// Accepted attachment; neither a reported balance nor a credited amount.
        accepted_cycles: NonZeroU128,
    },
    /// Missing transport evidence leaves the whole attachment potentially spent.
    TransferUnknown {
        /// Conservative bound for this attachment, excluding execution fees.
        offered_cycles: NonZeroU128,
    },
}

/// Identify remaining reconciliation without accepting balance reports as receipts.
///
/// A decoded success, provider error or malformed reply cannot change this result.
/// Invalid refund arithmetic must retain uncertainty rather than be replaced with
/// a fabricated zero acceptance. The workflow must preserve the original intent.
#[must_use]
pub const fn assess_funding_reconciliation(transfer: FundingTransfer) -> FundingReconciliation {
    match transfer.accepted() {
        Some(accepted) => match NonZeroU128::new(accepted) {
            Some(accepted_cycles) => FundingReconciliation::CreditRequired { accepted_cycles },
            None => FundingReconciliation::NoTransfer,
        },
        None => FundingReconciliation::TransferUnknown {
            offered_cycles: transfer.offered(),
        },
    }
}

/// Combine exact transport facts with a validated immutable journal receipt.
/// Acknowledgment comes only from the exact retained successful callback.
/// Its reported balance is never used to derive a credited amount.
#[must_use]
pub fn assess_reconciled_funding(
    transfer: FundingTransfer,
    credit: Option<FundingCreditReceipt>,
    acknowledged: Option<NonZeroU128>,
) -> FundingReconciliation {
    if let Some(credit) = credit
        && transfer.accepted() == Some(credit.accepted_cycles().get())
    {
        return FundingReconciliation::CreditConfirmed {
            accepted_cycles: credit.accepted_cycles(),
            receipt_digest: credit.receipt_digest(),
        };
    }
    if let Some(accepted_cycles) = acknowledged
        && transfer.accepted() == Some(accepted_cycles.get())
    {
        return FundingReconciliation::ProviderAcknowledged { accepted_cycles };
    }
    assess_funding_reconciliation(transfer)
}

#[cfg(test)]
mod tests;
