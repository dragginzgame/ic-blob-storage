//! Unsupported snapshot rollback restores heap authority as well as history.
//! This records an unsupported rollback path, not a requirement to preserve the gap.
use super::*;
use ic_blob_storage::dto::upload::{UploadState, UploadStatusFailure};

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
fn standalone_snapshot_rollback_bypasses_upgrade_fence_and_loses_later_obligations() {
    let f = Fixture::small(Harness::new(), Fake::principal(4));
    let pic = &f.harness.pic;
    f.enroll(f.operator).unwrap();
    let empty = snapshots::take(pic, f.service, f.controller);
    let manifest = f.small_manifest();
    let permission = manifest.permission;
    admit(&f, permission);
    f.prepare(f.uploader, &manifest).unwrap();
    let prepared = snapshots::take(pic, f.service, f.controller);
    let root = super::standalone_certificate::root(&manifest);
    let _: ic_blob_storage::dto::upload::certificate::CaffeineUploadCertificateResponse = pic
        .update_candid_as(
            f.service,
            f.uploader,
            ic_blob_storage::workflow::uploads::certificate::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
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
    assert!(!f.configuration(f.operator).unwrap().fenced);
    let restored = f.admission(permission);
    assert!(!restored.revoked);
    assert_eq!(restored.state, UploadState::Reserved);
    let assessment = super::standalone_certificate::inspect(&f, f.uploader, &root).unwrap();
    // This demonstrates why snapshot activation is unsupported: the heap cannot
    // know that exposure and revocation occurred after the restored snapshot.
    assert_eq!(assessment.blockers, []);
    let before = pic.get_stable_memory(f.service);
    unchanged(&pic.get_stable_memory(f.service), &before);

    // An even older snapshot forgets the entire operation and its reservation.
    snapshots::load(pic, f.service, f.controller, &empty);
    assert_eq!(
        f.upload_status(f.tenant, permission.upload),
        Err(UploadStatusFailure::Unknown)
    );
    let status = f.local_status(f.operator, f.operator_scope()).unwrap();
    assert!(!status.uploads.fenced);
    assert_eq!(status.uploads.operations, 0);
    assert_eq!(status.uploads.liability_bytes, 0);
    // Exact operation identity is reusable locally: absence is not freshness proof.
    admit(&f, permission);
    assert_eq!(f.admission(permission).state, UploadState::Reserved);
    snapshots::delete(pic, f.service, f.controller, prepared);
    snapshots::delete(pic, f.service, f.controller, empty);
}
