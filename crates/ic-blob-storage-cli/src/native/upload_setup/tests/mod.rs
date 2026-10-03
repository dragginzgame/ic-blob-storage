use super::*;
use ic_blob_storage::dto::{
    reference::ReferenceUpload, upload::admission::UploadAdmissionMutation,
};

fn saved(kind: Kind) -> (tempfile::TempDir, Input, UploadAdmissionRequest) {
    let temp = tempfile::tempdir().unwrap();
    let permission = UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: Principal::self_authenticating([1]),
            tenant: Principal::self_authenticating([2]),
            namespace: u128::MAX,
            upload: u128::MAX,
            object: u128::MAX,
            incarnation: u128::MAX,
            first_reference: u128::MAX,
            root: [1; 32],
            bytes: 10,
        },
        uploader: Principal::self_authenticating([3]),
        expires_at_ns: u64::MAX,
    };
    let input = Input {
        kind,
        service: permission.upload.service,
        namespace: permission.upload.namespace,
        request: temp.path().join("permission.candid"),
        directory: Some(temp.path().join("run")),
    };
    std::fs::write(&input.request, candid::encode_one(permission).unwrap()).unwrap();
    (temp, input, permission)
}
#[test]
fn exact_roles_scope_and_invalid_ids_refuse_before_artifact_claim() {
    let (_temp, mut input, p) = saved(Kind::Admit);
    assert!(request::load(&input, p.upload.tenant).is_ok());
    assert!(matches!(
        request::load(&input, p.uploader),
        Err(Failure::Binding)
    ));
    input.kind = Kind::Manifest;
    assert!(request::load(&input, p.uploader).is_ok());
    input.namespace -= 1;
    assert!(matches!(
        request::load(&input, p.uploader),
        Err(Failure::Binding)
    ));
    input.namespace = p.upload.namespace;
    let mut invalid = p;
    invalid.upload.first_reference = 0;
    std::fs::write(&input.request, candid::encode_one(invalid).unwrap()).unwrap();
    assert!(matches!(
        request::load(&input, p.upload.tenant),
        Err(Failure::InvalidReply)
    ));
    assert!(!input.directory.unwrap().exists());
}
#[test]
fn original_permission_echo_and_typed_refusal_are_required_for_acknowledgment() {
    let (_temp, input, p) = saved(Kind::Admit);
    let saved = request::load(&input, p.upload.tenant).unwrap();
    let mut reply = UploadAdmissionMutation {
        admission: UploadAdmissionResponse {
            permission: p,
            state: UploadState::Reserved,
            revoked: false,
        },
        replayed: false,
    };
    let encode = |r: &UploadAdmissionMutation| candid::encode_one(Ok::<_, A>(r)).unwrap();
    assert_eq!(
        observation(&input, &saved, &encode(&reply)).unwrap().json()["replayed"],
        false
    );
    reply.admission.permission.expires_at_ns -= 1;
    assert!(matches!(
        observation(&input, &saved, &encode(&reply)),
        Err(Failure::Binding)
    ));
    assert!(matches!(
        observation(
            &input,
            &saved,
            &candid::encode_one(Err::<UploadAdmissionMutation, _>(A::Fenced)).unwrap()
        ),
        Err(Failure::UploadAdmissionRefused(A::Fenced))
    ));
}
