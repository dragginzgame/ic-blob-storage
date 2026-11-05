//! Actual late uploader acknowledgment cannot undo either application's cancellation.
use super::*;
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep held acknowledgment, cancellation and exposed cleanup in one installed scenario"
)]
async fn cancellation_during_preparation_acknowledgment_preserves_tombstones_and_exposed_cleanup() {
    for exposed in [false, true] {
        let f = uploader_fixture().await;
        let (input, permission, manifest) = prepared(&f).await;
        admit(&f, &input).await.unwrap();
        let intent = ManifestIntent {
            id: 1,
            request: f.preparation_input(&manifest),
        };
        let saved: Result<ManifestIntentView, ConsumerFailure> = f
            .harness
            .pic
            .update_control_as(
                f.uploader,
                f.operator,
                "save_manifest",
                &intent,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        saved.unwrap();
        let dispatch = ManifestDispatch {
            id: 1,
            max_reply_bytes: 4096,
            fault: ManifestFault::None,
            hold: true,
        };
        let call = f
            .harness
            .pic
            .submit_call(
                f.uploader,
                f.operator,
                "dispatch_manifest",
                ic_blob_storage_contracts::document::encode(dispatch).unwrap(),
            )
            .await
            .unwrap();
        let mut held = false;
        for _ in 0..40 {
            f.harness.pic.tick().await.unwrap();
            let waiting: Result<bool, ConsumerFailure> = f
                .harness
                .pic
                .query_control_as(f.uploader, f.operator, "waiting", (), REPLY_BYTES)
                .await
                .unwrap();
            if waiting.unwrap() {
                held = true;
                break;
            }
        }
        assert!(held, "uploader reached the preparation acknowledgment hold");
        // Exposure/completion are explicitly test-host substitutes in this journey.
        if exposed {
            f.expose(permission.request).await.unwrap();
        }
        let charged = f.status().await;
        let withdrawn: Result<ManifestIntentView, ConsumerFailure> = f
            .harness
            .pic
            .update_control_as(
                f.uploader,
                f.operator,
                "cancel_manifest",
                1u128,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        let withdrawn = withdrawn.unwrap();
        assert!(withdrawn.cancelled && withdrawn.started);
        assert_eq!(withdrawn.result, None);
        assert_eq!(f.status().await, charged);
        cancel(&f, 1).await.unwrap();
        revocation::withdraw(&f, Fault::None, 4096).await.unwrap();
        f.enroll(Some(f.tenant().await.unwrap()), false)
            .await
            .unwrap();
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        let resumed: Result<(), ConsumerFailure> = f
            .harness
            .pic
            .update_control_as(f.uploader, f.operator, "resume", (), REPLY_BYTES)
            .await
            .unwrap();
        resumed.unwrap();
        let late: Result<ManifestIntentView, ConsumerFailure> =
            ic_blob_storage_contracts::document::decode(
                &f.harness.pic.await_call(call).await.unwrap(),
                REPLY_BYTES,
            )
            .unwrap();
        let late = late.unwrap();
        assert!(late.cancelled);
        assert_eq!(late.result, Some(Ok(intent.request.declaration.clone())));
        let retry: Result<ManifestIntentView, ConsumerFailure> = f
            .harness
            .pic
            .update_control_as(
                f.uploader,
                f.operator,
                "dispatch_manifest",
                ManifestDispatch {
                    hold: false,
                    ..dispatch
                },
                REPLY_BYTES,
            )
            .await
            .unwrap();
        assert_eq!(retry, Err(ConsumerFailure::State));
        let saved: Result<ManifestIntentView, ConsumerFailure> = f
            .harness
            .pic
            .update_control_as(
                f.uploader,
                f.operator,
                "save_manifest",
                &intent,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        assert_eq!(saved, Ok(late));
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
        finish_cancelled(&f, &input, permission, exposed).await;
    }
}
async fn finish_cancelled(f: &Fixture, input: &Run, _permission: Permission, exposed: bool) {
    assert!(!register(f, input).await.unwrap().published);
    if exposed {
        assert_eq!(
            f.status().await.usage,
            JourneyUsage {
                logical: 10,
                physical: 10,
                liability: 10
            }
        );
        let observed = recover(f, 1, false).await.unwrap();
        assert!(observed.cancelled && !observed.published);
        assert_eq!(observed.upload_state, Some(UploadState::Registered));
        assert_eq!(
            release(f, 1).await.unwrap().release_result,
            Some(Ok(ReferenceChange::Changed))
        );
        assert_eq!(
            f.status().await.usage,
            JourneyUsage {
                logical: 0,
                physical: 10,
                liability: 10
            }
        );
    } else {
        assert_eq!(
            f.status().await.usage,
            JourneyUsage {
                logical: 0,
                physical: 0,
                liability: 0
            }
        );
        assert_eq!(
            asset(f, 1).await.unwrap().upload_state,
            Some(UploadState::Cancelled)
        );
        assert_eq!(release(f, 1).await, Err(ConsumerFailure::State));
    }
    assert!(!register(f, input).await.unwrap().published);
}
