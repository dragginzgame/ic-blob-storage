use super::*;
use ic_blob_storage::dto::{
    funding::{
        FundingHistoryCursor as Cursor, FundingHistoryEntry as Entry,
        FundingHistoryFailure as HistoryFailure, FundingHistoryPage as Page,
        FundingHistoryRequest as Input, FundingPhase,
    },
    operator::OperatorScope as Scope,
};

impl Fixture {
    fn funding_history(&self, actor: Principal, input: Input) -> Result<Page, HistoryFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "blob_funding_history", (input,))
            .unwrap()
    }
}

fn entry(intent: Intent, phase: Phase) -> Entry {
    Entry {
        scope: Scope {
            service: intent.service,
            cashier: intent.cashier,
            payment_account: intent.account,
            namespace: intent.namespace,
        },
        operation: intent.operation,
        offered: intent.offered,
        target_balance: intent.target_balance,
        phase: match phase {
            Phase::Prepared => FundingPhase::Prepared,
            Phase::Uncertain => FundingPhase::Uncertain,
            Phase::NotEnqueued => FundingPhase::NotEnqueued,
            Phase::Callback(refunded) => FundingPhase::Callback { refunded },
        },
    }
}

#[test]
fn funding_history_recovers_original_requests_and_reserved_intents_across_upgrade() {
    let f = Fixture::new();
    let query = Input {
        scope: f.funding_scope(),
        cursor: None,
    };
    assert_eq!(
        f.funding_history(f.operator, query),
        Ok(Page {
            request: query,
            fenced: false,
            entries: vec![],
            next: None
        })
    );
    let mut entries = Vec::new();
    for (id, action, phase) in [
        (1, Action::NotEnqueued, Phase::NotEnqueued),
        (9, Action::Callback(20), Phase::Callback(20)),
        (u128::MAX - 1, Action::Callback(0), Phase::Callback(0)),
    ] {
        let intent = f.funding_intent(id, 20);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        f.funding(f.operator, intent, Action::Attempt).unwrap();
        f.funding(f.operator, intent, action).unwrap();
        entries.push(entry(intent, phase));
    }
    let old_page = f.funding_history(f.operator, query).unwrap();
    let newest = Intent {
        target_balance: Some(u128::MAX),
        ..f.funding_intent(u128::MAX, 400)
    };
    f.funding(f.operator, newest, Action::Prepare).unwrap();
    let prepared = f.funding_history(f.operator, query).unwrap();
    assert_eq!(prepared.entries[0], entry(newest, Phase::Prepared));
    f.funding(f.operator, newest, Action::Attempt).unwrap();
    entries.push(entry(newest, Phase::Uncertain));
    entries.reverse();
    let mut first = f.funding_history(f.operator, query).unwrap();
    assert_eq!(first.entries, entries[..2]);
    let continuation = Input {
        cursor: first.next,
        ..query
    };
    let mut second = f.funding_history(f.operator, continuation).unwrap();
    assert_eq!(second.entries, entries[2..]);
    assert_eq!(second.next, None);
    let old_tail = f
        .funding_history(
            f.operator,
            Input {
                cursor: old_page.next,
                ..query
            },
        )
        .unwrap();
    assert_eq!(old_tail.entries, vec![entries[3]]);
    let before = f.funding_allocation();
    assert_eq!((before.accepted, before.uncertain), (20, 400));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    first.fenced = true;
    second.fenced = true;
    assert_eq!(f.funding_history(f.operator, query), Ok(first.clone()));
    assert_eq!(f.funding_history(f.operator, continuation), Ok(second));
    // The discovered identity is sufficient for canonical request inspection;
    // it remains insufficient to repeat an effect or release the restore fence.
    let view = first.entries[0];
    let recovered = Intent {
        service: view.scope.service,
        cashier: view.scope.cashier,
        account: view.scope.payment_account,
        namespace: view.scope.namespace,
        operation: view.operation,
        offered: view.offered,
        target_balance: view.target_balance,
    };
    check_recovered_request(&f, recovered);
    assert_eq!(
        f.funding(f.operator, recovered, Action::Attempt),
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.funding_allocation(),
        Allocation {
            fenced: true,
            ..before
        }
    );
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(f.funding_history(actor, query), Err(HistoryFailure::Denied));
    }
}

