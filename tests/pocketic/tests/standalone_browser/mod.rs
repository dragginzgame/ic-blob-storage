//! Full restricted standalone/browser/native journey with a local provider substitute.
use crate::support::REPLY_LIMITS;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod client;
mod multi_user;
use crate::browser_driver::{BrowserDriver, BrowserPreparation};
use client::Trial;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::identity::ContentDigest;

#[derive(Clone, Copy)]
enum Scenario {
    Verified,
    CorruptRead,
    LostFinal,
    Withdrawn,
}

fn upload(
    trial: &Trial,
    scenario: Scenario,
) -> (BrowserDriver, UploadManifestRequest, client::UploadReport) {
    let input = trial.f.small_manifest();
    let config = serde_json::json!({
        "url": trial.url, "service": trial.f.service.to_text(),
        "rootKey": trial.f.harness.pic.root_key().unwrap(), "project": client::PROJECT,
        "corruptRead": matches!(scenario, Scenario::CorruptRead),
        "lostFinalReply": matches!(scenario, Scenario::LostFinal | Scenario::Withdrawn),
        "withdrawAfterObservation": matches!(scenario, Scenario::Withdrawn),
        "providerRootCertificate": trial.report.join("gateway-ca.pem"),
        "untrustedRootCertificate": matches!(scenario, Scenario::Verified)
            .then(|| trial.report.join("untrusted-ca.pem")),
        "refuseFirstRead": false,
    });
    let mut driver = BrowserDriver::start(&config, "standalone.mjs");
    let plan: BrowserPreparation = driver.read(8192);
    let generated = trial.inputs(&input, &plan, &[42; 1024]);
    trial.setup(&generated);
    driver.send(&trial.transfer_inputs());
    let uploaded: client::UploadReport = driver.read(8192);
    let lost = matches!(scenario, Scenario::LostFinal | Scenario::Withdrawn);
    assert_eq!(uploaded.uploaded, !lost);
    assert_eq!(uploaded.upload_failure.is_some(), lost);
    assert_eq!(
        uploaded.final_request_phase,
        if lost { "uncertain" } else { "responded" }
    );
    assert!(uploaded.recovered);
    assert!(!uploaded.provider_completion);
    assert_eq!(uploaded.root, plan.hash);
    trial.record(
        "browser-upload.json",
        &serde_json::to_value(&uploaded).unwrap(),
    );
    assert_eq!(
        trial.f.admission(input.permission).state,
        UploadState::Registered
    );
    (driver, input, uploaded)
}
fn finish(trial: &Trial, mut driver: BrowserDriver) -> serde_json::Value {
    driver.send(&serde_json::json!({"finish": true}));
    let report = driver.finish();
    assert_eq!(report["outcome"], "passed");
    trial.record("browser-result.json", &report);
    report
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_upload_verification_download_and_release() {
    let mut trial = Trial::new("success");
    let (driver, input, uploaded) = upload(&trial, Scenario::Verified);
    assert!(uploaded.gateway.starts_with("https://127.0.0.1:"));
    let delivered = trial.download(
        &input,
        input.permission.upload.first_reference,
        &uploaded.gateway,
        "download",
        0,
    );
    assert_eq!(
        delivered["content_digest"],
        serde_json::json!(ContentDigest::compute(&[42; 1024]).to_string())
    );
    assert_eq!(
        std::fs::read(trial.report.join("download/body.bin")).unwrap(),
        [42; 1024]
    );
    trial.release(&input);
    let released = trial.download(
        &input,
        input.permission.upload.first_reference,
        &uploaded.gateway,
        "released",
        3,
    );
    assert_eq!(released["error"], "download_unavailable");
    // Logical release retains the provider object and its physical/liability records.
    let status = trial
        .f
        .local_status(trial.f.operator, trial.f.operator_scope())
        .unwrap();
    assert_eq!(status.uploads.logical_bytes, 0);
    assert_eq!(status.uploads.physical_bytes, 1024);
    assert_eq!(status.uploads.liability_bytes, 1024);
    let final_report = finish(&trial, driver);
    trial.record("summary.json", &serde_json::json!({
        "schema": 1, "evidence": "actual_standalone_chromium_native_local_provider_substitute",
        "content_bytes": 1024, "content_digest": serde_json::json!(ContentDigest::compute(&[42; 1024]).to_string()),
        "certificate_issuer": "actual_installed_standalone_facts",
        "browser_journal": "maintained_one_slot_indexeddb",
        "provider_puts": final_report["puts"], "provider_gets": final_report["gets"],
        "trusted_registration": true, "provider_upload_verified": false, "tenant_download_verified": true,
        "reference_released": true, "physical_bytes_retained": 1024, "liability_bytes_retained": 1024,
        "provider_deletion_established": false, "billing_cessation_established": false,
        "live_provider_requests": 0, "paid_effects": 0,
    }));
    trial.f.harness.pic.stop_live();
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_corrupt_provider_download_cannot_confirm_or_free_obligations() {
    let mut trial = Trial::new("corrupt");
    let (driver, input, uploaded) = upload(&trial, Scenario::CorruptRead);
    let refusal = trial.download(
        &input,
        input.permission.upload.first_reference,
        &uploaded.gateway,
        "after-corrupt",
        3,
    );
    assert_eq!(refusal["error"], "content_mismatch");
    assert!(!trial.report.join("after-corrupt/body.bin").exists());
    let revoked: Result<super::UploadRevocationResponse, super::UploadAdmissionFailure> = trial
        .f
        .harness
        .pic
        .update_blob_as(
            trial.f.service,
            trial.f.tenant,
            "blob_revoke_upload",
            input.permission,
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap();
    assert!(revoked.unwrap().admission.revoked);
    let retained = trial.f.admission(input.permission);
    assert!(retained.revoked);
    assert_eq!(retained.state, UploadState::Registered);
    let status = trial
        .f
        .local_status(trial.f.operator, trial.f.operator_scope())
        .unwrap();
    assert_eq!(status.uploads.logical_bytes, 1024);
    assert_eq!(status.uploads.physical_bytes, 1024);
    assert_eq!(status.uploads.liability_bytes, 1024);
    let browser = finish(&trial, driver);
    trial.record(
        "summary.json",
        &serde_json::json!({
            "schema": 1, "evidence": "actual_standalone_chromium_native_corrupt_gateway_substitute",
            "provider_puts": browser["puts"], "provider_gets": browser["gets"],
            "verification_error": "content_mismatch", "registration_is_not_integrity_verification": true,
            "tenant_download_unavailable": true, "permission_revoked": true,
            "browser_cancelled": true, "state": "Registered", "retained_bytes": 1024,
            "provider_deletion_established": false, "billing_cessation_established": false,
            "live_provider_requests": 0, "paid_effects": 0,
        }),
    );
    trial.f.harness.pic.stop_live();
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_registration_preserves_lost_transfer_reply_and_withdrawal_obligations() {
    for scenario in [Scenario::LostFinal, Scenario::Withdrawn] {
        let mut trial = Trial::new(if matches!(scenario, Scenario::Withdrawn) {
            "withdrawn-registration"
        } else {
            "lost-final-registration"
        });
        let (driver, input, uploaded) = upload(&trial, scenario);
        assert_eq!(
            trial.f.admission(input.permission).state,
            UploadState::Registered
        );
        assert_eq!(uploaded.final_request_phase, "uncertain");
        if matches!(scenario, Scenario::Withdrawn) {
            let withdrawn: Result<super::UploadRevocationResponse, super::UploadAdmissionFailure> =
                trial
                    .f
                    .harness
                    .pic
                    .update_blob_as(
                        trial.f.service,
                        trial.f.tenant,
                        "blob_revoke_upload",
                        input.permission,
                        REPLY_LIMITS.reply_bytes,
                    )
                    .unwrap();
            assert!(withdrawn.unwrap().admission.revoked);
        }
        trial.download(
            &input,
            input.permission.upload.first_reference,
            &uploaded.gateway,
            "download",
            0,
        );
        trial.release(&input);
        let status = trial
            .f
            .local_status(trial.f.operator, trial.f.operator_scope())
            .unwrap();
        assert_eq!(status.uploads.logical_bytes, 0);
        assert_eq!(status.uploads.physical_bytes, 1024);
        assert_eq!(status.uploads.liability_bytes, 1024);
        finish(&trial, driver);
        trial.f.harness.pic.stop_live();
    }
}
