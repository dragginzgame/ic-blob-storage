use super::*;
use crate::dto::{
    reference::ReferenceUpload,
    upload::{admission::UploadAdmissionFailure as A, exposure::UploadExposureBlocker as B},
};
use candid::{Principal, encode_one};

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
    let root: String = candid::decode_one(&argument).unwrap();
    assert_eq!(root, format!("sha256:{}", "06".repeat(32)));
    let response = UploadCertificateAssessmentResponse {
        permission,
        assessed_at_ns: u64::MAX,
        blockers: vec![B::CurrentOwner],
    };
    let bytes = encode_one(Ok::<_, UploadExposureFailure>(&response)).unwrap();
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
fn assessments_preserve_empty_snapshots_refusals_and_decoder_limits() {
    let p = permission();
    let mut response = UploadCertificateAssessmentResponse {
        permission: p,
        assessed_at_ns: 1,
        blockers: vec![],
    };
    let maximum = 4096.try_into().unwrap();
    let bytes = encode_one(Ok::<_, UploadExposureFailure>(&response)).unwrap();
    assert_eq!(assessment(p, &bytes, maximum), Ok(response.clone()));
    assert_eq!(
        assessment(p, &bytes, 1.try_into().unwrap()),
        Err(UploadCertificateAssessmentReplyError::Limit)
    );
    assert_eq!(
        assessment(p, &encode_one(true).unwrap(), maximum),
        Err(UploadCertificateAssessmentReplyError::Invalid)
    );
    response.blockers = vec![B::CurrentOwner, B::CurrentOwner];
    assert_eq!(
        assessment(
            p,
            &encode_one(Ok::<_, UploadExposureFailure>(response)).unwrap(),
            maximum
        ),
        Err(UploadCertificateAssessmentReplyError::Invalid)
    );
    for error in [
        UploadExposureFailure::Unprepared,
        UploadExposureFailure::Revoked,
        UploadExposureFailure::Permission(A::Fenced),
    ] {
        let result: Result<UploadCertificateAssessmentResponse, _> = Err(error);
        assert_eq!(
            assessment(p, &encode_one(result).unwrap(), maximum),
            Err(UploadCertificateAssessmentReplyError::Remote(error))
        );
    }
}
