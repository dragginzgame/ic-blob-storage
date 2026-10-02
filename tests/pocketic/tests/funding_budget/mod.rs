//! Actual IC effects against an explicitly installed local attachment envelope.
use super::*;
use blob_test_protocol::funding::{
    budget::FundingBudgetInput,
    preview::{
        FundingPreviewBlocker, FundingPreviewFailure, FundingPreviewRequest, FundingPreviewView,
    },
};

#[test]
fn reserve_rejection_is_atomic_and_only_exact_returns_release_the_allocation() {
    let f = Fixture::with_budget(FundingBudgetInput {
        operating_reserve: 1_000_000_000,
        other_liabilities: 0,
        allocated: 200_000_000,
        reserve: 100_000_000,
    });
    let initial = f.status_unchanged().budget;
    let mut too_large = request(1, 0, FundingReplyMode::Success);
    too_large.offered += 1;
    let bytes = [f.sender, f.receiver].map(|id| f.harness.pic.get_stable_memory(id));
    assert_eq!(
        f.fund(f.driver, too_large),
        Err(FundingFailure::ReserveWouldBeViolated)
    );
    assert_eq!(f.receipts(), []);
    assert_eq!(f.status_unchanged().budget, initial);
    assert!(
        [f.sender, f.receiver].map(|id| f.harness.pic.get_stable_memory(id)) == bytes,
        "rejection must save no partial intent"
    );
    f.fund(f.driver, request(1, 40_000_000, FundingReplyMode::Success))
        .unwrap();
    let paid = f.status_unchanged().budget;
    assert_eq!(
        (
            paid.available,
            paid.accepted,
            paid.refunded,
            paid.reserved_or_uncertain,
            paid.revision
        ),
        (160_000_000, 40_000_000, 60_000_000, 0, 2)
    );
    assert_eq!(
        f.fund(f.driver, request(2, 0, FundingReplyMode::Success)),
        Err(FundingFailure::ReserveWouldBeViolated)
    );
    let mut refund = request(2, 0, FundingReplyMode::Success);
    refund.offered = 60_000_000;
    f.fund(f.driver, refund).unwrap();
    let refunded = f.status_unchanged().budget;
    assert_eq!(
        (refunded.available, refunded.refunded, refunded.revision),
        (160_000_000, 120_000_000, 4)
    );
    assert_eq!(
        f.fund(f.driver, refund),
        Err(FundingFailure::AlreadyAdmitted)
    );
    f.harness.pic.add_cycles(f.sender, 1_000_000_000_000);
    let incoming: Result<FundingObservation, FundingFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.receiver,
            f.driver,
            "fund",
            (request(99, 50_000_000, FundingReplyMode::Success),),
        )
        .unwrap();
    incoming.unwrap();
    assert_eq!(f.status_as(f.sender, f.driver).unwrap().budget, refunded);
}

#[test]
fn callback_rollback_and_restore_keep_the_full_attachment_reserved() {
    let f = Fixture::with_budget(FundingBudgetInput {
        operating_reserve: 1_000_000_000,
        other_liabilities: 0,
        allocated: 200_000_000,
        reserve: 100_000_000,
    });
    let mut paid = request(1, 40_000_000, FundingReplyMode::Success);
    paid.trap_callback = true;
    let failure = f
        .harness
        .pic
        .update_call(
            f.sender,
            f.driver,
            "fund",
            candid::encode_one(paid).unwrap(),
        )
        .unwrap_err();
    assert_eq!(failure.reject_code, RejectCode::CanisterError);
    let pending = f.assert_unknown_status(paid).budget;
    assert_eq!(
        (
            pending.revision,
            pending.available,
            pending.reserved_or_uncertain
        ),
        (1, 100_000_000, 100_000_000)
    );
    assert_eq!(f.receipts()[0].accepted, 40_000_000);
    f.harness
        .pic
        .upgrade_canister(
            f.sender,
            std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
            candid::encode_one(FundingUpgradeArgs {
                trap_after_restore: false,
            })
            .unwrap(),
            None,
        )
        .unwrap();
    assert_eq!(f.assert_unknown_status(paid).budget, pending);
    let preview: Result<FundingPreviewView, FundingPreviewFailure> = f
        .harness
        .pic
        .query_candid_as(
            f.sender,
            f.driver,
            "preview_funding",
            (FundingPreviewRequest {
                service: f.sender,
                peer: f.receiver,
                id: 2,
                requested_cycles: 1,
                revision: pending.revision,
            },),
        )
        .unwrap();
    let preview = preview.unwrap();
    assert_eq!(preview.budget, pending);
    for blocker in [
        FundingPreviewBlocker::RecoveryFenced,
        FundingPreviewBlocker::FundingUncertain,
        FundingPreviewBlocker::SpendabilityUnknown,
        FundingPreviewBlocker::BudgetReserveWouldBeViolated {
            transferable_cycles: 0,
        },
    ] {
        assert!(preview.blockers.contains(&blocker));
    }
    assert_eq!(
        f.fund(f.driver, request(2, 0, FundingReplyMode::Success)),
        Err(FundingFailure::Fenced)
    );
}

