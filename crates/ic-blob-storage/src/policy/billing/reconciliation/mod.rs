//! Diagnostic funding follow-up from transport facts, never settlement authority.

use std::num::NonZeroU128;

use crate::model::billing::transfer::FundingTransfer;

/// Work remaining for one exactly bound attempt, independent of Cashier replies.
///
/// No variant clears account-wide activity or authorizes a replacement payment.
/// Identity/recovery fences and other liabilities remain the workflow's concern.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingReconciliation {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::billing::transfer::FundingTransferError;

    #[test]
    fn known_transfer_amounts_require_credit_evidence_until_fully_refunded() {
        let offered = NonZeroU128::MAX;
        for refunded in [0, 1, u128::MAX - 1] {
            let transfer =
                FundingTransfer::unbounded_callback(offered, refunded).expect("bounded refund");
            assert_eq!(
                assess_funding_reconciliation(transfer),
                FundingReconciliation::CreditRequired {
                    accepted_cycles: NonZeroU128::new(u128::MAX - refunded)
                        .expect("positive acceptance"),
                },
            );
        }
        for transfer in [
            FundingTransfer::not_enqueued(offered),
            FundingTransfer::unbounded_callback(offered, offered.get()).expect("full refund"),
        ] {
            assert_eq!(
                assess_funding_reconciliation(transfer),
                FundingReconciliation::NoTransfer,
            );
        }
    }

    #[test]
    fn missing_or_invalid_transport_evidence_keeps_the_full_attachment_unresolved() {
        for offered in [1, u128::from(u64::MAX), u128::MAX] {
            let offered = NonZeroU128::new(offered).expect("positive amount");
            assert_eq!(
                assess_funding_reconciliation(FundingTransfer::unknown(offered)),
                FundingReconciliation::TransferUnknown {
                    offered_cycles: offered
                },
            );
        }
        let offered = NonZeroU128::new(100).expect("positive amount");
        let invalid = FundingTransfer::unbounded_callback(offered, 101);
        assert_eq!(invalid, Err(FundingTransferError::RefundExceedsOffer));
        let retained = invalid.unwrap_or_else(|_| FundingTransfer::unknown(offered));
        assert_eq!(
            assess_funding_reconciliation(retained),
            FundingReconciliation::TransferUnknown {
                offered_cycles: offered
            },
        );
    }
}
