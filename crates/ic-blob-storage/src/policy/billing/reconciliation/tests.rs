use super::*;
use ic_blob_storage_contracts::funding::transfer::FundingTransferError;

#[test]
fn maintained_allocation_activity_agrees_with_complete_history_at_amount_boundaries() {
    use crate::model::billing::allocation::FundingAllocation;
    use std::num::NonZeroUsize;
    let allocation =
        FundingAllocation::new(u128::MAX, NonZeroU128::MIN, NonZeroUsize::new(4).unwrap()).unwrap();
    for offered in [1, 900, u128::MAX - 1] {
        let offered = NonZeroU128::new(offered).unwrap();
        for transfer in [
            FundingTransfer::unknown(offered),
            FundingTransfer::not_enqueued(offered),
            FundingTransfer::unbounded_callback(offered, 0).unwrap(),
            FundingTransfer::unbounded_callback(offered, offered.get()).unwrap(),
        ] {
            let history = [FundingTransfer::not_enqueued(NonZeroU128::MIN), transfer];
            let totals = allocation.reconstruct(&history).unwrap();
            assert_eq!(
                assess_uncredited_allocation(totals),
                assess_uncredited_activity(history.into_iter().map(assess_funding_reconciliation))
            );
        }
    }
    assert_eq!(
        assess_uncredited_allocation(allocation.reconstruct(&[]).unwrap()),
        FundingActivity::Clear
    );
}

#[test]
fn uncredited_history_is_not_cleared_by_later_no_transfer_attempts() {
    assert_eq!(assess_uncredited_activity([]), FundingActivity::Clear);
    assert_eq!(
        assess_uncredited_activity([FundingReconciliation::NoTransfer; 3]),
        FundingActivity::Clear,
    );
    for unresolved in [
        FundingReconciliation::CreditRequired {
            accepted_cycles: NonZeroU128::MAX,
        },
        FundingReconciliation::TransferUnknown {
            offered_cycles: NonZeroU128::MAX,
        },
    ] {
        for entries in [
            [unresolved, FundingReconciliation::NoTransfer],
            [FundingReconciliation::NoTransfer, unresolved],
            [unresolved, unresolved],
        ] {
            assert_eq!(
                assess_uncredited_activity(entries),
                FundingActivity::Uncertain
            );
        }
    }
}

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
