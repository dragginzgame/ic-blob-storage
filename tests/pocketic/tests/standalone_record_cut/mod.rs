//! The preceding released installation refuses the new image without losing obligations.
use super::*;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_pocketic_tests::provider_calls;

// The released 0.51 Wire input has a retired read-budget field. Keep it only
// for installing a real pre-cut image in refusal tests, never as a fallback reader.
pub(super) fn prior_installation(config: &ServiceConfigurationInput) -> Vec<u8> {
    #[derive(serde::Serialize)]
    struct PriorInstallation {
        configuration: ic_blob_storage_pocketic_tests::historical::ServiceConfigurationInput,
        project: String,
    }
    ic_wire::native::encode_request(
        0x424c_4f42_0042_ff01,
        &PriorInstallation {
            configuration: ic_blob_storage_pocketic_tests::historical::project(config),
            project: PROJECT.into(),
        },
        ic_blob_storage_contracts::wire::request_limits("blob_install").unwrap(),
    )
    .unwrap()
}

#[test]
#[ignore = "Requires the retained 0.51 Wire-record standalone Wasm and its SHA-256"]
fn wire_read_session_installation_refuses_current_upgrade_without_resetting_obligations() {
    let old = std::fs::read(fixture_path("BLOB_PRIOR_READS_WASM")).unwrap();
    assert_eq!(
        ContentDigest::compute(&old).to_string(),
        std::env::var("BLOB_PRIOR_READS_SHA256").unwrap()
    );
    assert_ne!(old, wasm());
    let f = Fixture::new();
    install_code::settle_install_code_debt(&f.harness.pic);
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            old,
            prior_installation(&f.config),
            Some(f.controller),
        )
        .unwrap();
    f.enroll(f.operator).unwrap();
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
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &manifest).unwrap();
    let _: (CaffeineUploadCertificateResponse,) = provider_calls::update_provider(
        &f.harness.pic,
        f.service,
        f.uploader,
        ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
        (super::standalone_certificate::root(&manifest),),
    )
    .unwrap();
    let admission = f.admission(manifest.permission);
    assert_eq!(admission.state, UploadState::Registered);
    let tenant = f.tenant();
    let before = f.harness.pic.get_stable_memory(f.service);
    install_code::settle_install_code_debt(&f.harness.pic);
    let error = f
        .upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    // A rejected upgrade leaves the original owner and its exact obligations usable.
    assert_eq!(f.admission(manifest.permission), admission);
    assert_eq!(f.tenant(), tenant);
}

#[test]
#[ignore = "Requires retained pre-cell standalone Wasm and its SHA-256"]
fn singleton_map_installation_refuses_cell_upgrade_without_resetting_obligations() {
    let old = std::fs::read(fixture_path("BLOB_PRE_CELL_WASM")).unwrap();
    assert_eq!(
        ContentDigest::compute(&old).to_string(),
        std::env::var("BLOB_PRE_CELL_SHA256").unwrap()
    );
    assert_ne!(old, wasm());
    let f = Fixture::new();
    install_code::settle_install_code_debt(&f.harness.pic);
    f.harness
        .pic
        .reinstall_canister(f.service, old, installation(&f.config), Some(f.controller))
        .unwrap();
    f.enroll(f.operator).unwrap();
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
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &manifest).unwrap();
    let _: (CaffeineUploadCertificateResponse,) = provider_calls::update_provider(
        &f.harness.pic,
        f.service,
        f.uploader,
        ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
        (super::standalone_certificate::root(&manifest),),
    )
    .unwrap();
    let admission = f.admission(manifest.permission);
    let configuration = f.configuration(f.operator).unwrap();
    let status = f.local_status(f.operator, f.operator_scope()).unwrap();
    assert_eq!(admission.state, UploadState::Registered);
    assert!(status.uploads.physical_bytes > 0 && status.uploads.liability_bytes > 0);
    let before = f.harness.pic.get_stable_memory(f.service);
    install_code::settle_install_code_debt(&f.harness.pic);
    let error = f
        .upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert_eq!(f.admission(manifest.permission), admission);
    assert_eq!(f.configuration(f.operator).unwrap(), configuration);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        status
    );
}
