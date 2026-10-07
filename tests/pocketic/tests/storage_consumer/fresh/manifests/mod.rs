use super::*;
use blob_test_protocol::consumer::manifests::{
    ManifestDispatch, ManifestFault, ManifestIntent, ManifestIntentView,
};
mod cancellation;
fn uploader_fixture() -> Fixture {
    let mut f = fixture();
    let uploader = f.harness.pic.create_canister();
    f.harness.pic.install_canister(
        uploader,
        consumer_wasm(),
        candid::encode_args((f.operator, f.service)).unwrap(),
        None,
    );
    f.uploader = uploader;
    f
}
#[test]
fn separate_tenant_and_uploader_coordinate_preparation_recovery_registration_and_cleanup() {
    let f = uploader_fixture();
    assert_ne!(f.uploader, f.tenant);
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
    assert!(!saved.unwrap().started);
    // Tenant authority cannot be substituted for the admitted uploader.
    let wrong: Result<ManifestIntentView, ConsumerFailure> = f
        .harness
        .pic
        .update_candid_as(f.tenant, f.operator, "save_manifest", (&intent,))
        .unwrap();
    assert_eq!(wrong, Err(ConsumerFailure::Invalid));
    let sent: Result<ManifestIntentView, ConsumerFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.uploader,
            f.operator,
            "dispatch_manifest",
            (ManifestDispatch {
                hold: false,
                id: 1,
                max_reply_bytes: 1,
                fault: ManifestFault::None,
            },),
        )
        .unwrap();
    assert_eq!(sent, Err(ConsumerFailure::Transport));
    let before = f.harness.pic.get_stable_memory(f.service);
    let recovered: Result<ManifestIntentView, ConsumerFailure> = f
        .harness
        .pic
        .update_candid_as(f.uploader, f.operator, "recover_manifest", (1u128,))
        .unwrap();
    assert_eq!(
        recovered.unwrap().result,
        Some(Ok(intent.request.declaration))
    );
    assert!(!register(&f, &input).unwrap().published);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    // These remain labelled fixture controls, not a deployed-provider integration.
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let published = register(&f, &input).unwrap();
    assert!(published.published);
    assert!(!published.retain_started);
    cancel(&f, 1).unwrap();
    assert_eq!(
        release(&f, 1).unwrap().release_result,
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
}
