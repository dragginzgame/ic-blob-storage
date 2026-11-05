use super::*;
use blob_test_protocol::consumer::manifests::{
    ManifestDispatch, ManifestFault, ManifestIntent, ManifestIntentView,
};
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
mod cancellation;
async fn uploader_fixture() -> Fixture {
    let mut f = fixture().await;
    let uploader = f.harness.pic.create_canister().await.unwrap();
    f.harness
        .pic
        .install_canister(
            uploader,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_init", (f.operator, f.service))
                .unwrap(),
            None,
        )
        .await
        .unwrap();
    f.uploader = uploader;
    f
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn separate_tenant_and_uploader_coordinate_preparation_recovery_registration_and_cleanup() {
    let f = uploader_fixture().await;
    assert_ne!(f.uploader, f.tenant);
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
    assert!(!saved.unwrap().started);
    // Tenant authority cannot be substituted for the admitted uploader.
    let wrong: Result<ManifestIntentView, ConsumerFailure> = f
        .harness
        .pic
        .update_control_as(f.tenant, f.operator, "save_manifest", &intent, REPLY_BYTES)
        .await
        .unwrap();
    assert_eq!(wrong, Err(ConsumerFailure::Invalid));
    let sent: Result<ManifestIntentView, ConsumerFailure> = f
        .harness
        .pic
        .update_control_as(
            f.uploader,
            f.operator,
            "dispatch_manifest",
            ManifestDispatch {
                hold: false,
                id: 1,
                max_reply_bytes: 1,
                fault: ManifestFault::None,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(sent, Err(ConsumerFailure::Transport));
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let recovered: Result<ManifestIntentView, ConsumerFailure> = f
        .harness
        .pic
        .update_control_as(
            f.uploader,
            f.operator,
            "recover_manifest",
            1u128,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(
        recovered.unwrap().result,
        Some(Ok(intent.request.declaration))
    );
    assert!(!register(&f, &input).await.unwrap().published);
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    // These remain labelled fixture controls, not a deployed-provider integration.
    f.expose(permission.request).await.unwrap();
    let published = register(&f, &input).await.unwrap();
    assert!(published.published);
    assert!(!published.retain_started);
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
}
