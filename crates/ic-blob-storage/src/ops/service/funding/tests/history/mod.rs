use super::*;
use crate::model::billing::journal::FundingJournalScope;
use crate::ops::service::funding::history::{
    FundingHistoryCursor, FundingHistoryError, FundingHistoryPage,
};

fn scope() -> FundingJournalScope {
    let input = input(1, 1);
    FundingJournalScope {
        service: input.service,
        cashier: input.cashier,
        account: input.account,
        namespace: input.namespace,
    }
}
fn finish(
    store: &mut StableFundingJournal<VectorMemory>,
    intent: FundingIntent,
    outcome: FundingTransportOutcome,
) {
    store.prepare(execution(), intent).unwrap();
    store.mark_attempted(execution(), intent).unwrap();
    store
        .record_transport(execution(), intent, source(), outcome)
        .unwrap();
}

#[test]
fn newest_first_pages_recover_complete_intents_with_gaps_and_full_width_ids() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let empty = FundingHistoryPage {
        entries: vec![],
        next: None,
    };
    assert_eq!(
        store.history(execution(), scope(), None, NonZeroUsize::MIN),
        Ok(empty.clone())
    );
    let ids = [1, 9, u128::MAX - 1, u128::MAX];
    for id in ids {
        let intent = FundingIntent {
            target_balance: Some(n(id)),
            ..input(id, 10)
        };
        finish(
            &mut store,
            intent,
            FundingTransportOutcome::Callback { refunded: 3 },
        );
    }
    let before = store.allocation(execution()).unwrap();
    let mut cursor = None;
    let mut collected = Vec::new();
    loop {
        let page = store
            .history(execution(), scope(), cursor, NonZeroUsize::MIN)
            .unwrap();
        assert_eq!(page.entries.len(), 1);
        let view = page.entries[0];
        assert_eq!(store.lookup(execution(), view.intent), Ok(Some(view)));
        assert_eq!(
            store
                .request(execution(), view.intent)
                .unwrap()
                .target_balance(),
            view.intent.target_balance
        );
        assert_eq!(view.state, FundingIntentState::Callback { refunded: 3 });
        collected.push(view.intent.operation.get());
        cursor = page.next;
        if cursor.is_none() {
            break;
        }
    }
    assert_eq!(collected, ids.into_iter().rev().collect::<Vec<_>>());
    assert_eq!(
        store.history(
            execution(),
            scope(),
            Some(FundingHistoryCursor {
                scope: scope(),
                before_operation: n(1)
            }),
            NonZeroUsize::MIN
        ),
        Ok(empty)
    );
    let gap = store
        .history(
            execution(),
            scope(),
            Some(FundingHistoryCursor {
                scope: scope(),
                before_operation: n(8),
            }),
            NonZeroUsize::MIN,
        )
        .unwrap();
    assert_eq!(gap.entries[0].intent.operation, n(1));
    assert!(gap.next.is_none());
    assert_eq!(store.allocation(execution()), Ok(before));
}

#[test]
fn authority_and_each_scope_component_are_checked_even_for_empty_ranges() {
    let store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let foreign = Principal::anonymous();
    assert_eq!(
        store.history(
            UploadContext {
                actor: foreign,
                ..execution()
            },
            scope(),
            None,
            NonZeroUsize::MIN
        ),
        Err(FundingJournalError::NotOperator.into())
    );
    assert_eq!(
        store.history(
            UploadContext {
                service: foreign,
                ..execution()
            },
            scope(),
            None,
            NonZeroUsize::MIN
        ),
        Err(FundingJournalError::Binding.into())
    );
    for changed in [
        FundingJournalScope {
            service: foreign,
            ..scope()
        },
        FundingJournalScope {
            cashier: foreign,
            ..scope()
        },
        FundingJournalScope {
            account: foreign,
            ..scope()
        },
        FundingJournalScope {
            namespace: n(2),
            ..scope()
        },
    ] {
        assert_eq!(
            store.history(execution(), changed, None, NonZeroUsize::MIN),
            Err(FundingJournalError::Binding.into())
        );
        assert_eq!(
            store.history(
                execution(),
                scope(),
                Some(FundingHistoryCursor {
                    scope: changed,
                    before_operation: n(1)
                }),
                NonZeroUsize::MIN
            ),
            Err(FundingHistoryError::CursorScope)
        );
    }
}

