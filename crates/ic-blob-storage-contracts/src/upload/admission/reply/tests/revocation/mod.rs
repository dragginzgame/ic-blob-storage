use crate::upload::admission::reply::tests::*;
#[test]
fn revocation_reply_requires_withdrawal_and_the_complete_original_permission() {
    let u = ReferenceUpload {
        service: Principal::from_slice(&[1, 1]),
        tenant: Principal::from_slice(&[2, 1]),
        namespace: 1,
        upload: u128::MAX,
        object: u128::MAX - 1,
        incarnation: 3,
        first_reference: 4,
        root: [7; 32],
        bytes: 10,
    };
    let permission = UploadAdmissionRequest {
        upload: u,
        uploader: Principal::from_slice(&[3, 1]),
        expires_at_ns: 100,
    };
    let max = 4096.try_into().unwrap();
    let response = UploadRevocationResponse {
        admission: UploadAdmissionResponse {
            permission,
            state: UploadState::ExposurePossible,
            revoked: true,
        },
        changed: true,
    };
    let bytes = encode_one(Ok::<_, UploadAdmissionFailure>(response)).unwrap();
    assert_eq!(revocation(permission, &bytes, max), Ok(response));
    let unrevoked = UploadRevocationResponse {
        admission: UploadAdmissionResponse {
            revoked: false,
            ..response.admission
        },
        ..response
    };
    assert_eq!(
        revocation(
            permission,
            &encode_one(Ok::<_, UploadAdmissionFailure>(unrevoked)).unwrap(),
            max
        ),
        Err(UploadAdmissionReplyError::Invalid)
    );
    let changed = [
        UploadAdmissionRequest {
            uploader: u.tenant,
            ..permission
        },
        UploadAdmissionRequest {
            expires_at_ns: 99,
            ..permission
        },
        UploadAdmissionRequest {
            upload: ReferenceUpload {
                first_reference: 2,
                ..u
            },
            ..permission
        },
    ];
    for changed in changed {
        assert_eq!(
            revocation(changed, &bytes, max),
            Err(UploadAdmissionReplyError::Binding)
        );
    }
    assert_eq!(
        revocation(permission, &bytes, 1.try_into().unwrap()),
        Err(UploadAdmissionReplyError::Limit)
    );
    assert_eq!(
        revocation(permission, b"invalid", max),
        Err(UploadAdmissionReplyError::Invalid)
    );
    let refused: Result<UploadRevocationResponse, UploadAdmissionFailure> =
        Err(UploadAdmissionFailure::Fenced);
    assert_eq!(
        revocation(permission, &encode_one(refused).unwrap(), max),
        Err(UploadAdmissionReplyError::Remote(
            UploadAdmissionFailure::Fenced
        ))
    );
}
