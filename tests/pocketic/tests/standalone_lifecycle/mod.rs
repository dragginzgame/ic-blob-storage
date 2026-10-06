//! Current-instance restoration retains obligations and requires independent IC history.
use super::*;
use ic_blob_storage::dto::{
    account::{
        AccountInspectionFailure, AccountInspectionKind, AccountInspectionRequest,
        AccountInspectionResponse,
    },
    gateway::{GatewayRevocationFailure, GatewayRevocationRequest, GatewayRevocationResponse},
    operator::LocalServiceStatus,
};
use ic_blob_storage::ops::service::gateways::revocation::GATEWAY_REVOCATION_METHOD;

fn fences(status: &LocalServiceStatus, expected: bool) {
    assert_eq!(status.uploads.fenced, expected);
    assert_eq!(status.funding.fenced, expected);
    assert_eq!(status.gateways.fenced, expected);
    assert_eq!(status.reads.fenced, expected);
}
fn normalized(mut status: LocalServiceStatus) -> LocalServiceStatus {
    status.uploads.fenced = false;
    status.funding.fenced = false;
    status.gateways.fenced = false;
    status.reads.fenced = false;
    status
}
fn refuses(f: &Fixture, manifest: &UploadManifestRequest) {
    assert_eq!(
        f.prepare(f.uploader, manifest),
        Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Fenced
        ))
    );
    let removed: Result<GatewayRevocationResponse, GatewayRevocationFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            GATEWAY_REVOCATION_METHOD,
            (GatewayRevocationRequest {
                scope: f.operator_scope(),
                gateway: Fake::principal(9),
            },),
        )
        .unwrap();
    assert_eq!(removed, Err(GatewayRevocationFailure::Fenced));
    let account: Result<AccountInspectionResponse, AccountInspectionFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_inspect_account",
            (AccountInspectionRequest {
                scope: f.operator_scope(),
                kind: AccountInspectionKind::Balance,
            },),
        )
        .unwrap();
    assert_eq!(account, Err(AccountInspectionFailure::Fenced));
}

#[test]
fn stop_start_and_repeated_upgrade_preserve_installation_and_all_owner_fences() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let manifest = f.manifest();
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
    let prepared = f.prepare(f.uploader, &manifest).unwrap().observation;
    let installation = f.configuration(f.operator).unwrap();
    let admission = f.admission(manifest.permission);
    let status = f.local_status(f.operator, f.operator_scope()).unwrap();
    fences(&status, false);
    assert_eq!(
        status.uploads.reserved_bytes,
        u128::from(manifest.permission.upload.bytes)
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .unwrap();
    f.harness
        .pic
        .start_canister(f.service, Some(f.controller))
        .unwrap();
    fences(
        &f.local_status(f.operator, f.operator_scope()).unwrap(),
        true,
    );
    assert_eq!(
        f.resume(f.controller),
        Err(ic_blob_storage::dto::recovery::CurrentInstanceRecoveryFailure::Denied)
    );
    f.resume(f.operator).unwrap();
    assert_eq!(f.configuration(f.operator).unwrap(), installation);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        status
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    // Supplied replacement settings must not publish a partially restored host.
    assert_eq!(
        f.upgrade(super::installation(&f.config))
            .unwrap_err()
            .reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.configuration(f.operator).unwrap(), installation);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        status
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    for _ in 0..2 {
        f.upgrade(candid::encode_args(()).unwrap()).unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        let mut expected = installation.clone();
        expected.fenced = true;
        assert_eq!(f.configuration(f.operator).unwrap(), expected);
        let restored = f.local_status(f.operator, f.operator_scope()).unwrap();
        fences(&restored, true);
        assert_eq!(normalized(restored), status);
        assert_eq!(f.admission(manifest.permission), admission);
        let retained: Result<UploadManifestResponse, UploadManifestFailure> = f
            .harness
            .pic
            .query_candid_as(
                f.service,
                f.uploader,
                "blob_upload_manifest",
                (manifest.permission,),
            )
            .unwrap();
        assert_eq!(retained.unwrap(), prepared);
        refuses(&f, &manifest);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    // A fresh replicated IC history read proves this was an ordinary same-release
    // upgrade, while all original permissions, manifests and reservations survive.
    f.resume(f.operator).unwrap();
    fences(
        &f.local_status(f.operator, f.operator_scope()).unwrap(),
        false,
    );
    assert_eq!(f.admission(manifest.permission), admission);
    assert_eq!(
        f.prepare(f.uploader, &manifest).unwrap().observation,
        prepared
    );
    let root = super::standalone_certificate::root(&manifest);
    let assessment = super::standalone_certificate::inspect(&f, f.uploader, &root).unwrap();
    assert_eq!(assessment.blockers, []);
}

#[test]
fn current_instance_recovery_refuses_expired_platform_history_without_rotating_anchor() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let installation = f.configuration(f.operator).unwrap();
    // Each actual controller-settings change adds IC-owned history. Keep the
    // same controller; neither its authority nor a plausible local counter can
    // replace the missing beginning of this installation's retained history.
    for _ in 0..21 {
        f.harness
            .pic
            .set_controllers(f.service, Some(f.controller), vec![f.controller])
            .unwrap();
    }
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.resume(f.operator),
        Err(ic_blob_storage::dto::recovery::CurrentInstanceRecoveryFailure::IncompleteHistory)
    );
    let mut expected = installation;
    expected.fenced = true;
    assert_eq!(f.configuration(f.operator).unwrap(), expected);
    fences(
        &f.local_status(f.operator, f.operator_scope()).unwrap(),
        true,
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}

