//! A real retained Candid store refuses the Wire image without losing its owner.
use super::*;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_pocketic_tests::provider_calls;

#[test]
#[ignore = "Requires a retained pre-cut Candid-record standalone Wasm and its SHA-256"]
fn candid_records_refuse_current_upgrade_and_keep_obligations_and_old_owner() {
    let old = std::fs::read(fixture_path("BLOB_CANDID_RECORDS_WASM")).unwrap();
    assert_eq!(
        ContentDigest::compute(&old).to_string(),
        std::env::var("BLOB_CANDID_RECORDS_SHA256").expect("explicit pre-cut artifact hash")
    );
    assert_ne!(old, wasm(), "actual stored-format cut required");
    let f = Fixture::new();
    // Replace only the empty local fixture, before creating any obligation.
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
    let _: CaffeineUploadCertificateResponse = provider_calls::update_provider(
        &f.harness.pic,
        f.service,
        f.uploader,
        ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD,
        (super::standalone_certificate::root(&manifest),),
    )
    .map(|(reply,)| reply)
    .unwrap();
    let configuration = f.configuration(f.operator).unwrap();
    let tenant = f.tenant();
    let admission = f.admission(manifest.permission);
    assert_eq!(admission.state, UploadState::Registered);
    let status = f.local_status(f.operator, f.operator_scope()).unwrap();
    assert!(status.uploads.physical_bytes > 0 && status.uploads.liability_bytes > 0);
    let before = f.harness.pic.get_stable_memory(f.service);
    install_code::settle_install_code_debt(&f.harness.pic);
    let error = f
        .upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert!(error.reject_message.contains("current Wire stable record"));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert_eq!(f.configuration(f.operator).unwrap(), configuration);
    assert_eq!(f.tenant(), tenant);
    assert_eq!(f.admission(manifest.permission), admission);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        status
    );
}
