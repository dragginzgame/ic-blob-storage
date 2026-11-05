use super::*;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
fn permission(input: &Run) -> UploadAdmissionRequest {
    match input.registration.source {
        RegistrationSource::Fresh(p) => p,
        RegistrationSource::Existing(_) => panic!("fresh admission test"),
    }
}
async fn inspect_permission(
    f: &Fixture,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, A> {
    f.harness
        .pic
        .query_blob_as(
            f.service,
            f.tenant,
            "blob_upload_admission",
            input,
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn uncertain_admission_recovers_exact_permission_without_resending_after_cancellation() {
    for trap in [false, true] {
        let f = fixture().await;
        let (mut input, original, _) = prepared(&f).await;
        if trap {
            input.fault = Fault::AfterAdmission;
            let error = f
                .harness
                .pic
                .update_call(
                    f.tenant,
                    f.operator,
                    "admit",
                    ic_blob_storage_contracts::document::encode(&input).unwrap(),
                )
                .await
                .unwrap_err();
            assert_eq!(
                crate::canister_rejection(&error).reject_code,
                RejectCode::CanisterError
            );
        } else {
            input.max_reply_bytes = 1;
            assert_eq!(admit(&f, &input).await, Err(ConsumerFailure::Transport));
        }
        let pending = asset(&f, 1).await.unwrap();
        assert!(pending.admission_started);
        assert_eq!(pending.admission_result, None);
        assert_eq!(pending.upload_state, None);
        assert_eq!(f.status().await.operations, 1);
        input.fault = Fault::None;
        input.max_reply_bytes = 4096;
        assert_eq!(admit(&f, &input).await, Err(ConsumerFailure::Pending));
        for changed in [
            UploadAdmissionRequest {
                uploader: f.other.into(),
                ..permission(&input)
            },
            UploadAdmissionRequest {
                expires_at_ns: permission(&input).expires_at_ns - 1,
                ..permission(&input)
            },
        ] {
            assert_eq!(inspect_permission(&f, changed).await, Err(A::Conflict));
            let mut altered = input.clone();
            altered.registration.source = RegistrationSource::Fresh(changed);
            assert_eq!(admit(&f, &altered).await, Err(ConsumerFailure::Conflict));
        }
        cancel(&f, 1).await.unwrap();
        f.revoke(original).await.unwrap();
        f.enroll(Some(f.tenant().await.unwrap()), false)
            .await
            .unwrap();
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        let recovered = recover(&f, 1, false).await.unwrap();
        assert_eq!(recovered.admission_result, Some(Ok(())));
        assert_eq!(recovered.upload_state, Some(UploadState::Cancelled));
        assert!(recovered.cancelled && !recovered.published);
        assert_eq!(admit(&f, &input).await, Err(ConsumerFailure::State));
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn admission_intent_rolls_back_before_dispatch_and_refusal_is_retained_without_resend() {
    let f = fixture().await;
    let (mut input, _, _) = fresh_input(&f);
    input.fault = Fault::AfterIntent;
    let error = f
        .harness
        .pic
        .update_call(
            f.tenant,
            f.operator,
            "admit",
            ic_blob_storage_contracts::document::encode(&input).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(asset(&f, 1).await, Err(ConsumerFailure::Unknown));
    assert_eq!(
        inspect_permission(&f, permission(&input)).await,
        Err(A::Unknown)
    );
    input.fault = Fault::None;
    assert_eq!(
        admit(&f, &input).await.unwrap().admission_result,
        Some(Ok(()))
    );
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        admit(&f, &input).await.unwrap().admission_result,
        Some(Ok(()))
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );

    let f = fixture().await;
    let (mut expired, _, _) = fresh_input(&f);
    expired.registration.source = RegistrationSource::Fresh(UploadAdmissionRequest {
        expires_at_ns: 0,
        ..permission(&expired)
    });
    let rejected = admit(&f, &expired).await.unwrap();
    assert_eq!(rejected.admission_result, Some(Err(A::Expired)));
    assert_eq!(register(&f, &expired).await, Err(ConsumerFailure::State));
    assert_eq!(admit(&f, &expired).await, Ok(rejected));
    assert_eq!(f.status().await.operations, 0);
    assert_eq!(
        inspect_permission(&f, permission(&expired)).await,
        Err(A::Unknown)
    );
    assert_eq!(recover(&f, 1, false).await, Err(ConsumerFailure::Transport));
    assert_eq!(f.status().await.operations, 0);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn consumer_upgrade_retains_uncertain_admission_and_blocks_reconciliation_and_redispatch() {
    let f = fixture().await;
    let (mut input, _, _) = prepared(&f).await;
    input.max_reply_bytes = 1;
    assert_eq!(admit(&f, &input).await, Err(ConsumerFailure::Transport));
    let pending = cancel(&f, 1).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(asset(&f, 1).await, Ok(pending));
    assert_eq!(admit(&f, &input).await, Err(ConsumerFailure::Fenced));
    assert_eq!(recover(&f, 1, false).await, Err(ConsumerFailure::Fenced));
    assert_eq!(register(&f, &input).await, Err(ConsumerFailure::Fenced));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    assert_eq!(
        inspect_permission(&f, permission(&input))
            .await
            .unwrap()
            .state,
        UploadState::Reserved
    );
}