#[test]
fn current_instance_recovery_rejects_management_change_during_history_await() {
    use ic_blob_storage::dto::recovery::CurrentInstanceRecoveryFailure;
    let f = Fixture::new();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let message = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.operator,
            "blob_resume_current_instance",
            candid::encode_args(()).unwrap(),
        )
        .unwrap();
    f.harness.pic.tick();
    f.harness
        .pic
        .set_controllers(f.service, Some(f.controller), vec![f.controller])
        .unwrap();
    let reply: Result<(), CurrentInstanceRecoveryFailure> =
        candid::decode_one(&f.harness.pic.await_call(message).unwrap()).unwrap();
    assert_eq!(reply, Err(CurrentInstanceRecoveryFailure::Binding));
    assert!(f.configuration(f.operator).unwrap().fenced);
    // A fresh explicit history read may qualify this ordinary controller change;
    // the earlier reply is not reused and no provider operation is repeated.
    f.resume(f.operator).unwrap();
    assert!(!f.configuration(f.operator).unwrap().fenced);
}

pub(super) fn confirm_and_release_locally(f: &Fixture) {
    use ic_blob_storage::dto::upload::{
        certificate::CaffeineUploadCertificateResponse,
        completion::{
            UploadAttestationFailure, UploadAttestationMutation, UploadAttestationRequest,
        },
    };
    f.enroll(f.operator).unwrap();
    let manifest = f.small_manifest();
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
    let _: CaffeineUploadCertificateResponse = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.uploader,
            ic_blob_storage::workflow::uploads::certificate::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
            (super::standalone_certificate::root(&manifest),),
        )
        .unwrap();
    // This statement is a local verifier substitute, not a deployed byte observation.
    f.harness
        .pic
        .update_candid_as::<Result<UploadAttestationMutation, UploadAttestationFailure>, _>(
            f.service,
            Fake::principal(90),
            "blob_attest_upload",
            (UploadAttestationRequest {
                permission: manifest.permission,
                content_digest: [1; 32],
                observed_at_ns: f.harness.pic.get_time().as_nanos_since_unix_epoch(),
            },),
        )
        .unwrap()
        .unwrap();
    f.harness
        .pic
        .update_candid_as::<Result<ReferenceMutationResponse, ReferenceFailure>, _>(
            f.service,
            f.tenant,
            "blob_apply_reference",
            (ReferenceCommand {
                upload: manifest.permission.upload,
                reference: manifest.permission.upload.first_reference,
                operation: 2,
                action: ReferenceAction::Release,
            },),
        )
        .unwrap()
        .unwrap();
}

#[test]
fn active_current_instance_recovery_retains_released_physical_and_billing_obligations() {
    let f = Fixture::small(Harness::new(), Fake::principal(4));
    confirm_and_release_locally(&f);
    let permission = f.small_manifest().permission;
    let before = f.local_status(f.operator, f.operator_scope()).unwrap();
    assert_eq!(before.uploads.logical_bytes, 0);
    assert_eq!(before.uploads.physical_bytes, 1024);
    assert_eq!(before.uploads.liability_bytes, 1024);
    let admission = f.admission(permission);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    fences(
        &f.local_status(f.operator, f.operator_scope()).unwrap(),
        true,
    );
    f.resume(f.operator).unwrap();
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        before
    );
    assert_eq!(f.admission(permission), admission);
    super::standalone_certificate::refuses(
        &f,
        f.uploader,
        &super::standalone_certificate::root(&f.small_manifest()),
    );
}