#[test]
fn operating_holds_block_dispatch_and_liquidity_changes_without_a_budget_revision() {
    let f = Fixture::with_budget(FundingBudgetInput {
        allocated: 200_000_000,
        reserve: 100_000_000,
        operating_reserve: 1_000_000_000,
        other_liabilities: 3_000_000_000_000,
    });
    let proposed = FundingPreviewRequest {
        service: f.sender,
        peer: f.receiver,
        id: 1,
        requested_cycles: 100_000_000,
        revision: 0,
    };
    let preview = |request| -> FundingPreviewView {
        let result: Result<FundingPreviewView, FundingPreviewFailure> = f
            .harness
            .pic
            .query_candid_as(f.sender, f.driver, "preview_funding", (request,))
            .unwrap();
        result.unwrap()
    };
    let before = preview(proposed);
    assert!(before.liquidity.call_cost > 0);
    assert!(before.liquidity.liquid_cycles > proposed.requested_cycles);
    assert!(
        before
            .blockers
            .contains(&FundingPreviewBlocker::LiquidityWouldBeViolated {
                transferable_cycles: 0
            })
    );
    assert!(!before.blockers.iter().any(|b| matches!(
        b,
        FundingPreviewBlocker::BudgetReserveWouldBeViolated { .. }
    )));
    let outcome = f
        .fund(f.driver, request(1, 40_000_000, FundingReplyMode::Success))
        .unwrap();
    assert_eq!(
        outcome,
        FundingObservation {
            refunded: None,
            transport_accepted: Some(0),
            outcome: FundingOutcome::LiquidityBlocked,
            reconciliation: FundingReconciliationView::NoTransfer,
        }
    );
    assert_eq!(f.receipts(), []);
    let status = f.status_unchanged();
    assert_eq!(status.budget.available, before.budget.available);
    assert_eq!(status.budget.not_enqueued, proposed.requested_cycles);
    assert_eq!(status.budget.refunded, 0);
    assert_eq!(status.budget.revision, 2);
    assert_eq!(
        f.fund(f.driver, request(1, 0, FundingReplyMode::Success)),
        Err(FundingFailure::AlreadyAdmitted)
    );

    let proposed = FundingPreviewRequest {
        id: 2,
        revision: 2,
        ..proposed
    };
    let before_top_up = preview(proposed);
    f.harness.pic.add_cycles(f.sender, 3_000_000_000_000);
    // A repeated query may be cached independently of cycle top-ups. Observe a
    // different proposed operation; the update must still recheck its own funds.
    let after_top_up = preview(FundingPreviewRequest { id: 3, ..proposed });
    assert_eq!(after_top_up.budget, before_top_up.budget);
    assert!(after_top_up.liquidity.liquid_cycles > before_top_up.liquidity.liquid_cycles);
    assert!(
        !after_top_up
            .blockers
            .iter()
            .any(|b| matches!(b, FundingPreviewBlocker::LiquidityWouldBeViolated { .. }))
    );
    assert!(
        after_top_up
            .blockers
            .contains(&FundingPreviewBlocker::SpendabilityUnknown)
    );
    assert_eq!(after_top_up.available_cycles, None);
    let paid = f
        .fund(f.driver, request(2, 40_000_000, FundingReplyMode::Success))
        .unwrap();
    assert_eq!(paid.transport_accepted, Some(40_000_000));
    assert_eq!(f.receipts().len(), 1);
    let budget = f.status_unchanged().budget;
    assert_eq!(budget.other_liabilities, 3_000_000_000_000);
    assert_eq!(budget.operating_reserve, 1_000_000_000);
    assert_eq!(budget.accepted, 40_000_000);
    f.upgrade_both();
    assert_eq!(f.status_unchanged().budget, budget);
    assert_eq!(
        f.fund(f.driver, request(3, 0, FundingReplyMode::Success)),
        Err(FundingFailure::Fenced)
    );
}

