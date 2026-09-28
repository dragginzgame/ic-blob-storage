use super::*;
use crate::dto::{reference::ReferenceUpload, upload::UploadState};
use candid::{Principal, encode_one};
#[test]
fn admission_replies_bind_uploader_deadline_and_every_upload_field() {
    let input = UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: Principal::from_slice(&[1, 1]),
            tenant: Principal::from_slice(&[2, 1]),
            namespace: 3,
            upload: u128::MAX,
            object: u128::MAX - 1,
            incarnation: 4,
            first_reference: 5,
            root: [6; 32],
            bytes: 7,
        },
        uploader: Principal::from_slice(&[3, 1]),
        expires_at_ns: 100,
    };
    let response = UploadAdmissionResponse {
        permission: input,
        state: UploadState::Reserved,
        revoked: false,
    };
    let mutation_response = UploadAdmissionMutation {
        admission: response,
        replayed: false,
    };
    let observation = encode_one(Ok::<_, UploadAdmissionFailure>(response)).unwrap();
    let admission = encode_one(Ok::<_, UploadAdmissionFailure>(mutation_response)).unwrap();
    let max = 4096.try_into().unwrap();
    assert_eq!(inspection(input, &observation, max), Ok(response));
    assert_eq!(mutation(input, &admission, max), Ok(mutation_response));
    let u = input.upload;
    let mut changed = vec![
        UploadAdmissionRequest {
            uploader: u.tenant,
            ..input
        },
        UploadAdmissionRequest {
            expires_at_ns: 101,
            ..input
        },
    ];
    for upload in [
        ReferenceUpload {
            service: u.tenant,
            ..u
        },
        ReferenceUpload {
            tenant: u.service,
            ..u
        },
        ReferenceUpload { namespace: 1, ..u },
        ReferenceUpload { upload: 1, ..u },
        ReferenceUpload { object: 1, ..u },
        ReferenceUpload {
            incarnation: 1,
            ..u
        },
        ReferenceUpload {
            first_reference: 1,
            ..u
        },
        ReferenceUpload { root: [8; 32], ..u },
        ReferenceUpload { bytes: 8, ..u },
    ] {
        changed.push(UploadAdmissionRequest { upload, ..input });
    }
    for changed in changed {
        assert_eq!(
            inspection(changed, &observation, max),
            Err(UploadAdmissionReplyError::Binding)
        );
        assert_eq!(
            mutation(changed, &admission, max),
            Err(UploadAdmissionReplyError::Binding)
        );
    }
    assert_eq!(
        mutation(input, &admission, 1.try_into().unwrap()),
        Err(UploadAdmissionReplyError::Limit)
    );
    assert_eq!(
        inspection(input, &observation, 1.try_into().unwrap()),
        Err(UploadAdmissionReplyError::Limit)
    );
    assert_eq!(
        inspection(input, &admission, max),
        Err(UploadAdmissionReplyError::Invalid)
    );
    assert_eq!(
        mutation(input, &observation, max),
        Err(UploadAdmissionReplyError::Invalid)
    );
    let absent: Result<UploadAdmissionResponse, UploadAdmissionFailure> =
        Err(UploadAdmissionFailure::Unknown);
    assert_eq!(
        inspection(input, &encode_one(absent).unwrap(), max),
        Err(UploadAdmissionReplyError::Remote(
            UploadAdmissionFailure::Unknown
        ))
    );
}
