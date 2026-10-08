//! Standalone exact reference inspection: shared validation, bounded ingress and no synthetic completion.
use super::*;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;
impl Fixture {
    fn reference_status(
        &self,
        actor: Principal,
        input: ReferenceStatusRequest,
    ) -> Result<ReferenceStatusResponse, ReferenceFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "blob_reference_status", (input,))
            .unwrap()
    }
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One installation checks independent bindings, ingress and fenced inspection"
)]
fn standalone_reference_status_checks_exact_bindings_before_reporting_unconfirmed_content() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let mut permission = f.manifest().permission;
    permission.upload.upload = u128::MAX;
    permission.upload.object = u128::MAX - 1;
    permission.upload.incarnation = u128::MAX - 2;
    permission.upload.first_reference = u128::MAX - 3;
    let input = ReferenceStatusRequest {
        upload: permission.upload,
        reference: u128::MAX - 3,
    };
    assert_eq!(
        f.reference_status(f.tenant, input),
        Err(ReferenceFailure::Unknown)
    );
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap();
    admitted.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.reference_status(f.tenant, input),
        Err(ReferenceFailure::Unconfirmed)
    );
    let replicated: Result<ReferenceStatusResponse, ReferenceFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_reference_status", (input,))
        .unwrap();
    assert_eq!(replicated, Err(ReferenceFailure::Unconfirmed));
    for actor in [f.controller, f.operator, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.reference_status(actor, input),
            Err(ReferenceFailure::Denied)
        );
    }
    for (upload, failure) in [
        (
            ReferenceUpload {
                service: f.operator,
                ..input.upload
            },
            ReferenceFailure::Binding,
        ),
        (
            ReferenceUpload {
                namespace: 1,
                ..input.upload
            },
            ReferenceFailure::Binding,
        ),
        (
            ReferenceUpload {
                bytes: 0,
                ..input.upload
            },
            ReferenceFailure::Invalid,
        ),
        (
            ReferenceUpload {
                object: 1,
                ..input.upload
            },
            ReferenceFailure::Conflict,
        ),
        (
            ReferenceUpload {
                incarnation: 1,
                ..input.upload
            },
            ReferenceFailure::Conflict,
        ),
        (
            ReferenceUpload {
                first_reference: 1,
                ..input.upload
            },
            ReferenceFailure::Conflict,
        ),
        (
            ReferenceUpload {
                root: [9; 32],
                ..input.upload
            },
            ReferenceFailure::Conflict,
        ),
    ] {
        assert_eq!(
            f.reference_status(f.tenant, ReferenceStatusRequest { upload, ..input }),
            Err(failure)
        );
    }
    assert_eq!(
        f.reference_status(
            f.tenant,
            ReferenceStatusRequest {
                reference: 0,
                ..input
            }
        ),
        Err(ReferenceFailure::Invalid)
    );
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, "blob_reference_status", bytes)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.reference_status(f.tenant, input),
        Err(ReferenceFailure::Unconfirmed)
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
