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
        certificate::UploadCertificateAssessmentResponse,
        exposure::{UploadExposureBlocker as B, UploadExposureFailure as E},
        manifest::{UploadManifestFailure, UploadManifestMutation},
    },
    model::identity::ProviderRootHash,
    workflow::uploads::certificate::UPLOAD_CERTIFICATE_ASSESSMENT_METHOD as INSPECT,
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
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    f.upgrade_same_release(Duration::from_secs(5));
    let before = f.pic().get_stable_memory(f.app());
    assert_eq!(inspect(&f, uploader, &root), Err(E::Permission(A::Fenced)));
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
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
