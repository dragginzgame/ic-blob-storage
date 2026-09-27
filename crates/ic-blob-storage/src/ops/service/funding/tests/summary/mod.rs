use super::*;
use crate::{
    model::billing::journal::FundingJournalScope,
    policy::billing::{admission::FundingActivity, reconciliation::assess_uncredited_allocation},
};

fn scope() -> FundingJournalScope {
    let intent = input(1, 1);
    FundingJournalScope {
        service: intent.service,
        cashier: intent.cashier,
        account: intent.account,
        namespace: intent.namespace,
    }
}

#[test]
fn summary_keeps_earlier_acceptance_visible_after_later_no_transfer_history_and_restore() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let empty = store.summary(execution(), scope()).unwrap();
    assert_eq!(empty.retained_intents, 0);
    assert_eq!(empty.last_operation, None);
    assert_eq!(
        assess_uncredited_allocation(empty.allocation),
        FundingActivity::Clear
    );
    for (id, offered, outcome) in [
        (1, 900, FundingTransportOutcome::Callback { refunded: 800 }),
        (8, 700, FundingTransportOutcome::NotEnqueued),
        (
            u128::MAX - 1,
            500,
            FundingTransportOutcome::Callback { refunded: 500 },
        ),
    ] {
        let intent = input(id, offered);
        store.prepare(execution(), intent).unwrap();
        assert_eq!(
            assess_uncredited_allocation(store.summary(execution(), scope()).unwrap().allocation),
            FundingActivity::Uncertain
        );
        store.mark_attempted(execution(), intent).unwrap();
        store
            .record_transport(execution(), intent, source(), outcome)
            .unwrap();
        let observed = store.summary(execution(), scope()).unwrap();
        assert_eq!(observed.last_operation, Some(n(id)));
        assert_eq!(observed.allocation.accepted(), 100);
        assert_eq!(
            assess_uncredited_allocation(observed.allocation),
            FundingActivity::Uncertain
        );
    }
    store.prepare(execution(), input(u128::MAX, 800)).unwrap();
    let held = store.summary(execution(), scope()).unwrap();
    assert_eq!(held.retained_intents, held.intent_capacity);
    assert_eq!(held.allocation.reserved_or_uncertain(), 800);
    assert_eq!(held.last_operation, Some(NonZeroU128::MAX));
    drop(store);
    let restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    assert_eq!(
        restored.summary(execution(), scope()).unwrap(),
        super::super::summary::FundingJournalSummary {
            fenced: true,
            ..held
        }
    );
}

#[test]
fn summary_checks_empty_scope_authority_and_count_consistency_without_mutating() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let stranger = Principal::anonymous();
    assert_eq!(
        store.summary(
            UploadContext {
                actor: stranger,
                ..execution()
            },
            scope()
        ),
        Err(FundingJournalError::NotOperator)
    );
    assert_eq!(
        store.summary(
            UploadContext {
                service: stranger,
                ..execution()
            },
            scope()
        ),
        Err(FundingJournalError::Binding)
    );
    for changed in [
        FundingJournalScope {
            service: stranger,
            ..scope()
        },
        FundingJournalScope {
            cashier: stranger,
            ..scope()
        },
        FundingJournalScope {
            account: stranger,
            ..scope()
        },
        FundingJournalScope {
            namespace: n(2),
            ..scope()
        },
    ] {
        assert_eq!(
            store.summary(execution(), changed),
            Err(FundingJournalError::Binding)
        );
    }
    let before = m.accounting.borrow().clone();
    let empty = store.summary(execution(), scope()).unwrap();
    assert_eq!(*m.accounting.borrow(), before);
    drop(store);
    let mut restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    let view = restored.summary(execution(), scope()).unwrap();
    assert_eq!(view.allocation, empty.allocation);
    assert!(view.fenced);
    assert_eq!(
        assess_uncredited_allocation(view.allocation),
        FundingActivity::Clear
    );
    assert_eq!(
        restored.prepare(execution(), input(1, 1)),
        Err(FundingJournalError::Fenced)
    );
    drop(restored);
    store = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    store
        .intents
        .insert(1, FundingIntentRecord::new(input(1, 1)));
    assert_eq!(
        store.summary(execution(), scope()),
        Err(FundingJournalError::InvalidRecord)
    );
    assert_eq!(*m.accounting.borrow(), before);
}
