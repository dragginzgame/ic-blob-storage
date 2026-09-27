use super::*;

fn positive(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).unwrap()
}
fn allocation(amount: u128, reserve: u128, capacity: usize) -> FundingAllocation {
    FundingAllocation::new(
        amount,
        positive(reserve),
        NonZeroUsize::new(capacity).unwrap(),
    )
    .unwrap()
}
fn callback(offered: u128, refunded: u128) -> FundingTransfer {
    FundingTransfer::unbounded_callback(positive(offered), refunded).unwrap()
}

#[test]
fn full_offers_must_fit_before_refunds_or_proven_unsent_results() {
    let allocation = allocation(1000, 100, 4);
    for transfer in [
        callback(901, 901),
        FundingTransfer::not_enqueued(positive(901)),
    ] {
        assert_eq!(
            allocation.reconstruct(&[transfer]),
            Err(FundingAllocationError::ReserveWouldBeViolated { attempt_index: 0 })
        );
    }
    let paid = callback(900, 500); // 400 accepted, 600 left, only 500 transferable.
    for transfer in [
        callback(501, 501),
        FundingTransfer::not_enqueued(positive(501)),
    ] {
        assert_eq!(
            allocation.reconstruct(&[paid, transfer]),
            Err(FundingAllocationError::ReserveWouldBeViolated { attempt_index: 1 })
        );
    }
    let view = allocation.reconstruct(&[paid, callback(500, 500)]).unwrap();
    assert_eq!(view.available(), 600);
    assert_eq!(view.accepted(), 400);
    assert_eq!(view.refunded(), 1000);
    assert_eq!(view.transferable(), 500);
}

#[test]
fn callback_returns_and_unsent_offers_are_distinct_and_never_counted_twice() {
    let allocation = allocation(1000, 100, 4);
    let attempts = [
        callback(900, 500),
        FundingTransfer::not_enqueued(positive(500)),
        callback(500, 500),
        FundingTransfer::unknown(positive(500)),
    ];
    let view = allocation.reconstruct(&attempts).unwrap();
    assert_eq!(view.available(), 100);
    assert_eq!(view.accepted(), 400);
    assert_eq!(view.reserved_or_uncertain(), 500);
    assert_eq!(view.not_enqueued(), 500);
    assert_eq!(view.refunded(), 1000);
    assert_eq!(view.transferable(), 0);
    assert_eq!(
        view.available() + view.accepted() + view.reserved_or_uncertain(),
        1000
    );
    // Rebuilding the same evidence cannot compound charges or returns.
    assert_eq!(allocation.reconstruct(&attempts), Ok(view));
}

#[test]
fn unknown_transport_blocks_later_sequential_evidence_and_keeps_full_offer() {
    let allocation = allocation(1000, 100, 3);
    let unknown = FundingTransfer::unknown(positive(400));
    let view = allocation.reconstruct(&[unknown]).unwrap();
    assert_eq!((view.available(), view.reserved_or_uncertain()), (600, 400));
    assert_eq!(
        (view.accepted(), view.refunded(), view.not_enqueued()),
        (0, 0, 0)
    );
    for later in [
        callback(1, 1),
        FundingTransfer::not_enqueued(positive(1)),
        unknown,
    ] {
        assert_eq!(
            allocation.reconstruct(&[unknown, later]),
            Err(FundingAllocationError::PendingNotLast)
        );
    }
}

#[test]
fn explicit_bounds_include_no_transfer_history_and_reserve_only_allocations() {
    assert_eq!(
        FundingAllocation::new(99, positive(100), NonZeroUsize::MIN),
        Err(FundingAllocationError::ReserveExceedsAllocation)
    );
    let reserve_only = allocation(u128::MAX, u128::MAX, 1);
    let empty = reserve_only.reconstruct(&[]).unwrap();
    assert_eq!(empty.available(), u128::MAX);
    assert_eq!(empty.transferable(), 0);
    let unsent = FundingTransfer::not_enqueued(positive(1));
    assert_eq!(
        reserve_only.reconstruct(&[unsent]),
        Err(FundingAllocationError::ReserveWouldBeViolated { attempt_index: 0 })
    );
    let bounded = allocation(1000, 100, 1);
    assert_eq!(bounded.reconstruct(&[unsent]).unwrap().available(), 1000);
    assert_eq!(
        bounded.reconstruct(&[unsent, unsent]),
        Err(FundingAllocationError::Capacity)
    );
}

#[test]
fn maximum_amounts_conserve_allocation_and_overflow_never_returns_partial_totals() {
    let allocation = allocation(u128::MAX, 1, 3);
    for transfer in [
        callback(u128::MAX - 1, 0),
        FundingTransfer::unknown(positive(u128::MAX - 1)),
    ] {
        let view = allocation.reconstruct(&[transfer]).unwrap();
        assert_eq!(view.available(), 1);
        assert_eq!(
            view.accepted() + view.reserved_or_uncertain(),
            u128::MAX - 1
        );
    }
    for (returned, single) in [
        (callback(u128::MAX - 1, u128::MAX - 1), callback(1, 1)),
        (
            FundingTransfer::not_enqueued(positive(u128::MAX - 1)),
            FundingTransfer::not_enqueued(positive(1)),
        ),
    ] {
        let one = allocation.reconstruct(&[returned]).unwrap();
        assert_eq!(one.available(), u128::MAX);
        assert_eq!(
            allocation.reconstruct(&[returned, returned]),
            Err(FundingAllocationError::TotalsOverflow)
        );
        assert_eq!(allocation.reconstruct(&[returned]), Ok(one));
        let exact = allocation.reconstruct(&[returned, single]).unwrap();
        assert_eq!(exact.available(), u128::MAX);
        assert_eq!(exact.refunded().max(exact.not_enqueued()), u128::MAX);
        assert_eq!(
            allocation.reconstruct(&[returned, single, single]),
            Err(FundingAllocationError::TotalsOverflow)
        );
    }
    // Distinct lifetime totals may exceed u128 together; neither is current usage.
    let independent = allocation
        .reconstruct(&[
            callback(u128::MAX - 1, u128::MAX - 1),
            FundingTransfer::not_enqueued(positive(u128::MAX - 1)),
        ])
        .unwrap();
    assert_eq!(independent.available(), u128::MAX);
    assert_eq!(independent.refunded(), u128::MAX - 1);
    assert_eq!(independent.not_enqueued(), u128::MAX - 1);
}
