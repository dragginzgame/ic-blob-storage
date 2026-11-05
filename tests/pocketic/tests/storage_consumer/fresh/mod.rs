use super::*;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::UploadStatusFailure;
use ic_blob_storage_contracts::dto::upload::UploadStatusResponse;
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod admission;
mod manifests;
mod revocation;
fn fresh_input(f: &Fixture) -> (Run, Permission, PreparationInput) {
    let (permission, manifest) = f.permission(1, 1);
    let upload = ReferenceUpload {
        service: f.service.into(),
        tenant: f.tenant.into(),
        namespace: 1,
        upload: 1,
        object: 1,
        incarnation: 1,
        first_reference: 1,
        root: permission.request.root,
        bytes: 10,
    };
    let input = Run {
        registration: Registration {
            asset: 1,
            payload: vec![3; 8],
            source: RegistrationSource::Fresh(
                ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest {
                    upload,
                    uploader: permission.uploader.into(),
                    expires_at_ns: permission.expires_at_ns,
                },
            ),
            release_operation: 1,
        },
        fault: Fault::None,
        hold: false,
        max_reply_bytes: 4096,
    };
    (input, permission, manifest)
}
async fn prepared(f: &Fixture) -> (Run, Permission, PreparationInput) {
    let (input, permission, manifest) = fresh_input(f);
    let view: Result<AssetView, ConsumerFailure> = f
        .harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "prepare",
            &input.registration,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(view.unwrap().upload_state, None);
    (input, permission, manifest)
}
fn upload(input: &Run) -> ReferenceUpload {
    match input.registration.source {
        RegistrationSource::Fresh(p) => p.upload,
        RegistrationSource::Existing(_) => panic!("fresh-upload test"),
    }
}
async fn inspect(
    f: &Fixture,
    actor: Principal,
    upload: ReferenceUpload,
) -> Result<UploadStatusResponse, UploadStatusFailure> {
    f.harness
        .pic
        .query_blob_as(f.service, actor, "blob_upload_status", upload, REPLY_BYTES)
        .await
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn first_upload_reference_registers_and_releases_without_an_extra_retain_receipt() {
    let f = fixture().await;
    let (input, permission, manifest) = prepared(&f).await;
    assert_eq!(
        inspect(&f, f.tenant, upload(&input)).await,
        Err(UploadStatusFailure::Unknown)
    );
    admit(&f, &input).await.unwrap();
    assert_eq!(
        register(&f, &input).await.unwrap().upload_state,
        Some(UploadState::Reserved)
    );
    f.prepare(&manifest).await.unwrap();
    f.expose(permission.request).await.unwrap();
    let pending = register(&f, &input).await.unwrap();
    assert_eq!(pending.upload_state, Some(UploadState::Registered));
    assert!(pending.published); // Consumer publishes retention intent, not verified provider bytes.
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let published = register(&f, &input).await.unwrap();
    assert!(published.published);
    assert!(!published.retain_started);
    assert_eq!(published.retain_result, None);
    assert_eq!(published.upload_state, Some(UploadState::Registered));
    assert_eq!(register(&f, &input).await, Ok(published));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    cancel(&f, 1).await.unwrap();
    assert_eq!(
        release(&f, 1).await.unwrap().release_result,
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
    assert_eq!(
        inspect(&f, f.tenant, upload(&input)).await.unwrap().state,
        UploadState::Registered
    );
    assert!(!register(&f, &input).await.unwrap().published);
    let retired = asset(&f, 1).await.unwrap();
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
    assert_eq!(asset(&f, 1).await, Ok(retired));
    assert_eq!(recover(&f, 1, false).await, Err(ConsumerFailure::Fenced));
    assert_eq!(register(&f, &input).await, Err(ConsumerFailure::Fenced));
    assert_eq!(release(&f, 1).await, Err(ConsumerFailure::Fenced));
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn late_upload_completion_after_cancellation_is_reconciled_for_cleanup_under_suspension() {
    let f = fixture().await;
    let (mut input, permission, manifest) = prepared(&f).await;
    admit(&f, &input).await.unwrap();
    f.prepare(&manifest).await.unwrap();
    f.expose(permission.request).await.unwrap();
    register(&f, &input).await.unwrap();
    cancel(&f, 1).await.unwrap();
    revocation::withdraw(&f, Fault::None, 4096).await.unwrap();
    assert_eq!(
        recover(&f, 1, false).await.unwrap().upload_state,
        Some(UploadState::Registered)
    );
    f.enroll(Some(f.tenant().await.unwrap()), false)
        .await
        .unwrap();
    assert_eq!(
        recover(&f, 1, false).await.unwrap().upload_state,
        Some(UploadState::Registered)
    );
    assert_eq!(
        release(&f, 1).await.unwrap().release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    input.hold = false;
    assert!(!register(&f, &input).await.unwrap().published);
    assert_eq!(
        f.status().await.usage,
        JourneyUsage {
            logical: 0,
            physical: 10,
            liability: 10
        }
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upload_observation_trap_and_cancelled_reservation_cannot_fabricate_retention() {
    let f = fixture().await;
    let (mut input, permission, manifest) = prepared(&f).await;
    admit(&f, &input).await.unwrap();
    f.prepare(&manifest).await.unwrap();
    f.expose(permission.request).await.unwrap();
    input.fault = Fault::AfterUploadObservation;
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let error = f
        .harness
        .pic
        .update_call(
            f.tenant,
            f.operator,
            "register",
            ic_blob_storage_contracts::document::encode(&input).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        asset(&f, 1).await.unwrap().upload_state,
        Some(UploadState::Reserved)
    );
    input.fault = Fault::None;
    assert!(register(&f, &input).await.unwrap().published);
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    let f = fixture().await;
    let (input, _, _) = prepared(&f).await;
    admit(&f, &input).await.unwrap();
    cancel(&f, 1).await.unwrap();
    revocation::withdraw(&f, Fault::None, 4096).await.unwrap();
    let cancelled = register(&f, &input).await.unwrap();
    assert_eq!(cancelled.upload_state, Some(UploadState::Cancelled));
    assert!(!cancelled.published);
    cancel(&f, 1).await.unwrap();
    assert_eq!(release(&f, 1).await, Err(ConsumerFailure::State));
    assert_eq!(
        f.status().await.usage,
        JourneyUsage {
            logical: 0,
            physical: 0,
            liability: 0
        }
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upload_status_rejects_foreign_and_changed_bindings_and_remains_historical_after_restore() {
    let f = fixture().await;
    let (input, permission, manifest) = prepared(&f).await;
    admit(&f, &input).await.unwrap();
    f.prepare(&manifest).await.unwrap();
    f.expose(permission.request).await.unwrap();
    let u = upload(&input);
    for actor in [f.operator, f.controller, f.uploader, f.other] {
        assert_eq!(
            inspect(&f, actor, u).await,
            Err(UploadStatusFailure::Denied)
        );
    }
    assert_eq!(
        inspect(
            &f,
            f.tenant,
            ReferenceUpload {
                first_reference: 2,
                ..u
            }
        )
        .await,
        Err(UploadStatusFailure::Conflict)
    );
    let original = inspect(&f, f.tenant, u).await.unwrap();
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
    assert_eq!(inspect(&f, f.tenant, u).await, Ok(original));
    // Historical completion is readable, but operational descriptor delivery stays fenced.
    assert_eq!(register(&f, &input).await, Err(ConsumerFailure::Transport));
    let known = asset(&f, 1).await.unwrap();
    assert_eq!(known.upload_state, Some(UploadState::Registered));
    assert!(!known.published);
}
async fn admit(f: &Fixture, input: &Run) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(f.tenant, f.operator, "admit", input, REPLY_BYTES)
        .await
        .unwrap()
}
