use super::*;
use crate::model::FundingJournalRecord;
use blob_test_protocol::funding::{
    FundingFailure, FundingObservation, FundingReceiptRecord, FundingReconciliationView,
    FundingReplyMode, FundingRequest,
};
use candid::Principal;

fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn journal(allocated: u128, reserve: u128) -> FundingJournalRecord {
    FundingJournalRecord::new(
        p(1),
        p(2),
        p(3),
        FundingBudgetRecord::new(allocated, reserve, 1, 0).unwrap(),
    )
}
fn request(id: u64, offered: u128) -> FundingRequest {
    FundingRequest {
        id,
        offered,
        accept: offered,
        reply: FundingReplyMode::Success,
        trap_callback: false,
    }
}
fn observation(offered: u128, refund: u128) -> FundingObservation {
    let accepted = offered - refund;
    FundingObservation {
        refunded: Some(refund),
        transport_accepted: Some(accepted),
        outcome: FundingOutcome::ReportedSuccess,
        reconciliation: if accepted == 0 {
            FundingReconciliationView::NoTransfer
        } else {
            FundingReconciliationView::CreditRequired(accepted)
        },
    }
}

#[test]
fn full_reservation_and_exact_returns_preserve_the_installed_reserve() {
    let mut j = journal(1000, 100);
    assert_eq!(
        j.admit(p(3), request(1, 901)),
        Err(FundingFailure::ReserveWouldBeViolated)
    );
    assert_eq!(j.budget().revision, 0);
    let r = request(1, 900);
    j.admit(p(3), r).unwrap();
    assert_eq!(j.budget().available, 100);
    assert_eq!(j.budget().reserved_or_uncertain, 900);
    assert_eq!(
        j.admit(p(3), request(2, 1)),
        Err(FundingFailure::InProgress)
    );
    j.complete(r, observation(900, 500));
    let view = j.budget();
    assert_eq!(
        (view.available, view.accepted, view.refunded, view.revision),
        (600, 400, 500, 2)
    );
    assert_eq!(view.reserved_or_uncertain, 0);
    assert_eq!(
        j.admit(p(3), request(2, 501)),
        Err(FundingFailure::ReserveWouldBeViolated)
    );
    let unsent = request(2, 500);
    j.admit(p(3), unsent).unwrap();
    j.complete(
        unsent,
        FundingObservation {
            refunded: None,
            transport_accepted: Some(0),
            outcome: FundingOutcome::NotEnqueued,
            reconciliation: FundingReconciliationView::NoTransfer,
        },
    );
    assert_eq!(
        (
            j.budget().available,
            j.budget().not_enqueued,
            j.budget().refunded,
            j.budget().revision
        ),
        (600, 500, 500, 4)
    );
    let before = j.budget();
    j.record_acceptance(FundingReceiptRecord {
        id: 9,
        available: 100,
        accepted: 100,
    });
    assert_eq!(j.budget(), before); // Incoming receipts never replenish this allocation.
    j.fence();
    assert_eq!(j.budget(), before);
    assert_eq!(j.validate(p(1)), Ok(()));
}

#[test]
fn full_refunds_cannot_hide_an_original_over_budget_reservation() {
    assert!(matches!(
        FundingBudgetRecord::new(1000, 100, 0, 0),
        Err(JournalFailure::Budget)
    ));
    let mut j = journal(1000, 100);
    let r = request(1, 900);
    j.admit(p(3), r).unwrap();
    j.complete(r, observation(900, 900));
    assert_eq!(j.budget().available, 1000);
    j.budget = FundingBudgetRecord::new(999, 100, 1, 0).unwrap();
    assert_eq!(j.validate(p(1)), Err(JournalFailure::Budget));
    j.budget.reserve = 0;
    assert_eq!(j.validate(p(1)), Err(JournalFailure::Budget));
}

#[test]
fn unknown_attachments_remain_reserved_and_maximum_allocations_do_not_overflow() {
    let mut j = journal(u128::MAX, 1);
    let r = request(1, 100);
    j.admit(p(3), r).unwrap();
    assert_eq!(j.budget().available, u128::MAX - 100);
    assert_eq!(j.budget().reserved_or_uncertain, 100);
    let before = j.budget();
    j.fence();
    assert_eq!(j.budget(), before);
    assert_eq!(j.validate(p(1)), Ok(()));
    assert_eq!(j.admit(p(3), request(2, 1)), Err(FundingFailure::Fenced));
}
