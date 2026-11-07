//! Receiver recovery witnessed through the production sender, never a second funding engine.
use super::*;
use blob_test_protocol::funding::FundingUpgradeArgs;
use ic_testkit::pocket_ic::common::rest::BlobCompression;

async fn upgrade(f: &Fixture, trap: bool) -> Result<(), ic_testkit::pocket_ic::ClientError> {
    f.harness
        .pic
        .upgrade_canister(
            f.operator,
            std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
            blob_test_protocol::fixture_wire::encode_one(
                "fixture_post_upgrade",
                FundingUpgradeArgs {
                    trap_after_restore: trap,
                },
            )
            .unwrap(),
            None,
        )
        .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn restored_receiver_retains_receipts_and_refunds_new_calls_before_acceptance() {
    for stale in [false, true] {
        let f = Fixture::with_cashier().await;
        let empty = f.harness.pic.get_stable_memory(f.operator).await.unwrap();
        let paid = f.funding_intent(1, 400);
        f.configure_cashier(paid, 200, FundingReplyMode::Success)
            .await;
        f.funding(f.operator, paid, Action::Prepare).await.unwrap();
        f.transport(f.operator, input(paid)).await.unwrap();
        let receipt = f.incoming().await;
        let known = f.retained_outcome(f.operator, paid).await.unwrap();
        let next = f.funding_intent(2, 300);
        // Receiver control remains ephemeral; restoring must fence it even with exact arguments.
        f.configure_cashier(next, 100, FundingReplyMode::Success)
            .await;
        if stale {
            f.harness
                .pic
                .set_stable_memory(f.operator, empty, BlobCompression::NoCompression)
                .await
                .unwrap();
        }
        upgrade(&f, false).await.unwrap();
        assert_eq!(f.incoming().await, if stale { vec![] } else { receipt });
        // Reconfiguration must not release the fence on acceptance.
        f.configure_cashier(next, 100, FundingReplyMode::Success)
            .await;
        let frozen = f.harness.pic.get_stable_memory(f.operator).await.unwrap();
        f.funding(f.operator, next, Action::Prepare).await.unwrap();
        let observed = f.transport(f.operator, input(next)).await.unwrap();
        assert_eq!(
            (observed.accepted, observed.refunded, observed.outcome),
            (
                0,
                Some(300),
                FundingOutcome::Rejected(RejectCode::CanisterError as u32)
            )
        );
        assert_eq!(
            f.harness.pic.get_stable_memory(f.operator).await.unwrap(),
            frozen
        );
        assert_eq!(f.retained_outcome(f.operator, paid).await.unwrap(), known);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failed_receiver_restore_is_atomic_and_preserves_the_active_receipts() {
    let f = Fixture::with_cashier().await;
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 200, FundingReplyMode::Success)
        .await;
    f.funding(f.operator, first, Action::Prepare).await.unwrap();
    f.transport(f.operator, input(first)).await.unwrap();
    let receipt = f.incoming().await;
    let saved = f.harness.pic.get_stable_memory(f.operator).await.unwrap();
    let failure = upgrade(&f, true).await.expect_err("incoming restore traps");
    assert!(
        crate::canister_rejection(&failure)
            .reject_message
            .contains("deliberate fixture restore failure")
    );
    assert_eq!(f.incoming().await, receipt);
    assert_eq!(
        f.harness.pic.get_stable_memory(f.operator).await.unwrap(),
        saved
    );
    f.harness
        .pic
        .set_stable_memory(f.operator, vec![], BlobCompression::NoCompression)
        .await
        .unwrap();
    upgrade(&f, false)
        .await
        .expect_err("missing journal cannot reset receiver");
    assert_eq!(f.incoming().await, receipt);
    f.harness
        .pic
        .set_stable_memory(f.operator, saved, BlobCompression::NoCompression)
        .await
        .unwrap();
    let next = f.funding_intent(2, 300);
    f.configure_cashier(next, 100, FundingReplyMode::Success)
        .await;
    f.funding(f.operator, next, Action::Prepare).await.unwrap();
    assert_eq!(
        f.transport(f.operator, input(next)).await.unwrap().accepted,
        100
    );
    assert_eq!(f.incoming().await.len(), 2);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn receiver_upgrade_after_acceptance_preserves_spent_cycles_and_exact_refund() {
    let f = Fixture::with_cashier().await;
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 400, FundingReplyMode::DelayedSuccess)
        .await;
    f.funding(f.operator, intent, Action::Prepare)
        .await
        .unwrap();
    let call = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.operator,
            "fixture_funding_transport",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(input(intent))
                .unwrap(),
        )
        .await
        .unwrap();
    for _ in 0..20 {
        f.harness.pic.tick().await.unwrap();
        if !f.incoming().await.is_empty() {
            break;
        }
    }
    let receipts = vec![FundingReceiptRecord {
        id: 1,
        available: 900,
        accepted: 400,
    }];
    assert_eq!(f.incoming().await, receipts);
    let pending = f
        .retained_outcome(f.operator, intent)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(pending.phase, FundingPhase::Uncertain);
    assert_eq!(pending.response, None);
    upgrade(&f, false).await.unwrap();
    let response: Result<Observation, Failure> =
        ic_blob_storage_pocketic_tests::control_calls::decode_control_one(
            &f.harness.pic.await_call(call).await.unwrap(),
            REPLY_BYTES,
        )
        .unwrap();
    let response = response.unwrap();
    assert_eq!(
        (response.accepted, response.refunded, response.outcome),
        (
            400,
            Some(500),
            FundingOutcome::Rejected(RejectCode::CanisterError as u32)
        )
    );
    assert_eq!(f.incoming().await, receipts);
    let retained = f
        .retained_outcome(f.operator, intent)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        retained.reconciliation,
        FundingReconciliationView::CreditRequired(400)
    );
    assert_eq!(
        f.transport(f.operator, input(intent)).await,
        Err(Failure::Phase)
    );
    f.upgrade_storage().await;
    assert_eq!(
        f.retained_outcome(f.operator, intent).await,
        Ok(Some(FundingOutcomeResponse {
            fenced: true,
            ..retained
        }))
    );
    assert_eq!(f.incoming().await, receipts);
}
