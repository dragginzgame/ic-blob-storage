//! Actual snapshot loads bypass lifecycle hooks but cannot regain operational authority.
use super::*;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::UploadStatusFailure;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::provider_calls;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

async fn admit(f: &Fixture, permission: UploadAdmissionRequest) {
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            permission,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_snapshot_rollback_stays_fenced_and_refuses_current_instance_recovery() {
    let f = Fixture::small(Harness::new().await, fake_principal(4)).await;
    let pic = &f.harness.pic;
    f.enroll(f.operator).await.unwrap();
    let empty = snapshots::take(pic, f.service, f.controller).await;
    f.resume(f.operator).await.unwrap();
    let manifest = f.small_manifest();
    let permission = manifest.permission;
    admit(&f, permission).await;
    f.prepare(f.uploader, &manifest).await.unwrap();
    let prepared = snapshots::take(pic, f.service, f.controller).await;
    f.resume(f.operator).await.unwrap();
    let root = super::standalone_certificate::root(&manifest);
    let _: ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse =
        provider_calls::update_provider(
            pic,
            f.service,
            f.uploader,
            ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
            (root.clone(),),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(f.admission(permission).await.state, UploadState::Registered);
    pic.update_blob_as::<Result<UploadRevocationResponse, UploadAdmissionFailure>, _>(
        f.service,
        f.tenant,
        "blob_revoke_upload",
        permission,
        REPLY_BYTES,
    )
    .await
    .unwrap()
    .unwrap();
    assert!(f.admission(permission).await.revoked);
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    assert!(f.configuration(f.operator).await.unwrap().fenced);

    // Loading includes the old heap owner; it does not run ops::restore.
    snapshots::load(pic, f.service, f.controller, &prepared).await;
    assert!(f.configuration(f.operator).await.unwrap().fenced);
    let restored = f.admission(permission).await;
    assert!(!restored.revoked);
    assert_eq!(restored.state, UploadState::Reserved);
    assert_eq!(
        super::standalone_certificate::inspect(&f, f.uploader, &root).await,
        Err(
            ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure::Permission(
                UploadAdmissionFailure::Fenced
            )
        )
    );
    let before = pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        f.resume(f.operator).await,
        Err(ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure::SnapshotRestored)
    );
    unchanged(&pic.get_stable_memory(f.service).await.unwrap(), &before);

    // An even older snapshot forgets the entire operation and its reservation.
    snapshots::load(pic, f.service, f.controller, &empty).await;
    assert_eq!(
        f.upload_status(f.tenant, permission.upload).await,
        Err(UploadStatusFailure::Unknown)
    );
    let status = f
        .local_status(f.operator, f.operator_scope())
        .await
        .unwrap();
    assert!(status.uploads.fenced);
    assert_eq!(status.uploads.operations, 0);
    assert_eq!(status.uploads.liability_bytes, 0);
    // The old backup may omit later history, but neither absence nor its local
    // counters authorize admission or a fresh certificate for the reused identity.
    assert_eq!(
        pic.update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            permission,
            REPLY_BYTES
        )
        .await
        .unwrap(),
        Err(UploadAdmissionFailure::Fenced)
    );
    assert_eq!(
        f.resume(f.operator).await,
        Err(ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure::SnapshotRestored)
    );
    snapshots::delete(pic, f.service, f.controller, prepared).await;
    snapshots::delete(pic, f.service, f.controller, empty).await;
}
