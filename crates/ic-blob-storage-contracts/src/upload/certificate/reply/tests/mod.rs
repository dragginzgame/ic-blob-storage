use crate::dto::reference::ReferenceUpload;
use crate::dto::upload::admission::UploadAdmissionFailure as A;
use crate::upload::certificate::reply::*;
fn encode<T: serde::Serialize>(value: T) -> Result<Vec<u8>, crate::wire::Error> {
    crate::wire::encode_response("blob_upload_certificate_assessment", value)
}
use ic_platform::Principal;

fn permission() -> UploadAdmissionRequest {
    UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: Principal::from_slice(&[1, 1]),
            tenant: Principal::from_slice(&[2, 1]),
            namespace: u128::MAX,
            upload: u128::MAX - 1,
            object: u128::MAX - 2,
            incarnation: u128::MAX - 3,
            first_reference: u128::MAX - 4,
            root: [6; 32],
            bytes: u64::MAX,
        },
        uploader: Principal::from_slice(&[3, 1]),
        expires_at_ns: u64::MAX,
    }
}

#[test]
fn assessment_validates_the_full_permission_before_root_lookup_and_after_reply() {
    let permission = permission();
    let argument = assessment_request(permission).unwrap();
    let root: String = crate::wire::decode_request(
        "blob_upload_certificate_assessment",
        &argument,
        crate::wire::MAX_MESSAGE_BYTES,
    )
    .unwrap();
    assert_eq!(root, format!("sha256:{}", "06".repeat(32)));
    let response = UploadCertificateAssessmentResponse {
        permission,
        assessed_at_ns: u64::MAX,
    };
    let bytes = encode(Ok::<_, UploadExposureFailure>(&response)).unwrap();
    let maximum = 4096.try_into().unwrap();
    assert_eq!(assessment(permission, &bytes, maximum), Ok(response));
    let u = permission.upload;
    let changed_uploads = [
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
        ReferenceUpload { bytes: 1, ..u },
    ];
    let changed = changed_uploads
        .into_iter()
        .map(|upload| UploadAdmissionRequest {
            upload,
            ..permission
        })
        .chain([
            UploadAdmissionRequest {
                uploader: u.tenant,
                ..permission
            },
            UploadAdmissionRequest {
                expires_at_ns: 1,
                ..permission
            },
        ]);
    for changed in changed {
        assert_eq!(
            assessment(changed, &bytes, maximum),
            Err(UploadCertificateAssessmentReplyError::Binding)
        );
    }
    let mut invalid = permission;
    invalid.upload.first_reference = 0;
    assert_eq!(
        assessment_request(invalid),
        Err(UploadCertificateAssessmentReplyError::Invalid)
    );
}

#[test]
fn assessments_preserve_snapshots_refusals_and_decoder_limits() {
    let p = permission();
    let response = UploadCertificateAssessmentResponse {
        permission: p,
        assessed_at_ns: 1,
    };
    let maximum = 4096.try_into().unwrap();
    let bytes = encode(Ok::<_, UploadExposureFailure>(&response)).unwrap();
    assert_eq!(assessment(p, &bytes, maximum), Ok(response.clone()));
    assert_eq!(
        assessment(p, &bytes, 1.try_into().unwrap()),
        Err(UploadCertificateAssessmentReplyError::Limit)
    );
    assert_eq!(
        assessment(p, &encode(true).unwrap(), maximum),
        Err(UploadCertificateAssessmentReplyError::Invalid)
    );
    for error in [
        UploadExposureFailure::Unprepared,
        UploadExposureFailure::Revoked,
        UploadExposureFailure::Permission(A::Fenced),
    ] {
        let result: Result<UploadCertificateAssessmentResponse, _> = Err(error);
        assert_eq!(
            assessment(p, &encode(result).unwrap(), maximum),
            Err(UploadCertificateAssessmentReplyError::Remote(error))
        );
    }
}

#[test]
fn assessment_hard_cut_rejects_retired_identity_and_trailing_blocker_bytes() {
    let p = permission();
    let response = UploadCertificateAssessmentResponse {
        permission: p,
        assessed_at_ns: 1,
    };
    let bytes = encode(Ok::<_, UploadExposureFailure>(response)).unwrap();
    let mut retired = bytes.clone();
    retired[2..10].copy_from_slice(&0x424c_4f42_0042_0009u64.to_le_bytes());
    let mut trailing = bytes;
    trailing.push(0); // The retired success layout ended with an empty blocker vector.
    for invalid in [retired, trailing] {
        assert_eq!(
            assessment(p, &invalid, 4096.try_into().unwrap()),
            Err(UploadCertificateAssessmentReplyError::Invalid)
        );
    }
    let mut request = assessment_request(p).unwrap();
    request[2..10].copy_from_slice(&0x424c_4f42_0042_0009u64.to_le_bytes());
    assert!(
        crate::wire::decode_request::<String>("blob_upload_certificate_assessment", &request, 4096)
            .is_err()
    );
}
