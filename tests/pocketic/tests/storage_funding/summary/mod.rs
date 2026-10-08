use super::*;
use blob_test_protocol::status::FundingActivityView;
use ic_blob_storage_contracts::dto::operator::OperatorScope as Scope;

#[test]
fn local_funding_summary_preserves_old_uncredited_amounts_through_returns_traps_and_upgrade() {
    let f = Fixture::new();
    let read = || f.funding_summary(f.operator, f.funding_scope()).unwrap();
    let empty = read();
    assert_eq!(empty.local_activity, FundingActivityView::Clear);
    let first = f.funding_intent(1, 900);
    f.funding_trap(first, Action::Prepare, WriteFault::FundingAccounting);
    assert_eq!(read(), empty);
    f.funding(f.operator, first, Action::Prepare).unwrap();
    let held = read();
    assert_eq!(held.local_activity, FundingActivityView::Uncertain);
    assert_eq!(held.allocation.uncertain, 900);
    f.funding(f.operator, first, Action::Attempt).unwrap();
    f.funding_trap(first, Action::Callback(800), WriteFault::FundingAccounting);
    assert_eq!(read(), held);
    f.funding(f.operator, first, Action::Callback(800)).unwrap();
    for (id, action) in [(8, Action::NotEnqueued), (u128::MAX, Action::Callback(500))] {
        let intent = f.funding_intent(id, 500);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        f.funding(f.operator, intent, Action::Attempt).unwrap();
        f.funding(f.operator, intent, action).unwrap();
        let view = read();
        assert_eq!(view.last_operation, Some(id));
        assert_eq!(view.allocation.accepted, 100);
        assert_eq!(view.allocation.uncertain, 0);
        assert_eq!(view.local_activity, FundingActivityView::Uncertain);
    }
    let before = read();
    let stable = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(read(), before);
    let local = f.local_status().funding;
    assert_eq!(local.transport_accepted, 100);
    assert_eq!(local.refunded, 1300);
    assert_eq!(local.not_enqueued, 500);
    assert_eq!(local.reserved_or_uncertain, 0);
    assert_eq!(local.available_allocation, 900);
    assert_eq!(local.attachment_allowance, 800);
    assert_eq!(local.retained_intents, 3);
    assert_eq!(local.last_operation, Some(u128::MAX));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), stable);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(
        read(),
        blob_test_protocol::storage::funding::summary::Summary {
            allocation: Allocation {
                fenced: true,
                ..before.allocation
            },
            ..before
        }
    );
    assert_eq!(
        f.local_status().funding,
        ic_blob_storage_contracts::dto::operator::LocalFundingStatus {
            cumulative_allocation: 1000,
            renewal_ceiling: 2000,
            fenced: true,
            ..local
        }
    );
}

#[test]
fn empty_summary_rejects_foreign_scope_and_clear_restored_history_stays_fenced() {
    let f = Fixture::new();
    let scope = f.funding_scope();
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(f.funding_summary(actor, scope), Err(Failure::Denied));
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
            f.funding_summary(f.operator, changed),
            Err(Failure::Binding)
        );
    }
    let intent = f.funding_intent(1, 900);
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    f.funding(f.operator, intent, Action::Attempt).unwrap();
    f.funding(f.operator, intent, Action::Callback(900))
        .unwrap();
    let clear = f.funding_summary(f.operator, scope).unwrap();
    assert_eq!(clear.local_activity, FundingActivityView::Clear);
    assert_eq!(clear.retained_intents, 1);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let restored = f.funding_summary(f.operator, scope).unwrap();
    assert!(restored.allocation.fenced);
    assert_eq!(restored.local_activity, FundingActivityView::Clear);
    assert_eq!(
        f.funding(f.operator, f.funding_intent(2, 1), Action::Prepare),
        Err(Failure::Fenced)
    );
}
