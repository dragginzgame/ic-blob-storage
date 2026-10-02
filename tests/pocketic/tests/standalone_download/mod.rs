//! Operational descriptor delivery uses the installed scope and the shared live-reference gate.
//! Successful confirmed serving is covered by labelled storage fixtures until completion is wired.
mod native;
use super::*;
use ic_blob_storage::{
    dto::download::{DownloadFailure as F, DownloadRequest, DownloadResponse},
    ops::service::reads::download::DOWNLOAD_METHOD,
};

impl Fixture {
    fn download(&self, actor: Principal, request: DownloadRequest) -> Result<DownloadResponse, F> {
        self.harness
            .pic
            .update_candid_as(self.service, actor, DOWNLOAD_METHOD, (request,))
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

#[test]
fn standalone_download_authenticates_scope_bounds_ingress_and_requires_replicated_delivery() {
    let f = Fixture::new();
    let input = request(f.manifest().permission.upload);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.download(f.tenant, input), Err(F::Inactive));
    for actor in [f.operator, f.controller, f.uploader, Principal::anonymous()] {
        assert_eq!(f.download(actor, input), Err(F::Denied));
    }
    for input in [
        DownloadRequest {
            service: f.operator,
            ..input
        },
        DownloadRequest {
            namespace: 1,
            ..input
        },
    ] {
        assert_eq!(f.download(f.tenant, input), Err(F::Binding));
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
        assert_eq!(f.download(f.tenant, input), Err(F::Invalid));
    }
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let error = f
            .harness
            .pic
            .update_call(f.service, f.tenant, DOWNLOAD_METHOD, bytes)
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
    }
    let query = f
        .harness
        .pic
        .query_call(
            f.service,
            f.tenant,
            DOWNLOAD_METHOD,
            candid::encode_one(input).unwrap(),
        )
        .unwrap_err();
    assert_eq!(query.reject_code, RejectCode::CanisterError);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}

#[test]
fn standalone_download_never_serves_prepared_content_and_keeps_restore_fence() {
    let f = Fixture::new();
    let enrolled = f.enroll(f.operator).unwrap();
    let mut manifest = f.manifest();
    manifest.permission.upload.object = u128::MAX;
    manifest.permission.upload.incarnation = u128::MAX - 1;
    manifest.permission.upload.first_reference = u128::MAX - 2;
    let input = request(manifest.permission.upload);
    assert_eq!(f.download(f.tenant, input), Err(F::Unavailable));
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (manifest.permission,),
        )
        .unwrap();
    admitted.unwrap();
    assert_eq!(f.download(f.tenant, input), Err(F::Unavailable));
    f.prepare(f.uploader, &manifest).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.download(f.tenant, input), Err(F::Unavailable));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope: f.scope(),
                expected: enrolled.enrollment,
                active: false,
            },),
        )
        .unwrap();
    suspended.unwrap();
    assert_eq!(f.download(f.tenant, input), Err(F::Inactive));
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.configuration(f.operator).unwrap().project, PROJECT);
    assert_eq!(f.download(f.tenant, input), Err(F::Fenced));
    assert_eq!(f.download(f.operator, input), Err(F::Denied));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}

#[test]
fn standalone_project_validation_rolls_back_installation_and_rejects_corrupt_restore() {
    let f = Fixture::new();
    let enrolled = f.enroll(f.operator).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    for project in [
        String::new(),
        " project".into(),
        "project ".into(),
        "a\nb".into(),
        "é".repeat(129),
    ] {
        let input = candid::encode_one(ServiceInstallationInput {
            configuration: f.config,
            project,
            completion_verifier: Fake::principal(90),
            trusted_uploader: Fake::principal(4),
        })
        .unwrap();
        let error = f
            .harness
            .pic
            .reinstall_canister(f.service, wasm(), input, Some(f.controller))
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert_eq!(f.tenant(), enrolled);
        assert_eq!(f.configuration(f.operator).unwrap().project, PROJECT);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
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
    f.harness.pic.set_stable_memory(
        f.service,
        corrupted.clone(),
        ic_testkit::pocket_ic::common::rest::BlobCompression::NoCompression,
    );
    let error = f.upgrade(candid::encode_args(()).unwrap()).unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &corrupted);
}

#[test]
fn standalone_download_client_propagates_inactive_unconfirmed_and_restored_refusals() {
    use blob_test_protocol::storage::read::{DownloadClientInput, DownloadProbeFailure};
    let mut f = Fixture::new();
    let client = f.harness.pic.create_canister();
    f.harness.pic.install_canister(
        client,
        std::fs::read(fixture_path("BLOB_STORAGE_PROBE_WASM")).unwrap(),
        candid::encode_one(f.operator).unwrap(),
        None,
    );
    f.tenant = client;
    let input = DownloadClientInput {
        request: request(f.manifest().permission.upload),
        tenant: client,
        project: PROJECT.into(),
        max_reply_bytes: 4096,
    };
    let fetch = || -> Result<DownloadResponse, DownloadProbeFailure> {
        f.harness
            .pic
            .update_candid_as(
                client,
                f.operator,
                "fixture_fetch_descriptor",
                (input.clone(),),
            )
            .unwrap()
    };
    let client_before = f.harness.pic.get_stable_memory(client);
    assert_eq!(fetch(), Err(DownloadProbeFailure::Remote(F::Inactive)));
    f.enroll(f.operator).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(fetch(), Err(DownloadProbeFailure::Remote(F::Unavailable)));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(fetch(), Err(DownloadProbeFailure::Remote(F::Fenced)));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
    unchanged(&f.harness.pic.get_stable_memory(client), &client_before);
}

#[test]
fn standalone_project_at_utf8_byte_limit_survives_current_schema_restore() {
    let f = Fixture::new();
    let project = "é".repeat(128);
    let installation = ServiceInstallationInput {
        configuration: f.config,
        project: project.clone(),
        completion_verifier: Fake::principal(90),
        trusted_uploader: Fake::principal(4),
    };
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            candid::encode_one(installation).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    let installed = f.configuration(f.operator).unwrap();
    assert_eq!(installed.project, project);
    assert_eq!(installed.configuration, f.config);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.configuration(f.operator).unwrap(),
        HostConfigurationView {
            fenced: true,
            ..installed
        }
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
