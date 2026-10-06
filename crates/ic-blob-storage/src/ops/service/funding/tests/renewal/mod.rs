//! Bounded local grants preserve chronological offers and immutable spent receipts.
use super::credit::{paid, receipt};
use super::*;
use crate::model::billing::journal::renewal::{FundingBudgetRenewal, FundingRenewalError};
fn grant(intent: FundingIntent, additional: u128) -> FundingBudgetRenewal {
    FundingBudgetRenewal {
        intent,
        additional: n(additional),
    }
}
fn bounded() -> FundingAllocation {
    allocation().with_renewal_ceiling(2000).unwrap()
}
#[test]
fn bounded_renewal_preserves_spent_totals_original_receipts_and_restored_offer_order() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), bounded()).unwrap();
    let first = input(1, 900);
    paid(&mut store, first, 0);
    store
        .record_credit(execution(), receipt(first, 900, 1))
        .unwrap();
    assert!(store.renew_budget(execution(), grant(first, 900)).unwrap());
    let totals = store.allocation(execution()).unwrap();
    assert_eq!(
        (
            totals.allocated(),
            totals.available(),
            totals.accepted(),
            totals.credit_confirmed()
        ),
        (1900, 1000, 900, 900)
    );
    assert_eq!(
        store
            .outcome(execution(), first)
            .unwrap()
            .unwrap()
            .renewed_allocation,
        Some(n(900))
    );
    let second = input(2, 900);
    paid(&mut store, second, 0);
    store
        .record_credit(execution(), receipt(second, 900, 2))
        .unwrap();
    assert!(!store.renew_budget(execution(), grant(first, 900)).unwrap());
    assert_eq!(
        store.renew_budget(execution(), grant(second, 900)),
        Err(FundingAllocationError::RenewalCeiling.into())
    );
    assert!(store.renew_budget(execution(), grant(second, 100)).unwrap());
    let totals = store.allocation(execution()).unwrap();
    assert_eq!(
        (
            totals.allocated(),
            totals.available(),
            totals.accepted(),
            totals.credit_confirmed()
        ),
        (2000, 200, 1800, 1800)
    );
    let outcomes = [
        store.outcome(execution(), first).unwrap(),
        store.outcome(execution(), second).unwrap(),
    ];
    drop(store);
    let mut reopened = StableFundingJournal::open(copy(&m), config(), bounded()).unwrap();
    assert_eq!(reopened.allocation(execution()).unwrap(), totals);
    assert_eq!(
        [
            reopened.outcome(execution(), first).unwrap(),
            reopened.outcome(execution(), second).unwrap()
        ],
        outcomes
    );
    assert_eq!(
        reopened.renew_budget(execution(), grant(first, 900)),
        Err(FundingJournalError::Fenced)
    );
}
#[test]
fn renewal_refuses_uncredited_unknown_wrong_authority_amount_identity_and_conflict_without_writes()
{
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), bounded()).unwrap();
    let first = input(1, 100);
    paid(&mut store, first, 0);
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    assert_eq!(
        store.renew_budget(execution(), grant(first, 100)),
        Err(FundingRenewalError::CreditRequired.into())
    );
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
    store
        .record_credit(execution(), receipt(first, 100, 1))
        .unwrap();
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    assert_eq!(
        store.renew_budget(
            UploadContext {
                actor: Principal::anonymous(),
                ..execution()
            },
            grant(first, 100)
        ),
        Err(FundingJournalError::NotOperator)
    );
    assert_eq!(
        store.renew_budget(execution(), grant(first, 101)),
        Err(FundingRenewalError::Amount.into())
    );
    assert_eq!(
        store.renew_budget(execution(), grant(input(1, 99), 100)),
        Err(FundingIntentError::Conflict.into())
    );
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
    assert!(store.renew_budget(execution(), grant(first, 50)).unwrap());
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    assert_eq!(
        store.renew_budget(execution(), grant(first, 51)),
        Err(FundingRenewalError::Conflict.into())
    );
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
    let second = input(2, 100);
    paid(&mut store, second, 0);
    let third = input(3, 100);
    paid(&mut store, third, 0);
    store
        .record_credit(execution(), receipt(third, 100, 3))
        .unwrap();
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    assert_eq!(
        store.renew_budget(execution(), grant(third, 100)),
        Err(FundingRenewalError::Unreconciled.into())
    );
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
    store
        .record_credit(execution(), receipt(second, 100, 2))
        .unwrap();
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    assert_eq!(
        store.renew_budget(execution(), grant(second, 100)),
        Err(FundingRenewalError::NotLatest.into())
    );
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
}
#[test]
fn renewal_never_recycles_lifetime_intent_slots_or_exceeds_the_disabled_ceiling() {
    let mut store = StableFundingJournal::install(memories(), config(), bounded()).unwrap();
    for operation in 1..=4 {
        let original = input(operation, 100);
        paid(&mut store, original, 0);
        store
            .record_credit(
                execution(),
                receipt(original, 100, u8::try_from(operation).unwrap()),
            )
            .unwrap();
        store
            .renew_budget(execution(), grant(original, 100))
            .unwrap();
    }
    assert_eq!(store.allocation(execution()).unwrap().accepted(), 400);
    assert_eq!(
        store.prepare(execution(), input(5, 100)),
        Err(FundingAllocationError::Capacity.into())
    );
    let mut finite = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let first = input(1, 100);
    paid(&mut finite, first, 0);
    finite
        .record_credit(execution(), receipt(first, 100, 1))
        .unwrap();
    assert_eq!(
        finite.renew_budget(execution(), grant(first, 100)),
        Err(FundingAllocationError::RenewalCeiling.into())
    );
    assert_eq!(
        allocation().with_renewal_ceiling(999),
        Err(FundingAllocationError::RenewalCeiling)
    );
}
#[test]
fn restoration_refuses_grants_moved_past_original_offer_without_repair() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), bounded()).unwrap();
    let first = input(1, 900);
    paid(&mut store, first, 0);
    store
        .record_credit(execution(), receipt(first, 900, 1))
        .unwrap();
    store.renew_budget(execution(), grant(first, 900)).unwrap();
    let second = input(2, 900);
    paid(&mut store, second, 0);
    store
        .record_credit(execution(), receipt(second, 900, 2))
        .unwrap();
    let original = FundingIntentRecord::new(first)
        .attempted()
        .unwrap()
        .complete(FundingTransportOutcome::Callback { refunded: 0 })
        .unwrap()
        .confirm_credit(receipt(first, 900, 1))
        .unwrap();
    store.intents.insert(1, original);
    let moved = store
        .required(second)
        .unwrap()
        .renew(grant(second, 900))
        .unwrap();
    store.intents.insert(2, moved);
    drop(store);
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    assert!(StableFundingJournal::open(copy(&m), config(), bounded()).is_err());
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
}
