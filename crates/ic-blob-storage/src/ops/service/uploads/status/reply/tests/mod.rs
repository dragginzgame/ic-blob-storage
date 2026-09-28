use super::*;
use crate::dto::upload::UploadState;
use candid::{Principal, encode_one};
fn upload() -> ReferenceUpload {
    ReferenceUpload {
        service: Principal::from_slice(&[1, 1]),
        tenant: Principal::from_slice(&[2, 1]),
        namespace: 3,
        upload: u128::MAX,
        object: u128::MAX - 1,
        incarnation: 4,
        first_reference: 5,
        root: [8; 32],
        bytes: 9,
    }
}
#[test]
fn exact_upload_status_preserves_phases_refusals_and_full_original_binding() {
    let u = upload();
    let max = 4096.try_into().unwrap();
    for state in [
        UploadState::Reserved,
        UploadState::ExposurePossible,
        UploadState::Confirmed,
        UploadState::Cancelled,
    ] {
        let response = UploadStatusResponse {
            upload: u,
            state,
            revoked: true,
        };
        let bytes = encode_one(Ok::<_, UploadStatusFailure>(response)).unwrap();
        assert_eq!(decode(u, &bytes, max), Ok(response));
        for changed in [
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
            ReferenceUpload { root: [7; 32], ..u },
            ReferenceUpload { bytes: 10, ..u },
        ] {
            assert_eq!(
                decode(changed, &bytes, max),
                Err(UploadStatusReplyError::Binding)
            );
        }
        assert_eq!(
            decode(u, &bytes, 1.try_into().unwrap()),
            Err(UploadStatusReplyError::Limit)
        );
    }
    let absent: Result<UploadStatusResponse, UploadStatusFailure> =
        Err(UploadStatusFailure::Unknown);
    assert_eq!(
        decode(u, &encode_one(absent).unwrap(), max),
        Err(UploadStatusReplyError::Remote(UploadStatusFailure::Unknown))
    );
    for bad in [b"not candid".to_vec(), encode_one(7_u64).unwrap()] {
        assert_eq!(decode(u, &bad, max), Err(UploadStatusReplyError::Invalid));
    }
}