fn check_recovered_request(f: &Fixture, recovered: Intent) {
    let wire: Result<blob_test_protocol::storage::funding::Request, Failure> = f
        .harness
        .pic
        .query_candid_as(f.service, f.operator, "funding_request", (recovered,))
        .unwrap();
    let expected = ic_blob_storage::ops::caffeine::funding::request::CashierTopUpRequest::new(
        recovered.cashier,
        recovered.account,
        std::num::NonZeroU128::new(recovered.offered).unwrap(),
        recovered
            .target_balance
            .map(|amount| std::num::NonZeroU128::new(amount).unwrap()),
    )
    .unwrap();
    let wire = wire.unwrap();
    assert_eq!(wire.cashier, expected.cashier());
    assert_eq!(wire.method, expected.method_name());
    assert_eq!(wire.arguments, expected.arguments());
    assert_eq!(wire.offered, expected.offered().get());
}

#[test]
fn funding_history_rejects_foreign_scope_and_cursors_even_when_the_range_is_empty() {
    let f = Fixture::new();
    let scope = f.funding_scope();
    let query = Input {
        scope,
        cursor: None,
    };
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(f.funding_history(actor, query), Err(HistoryFailure::Denied));
    }
    for changed in [
        Scope {
            service: f.other,
            ..scope
        },
        Scope {
            cashier: f.other,
            ..scope
        },
        Scope {
            payment_account: f.other,
            ..scope
        },
        Scope {
            namespace: 2,
            ..scope
        },
    ] {
        assert_eq!(
            f.funding_history(
                f.operator,
                Input {
                    scope: changed,
                    ..query
                }
            ),
            Err(HistoryFailure::Binding)
        );
        assert_eq!(
            f.funding_history(
                f.operator,
                Input {
                    cursor: Some(Cursor {
                        scope: changed,
                        before_operation: 1
                    }),
                    ..query
                }
            ),
            Err(HistoryFailure::CursorScope)
        );
    }
    for invalid in [
        Input {
            scope: Scope {
                namespace: 0,
                ..scope
            },
            ..query
        },
        Input {
            cursor: Some(Cursor {
                scope,
                before_operation: 0,
            }),
            ..query
        },
    ] {
        assert_eq!(
            f.funding_history(f.operator, invalid),
            Err(HistoryFailure::Invalid)
        );
    }
}

#[test]
fn funding_history_preserves_reservations_and_excludes_rolled_back_intents() {
    let f = Fixture::new();
    let scope = f.funding_scope();
    let query = Input {
        scope,
        cursor: None,
    };
    let original = Intent {
        target_balance: Some(100),
        ..f.funding_intent(3, 400)
    };
    let initial = f.funding_allocation();
    f.funding_trap(original, Action::Prepare, WriteFault::FundingAccounting);
    assert!(
        f.funding_history(f.operator, query)
            .unwrap()
            .entries
            .is_empty()
    );
    assert_eq!(f.funding_allocation(), initial);
    f.funding(f.operator, original, Action::Prepare).unwrap();
    let before = f.funding_allocation();
    f.funding_trap(original, Action::Attempt, WriteFault::FundingIntents);
    let empty = f
        .funding_history(
            f.operator,
            Input {
                cursor: Some(Cursor {
                    scope,
                    before_operation: 1,
                }),
                ..query
            },
        )
        .unwrap();
    assert_eq!(
        empty,
        Page {
            request: Input {
                cursor: Some(Cursor {
                    scope,
                    before_operation: 1
                }),
                ..query
            },
            fenced: false,
            entries: vec![],
            next: None
        }
    );
    assert_eq!(
        f.funding_history(f.operator, query).unwrap().entries,
        vec![entry(original, Phase::Prepared)]
    );
    assert_eq!(f.funding_allocation(), before);
}