#[test]
fn fresh_sweeps_find_new_intents_and_state_changes_without_restoring_mutation_authority() {
    let memory = memories();
    let mut store = StableFundingJournal::install(copy(&memory), config(), allocation()).unwrap();
    finish(
        &mut store,
        input(1, 20),
        FundingTransportOutcome::NotEnqueued,
    );
    finish(
        &mut store,
        input(2, 20),
        FundingTransportOutcome::Callback { refunded: 0 },
    );
    let retained = store
        .history(execution(), scope(), None, NonZeroUsize::MIN)
        .unwrap();
    let intent = FundingIntent {
        target_balance: Some(n(77)),
        ..input(3, 400)
    };
    store.prepare(execution(), intent).unwrap();
    let prepared = store
        .history(execution(), scope(), None, NonZeroUsize::MIN)
        .unwrap();
    assert_eq!(
        prepared.entries,
        vec![FundingIntentView {
            intent,
            state: FundingIntentState::Prepared
        }]
    );
    store.mark_attempted(execution(), intent).unwrap();
    // Continuing the old sweep does not silently restart at newly inserted rows.
    let tail = store
        .history(execution(), scope(), retained.next, NonZeroUsize::MIN)
        .unwrap();
    assert_eq!(tail.entries[0].intent.operation, n(1));
    let newest = store
        .history(execution(), scope(), None, NonZeroUsize::MIN)
        .unwrap();
    assert_eq!(
        newest.entries,
        vec![FundingIntentView {
            intent,
            state: FundingIntentState::Uncertain
        }]
    );
    assert_eq!(prepared.entries[0].state, FundingIntentState::Prepared);
    let totals = store.allocation(execution()).unwrap();
    drop(store);
    let mut restored = StableFundingJournal::open(copy(&memory), config(), allocation()).unwrap();
    assert_eq!(
        restored.history(execution(), scope(), None, NonZeroUsize::MIN),
        Ok(newest)
    );
    assert_eq!(
        restored.history(execution(), scope(), retained.next, NonZeroUsize::MIN),
        Ok(tail)
    );
    assert_eq!(restored.allocation(execution()), Ok(totals));
    assert_eq!(
        restored.mark_attempted(execution(), intent),
        Err(FundingJournalError::Fenced)
    );
}

#[test]
fn pages_check_only_returned_rows_and_reject_invalid_identity_or_scope() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    finish(
        &mut store,
        input(1, 20),
        FundingTransportOutcome::NotEnqueued,
    );
    finish(
        &mut store,
        input(2, 20),
        FundingTransportOutcome::NotEnqueued,
    );
    // A bad older row must not be inspected by a page containing only the newest.
    // The next page rejects it rather than returning mismatched identity/scope.
    for bad in [
        input(9, 20),
        FundingIntent {
            account: Principal::anonymous(),
            ..input(1, 20)
        },
    ] {
        store.intents.insert(1, FundingIntentRecord::new(bad));
        let first = store
            .history(execution(), scope(), None, NonZeroUsize::MIN)
            .unwrap();
        assert_eq!(first.entries[0].intent.operation, n(2));
        assert_eq!(
            store.history(execution(), scope(), first.next, NonZeroUsize::MIN),
            Err(FundingJournalError::InvalidRecord.into())
        );
        assert_eq!(
            store.history(execution(), scope(), None, NonZeroUsize::new(2).unwrap()),
            Err(FundingJournalError::InvalidRecord.into())
        );
    }
}
