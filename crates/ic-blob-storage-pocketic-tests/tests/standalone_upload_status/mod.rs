//! Exact historical observation through the production host, including fenced restoration.
use super::*;
use ic_blob_storage::{
    dto::upload::{UploadState, UploadStatusFailure, UploadStatusResponse},
    ops::service::uploads::status::UPLOAD_STATUS_METHOD,
};

impl Fixture {
    pub(super) fn upload_status(
        &self,
        actor: Principal,
        upload: ReferenceUpload,
    ) -> Result<UploadStatusResponse, UploadStatusFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, UPLOAD_STATUS_METHOD, (upload,))
            .unwrap()
    }
}

#[test]
fn standalone_upload_status_preserves_exact_history_through_suspension_and_restore() {
    let f = Fixture::new();
    let enrollment = f.enroll(f.operator).unwrap();
    let mut permission = f.manifest().permission;
    permission.upload.upload = u128::MAX;
    permission.upload.object = u128::MAX - 1;
    permission.upload.incarnation = u128::MAX - 2;
    permission.upload.first_reference = u128::MAX - 3;
    assert_eq!(
        f.upload_status(f.tenant, permission.upload),
        Err(UploadStatusFailure::Unknown)
    );
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap();
    admitted.unwrap();
    let mut expected = UploadStatusResponse {
        upload: permission.upload,
        state: UploadState::Reserved,
        revoked: false,
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.upload_status(f.tenant, permission.upload), Ok(expected));
    let replicated: Result<UploadStatusResponse, UploadStatusFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.tenant,
            UPLOAD_STATUS_METHOD,
            (permission.upload,),
        )
        .unwrap();
    assert_eq!(replicated, Ok(expected));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope: f.scope(),
                expected: enrollment.enrollment,
                active: false,
            },),
        )
        .unwrap();
    suspended.unwrap();
    assert_eq!(f.upload_status(f.tenant, permission.upload), Ok(expected));
    // Withdrawal still cleans up an unexposed reservation while suspended.
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_revoke_upload", (permission,))
        .unwrap();
    revoked.unwrap();
    expected.state = UploadState::Cancelled;
    expected.revoked = true;
    assert_eq!(f.upload_status(f.tenant, permission.upload), Ok(expected));
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert!(f.configuration(f.operator).unwrap().fenced);
    assert_eq!(f.upload_status(f.tenant, permission.upload), Ok(expected));
    assert_eq!(
        f.upload_status(f.operator, permission.upload),
        Err(UploadStatusFailure::Denied)
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One retained upload checks each independent original binding and ingress bound"
)]
fn standalone_upload_status_rejects_foreign_malformed_and_changed_original_bindings() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let permission = f.manifest().permission;
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap();
    admitted.unwrap();
    let upload = permission.upload;
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.operator, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.upload_status(actor, upload),
            Err(UploadStatusFailure::Denied)
        );
    }
    for changed in [
        ReferenceUpload {
            service: f.operator,
            ..upload
        },
        ReferenceUpload {
            namespace: 1,
            ..upload
        },
    ] {
        assert_eq!(
            f.upload_status(f.tenant, changed),
            Err(UploadStatusFailure::Binding)
        );
    }
    for changed in [
        ReferenceUpload {
            namespace: 0,
            ..upload
        },
        ReferenceUpload {
            upload: 0,
            ..upload
        },
        ReferenceUpload {
            object: 0,
            ..upload
        },
        ReferenceUpload {
            incarnation: 0,
            ..upload
        },
        ReferenceUpload {
            first_reference: 0,
            ..upload
        },
        ReferenceUpload { bytes: 0, ..upload },
    ] {
        assert_eq!(
            f.upload_status(f.tenant, changed),
            Err(UploadStatusFailure::Invalid)
        );
    }
    for changed in [
        ReferenceUpload {
            object: upload.object + 1,
            ..upload
        },
        ReferenceUpload {
            incarnation: upload.incarnation + 1,
            ..upload
        },
        ReferenceUpload {
            first_reference: upload.first_reference + 1,
            ..upload
        },
        ReferenceUpload {
            bytes: upload.bytes + 1,
            ..upload
        },
        ReferenceUpload {
            root: [7; 32],
            ..upload
        },
    ] {
        assert_eq!(
            f.upload_status(f.tenant, changed),
            Err(UploadStatusFailure::Conflict)
        );
    }
    assert_eq!(
        f.upload_status(
            f.tenant,
            ReferenceUpload {
                upload: 2,
                ..upload
            }
        ),
        Err(UploadStatusFailure::Unknown)
    );
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, UPLOAD_STATUS_METHOD, bytes)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
