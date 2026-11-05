//! Current-instance restoration retains obligations and requires independent IC history.
use super::*;
use ic_blob_storage_contracts::dto::account::AccountInspectionFailure;
use ic_blob_storage_contracts::dto::account::AccountInspectionKind;
use ic_blob_storage_contracts::dto::account::AccountInspectionRequest;
use ic_blob_storage_contracts::dto::account::AccountInspectionResponse;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationFailure;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationRequest;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationResponse;
use ic_blob_storage_contracts::dto::operator::LocalServiceStatus;
use ic_blob_storage_contracts::protocol::GATEWAY_REVOCATION_METHOD;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::provider_calls;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

fn fences(status: &LocalServiceStatus, expected: bool) {
    assert_eq!(status.uploads.fenced, expected);
    assert_eq!(status.funding.fenced, expected);
    assert_eq!(status.gateways.fenced, expected);
}
fn normalized(mut status: LocalServiceStatus) -> LocalServiceStatus {
    status.uploads.fenced = false;
    status.funding.fenced = false;
    status.gateways.fenced = false;

    status
}
async fn refuses(f: &Fixture, manifest: &UploadManifestRequest) {
    assert_eq!(
        f.prepare(f.uploader, manifest).await,
        Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Fenced
        ))
    );
    let removed: Result<GatewayRevocationResponse, GatewayRevocationFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.operator,
            GATEWAY_REVOCATION_METHOD,
            GatewayRevocationRequest {
                scope: f.operator_scope(),
                gateway: fake_principal(9).into(),
            },
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(removed, Err(GatewayRevocationFailure::Fenced));
    let account: Result<AccountInspectionResponse, AccountInspectionFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.operator,
            "blob_inspect_account",
            AccountInspectionRequest {
                scope: f.operator_scope(),
                kind: AccountInspectionKind::Balance,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(account, Err(AccountInspectionFailure::Fenced));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered installed scenario retains lifecycle and byte-preservation assertions while making fallible client operations explicit"
)]
async fn stop_start_and_repeated_upgrade_preserve_installation_and_all_owner_fences() {
    let f = Fixture::new().await;
    f.enroll(f.operator).await.unwrap();
    let manifest = f.manifest();
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            manifest.permission,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
    let prepared = f.prepare(f.uploader, &manifest).await.unwrap().observation;
    let installation = f.configuration(f.operator).await.unwrap();
    let admission = f.admission(manifest.permission).await;
    let status = f
        .local_status(f.operator, f.operator_scope())
        .await
        .unwrap();
    fences(&status, false);
    assert_eq!(
        status.uploads.reserved_bytes,
        u128::from(manifest.permission.upload.bytes)
    );
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .await
        .unwrap();
    f.harness
        .pic
        .start_canister(f.service, Some(f.controller))
        .await
        .unwrap();
    fences(
        &f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        true,
    );
    assert_eq!(
        f.resume(f.controller).await,
        Err(ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure::Denied)
    );
    f.resume(f.operator).await.unwrap();
    assert_eq!(f.configuration(f.operator).await.unwrap(), installation);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        status
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    // Supplied replacement settings must not publish a partially restored host.
    assert_eq!(
        crate::canister_rejection(&f.upgrade(super::installation(&f.config)).await.unwrap_err())
            .reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.configuration(f.operator).await.unwrap(), installation);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        status
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    for _ in 0..2 {
        f.upgrade(
            ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap(),
        )
        .await
        .unwrap();
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        let mut expected = installation.clone();
        expected.fenced = true;
        assert_eq!(f.configuration(f.operator).await.unwrap(), expected);
        let restored = f
            .local_status(f.operator, f.operator_scope())
            .await
            .unwrap();
        fences(&restored, true);
        assert_eq!(normalized(restored), status);
        assert_eq!(f.admission(manifest.permission).await, admission);
        let retained: Result<UploadManifestResponse, UploadManifestFailure> = f
            .harness
            .pic
            .query_blob_as(
                f.service,
                f.uploader,
                "blob_upload_manifest",
                manifest.permission,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        assert_eq!(retained.unwrap(), prepared);
        refuses(&f, &manifest).await;
        unchanged(
            &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            &before,
        );
    }
    // A fresh replicated IC history read proves this was an ordinary same-release
    // upgrade, while all original permissions, manifests and reservations survive.
    f.resume(f.operator).await.unwrap();
    fences(
        &f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        false,
    );
    assert_eq!(f.admission(manifest.permission).await, admission);
    assert_eq!(
        f.prepare(f.uploader, &manifest).await.unwrap().observation,
        prepared
    );
    let root = super::standalone_certificate::root(&manifest);
    let assessment = super::standalone_certificate::inspect(&f, f.uploader, &root)
        .await
        .unwrap();
    assert_eq!(assessment.permission, manifest.permission);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn current_instance_recovery_refuses_expired_platform_history_without_rotating_anchor() {
    let f = Fixture::new().await;
    f.enroll(f.operator).await.unwrap();
    let installation = f.configuration(f.operator).await.unwrap();
    // Each actual controller-settings change adds IC-owned history. Keep the
    // same controller; neither its authority nor a plausible local counter can
    // replace the missing beginning of this installation's retained history.
    for _ in 0..21 {
        f.harness
            .pic
            .set_controllers(f.service, Some(f.controller), vec![f.controller])
            .await
            .unwrap();
    }
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        f.resume(f.operator).await,
        Err(ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure::IncompleteHistory)
    );
    let mut expected = installation;
    expected.fenced = true;
    assert_eq!(f.configuration(f.operator).await.unwrap(), expected);
    fences(
        &f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        true,
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn current_instance_recovery_rejects_management_change_during_history_await() {
    use ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure;
    let f = Fixture::new().await;
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let message = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.operator,
            "blob_resume_current_instance",
            ic_blob_storage_contracts::wire::encode_request("blob_resume_current_instance", ())
                .unwrap(),
        )
        .await
        .unwrap();
    f.harness.pic.tick().await.unwrap();
    f.harness
        .pic
        .set_controllers(f.service, Some(f.controller), vec![f.controller])
        .await
        .unwrap();
    let reply: Result<(), CurrentInstanceRecoveryFailure> =
        ic_blob_storage_contracts::wire::decode_response(
            "blob_resume_current_instance",
            &f.harness.pic.await_call(message).await.unwrap(),
            ic_blob_storage_contracts::wire::MAX_MESSAGE_BYTES,
        )
        .unwrap();
    assert_eq!(reply, Err(CurrentInstanceRecoveryFailure::Binding));
    assert!(f.configuration(f.operator).await.unwrap().fenced);
    // A fresh explicit history read may qualify this ordinary controller change;
    // the earlier reply is not reused and no provider operation is repeated.
    f.resume(f.operator).await.unwrap();
    assert!(!f.configuration(f.operator).await.unwrap().fenced);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn recovery_survives_denied_updates_during_history_without_losing_obligations() {
    use ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure;
    let f = Fixture::small(Harness::new().await, fake_principal(4)).await;
    confirm_and_release_locally(&f).await;
    let before = f
        .local_status(f.operator, f.operator_scope())
        .await
        .unwrap();
    assert_eq!(before.uploads.physical_bytes, 1024);
    assert_eq!(before.uploads.liability_bytes, 1024);
    let admission = f.admission(f.small_manifest().permission).await;
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let submit = async |actor| {
        f.harness
            .pic
            .submit_call(
                f.service,
                actor,
                "blob_resume_current_instance",
                ic_blob_storage_contracts::wire::encode_request("blob_resume_current_instance", ())
                    .unwrap(),
            )
            .await
            .unwrap()
    };
    let recovery = submit(f.operator).await;
    f.harness.pic.tick().await.unwrap();
    // Queue repeated unauthorized updates alongside the history request. They
    // must reach the canister (not merely be rejected by ingress inspection).
    let mut denied = Vec::with_capacity(16);
    for _ in 0..16 {
        denied.push(submit(f.uploader).await);
    }
    let decode = |bytes: &[u8]| {
        ic_blob_storage_contracts::wire::decode_response::<
            Result<(), CurrentInstanceRecoveryFailure>,
        >("blob_resume_current_instance", bytes, REPLY_BYTES)
        .unwrap()
    };
    assert_eq!(
        decode(&f.harness.pic.await_call(recovery).await.unwrap()),
        Ok(())
    );
    for message in denied {
        assert_eq!(
            decode(&f.harness.pic.await_call(message).await.unwrap()),
            Err(CurrentInstanceRecoveryFailure::Denied)
        );
    }
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        before
    );
    assert_eq!(f.admission(f.small_manifest().permission).await, admission);
}

pub(super) async fn confirm_and_release_locally(f: &Fixture) {
    use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
    f.enroll(f.operator).await.unwrap();
    let manifest = f.small_manifest();
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            manifest.permission,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &manifest).await.unwrap();
    let _: CaffeineUploadCertificateResponse = provider_calls::update_provider(
        &f.harness.pic,
        f.service,
        f.uploader,
        ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
        (super::standalone_certificate::root(&manifest),),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    // This statement is a local verifier substitute, not a deployed byte observation.
    f.harness
        .pic
        .update_blob_as::<Result<ReferenceMutationResponse, ReferenceFailure>, _>(
            f.service,
            f.tenant,
            "blob_apply_reference",
            ReferenceCommand {
                upload: manifest.permission.upload,
                reference: manifest.permission.upload.first_reference,
                operation: 2,
                action: ReferenceAction::Release,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn active_current_instance_recovery_retains_released_physical_and_billing_obligations() {
    let f = Fixture::small(Harness::new().await, fake_principal(4)).await;
    confirm_and_release_locally(&f).await;
    let permission = f.small_manifest().permission;
    let before = f
        .local_status(f.operator, f.operator_scope())
        .await
        .unwrap();
    assert_eq!(before.uploads.logical_bytes, 0);
    assert_eq!(before.uploads.physical_bytes, 1024);
    assert_eq!(before.uploads.liability_bytes, 1024);
    let admission = f.admission(permission).await;
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    fences(
        &f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        true,
    );
    f.resume(f.operator).await.unwrap();
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        before
    );
    assert_eq!(f.admission(permission).await, admission);
    super::standalone_certificate::refuses(
        &f,
        f.uploader,
        &super::standalone_certificate::root(&f.small_manifest()),
    )
    .await;
}
