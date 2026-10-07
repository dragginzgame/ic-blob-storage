//! Actual late uploader acknowledgment cannot undo either application's cancellation.
use super::*;
#[test]
fn cancellation_during_preparation_acknowledgment_preserves_tombstones_and_exposed_cleanup() {
    for exposed in [false, true] {
        let f = uploader_fixture();
        let (input, permission, manifest) = prepared(&f);
        admit(&f, &input).unwrap();
        let intent = ManifestIntent {
            id: 1,
            request: f.preparation_input(&manifest),
        };
        let saved: Result<ManifestIntentView, ConsumerFailure> = f
            .harness
            .pic
            .update_candid_as(f.uploader, f.operator, "save_manifest", (&intent,))
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
                candid::encode_one(dispatch).unwrap(),
            )
            .unwrap();
        let mut held = false;
        for _ in 0..40 {
            f.harness.pic.tick();
            let waiting: Result<bool, ConsumerFailure> = f
                .harness
                .pic
                .query_candid_as(f.uploader, f.operator, "waiting", ())
                .unwrap();
            if waiting.unwrap() {
                held = true;
                break;
            }
        }
        assert!(held, "uploader reached the preparation acknowledgment hold");
        // Exposure/completion are explicitly test-host substitutes in this journey.
        if exposed {
            f.expose(permission.request).unwrap();
        }
        let charged = f.status();
        let withdrawn: Result<ManifestIntentView, ConsumerFailure> = f
            .harness
            .pic
            .update_candid_as(f.uploader, f.operator, "cancel_manifest", (1u128,))
            .unwrap();
        let withdrawn = withdrawn.unwrap();
        assert!(withdrawn.cancelled && withdrawn.started);
        assert_eq!(withdrawn.result, None);
        assert_eq!(f.status(), charged);
        cancel(&f, 1).unwrap();
        revocation::withdraw(&f, Fault::None, 4096).unwrap();
        f.enroll(Some(f.tenant().unwrap()), false).unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        let resumed: Result<(), ConsumerFailure> = f
            .harness
            .pic
            .update_candid_as(f.uploader, f.operator, "resume", ())
            .unwrap();
        resumed.unwrap();
        let late: Result<ManifestIntentView, ConsumerFailure> =
            candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
        let late = late.unwrap();
        assert!(late.cancelled);
        assert_eq!(late.result, Some(Ok(intent.request.declaration.clone())));
        let retry: Result<ManifestIntentView, ConsumerFailure> = f
            .harness
            .pic
            .update_candid_as(
                f.uploader,
                f.operator,
                "dispatch_manifest",
                (ManifestDispatch {
                    hold: false,
                    ..dispatch
                },),
            )
            .unwrap();
        assert_eq!(retry, Err(ConsumerFailure::State));
        let saved: Result<ManifestIntentView, ConsumerFailure> = f
            .harness
            .pic
            .update_candid_as(f.uploader, f.operator, "save_manifest", (&intent,))
            .unwrap();
        assert_eq!(saved, Ok(late));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
        finish_cancelled(&f, &input, permission, exposed);
    }
}
fn finish_cancelled(f: &Fixture, input: &Run, permission: Permission, exposed: bool) {
    assert!(!register(f, input).unwrap().published);
    if exposed {
        assert_eq!(
            f.status().usage,
            JourneyUsage {
                logical: 10,
                physical: 10,
                liability: 10
            }
        );
        f.fact(permission.request, ProviderFact::Uploaded).unwrap();
        let observed = recover(f, 1, false).unwrap();
        assert!(observed.cancelled && !observed.published);
        assert_eq!(observed.upload_state, Some(UploadState::Confirmed));
        assert_eq!(
            release(f, 1).unwrap().release_result,
            Some(Ok(ReferenceChange::Changed))
        );
        assert_eq!(
            f.status().usage,
            JourneyUsage {
                logical: 0,
                physical: 10,
                liability: 10
            }
        );
    } else {
        assert_eq!(
            f.status().usage,
            JourneyUsage {
                logical: 0,
                physical: 0,
                liability: 0
            }
        );
        assert_eq!(
            asset(f, 1).unwrap().upload_state,
            Some(UploadState::Cancelled)
        );
        assert_eq!(release(f, 1), Err(ConsumerFailure::State));
    }
    assert!(!register(f, input).unwrap().published);
}
