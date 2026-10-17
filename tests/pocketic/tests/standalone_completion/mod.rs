use super::*;
use crate::support::REPLY_LIMITS;
use ic_blob_storage_contracts::dto::upload::completion::*;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
fn verification_manifest(f: &Fixture, manifest: &UploadManifestRequest) {
    for actor in [Fake::principal(90), f.operator, f.tenant, f.uploader] {
        let response: Result<UploadManifestResponse, UploadManifestFailure> = f
            .harness
            .pic
            .query_blob_as(
                f.service,
                actor,
                "blob_verification_manifest",
                manifest.permission,
                REPLY_LIMITS.reply_bytes,
            )
            .unwrap();
        if actor == Fake::principal(90) {
            assert_eq!(
                response.unwrap().manifest,
                ic_blob_storage_contracts::dto::upload::manifest::UploadManifestInspection::Prepared(
                    manifest.declaration.clone()
                )
            );
        } else {
            assert_eq!(
                response,
                Err(UploadManifestFailure::Permission(
                    UploadAdmissionFailure::Denied
                ))
            );
        }
    }
}
fn plan_refuses(f: &Fixture, permission: UploadAdmissionRequest, fenced: bool) {
    for actor in [Fake::principal(90), f.operator, f.tenant, f.uploader] {
        let result: Result<UploadVerificationPlan, UploadAttestationFailure> = f
            .harness
            .pic
            .query_blob_as(
                f.service,
                actor,
                "blob_verification_plan",
                permission,
                REPLY_LIMITS.reply_bytes,
            )
            .unwrap();
        let expected = if actor != Fake::principal(90) {
            UploadAttestationFailure::Denied
        } else if fenced {
            UploadAttestationFailure::Permission(UploadAdmissionFailure::Fenced)
        } else {
            UploadAttestationFailure::Phase
        };
        assert_eq!(result, Err(expected));
    }
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "Keep verifier, exposure and owner refusals in one installed completion scenario"
)]
fn standalone_completion_requires_installed_verifier_exposure_and_active_owner() {
    let f = Fixture::new();
    let verifier = Fake::principal(90);
    assert_eq!(
        f.configuration(f.operator).unwrap().completion_verifier,
        verifier
    );
    f.enroll(f.operator).unwrap();
    let manifest = f.manifest();
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            manifest.permission,
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &manifest).unwrap();
    verification_manifest(&f, &manifest);
    plan_refuses(&f, manifest.permission, false);
    let input = UploadAttestationRequest {
        permission: manifest.permission,
        content_digest: [1; 32],
        observed_at_ns: f.harness.pic.get_time().as_nanos_since_unix_epoch(),
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.operator, f.tenant, f.uploader] {
        let result: Result<UploadAttestationMutation, UploadAttestationFailure> = f
            .harness
            .pic
            .update_blob_as(
                f.service,
                actor,
                "blob_attest_upload",
                input,
                REPLY_LIMITS.reply_bytes,
            )
            .unwrap();
        assert_eq!(result, Err(UploadAttestationFailure::Denied));
    }
    let result: Result<UploadAttestationMutation, UploadAttestationFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            verifier,
            "blob_attest_upload",
            input,
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap();
    assert_eq!(result, Err(UploadAttestationFailure::Phase));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        let args = ic_blob_storage_contracts::wire::encode(ServiceInstallationInput {
            configuration: f.config,
            project: PROJECT.into(),
            completion_verifier: invalid,
        })
        .unwrap();
        assert!(
            f.harness
                .pic
                .reinstall_canister(f.service, wasm(), args, Some(f.controller))
                .is_err()
        );
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    f.upgrade(ic_blob_storage_contracts::wire::encode(()).unwrap())
        .unwrap();
    verification_manifest(&f, &manifest);
    plan_refuses(&f, manifest.permission, true);
    let result: Result<UploadAttestationMutation, UploadAttestationFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            verifier,
            "blob_attest_upload",
            input,
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap();
    assert_eq!(
        result,
        Err(UploadAttestationFailure::Permission(
            UploadAdmissionFailure::Fenced
        ))
    );
    for actor in [verifier, f.tenant, f.uploader] {
        let result: Result<UploadAttestationResponse, UploadAttestationFailure> = f
            .harness
            .pic
            .query_blob_as(
                f.service,
                actor,
                "blob_upload_attestation",
                input.permission,
                REPLY_LIMITS.reply_bytes,
            )
            .unwrap();
        let response = result.unwrap();
        assert!(response.fenced);
        assert_eq!(response.permission, input.permission);
        assert_eq!(response.attestation, UploadAttestationLookup::Absent);
    }
}
