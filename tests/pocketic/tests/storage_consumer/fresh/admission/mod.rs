use super::*;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
fn permission(input: &Run) -> UploadAdmissionRequest {
    match input.registration.source {
        RegistrationSource::Fresh(p) => p,
        RegistrationSource::Existing(_) => panic!("fresh admission test"),
    }
}
fn inspect_permission(
    f: &Fixture,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, A> {
    f.harness
        .pic
        .query_candid_as(f.service, f.tenant, "blob_upload_admission", (input,))
        .unwrap()
}
#[test]
fn uncertain_admission_recovers_exact_permission_without_resending_after_cancellation() {
    for trap in [false, true] {
        let f = fixture();
        let (mut input, original, _) = prepared(&f);
        if trap {
            input.fault = Fault::AfterAdmission;
            let error = f
                .harness
                .pic
                .update_call(
                    f.tenant,
                    f.operator,
                    "admit",
                    candid::encode_one(&input).unwrap(),
                )
                .unwrap_err();
            assert_eq!(error.reject_code, RejectCode::CanisterError);
        } else {
            input.max_reply_bytes = 1;
            assert_eq!(admit(&f, &input), Err(ConsumerFailure::Transport));
        }
        let pending = asset(&f, 1).unwrap();
        assert!(pending.admission_started);
        assert_eq!(pending.admission_result, None);
        assert_eq!(pending.upload_state, None);
        assert_eq!(f.status().operations, 1);
        input.fault = Fault::None;
        input.max_reply_bytes = 4096;
        assert_eq!(admit(&f, &input), Err(ConsumerFailure::Pending));
        for changed in [
            UploadAdmissionRequest {
                uploader: f.other,
                ..permission(&input)
            },
            UploadAdmissionRequest {
                expires_at_ns: permission(&input).expires_at_ns - 1,
                ..permission(&input)
            },
        ] {
            assert_eq!(inspect_permission(&f, changed), Err(A::Conflict));
            let mut altered = input.clone();
            altered.registration.source = RegistrationSource::Fresh(changed);
            assert_eq!(admit(&f, &altered), Err(ConsumerFailure::Conflict));
        }
        cancel(&f, 1).unwrap();
        f.revoke(original).unwrap();
        f.enroll(Some(f.tenant().unwrap()), false).unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        let recovered = recover(&f, 1, false).unwrap();
        assert_eq!(recovered.admission_result, Some(Ok(())));
        assert_eq!(recovered.upload_state, Some(UploadState::Cancelled));
        assert!(recovered.cancelled && !recovered.published);
        assert_eq!(admit(&f, &input), Err(ConsumerFailure::State));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    }
}
#[test]
fn admission_intent_rolls_back_before_dispatch_and_refusal_is_retained_without_resend() {
    let f = fixture();
    let (mut input, _, _) = fresh_input(&f);
    input.fault = Fault::AfterIntent;
    let error = f
        .harness
        .pic
        .update_call(
            f.tenant,
            f.operator,
            "admit",
            candid::encode_one(&input).unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(asset(&f, 1), Err(ConsumerFailure::Unknown));
    assert_eq!(inspect_permission(&f, permission(&input)), Err(A::Unknown));
    input.fault = Fault::None;
    assert_eq!(admit(&f, &input).unwrap().admission_result, Some(Ok(())));
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(admit(&f, &input).unwrap().admission_result, Some(Ok(())));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);

    let f = fixture();
    let (mut expired, _, _) = fresh_input(&f);
    expired.registration.source = RegistrationSource::Fresh(UploadAdmissionRequest {
        expires_at_ns: 0,
        ..permission(&expired)
    });
    let rejected = admit(&f, &expired).unwrap();
    assert_eq!(rejected.admission_result, Some(Err(A::Expired)));
    assert_eq!(register(&f, &expired), Err(ConsumerFailure::State));
    assert_eq!(admit(&f, &expired), Ok(rejected));
    assert_eq!(f.status().operations, 0);
    assert_eq!(
        inspect_permission(&f, permission(&expired)),
        Err(A::Unknown)
    );
    assert_eq!(recover(&f, 1, false), Err(ConsumerFailure::Transport));
    assert_eq!(f.status().operations, 0);
}
#[test]
fn consumer_upgrade_retains_uncertain_admission_and_blocks_reconciliation_and_redispatch() {
    let f = fixture();
    let (mut input, _, _) = prepared(&f);
    input.max_reply_bytes = 1;
    assert_eq!(admit(&f, &input), Err(ConsumerFailure::Transport));
    let pending = cancel(&f, 1).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(asset(&f, 1), Ok(pending));
    assert_eq!(admit(&f, &input), Err(ConsumerFailure::Fenced));
    assert_eq!(recover(&f, 1, false), Err(ConsumerFailure::Fenced));
    assert_eq!(register(&f, &input), Err(ConsumerFailure::Fenced));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(
        inspect_permission(&f, permission(&input)).unwrap().state,
        UploadState::Reserved
    );
}
