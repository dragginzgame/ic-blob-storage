//! Actual configured host: project-bound multi-user issuance, quotas and restore fences.
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::provider_calls;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod signed;
use super::*;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_contracts::dto::upload::certificate::UploadCertificateAssessmentResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure as E;
use ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD as ISSUE;
use ic_blob_storage_contracts::protocol::UPLOAD_CERTIFICATE_ASSESSMENT_METHOD as INSPECT;
pub(super) async fn inspect(
    f: &Fixture,
    actor: Principal,
    root: &str,
) -> Result<UploadCertificateAssessmentResponse, E> {
    f.harness
        .pic
        .query_blob_as(f.service, actor, INSPECT, root.to_owned(), REPLY_BYTES)
        .await
        .unwrap()
}
pub(super) async fn refuses(f: &Fixture, actor: Principal, root: &str) {
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            actor,
            ISSUE,
            ic_platform::candid::contract::encode_one(root).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
}
pub(super) fn root(input: &UploadManifestRequest) -> String {
    ic_blob_storage_contracts::identity::ProviderRootHash::try_from(
        input.permission.upload.root.as_slice(),
    )
    .unwrap()
    .to_string()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered installed scenario retains lifecycle and byte-preservation assertions while making fallible client operations explicit"
)]
async fn standalone_certificate_issues_configured_multifile_uploads_and_preserves_quotas_and_authority()
 {
    use ic_blob_storage_contracts::dto::upload::UploadState;
    let f = Fixture::new().await;
    f.enroll(f.operator).await.unwrap();
    let manifest = f.manifest();
    let root = root(&manifest);
    // The configured 10 MiB object maximum remains an admission boundary.
    let mut oversized = manifest.permission;
    oversized.upload.bytes += 1;
    assert_eq!(
        admit(&f, oversized).await,
        Err(UploadAdmissionFailure::Capacity)
    );
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
    assert_eq!(inspect(&f, f.uploader, &root).await, Err(E::Unprepared));
    refuses(&f, f.uploader, &root).await;
    f.prepare(f.uploader, &manifest).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let assessment = inspect(&f, f.uploader, &root).await.unwrap();
    assert_eq!(assessment.permission, manifest.permission);
    assert!(
        assessment.assessed_at_ns
            <= f.harness
                .pic
                .get_time()
                .await
                .unwrap()
                .as_nanos_since_unix_epoch()
    );
    check_authority(&f, &root).await;
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    assert_eq!(
        f.admission(manifest.permission).await.state,
        ic_blob_storage_contracts::dto::upload::UploadState::Reserved
    );
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
    f.resume(f.operator).await.unwrap();
    assert_eq!(
        inspect(&f, f.uploader, &root).await.unwrap().permission,
        assessment.permission
    );
    let reply: CaffeineUploadCertificateResponse = provider_calls::update_provider(
        &f.harness.pic,
        f.service,
        f.uploader,
        ISSUE,
        (root.clone(),),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    assert_eq!(reply.method, "upload");
    assert_eq!(reply.blob_hash, root);
    assert_eq!(
        f.admission(manifest.permission).await.state,
        UploadState::Registered
    );
    refuses(&f, f.uploader, &root).await;

    let second = second_upload_at_capacity(&f, &manifest).await;
    let second_root = self::root(&second);
    let usage = f
        .local_status(f.operator, f.operator_scope())
        .await
        .unwrap();
    f.harness
        .pic
        .update_blob_as::<Result<UploadRevocationResponse, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_revoke_upload",
            manifest.permission,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(inspect(&f, f.uploader, &root).await, Err(E::Revoked));
    refuses(&f, f.uploader, &root).await;
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        usage
    );
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    assert_eq!(
        inspect(&f, f.uploader, &root).await,
        Err(E::Permission(UploadAdmissionFailure::Fenced))
    );
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    refuses(&f, f.uploader, &root).await;
    refuses(&f, second.permission.uploader.into(), &second_root).await;
    assert_eq!(
        f.admission(second.permission).await.state,
        UploadState::Registered
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
}

async fn check_authority(f: &Fixture, root: &str) {
    for actor in [
        f.controller,
        f.operator,
        f.tenant,
        fake_principal(90),
        Principal::anonymous(),
    ] {
        assert_eq!(
            inspect(f, actor, root).await,
            Err(E::Permission(UploadAdmissionFailure::Denied))
        );
        refuses(f, actor, root).await;
    }
    assert_eq!(
        inspect(f, f.uploader, "sha256:00").await,
        Err(E::Permission(UploadAdmissionFailure::Invalid))
    );
    refuses(f, f.uploader, "sha256:00").await;
    let unknown = format!("sha256:{}", "ff".repeat(32));
    assert_eq!(
        inspect(f, f.uploader, &unknown).await,
        Err(E::Permission(UploadAdmissionFailure::Denied))
    );
    refuses(f, f.uploader, &unknown).await;
    malformed(f, root).await;
}

async fn second_upload_at_capacity(
    f: &Fixture,
    first: &UploadManifestRequest,
) -> UploadManifestRequest {
    // A second project-approved uploader crosses a chunk boundary under shared quotas.
    let mut second = f.manifest_bytes(1024 * 1024 + 1);
    second.permission.uploader = fake_principal(91).into();
    second.permission.upload.upload = 2;
    second.permission.upload.object = 3;
    let second_root = self::root(&second);
    admit(f, second.permission).await.unwrap();
    f.prepare(second.permission.uploader.into(), &second)
        .await
        .unwrap();
    assert_eq!(
        inspect(f, second.permission.uploader.into(), &second_root)
            .await
            .unwrap()
            .permission,
        second.permission
    );
    let reply: CaffeineUploadCertificateResponse = provider_calls::update_provider(
        &f.harness.pic,
        f.service,
        second.permission.uploader.into(),
        ISSUE,
        (second_root.clone(),),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    assert_eq!(reply.blob_hash, second_root);
    assert_eq!(
        f.admission(second.permission).await.state,
        UploadState::Registered
    );
    refuses(f, second.permission.uploader.into(), &second_root).await;
    let usage = f
        .local_status(f.operator, f.operator_scope())
        .await
        .unwrap();
    let total = u128::from(first.permission.upload.bytes + second.permission.upload.bytes);
    assert_eq!(usage.uploads.active_reservations, 0);
    assert_eq!(usage.uploads.physical_bytes, total);
    assert_eq!(usage.uploads.liability_bytes, total);
    let mut third = second.permission;
    third.upload.upload = 3;
    third.upload.object = 4;
    third.upload.root = [99; 32];
    third.upload.bytes = 1;
    assert_eq!(admit(f, third).await, Err(UploadAdmissionFailure::Capacity));
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        usage
    );
    second
}

async fn admit(
    f: &Fixture,
    permission: UploadAdmissionRequest,
) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    f.harness
        .pic
        .update_blob_as(
            f.service,
            f.tenant,
            "blob_admit_upload",
            permission,
            REPLY_BYTES,
        )
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered installed scenario retains lifecycle and byte-preservation assertions while making fallible client operations explicit"
)]
async fn standalone_small_configuration_issues_once_and_retains_uncertainty_across_stop_and_restore()
 {
    let f = Fixture::small(Harness::new().await, fake_principal(4)).await;
    f.enroll(f.operator).await.unwrap();
    let manifest = f.small_manifest();
    let root = root(&manifest);
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
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        inspect(&f, f.uploader, &root).await.unwrap().permission,
        manifest.permission
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    for actor in [f.controller, f.operator, f.tenant, Principal::anonymous()] {
        refuses(&f, actor, &root).await;
    }
    malformed(&f, &root).await;
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    let mut usage = f
        .local_status(f.operator, f.operator_scope())
        .await
        .unwrap();
    let reply: CaffeineUploadCertificateResponse = provider_calls::update_provider(
        &f.harness.pic,
        f.service,
        f.uploader,
        ISSUE,
        (root.clone(),),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    assert_eq!(reply.method, "upload");
    assert_eq!(reply.blob_hash, root);
    assert_eq!(
        f.admission(manifest.permission).await.state,
        UploadState::Registered
    );
    usage.uploads.active_reservations = 0;
    usage.uploads.reserved_bytes = 0;
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        usage
    );
    let exposed = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    refuses(&f, f.uploader, &root).await;
    assert_eq!(inspect(&f, f.uploader, &root).await, Err(E::Phase));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &exposed,
    );
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
    f.resume(f.operator).await.unwrap();
    refuses(&f, f.uploader, &root).await;
    f.harness
        .pic
        .update_blob_as::<Result<UploadRevocationResponse, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_revoke_upload",
            manifest.permission,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
    let retained = f.admission(manifest.permission).await;
    assert!(retained.revoked);
    assert_eq!(retained.state, UploadState::Registered);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        usage
    );
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    assert_eq!(f.admission(manifest.permission).await, retained);
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    refuses(&f, f.uploader, &root).await;
    assert_eq!(
        inspect(&f, f.uploader, &root).await,
        Err(E::Permission(UploadAdmissionFailure::Fenced))
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
    f.resume(f.operator).await.unwrap();
    assert_eq!(f.admission(manifest.permission).await, retained);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap(),
        usage
    );
    refuses(&f, f.uploader, &root).await;
    assert_eq!(inspect(&f, f.uploader, &root).await, Err(E::Revoked));
}

