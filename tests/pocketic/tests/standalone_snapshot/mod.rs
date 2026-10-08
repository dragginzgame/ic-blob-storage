//! Actual snapshot loads bypass lifecycle hooks but cannot regain operational authority.
use super::*;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::UploadStatusFailure;

fn admit(f: &Fixture, permission: UploadAdmissionRequest) {
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (permission,),
        )
        .unwrap()
        .unwrap();
}

#[test]
fn standalone_snapshot_rollback_stays_fenced_and_refuses_current_instance_recovery() {
    let f = Fixture::small(Harness::new(), Fake::principal(4));
    let pic = &f.harness.pic;
    f.enroll(f.operator).unwrap();
    let empty = snapshots::take(pic, f.service, f.controller);
    f.resume(f.operator).unwrap();
    let manifest = f.small_manifest();
    let permission = manifest.permission;
    admit(&f, permission);
    f.prepare(f.uploader, &manifest).unwrap();
    let prepared = snapshots::take(pic, f.service, f.controller);
    f.resume(f.operator).unwrap();
    let root = super::standalone_certificate::root(&manifest);
    let _: ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse =
        pic.update_candid_as(
            f.service,
            f.uploader,
            ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
            (root.clone(),),
        )
        .unwrap();
    assert_eq!(f.admission(permission).state, UploadState::ExposurePossible);
    pic.update_candid_as::<Result<UploadRevocationResponse, UploadAdmissionFailure>, _>(
        f.service,
        f.tenant,
        "blob_revoke_upload",
        (permission,),
    )
    .unwrap()
    .unwrap();
    assert!(f.admission(permission).revoked);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    assert!(f.configuration(f.operator).unwrap().fenced);

    // Loading includes the old heap owner; it does not run ops::restore.
    snapshots::load(pic, f.service, f.controller, &prepared);
    assert!(f.configuration(f.operator).unwrap().fenced);
    let restored = f.admission(permission);
    assert!(!restored.revoked);
    assert_eq!(restored.state, UploadState::Reserved);
    assert_eq!(
        super::standalone_certificate::inspect(&f, f.uploader, &root),
        Err(
            ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure::Permission(
                UploadAdmissionFailure::Fenced
            )
        )
    );
    let before = pic.get_stable_memory(f.service);
    assert_eq!(
        f.resume(f.operator),
        Err(ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure::SnapshotRestored)
    );
    unchanged(&pic.get_stable_memory(f.service), &before);

    // An even older snapshot forgets the entire operation and its reservation.
    snapshots::load(pic, f.service, f.controller, &empty);
    assert_eq!(
        f.upload_status(f.tenant, permission.upload),
        Err(UploadStatusFailure::Unknown)
    );
    let status = f.local_status(f.operator, f.operator_scope()).unwrap();
    assert!(status.uploads.fenced);
    assert_eq!(status.uploads.operations, 0);
    assert_eq!(status.uploads.liability_bytes, 0);
    // The old backup may omit later history, but neither absence nor its local
    // counters authorize admission or a fresh certificate for the reused identity.
    assert_eq!(
        pic.update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (permission,)
        )
        .unwrap(),
        Err(UploadAdmissionFailure::Fenced)
    );
    assert_eq!(
        f.resume(f.operator),
        Err(ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure::SnapshotRestored)
    );
    snapshots::delete(pic, f.service, f.controller, prepared);
    snapshots::delete(pic, f.service, f.controller, empty);
}
