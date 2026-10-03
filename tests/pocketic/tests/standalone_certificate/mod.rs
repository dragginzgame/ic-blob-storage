//! Actual configured host: bounded multi-file issuance, explicit trust and restore fences.
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
fn standalone_certificate_issues_configured_multifile_uploads_and_preserves_quotas_and_authority() {
    use ic_blob_storage::dto::upload::{
        UploadState, certificate::CaffeineUploadCertificateResponse,
    };
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let manifest = f.manifest();
    let root = root(&manifest);
    // The configured 10 MiB object maximum remains an admission boundary.
    let mut oversized = manifest.permission;
    oversized.upload.bytes += 1;
    assert_eq!(admit(&f, oversized), Err(UploadAdmissionFailure::Capacity));
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
    assert_eq!(assessment.blockers, []);
    check_authority(&f, &root);
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
    f.resume(f.operator).unwrap();
    assert_eq!(
        inspect(&f, f.uploader, &root).unwrap().blockers,
        assessment.blockers
    );
    let reply: CaffeineUploadCertificateResponse = f
        .harness
        .pic
        .update_candid_as(f.service, f.uploader, ISSUE, (root.clone(),))
        .unwrap();
    assert_eq!(reply.method, "upload");
    assert_eq!(reply.blob_hash, root);
    assert_eq!(
        f.admission(manifest.permission).state,
        UploadState::ExposurePossible
    );
    refuses(&f, f.uploader, &root);

    let second = second_upload_at_capacity(&f, &manifest);
    let second_root = self::root(&second);
    let usage = f.local_status(f.operator, f.operator_scope()).unwrap();
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
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        usage
    );
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    assert_eq!(
        inspect(&f, f.uploader, &root),
        Err(E::Permission(UploadAdmissionFailure::Fenced))
    );
    let restored = f.harness.pic.get_stable_memory(f.service);
    refuses(&f, f.uploader, &root);
    refuses(&f, f.uploader, &second_root);
    assert_eq!(
        f.admission(second.permission).state,
        UploadState::ExposurePossible
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}

fn check_authority(f: &Fixture, root: &str) {
    for actor in [
        f.controller,
        f.operator,
        f.tenant,
        Fake::principal(90),
        Principal::anonymous(),
    ] {
        assert_eq!(
            inspect(f, actor, root),
            Err(E::Permission(UploadAdmissionFailure::Denied))
        );
        refuses(f, actor, root);
    }
    assert_eq!(
        inspect(f, f.uploader, "sha256:00"),
        Err(E::Permission(UploadAdmissionFailure::Invalid))
    );
    refuses(f, f.uploader, "sha256:00");
    let unknown = format!("sha256:{}", "ff".repeat(32));
    assert_eq!(
        inspect(f, f.uploader, &unknown),
        Err(E::Permission(UploadAdmissionFailure::Denied))
    );
    refuses(f, f.uploader, &unknown);
    malformed(f);
}

fn second_upload_at_capacity(f: &Fixture, first: &UploadManifestRequest) -> UploadManifestRequest {
    use ic_blob_storage::dto::upload::{
        UploadState, certificate::CaffeineUploadCertificateResponse,
    };
    // A second object crosses a chunk boundary and shares the same trusted owner.
    let mut second = f.manifest_bytes(1024 * 1024 + 1);
    second.permission.upload.upload = 2;
    second.permission.upload.object = 3;
    let second_root = self::root(&second);
    admit(f, second.permission).unwrap();
    f.prepare(f.uploader, &second).unwrap();
    assert_eq!(inspect(f, f.uploader, &second_root).unwrap().blockers, []);
    let reply: CaffeineUploadCertificateResponse = f
        .harness
        .pic
        .update_candid_as(f.service, f.uploader, ISSUE, (second_root.clone(),))
        .unwrap();
    assert_eq!(reply.blob_hash, second_root);
    assert_eq!(
        f.admission(second.permission).state,
        UploadState::ExposurePossible
    );
    refuses(f, f.uploader, &second_root);
    let usage = f.local_status(f.operator, f.operator_scope()).unwrap();
    let total = u128::from(first.permission.upload.bytes + second.permission.upload.bytes);
    assert_eq!(usage.uploads.active_reservations, 2);
    assert_eq!(usage.uploads.physical_bytes, total);
    assert_eq!(usage.uploads.liability_bytes, total);
    let mut third = second.permission;
    third.upload.upload = 3;
    third.upload.object = 4;
    third.upload.root = [99; 32];
    third.upload.bytes = 1;
    assert_eq!(admit(f, third), Err(UploadAdmissionFailure::Capacity));
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        usage
    );
    second
}

fn admit(
    f: &Fixture,
    permission: UploadAdmissionRequest,
) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    f.harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap()
}

#[test]
fn standalone_small_configuration_issues_once_and_retains_uncertainty_across_stop_and_restore() {
    use ic_blob_storage::dto::upload::{
        UploadState, certificate::CaffeineUploadCertificateResponse,
    };
    let f = Fixture::small(Harness::new(), Fake::principal(4));
    f.enroll(f.operator).unwrap();
    let manifest = f.small_manifest();
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
    f.prepare(f.uploader, &manifest).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(inspect(&f, f.uploader, &root).unwrap().blockers, []);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    for actor in [f.controller, f.operator, f.tenant, Principal::anonymous()] {
        refuses(&f, actor, &root);
    }
    malformed(&f);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let usage = f.local_status(f.operator, f.operator_scope()).unwrap();
    let reply: CaffeineUploadCertificateResponse = f
        .harness
        .pic
        .update_candid_as(f.service, f.uploader, ISSUE, (root.clone(),))
        .unwrap();
    assert_eq!(reply.method, "upload");
    assert_eq!(reply.blob_hash, root);
    assert_eq!(
        f.admission(manifest.permission).state,
        UploadState::ExposurePossible
    );
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        usage
    );
    let exposed = f.harness.pic.get_stable_memory(f.service);
    refuses(&f, f.uploader, &root);
    assert_eq!(inspect(&f, f.uploader, &root), Err(E::Phase));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &exposed);
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .unwrap();
    f.harness
        .pic
        .start_canister(f.service, Some(f.controller))
        .unwrap();
    f.resume(f.operator).unwrap();
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
    let retained = f.admission(manifest.permission);
    assert!(retained.revoked);
    assert_eq!(retained.state, UploadState::ExposurePossible);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        usage
    );
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    assert_eq!(f.admission(manifest.permission), retained);
    let restored = f.harness.pic.get_stable_memory(f.service);
    refuses(&f, f.uploader, &root);
    assert_eq!(
        inspect(&f, f.uploader, &root),
        Err(E::Permission(UploadAdmissionFailure::Fenced))
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
    f.resume(f.operator).unwrap();
    assert_eq!(f.admission(manifest.permission), retained);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        usage
    );
    refuses(&f, f.uploader, &root);
    assert_eq!(inspect(&f, f.uploader, &root), Err(E::Revoked));
}

#[test]
fn standalone_tenant_permission_cannot_grant_installed_uploader_trust() {
    let mut f = Fixture::small(Harness::new(), Fake::principal(4));
    f.uploader = Fake::principal(91);
    f.enroll(f.operator).unwrap();
    let manifest = f.small_manifest();
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
    f.prepare(f.uploader, &manifest).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        inspect(&f, f.uploader, &root).unwrap().blockers,
        vec![B::TrustedUploader]
    );
    refuses(&f, f.uploader, &root);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
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
