//! Real managed certificate assessment with unqualified host facts, never simulated permits.
use super::{Fixture, endpoints::manifest, installation::enroll};
use candid::Principal;
use ic_blob_storage::{
    dto::upload::{
        UploadState,
        admission::{
            UploadAdmissionFailure as A, UploadAdmissionMutation, UploadAdmissionResponse,
            UploadRevocationResponse,
        },
        certificate::{CaffeineUploadCertificateResponse, UploadCertificateAssessmentResponse},
        exposure::{UploadExposureBlocker as B, UploadExposureFailure as E},
        manifest::{UploadManifestFailure, UploadManifestMutation},
    },
    model::identity::ProviderRootHash,
    workflow::uploads::certificate::{
        CAFFEINE_UPLOAD_CERTIFICATE_METHOD as ISSUE,
        UPLOAD_CERTIFICATE_ASSESSMENT_METHOD as INSPECT,
    },
};
use ic_testkit::{pic::CandidCallExt, pocket_ic::RejectCode};
use std::time::Duration;

fn inspect(
    f: &Fixture,
    actor: Principal,
    root: &str,
) -> Result<UploadCertificateAssessmentResponse, E> {
    f.pic()
        .query_candid_as(f.app(), actor, INSPECT, (root,))
        .unwrap()
}
#[test]
fn managed_certificate_assessment_keeps_real_blockers_through_revocation_and_restore() {
    let f = Fixture::new();
    let (scope, _) = enroll(&f);
    let uploader = Principal::from_slice(&[6, 1]);
    let input = manifest(&f, scope.tenant, uploader);
    let root = ProviderRootHash::try_from(input.permission.upload.root.as_slice())
        .unwrap()
        .to_string();
    f.pic()
        .update_candid_as::<Result<UploadAdmissionMutation, A>, _>(
            f.app(),
            scope.tenant,
            "blob_admit_upload",
            (input.permission,),
        )
        .unwrap()
        .unwrap();
    let before = f.pic().get_stable_memory(f.app());
    assert_eq!(inspect(&f, uploader, &root), Err(E::Unprepared));
    refuse(&f, uploader, &root);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    f.pic()
        .update_candid_as::<Result<UploadManifestMutation, UploadManifestFailure>, _>(
            f.app(),
            uploader,
            "blob_prepare_upload",
            (&input,),
        )
        .unwrap()
        .unwrap();
    let before = f.pic().get_stable_memory(f.app());
    let assessment = inspect(&f, uploader, &root).unwrap();
    assert_eq!(assessment.permission, input.permission);
    assert!(assessment.assessed_at_ns <= f.pic().get_time().as_nanos_since_unix_epoch());
    assert_eq!(
        assessment.blockers,
        vec![
            B::PrechargeLimits,
            B::ProviderNamespace,
            B::ReplayCharging,
            B::Recovery
        ]
    );
    authority(&f, uploader, scope.tenant, &root);
    refuse(&f, uploader, &root);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    let admission: Result<UploadAdmissionResponse, A> = f
        .pic()
        .query_candid_as(
            f.app(),
            scope.tenant,
            "blob_upload_admission",
            (input.permission,),
        )
        .unwrap();
    assert_eq!(admission.unwrap().state, UploadState::Reserved);
    f.pic()
        .update_candid_as::<Result<UploadRevocationResponse, A>, _>(
            f.app(),
            scope.tenant,
            "blob_revoke_upload",
            (input.permission,),
        )
        .unwrap()
        .unwrap();
    let before = f.pic().get_stable_memory(f.app());
    assert_eq!(inspect(&f, uploader, &root), Err(E::Revoked));
    refuse(&f, uploader, &root);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    f.upgrade_same_release(Duration::from_secs(5));
    let before = f.pic().get_stable_memory(f.app());
    assert_eq!(inspect(&f, uploader, &root), Err(E::Permission(A::Fenced)));
    refuse(&f, uploader, &root);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

fn refuse(f: &Fixture, actor: Principal, root: &str) {
    let error = f
        .pic()
        .update_candid_as::<CaffeineUploadCertificateResponse, _>(f.app(), actor, ISSUE, (root,))
        .unwrap_err();
    assert_eq!(
        error.reject_response.unwrap().reject_code,
        RejectCode::CanisterError
    );
}

fn authority(f: &Fixture, uploader: Principal, tenant: Principal, root: &str) {
    let mut denied = vec![
        tenant,
        Principal::from_slice(&[2, 1]),
        Principal::from_slice(&[5, 1]),
        Principal::anonymous(),
        Principal::from_slice(&[7, 1]),
        f.root(),
    ];
    denied.extend(
        f.pic()
            .canister_status(f.app(), Some(f.root()))
            .unwrap()
            .settings
            .controllers,
    );
    denied.sort_unstable();
    denied.dedup();
    for actor in denied {
        assert_eq!(inspect(f, actor, root), Err(E::Permission(A::Denied)));
        refuse(f, actor, root);
    }
    assert_eq!(
        inspect(f, uploader, "sha256:00"),
        Err(E::Permission(A::Invalid))
    );
    let unknown = format!("sha256:{}", "ff".repeat(32));
    assert_eq!(
        inspect(f, uploader, &unknown),
        Err(E::Permission(A::Denied))
    );
    let error = f
        .pic()
        .query_call(
            f.app(),
            uploader,
            INSPECT,
            candid::encode_one(true).unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
}

#[test]
fn managed_certificate_decoding_rejects_invalid_oversized_and_complex_arguments_without_mutation() {
    let f = Fixture::new();
    let actor = Principal::from_slice(&[6, 1]);
    let before = f.pic().get_stable_memory(f.app());
    let invalid = "x".repeat(4096);
    let oversized = candid::encode_one(&invalid).unwrap();
    assert!(oversized.len() > 4096);
    for (bytes, rejection) in [
        (candid::encode_one(true).unwrap(), RejectCode::CanisterError),
        (oversized, RejectCode::CanisterReject),
        (type_table(), RejectCode::CanisterError),
        (large_header(), RejectCode::CanisterError),
    ] {
        let query = f
            .pic()
            .query_call(f.app(), actor, INSPECT, bytes.clone())
            .unwrap_err();
        assert_eq!(query.reject_code, RejectCode::CanisterError);
        let update = f
            .pic()
            .update_call(f.app(), actor, ISSUE, bytes)
            .unwrap_err();
        assert_eq!(update.reject_code, rejection);
        assert_eq!(f.pic().get_stable_memory(f.app()), before);
    }
    // A valid envelope exactly at the byte limit reaches shared root validation.
    let mut root = "x".repeat(4000);
    let overhead = candid::encode_one(&root).unwrap().len() - root.len();
    root.push_str(&"x".repeat(4096 - overhead - root.len()));
    assert_eq!(candid::encode_one(&root).unwrap().len(), 4096);
    assert_eq!(inspect(&f, actor, &root), Err(E::Permission(A::Invalid)));
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

fn type_table() -> Vec<u8> {
    // Valid Candid with unused opt-text entries; ordinary decoding accepts it.
    // The artifact's type-count limit refuses before access/handler dispatch.
    let mut bytes = b"DIDL".to_vec();
    bytes.extend([0x81, 0x01]); // 129 type entries, above the maintained 128 bound.
    for _ in 0..129 {
        bytes.extend([0x6e, 0x71]);
    }
    bytes.extend([0x01, 0x71, 0x01, b'x']); // One text argument: "x".
    assert_eq!(candid::decode_one::<String>(&bytes).unwrap(), "x");
    assert!(bytes.len() < 4096);
    bytes
}

fn large_header() -> Vec<u8> {
    // A single unused record type keeps type count and total bytes in bounds,
    // while its field declarations exceed the header budget.
    let mut bytes = b"DIDL".to_vec();
    bytes.extend([0x01, 0x6c]); // One record type.
    uleb(&mut bytes, 800);
    for field in 0..800 {
        uleb(&mut bytes, field);
        bytes.push(0x7f); // Null field type.
    }
    bytes.extend([0x01, 0x71, 0x01, b'x']);
    assert_eq!(candid::decode_one::<String>(&bytes).unwrap(), "x");
    assert!(bytes.len() > 2048 && bytes.len() < 4096);
    bytes
}

fn uleb(bytes: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = u8::try_from(value & 0x7f).unwrap();
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if value == 0 {
            break;
        }
    }
}
