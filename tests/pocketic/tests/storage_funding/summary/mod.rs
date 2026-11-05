use super::*;
use blob_test_protocol::status::FundingActivityView;
use ic_blob_storage_contracts::dto::operator::OperatorScope as Scope;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn local_funding_summary_preserves_old_uncredited_amounts_through_returns_traps_and_upgrade()
{
    let f = Fixture::new().await;
    let read = async || {
        f.funding_summary(f.operator, f.funding_scope())
            .await
            .unwrap()
    };
    let empty = read().await;
    assert_eq!(empty.local_activity, FundingActivityView::Clear);
    let first = f.funding_intent(1, 900);
    f.funding_trap(first, Action::Prepare, WriteFault::FundingAccounting)
        .await;
    assert_eq!(read().await, empty);
    f.funding(f.operator, first, Action::Prepare).await.unwrap();
    let held = read().await;
    assert_eq!(held.local_activity, FundingActivityView::Uncertain);
    assert_eq!(held.allocation.uncertain, 900);
    f.funding(f.operator, first, Action::Attempt).await.unwrap();
    f.funding_trap(first, Action::Callback(800), WriteFault::FundingAccounting)
        .await;
    assert_eq!(read().await, held);
    f.funding(f.operator, first, Action::Callback(800))
        .await
        .unwrap();
    for (id, action) in [(8, Action::NotEnqueued), (u128::MAX, Action::Callback(500))] {
        let intent = f.funding_intent(id, 500);
        f.funding(f.operator, intent, Action::Prepare)
            .await
            .unwrap();
        f.funding(f.operator, intent, Action::Attempt)
            .await
            .unwrap();
        f.funding(f.operator, intent, action).await.unwrap();
        let view = read().await;
        assert_eq!(view.last_operation, Some(id));
        assert_eq!(view.allocation.accepted, 100);
        assert_eq!(view.allocation.uncertain, 0);
        assert_eq!(view.local_activity, FundingActivityView::Uncertain);
    }
    let before = read().await;
    let stable = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(read().await, before);
    let local = f.local_status().await.funding;
    assert_eq!(local.transport_accepted, 100);
    assert_eq!(local.refunded, 1300);
    assert_eq!(local.not_enqueued, 500);
    assert_eq!(local.reserved_or_uncertain, 0);
    assert_eq!(local.available_allocation, 900);
    assert_eq!(local.attachment_allowance, 800);
    assert_eq!(local.retained_intents, 3);
    assert_eq!(local.last_operation, Some(u128::MAX));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        stable
    );
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(
        read().await,
        blob_test_protocol::storage::funding::summary::Summary {
            allocation: Allocation {
                fenced: true,
                ..before.allocation
            },
            ..before
        }
    );
    assert_eq!(
        f.local_status().await.funding,
        ic_blob_storage_contracts::dto::operator::LocalFundingStatus {
            cumulative_allocation: 1000,
            renewal_ceiling: 2000,
            fenced: true,
            ..local
        }
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn empty_summary_rejects_foreign_scope_and_clear_restored_history_stays_fenced() {
    let f = Fixture::new().await;
    let scope = f.funding_scope();
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(f.funding_summary(actor, scope).await, Err(Failure::Denied));
    }
    for changed in [
        Scope {
            service: f.other.into(),
            ..scope
        },
        Scope {
            cashier: f.other.into(),
            ..scope
        },
        Scope {
            payment_account: f.other.into(),
            ..scope
        },
        Scope {
            namespace: 2,
            ..scope
        },
    ] {
        assert_eq!(
            f.funding_summary(f.operator, changed).await,
            Err(Failure::Binding)
        );
    }
    let intent = f.funding_intent(1, 900);
    f.funding(f.operator, intent, Action::Prepare)
        .await
        .unwrap();
    f.funding(f.operator, intent, Action::Attempt)
        .await
        .unwrap();
    f.funding(f.operator, intent, Action::Callback(900))
        .await
        .unwrap();
    let clear = f.funding_summary(f.operator, scope).await.unwrap();
    assert_eq!(clear.local_activity, FundingActivityView::Clear);
    assert_eq!(clear.retained_intents, 1);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    let restored = f.funding_summary(f.operator, scope).await.unwrap();
    assert!(restored.allocation.fenced);
    assert_eq!(restored.local_activity, FundingActivityView::Clear);
    assert_eq!(
        f.funding(f.operator, f.funding_intent(2, 1), Action::Prepare)
            .await,
        Err(Failure::Fenced)
    );
}
