//! Production host qualifications remain unknown. No substituted facts enable issuance.
use super::*;
use ic_blob_storage::{
    dto::upload::{
        certificate::UploadCertificateAssessmentResponse,
        exposure::{UploadExposureBlocker as B, UploadExposureFailure as E},
    },
    workflow::uploads::certificate::{
        CAFFEINE_UPLOAD_CERTIFICATE_METHOD as ISSUE,
        UPLOAD_CERTIFICATE_ASSESSMENT_METHOD as INSPECT,
    },
};
pub(super) fn inspect(
    f: &Fixture,
    actor: Principal,
    root: &str,
) -> Result<UploadCertificateAssessmentResponse, E> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, INSPECT, (root.to_owned(),))
        .unwrap()
}
pub(super) fn refuses(f: &Fixture, actor: Principal, root: &str) {
    let error = f
        .harness
        .pic
        .update_call(f.service, actor, ISSUE, candid::encode_one(root).unwrap())
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
}
pub(super) fn root(input: &UploadManifestRequest) -> String {
    ic_blob_storage::model::identity::ProviderRootHash::try_from(
        input.permission.upload.root.as_slice(),
    )
    .unwrap()
    .to_string()
}
#[test]
fn standalone_certificate_reports_real_blockers_and_never_treats_preparation_as_authority() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let manifest = f.manifest();
    let root = root(&manifest);
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (manifest.permission,),
        )
        .unwrap()
        .unwrap();
    assert_eq!(inspect(&f, f.uploader, &root), Err(E::Unprepared));
    refuses(&f, f.uploader, &root);
    f.prepare(f.uploader, &manifest).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let assessment = inspect(&f, f.uploader, &root).unwrap();
    assert_eq!(assessment.permission, manifest.permission);
    assert!(assessment.assessed_at_ns <= f.harness.pic.get_time().as_nanos_since_unix_epoch());
    assert_eq!(
        assessment.blockers,
        vec![
            B::PrechargeLimits,
            B::ProviderNamespace,
            B::ReplayCharging,
            B::Recovery
        ]
    );
    refuses(&f, f.uploader, &root);
    for actor in [
        f.controller,
        f.operator,
        f.tenant,
        Fake::principal(90),
        Principal::anonymous(),
    ] {
        assert_eq!(
            inspect(&f, actor, &root),
            Err(E::Permission(UploadAdmissionFailure::Denied))
        );
        refuses(&f, actor, &root);
    }
    assert_eq!(
        inspect(&f, f.uploader, "sha256:00"),
        Err(E::Permission(UploadAdmissionFailure::Invalid))
    );
    refuses(&f, f.uploader, "sha256:00");
    let unknown = format!("sha256:{}", "ff".repeat(32));
    assert_eq!(
        inspect(&f, f.uploader, &unknown),
        Err(E::Permission(UploadAdmissionFailure::Denied))
    );
    refuses(&f, f.uploader, &unknown);
    malformed(&f);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert_eq!(
        f.admission(manifest.permission).state,
        ic_blob_storage::dto::upload::UploadState::Reserved
    );
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .unwrap();
    f.harness
        .pic
        .start_canister(f.service, Some(f.controller))
        .unwrap();
    assert_eq!(
        inspect(&f, f.uploader, &root).unwrap().blockers,
        assessment.blockers
    );
    refuses(&f, f.uploader, &root);
    f.harness
        .pic
        .update_candid_as::<Result<UploadRevocationResponse, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_revoke_upload",
            (manifest.permission,),
        )
        .unwrap()
        .unwrap();
    assert_eq!(inspect(&f, f.uploader, &root), Err(E::Revoked));
    refuses(&f, f.uploader, &root);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    assert_eq!(
        inspect(&f, f.uploader, &root),
        Err(E::Permission(UploadAdmissionFailure::Fenced))
    );
    let restored = f.harness.pic.get_stable_memory(f.service);
    refuses(&f, f.uploader, &root);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}

fn malformed(f: &Fixture) {
    // Oversized ingress and type confusion must never create provider reply bytes.
    for argument in [
        candid::encode_one("x".repeat(4097)).unwrap(),
        candid::encode_one(true).unwrap(),
    ] {
        for method in [ISSUE, INSPECT] {
            let error = if method == ISSUE {
                f.harness
                    .pic
                    .update_call(f.service, f.uploader, method, argument.clone())
                    .unwrap_err()
            } else {
                f.harness
                    .pic
                    .query_call(f.service, f.uploader, method, argument.clone())
                    .unwrap_err()
            };
            assert_eq!(error.reject_code, RejectCode::CanisterError);
        }
    }
}
