use super::*;
use ic_blob_storage::dto::upload::{UploadState, UploadStatusFailure, UploadStatusResponse};
mod admission;
mod manifests;
mod revocation;
fn fresh_input(f: &Fixture) -> (Run, Permission, PreparationInput) {
    let (permission, manifest) = f.permission(1, 1);
    let upload = ReferenceUpload {
        service: f.service,
        tenant: f.tenant,
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
                ic_blob_storage::dto::upload::admission::UploadAdmissionRequest {
                    upload,
                    uploader: permission.uploader,
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
fn prepared(f: &Fixture) -> (Run, Permission, PreparationInput) {
    let (input, permission, manifest) = fresh_input(f);
    let view: Result<AssetView, ConsumerFailure> = f
        .harness
        .pic
        .update_candid_as(f.tenant, f.operator, "prepare", (&input.registration,))
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
fn inspect(
    f: &Fixture,
    actor: Principal,
    upload: ReferenceUpload,
) -> Result<UploadStatusResponse, UploadStatusFailure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "blob_upload_status", (upload,))
        .unwrap()
}
#[test]
fn first_upload_reference_registers_and_releases_without_an_extra_retain_receipt() {
    let f = fixture();
    let (input, permission, manifest) = prepared(&f);
    assert_eq!(
        inspect(&f, f.tenant, upload(&input)),
        Err(UploadStatusFailure::Unknown)
    );
    admit(&f, &input).unwrap();
    assert_eq!(
        register(&f, &input).unwrap().upload_state,
        Some(UploadState::Reserved)
    );
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    let pending = register(&f, &input).unwrap();
    assert_eq!(pending.upload_state, Some(UploadState::ExposurePossible));
    assert!(!pending.published);
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let published = register(&f, &input).unwrap();
    assert!(published.published);
    assert!(!published.retain_started);
    assert_eq!(published.retain_result, None);
    assert_eq!(published.upload_state, Some(UploadState::Confirmed));
    assert_eq!(register(&f, &input), Ok(published));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
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
    assert_eq!(
        inspect(&f, f.tenant, upload(&input)).unwrap().state,
        UploadState::Confirmed
    );
    assert!(!register(&f, &input).unwrap().published);
    let retired = asset(&f, 1).unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(asset(&f, 1), Ok(retired));
    assert_eq!(recover(&f, 1, false), Err(ConsumerFailure::Fenced));
    assert_eq!(register(&f, &input), Err(ConsumerFailure::Fenced));
    assert_eq!(release(&f, 1), Err(ConsumerFailure::Fenced));
}
#[test]
fn late_upload_completion_after_cancellation_is_reconciled_for_cleanup_under_suspension() {
    let f = fixture();
    let (mut input, permission, manifest) = prepared(&f);
    admit(&f, &input).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    register(&f, &input).unwrap();
    cancel(&f, 1).unwrap();
    revocation::withdraw(&f, Fault::None, 4096).unwrap();
    assert_eq!(
        recover(&f, 1, false).unwrap().upload_state,
        Some(UploadState::ExposurePossible)
    );
    assert_eq!(release(&f, 1), Err(ConsumerFailure::State));
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    assert_eq!(
        recover(&f, 1, false).unwrap().upload_state,
        Some(UploadState::Confirmed)
    );
    assert_eq!(
        release(&f, 1).unwrap().release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    input.hold = false;
    assert!(!register(&f, &input).unwrap().published);
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 0,
            physical: 10,
            liability: 10
        }
    );
}
#[test]
fn upload_observation_trap_and_cancelled_reservation_cannot_fabricate_retention() {
    let f = fixture();
    let (mut input, permission, manifest) = prepared(&f);
    admit(&f, &input).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    input.fault = Fault::AfterUploadObservation;
    let before = f.harness.pic.get_stable_memory(f.service);
    let error = f
        .harness
        .pic
        .update_call(
            f.tenant,
            f.operator,
            "register",
            candid::encode_one(&input).unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(
        asset(&f, 1).unwrap().upload_state,
        Some(UploadState::Reserved)
    );
    input.fault = Fault::None;
    assert!(register(&f, &input).unwrap().published);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    let f = fixture();
    let (input, _, _) = prepared(&f);
    admit(&f, &input).unwrap();
    cancel(&f, 1).unwrap();
    revocation::withdraw(&f, Fault::None, 4096).unwrap();
    let cancelled = register(&f, &input).unwrap();
    assert_eq!(cancelled.upload_state, Some(UploadState::Cancelled));
    assert!(!cancelled.published);
    cancel(&f, 1).unwrap();
    assert_eq!(release(&f, 1), Err(ConsumerFailure::State));
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 0,
            physical: 0,
            liability: 0
        }
    );
}
#[test]
fn upload_status_rejects_foreign_and_changed_bindings_and_remains_historical_after_restore() {
    let f = fixture();
    let (input, permission, manifest) = prepared(&f);
    admit(&f, &input).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let u = upload(&input);
    for actor in [f.operator, f.controller, f.uploader, f.other] {
        assert_eq!(inspect(&f, actor, u), Err(UploadStatusFailure::Denied));
    }
    assert_eq!(
        inspect(
            &f,
            f.tenant,
            ReferenceUpload {
                first_reference: 2,
                ..u
            }
        ),
        Err(UploadStatusFailure::Conflict)
    );
    let original = inspect(&f, f.tenant, u).unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(inspect(&f, f.tenant, u), Ok(original));
    // Historical completion is readable, but operational descriptor delivery stays fenced.
    assert_eq!(register(&f, &input), Err(ConsumerFailure::Transport));
    let known = asset(&f, 1).unwrap();
    assert_eq!(known.upload_state, Some(UploadState::Confirmed));
    assert!(!known.published);
}
fn admit(f: &Fixture, input: &Run) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(f.tenant, f.operator, "admit", (input,))
        .unwrap()
}