#[test]
fn platform_call_cost_blocks_an_attachment_that_fits_the_liquid_balance() {
    let f = Fixture::new();
    let preview = |requested_cycles| -> FundingPreviewView {
        let result: Result<FundingPreviewView, FundingPreviewFailure> = f
            .harness
            .pic
            .query_candid_as(
                f.sender,
                f.driver,
                "preview_funding",
                (FundingPreviewRequest {
                    service: f.sender,
                    peer: f.receiver,
                    id: 1,
                    requested_cycles,
                    revision: 0,
                },),
            )
            .unwrap();
        result.unwrap()
    };
    let observed = preview(1);
    let amount = observed.liquidity.liquid_cycles
        - observed.budget.operating_reserve
        - observed.budget.other_liabilities
        - observed.liquidity.call_cost / 2;
    let checked = preview(amount);
    assert!(
        amount
            < checked.liquidity.liquid_cycles
                - checked.budget.operating_reserve
                - checked.budget.other_liabilities
    );
    assert!(
        checked
            .blockers
            .iter()
            .any(|b| matches!(b, FundingPreviewBlocker::LiquidityWouldBeViolated { .. }))
    );
    let attempt = FundingRequest {
        offered: amount,
        ..request(1, 0, FundingReplyMode::Success)
    };
    let outcome = f.fund(f.driver, attempt).unwrap();
    assert_eq!(outcome.outcome, FundingOutcome::LiquidityBlocked);
    assert_eq!(outcome.refunded, None);
    assert_eq!(outcome.transport_accepted, Some(0));
    assert_eq!(f.receipts(), []);
    assert_eq!(f.status_unchanged().budget.not_enqueued, amount);
}

#[test]
fn unsent_attempts_ignore_callback_controls_and_remain_bounded_after_restore() {
    let f = Fixture::with_budget(FundingBudgetInput {
        allocated: 1_000_000_000,
        reserve: 1,
        operating_reserve: 1,
        other_liabilities: u128::MAX,
    });
    let initial = f.status_unchanged().budget;
    let modes = [
        FundingReplyMode::Success,
        FundingReplyMode::DelayedSuccess,
        FundingReplyMode::Trap,
    ];
    for (id, reply) in (0..16).zip(modes.into_iter().cycle()) {
        let attempt = FundingRequest {
            trap_callback: true,
            ..request(id, 40_000_000, reply)
        };
        let outcome = f.fund(f.driver, attempt).unwrap();
        assert_eq!(
            outcome,
            FundingObservation {
                refunded: None,
                transport_accepted: Some(0),
                outcome: FundingOutcome::LiquidityBlocked,
                reconciliation: FundingReconciliationView::NoTransfer,
            }
        );
        assert_eq!(
            f.fund(f.driver, attempt),
            Err(FundingFailure::AlreadyAdmitted)
        );
    }
    assert_eq!(f.receipts(), []);
    assert_eq!(
        f.fund(f.driver, request(16, 0, FundingReplyMode::Success)),
        Err(FundingFailure::Limit)
    );
    let retained = f.status_unchanged();
    assert_eq!(retained.budget.available, initial.available);
    assert_eq!(retained.budget.refunded, 0);
    assert_eq!(retained.budget.accepted, 0);
    assert_eq!(retained.budget.reserved_or_uncertain, 0);
    assert_eq!(retained.funding_activity, FundingActivityView::Clear);
    f.upgrade_both();
    let restored = f.status_unchanged();
    assert_eq!(restored.budget, retained.budget);
    assert_eq!(restored.attempts, retained.attempts);
    assert!(restored.fenced);
    assert_eq!(
        f.fund(f.driver, request(17, 0, FundingReplyMode::Success)),
        Err(FundingFailure::Fenced)
    );
}

#[test]
fn actual_zero_and_full_refund_callbacks_still_trap_and_keep_the_full_reservation() {
    for accepted in [0, 100_000_000] {
        let f = Fixture::new();
        let attempt = FundingRequest {
            trap_callback: true,
            ..request(1, accepted, FundingReplyMode::Success)
        };
        let failure = f
            .harness
            .pic
            .update_call(
                f.sender,
                f.driver,
                "fund",
                candid::encode_one(attempt).unwrap(),
            )
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        let status = f.assert_unknown_status(attempt);
        assert_eq!(status.budget.reserved_or_uncertain, attempt.offered);
        assert_eq!(f.receipts()[0].accepted, accepted);
        assert_eq!(
            f.fund(f.driver, request(2, 0, FundingReplyMode::Success)),
            Err(FundingFailure::InProgress)
        );
    }
}
