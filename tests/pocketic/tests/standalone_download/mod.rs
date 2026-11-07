//! Operational descriptor delivery uses the installed scope and the shared live-reference gate.
//! Verified serving and withheld delivery are exercised by standalone browser journeys.
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt as _;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod native;
use super::*;
use ic_blob_storage_contracts::dto::download::DownloadFailure as F;
use ic_blob_storage_contracts::dto::download::DownloadRequest;
use ic_blob_storage_contracts::dto::download::DownloadResponse;
use ic_blob_storage_contracts::protocol::DOWNLOAD_METHOD;

impl Fixture {
    async fn download(
        &self,
        actor: Principal,
        request: DownloadRequest,
    ) -> Result<DownloadResponse, F> {
        self.harness
            .pic
            .update_blob_as(self.service, actor, DOWNLOAD_METHOD, request, REPLY_BYTES)
            .await
            .unwrap()
    }
}
fn request(upload: ReferenceUpload) -> DownloadRequest {
    DownloadRequest {
        service: upload.service,
        tenant: upload.tenant,
        namespace: upload.namespace,
        root: upload.root,
        object: upload.object,
        incarnation: upload.incarnation,
        reference: upload.first_reference,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_download_authenticates_scope_bounds_ingress_and_requires_replicated_delivery() {
    let f = Fixture::new().await;
    let input = request(f.manifest().permission.upload);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(f.download(f.tenant, input).await, Err(F::Inactive));
    for actor in [f.operator, f.controller, f.uploader, Principal::anonymous()] {
        assert_eq!(f.download(actor, input).await, Err(F::Denied));
    }
    for input in [
        DownloadRequest {
            service: f.operator.into(),
            ..input
        },
        DownloadRequest {
            namespace: 1,
            ..input
        },
    ] {
        assert_eq!(f.download(f.tenant, input).await, Err(F::Binding));
    }
    for input in [
        DownloadRequest {
            namespace: 0,
            ..input
        },
        DownloadRequest { object: 0, ..input },
        DownloadRequest {
            incarnation: 0,
            ..input
        },
        DownloadRequest {
            reference: 0,
            ..input
        },
    ] {
        assert_eq!(f.download(f.tenant, input).await, Err(F::Invalid));
    }
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let error = f
            .harness
            .pic
            .update_call(f.service, f.tenant, DOWNLOAD_METHOD, bytes)
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
    }
    let query = f
        .harness
        .pic
        .query_call(
            f.service,
            f.tenant,
            DOWNLOAD_METHOD,
            ic_blob_storage_contracts::wire::encode_request("blob_install", input).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&query).reject_code,
        RejectCode::CanisterError
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_download_never_serves_prepared_content_and_keeps_restore_fence() {
    let f = Fixture::new().await;
    let enrolled = f.enroll(f.operator).await.unwrap();
    let mut manifest = f.manifest();
    manifest.permission.upload.object = u128::MAX;
    manifest.permission.upload.incarnation = u128::MAX - 1;
    manifest.permission.upload.first_reference = u128::MAX - 2;
    let input = request(manifest.permission.upload);
    assert_eq!(f.download(f.tenant, input).await, Err(F::Unavailable));
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.tenant,
            "blob_admit_upload",
            manifest.permission,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    admitted.unwrap();
    assert_eq!(f.download(f.tenant, input).await, Err(F::Unavailable));
    f.prepare(f.uploader, &manifest).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(f.download(f.tenant, input).await, Err(F::Unavailable));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_tenant_as(
            f.service,
            f.operator,
            TenantUpdateRequest {
                scope: f.scope(),
                expected: enrolled.enrollment,
                active: false,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap();
    suspended.unwrap();
    assert_eq!(f.download(f.tenant, input).await, Err(F::Inactive));
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(f.configuration(f.operator).await.unwrap().project, PROJECT);
    assert_eq!(f.download(f.tenant, input).await, Err(F::Fenced));
    assert_eq!(f.download(f.operator, input).await, Err(F::Denied));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_project_validation_rolls_back_installation_and_rejects_corrupt_restore() {
    let f = Fixture::new().await;
    let enrolled = f.enroll(f.operator).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for project in [
        String::new(),
        " project".into(),
        "project ".into(),
        "a\nb".into(),
        "é".repeat(129),
    ] {
        let input = ic_blob_storage_contracts::wire::encode_request(
            "blob_install",
            ServiceInstallationInput {
                configuration: f.config,
                project,
            },
        )
        .unwrap();
        let error = f
            .harness
            .pic
            .reinstall_canister(f.service, wasm(), input, Some(f.controller))
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
        assert_eq!(f.tenant().await, enrolled);
        assert_eq!(f.configuration(f.operator).await.unwrap().project, PROJECT);
        unchanged(
            &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            &before,
        );
    }
    // Corrupt the current schema's project in emulator memory, preserving its byte width.
    let offsets: Vec<_> = before
        .windows(PROJECT.len())
        .enumerate()
        .filter_map(|(offset, value)| (value == PROJECT.as_bytes()).then_some(offset))
        .collect();
    let [offset] = offsets.as_slice() else {
        panic!("expected one installation project");
    };
    let mut corrupted = before;
    corrupted[*offset] = b'\n';
    f.harness
        .pic
        .set_stable_memory(
            f.service,
            corrupted.clone(),
            ic_testkit::pocket_ic::common::rest::BlobCompression::NoCompression,
        )
        .await
        .unwrap();
    let error = f
        .upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &corrupted,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_download_client_propagates_inactive_unconfirmed_and_restored_refusals() {
    use blob_test_protocol::storage::read::{DownloadClientInput, DownloadProbeFailure};
    let mut f = Fixture::new().await;
    let client = f.harness.pic.create_canister().await.unwrap();
    f.harness
        .pic
        .install_canister(
            client,
            std::fs::read(fixture_path("BLOB_STORAGE_PROBE_WASM")).unwrap(),
            blob_test_protocol::fixture_wire::encode_one(
                "fixture_init",
                blob_test_protocol::storage::resources::StorageProbeInstallation {
                    operator: f.operator,
                    resources: blob_test_protocol::storage::resources::small_resources(2, 2, 10),
                    max_funding_attempts: 4,
                },
            )
            .unwrap(),
            None,
        )
        .await
        .unwrap();
    f.tenant = client;
    let input = DownloadClientInput {
        request: request(f.manifest().permission.upload),
        tenant: client.into(),
        project: PROJECT.into(),
        max_reply_bytes: 4096,
    };
    let fetch = async || -> Result<DownloadResponse, DownloadProbeFailure> {
        f.harness
            .pic
            .update_control_as(
                client,
                f.operator,
                "fixture_fetch_descriptor",
                (input.clone(),),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    };
    let client_before = f.harness.pic.get_stable_memory(client).await.unwrap();
    assert_eq!(
        fetch().await,
        Err(DownloadProbeFailure::Remote(F::Inactive))
    );
    f.enroll(f.operator).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        fetch().await,
        Err(DownloadProbeFailure::Remote(F::Unavailable))
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(fetch().await, Err(DownloadProbeFailure::Remote(F::Fenced)));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
    unchanged(
        &f.harness.pic.get_stable_memory(client).await.unwrap(),
        &client_before,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_project_at_utf8_byte_limit_survives_current_schema_restore() {
    let f = Fixture::new().await;
    let project = "é".repeat(128);
    let installation = ServiceInstallationInput {
        configuration: f.config,
        project: project.clone(),
    };
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            ic_blob_storage_contracts::wire::encode_request("blob_install", installation).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap();
    let installed = f.configuration(f.operator).await.unwrap();
    assert_eq!(installed.project, project);
    assert_eq!(installed.configuration, f.config);
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        f.configuration(f.operator).await.unwrap(),
        HostConfigurationView {
            fenced: true,
            ..installed
        }
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
}
