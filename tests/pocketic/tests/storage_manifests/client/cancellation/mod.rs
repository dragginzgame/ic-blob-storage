use super::*;
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
async fn cancel(f: &Fixture, id: u128) -> Result<ManifestIntentView, C> {
    f.harness
        .pic
        .update_control_as(f.uploader, f.operator, "cancel_manifest", id, REPLY_BYTES)
        .await
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn uploader_cancellation_keeps_unsent_and_uncertain_history_through_recovery_and_upgrade() {
    for reconcile in [false, true] {
        let f = fixture().await;
        let (first, input) = intent(&f, 1, 2);
        f.admit(f.tenant, first).await.unwrap();
        save(&f, &input).await.unwrap();
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        let denied: Result<ManifestIntentView, C> = f
            .harness
            .pic
            .update_control_as(f.uploader, f.other, "cancel_manifest", 1u128, REPLY_BYTES)
            .await
            .unwrap();
        assert_eq!(denied, Err(C::Denied));
        assert!(!view(&f, 1).await.unwrap().cancelled);
        let unsent = cancel(&f, 1).await.unwrap();
        assert!(unsent.cancelled && !unsent.started);
        assert_eq!(unsent.result, None);
        assert_eq!(cancel(&f, 1).await, Ok(unsent.clone()));
        assert_eq!(save(&f, &input).await, Ok(unsent.clone()));
        assert_eq!(dispatch(&f, run(1)).await, Err(C::State));
        assert_eq!(recover(&f, 1).await, Err(C::State));
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
        let (second, pending_input) = intent(&f, 2, 3);
        f.admit(f.tenant, second).await.unwrap();
        save(&f, &pending_input).await.unwrap();
        assert_eq!(
            dispatch(
                &f,
                ManifestDispatch {
                    max_reply_bytes: 1,
                    ..run(2)
                }
            )
            .await,
            Err(C::Transport)
        );
        let pending = cancel(&f, 2).await.unwrap();
        assert!(pending.cancelled && pending.started);
        assert_eq!(pending.result, None);
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        let retained = if reconcile {
            let recovered = recover(&f, 2).await.unwrap();
            assert!(recovered.cancelled);
            assert_eq!(
                recovered.result,
                Some(Ok(pending_input.request.declaration.clone()))
            );
            recovered
        } else {
            pending
        };
        assert_eq!(dispatch(&f, run(2)).await, Err(C::State));
        let (_, third) = intent(&f, 3, 4);
        assert_eq!(save(&f, &third).await, Err(C::Capacity));
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
        f.harness
            .pic
            .upgrade_canister(
                f.uploader,
                wasm(),
                blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
                Some(f.controller),
            )
            .await
            .unwrap();
        assert_eq!(view(&f, 1).await, Ok(unsent));
        assert_eq!(view(&f, 2).await, Ok(retained));
        assert_eq!(cancel(&f, 2).await, Err(C::Fenced));
        assert_eq!(dispatch(&f, run(1)).await, Err(C::Fenced));
        assert_eq!(recover(&f, 2).await, Err(C::Fenced));
    }
}