async fn malformed(f: &Fixture, root: &str) {
    // Oversized ingress and type confusion must never create provider reply bytes.
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let assessment = inspect(f, f.uploader, root).await.unwrap();
    for argument in [
        ic_platform::candid::contract::encode_one("x".repeat(4097)).unwrap(),
        ic_platform::candid::contract::encode_one(true).unwrap(),
    ]
    .into_iter()
    .chain(super::standalone_ingress::refusal_arguments(
        &root.to_owned(),
        4096,
    )) {
        for method in [ISSUE, INSPECT] {
            let error = if method == ISSUE {
                f.harness
                    .pic
                    .update_call(f.service, f.uploader, method, argument.clone())
                    .await
                    .unwrap_err()
            } else {
                f.harness
                    .pic
                    .query_call(f.service, f.uploader, method, argument.clone())
                    .await
                    .unwrap_err()
            };
            assert_eq!(
                crate::canister_rejection(&error).reject_code,
                RejectCode::CanisterError
            );
            let current = inspect(f, f.uploader, root).await.unwrap();
            assert_eq!(current.permission, assessment.permission);
            unchanged(
                &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
                &before,
            );
        }
    }
    // The owned assessment no longer accepts Candid subtyping or extra arguments.
    assert!(
        f.harness
            .pic
            .query_call(
                f.service,
                f.uploader,
                INSPECT,
                ic_platform::candid::contract::encode_args((root, "small")).unwrap()
            )
            .await
            .is_err()
    );
    let current = inspect(f, f.uploader, root).await.unwrap();
    assert_eq!(current.permission, assessment.permission);
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
}
