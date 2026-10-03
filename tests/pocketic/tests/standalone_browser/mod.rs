//! Full restricted standalone/browser/native journey with a local provider substitute.
mod client;
mod interruption;
mod serial;
use crate::browser_driver::{BrowserDriver, BrowserPreparation};
use client::Trial;
use ic_blob_storage::{
    dto::upload::{UploadState, manifest::UploadManifestRequest},
    model::identity::ContentDigest,
};

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
        "refuseFirstRead": matches!(scenario, Scenario::Verified),
    });
    let mut driver = BrowserDriver::start(&config, "standalone.mjs");
    let plan: BrowserPreparation = driver.read(8192);
    let generated = trial.inputs(&input, &plan);
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
        UploadState::ExposurePossible
    );
    // HTTP success cannot expose a tenant download before independent verification.
    trial.download(&input, &uploaded.gateway, "unconfirmed", 3);
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
    let refused = trial.observe_with_untrusted_root(&input, &uploaded.gateway);
    assert_eq!(refused["error"], "transport");
    assert!(
        !trial
            .report
            .join("untrusted-observation/statement.candid")
            .exists()
    );
    assert_eq!(
        trial.f.admission(input.permission).state,
        UploadState::ExposurePossible
    );
    let interrupted = trial.observe_refused_stream(&input, &uploaded.gateway);
    assert_eq!(interrupted["error"], "transport");
    assert!(
        !trial
            .report
            .join("refused-observation/statement.candid")
            .exists()
    );
    let observation = trial.observe(&input, &uploaded.gateway, 0);
    assert_eq!(observation["bytes"], "1024");
    assert_eq!(
        observation["content_digest"],
        ContentDigest::compute(&[42; 1024]).to_string()
    );
    assert_eq!(observation["attestation_dispatched"], false);
    assert_eq!(trial.attest(0, "attestation")["outcome"], "accepted");
    assert_eq!(
        trial.f.admission(input.permission).state,
        UploadState::Confirmed
    );
    let delivered = trial.download(&input, &uploaded.gateway, "download", 0);
    assert_eq!(delivered["content_digest"], observation["content_digest"]);
    assert_eq!(
        std::fs::read(trial.report.join("download/body.bin")).unwrap(),
        [42; 1024]
    );
    trial.release(&input);
    trial.download(&input, &uploaded.gateway, "released", 3);
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
        "content_bytes": 1024, "content_digest": observation["content_digest"],
        "certificate_issuer": "actual_installed_standalone_facts",
        "browser_journal": "maintained_one_slot_indexeddb",
        "provider_puts": final_report["puts"], "provider_gets": final_report["gets"],
        "independent_observation": true, "attestation_accepted": true, "tenant_download_verified": true,
        "reference_released": true, "physical_bytes_retained": 1024, "liability_bytes_retained": 1024,
        "provider_deletion_established": false, "billing_cessation_established": false,
        "live_provider_requests": 0, "paid_effects": 0,
    }));
    trial.f.harness.pic.stop_live();
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_corrupt_provider_download_cannot_confirm_or_free_obligations() {
    use ic_testkit::pic::CandidCallExt;
    let mut trial = Trial::new("corrupt");
    let (driver, input, uploaded) = upload(&trial, Scenario::CorruptRead);
    assert_eq!(
        trial.observe(&input, &uploaded.gateway, 3)["error"],
        "content_mismatch"
    );
    for path in [
        "observation/statement.candid",
        "observation/summary.json",
        "observation/attestation",
    ] {
        assert!(!trial.report.join(path).exists());
    }
    trial.download(&input, &uploaded.gateway, "after-corrupt", 3);
    let revoked: Result<super::UploadRevocationResponse, super::UploadAdmissionFailure> = trial
        .f
        .harness
        .pic
        .update_candid_as(
            trial.f.service,
            trial.f.tenant,
            "blob_revoke_upload",
            (input.permission,),
        )
        .unwrap();
    assert!(revoked.unwrap().admission.revoked);
    let retained = trial.f.admission(input.permission);
    assert!(retained.revoked);
    assert_eq!(retained.state, UploadState::ExposurePossible);
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
            "verification_error": "content_mismatch", "attestation_created": false,
            "tenant_download_unavailable": true, "permission_revoked": true,
            "browser_cancelled": true, "state": "ExposurePossible", "retained_bytes": 1024,
            "provider_deletion_established": false, "billing_cessation_established": false,
            "live_provider_requests": 0, "paid_effects": 0,
        }),
    );
    trial.f.harness.pic.stop_live();
}
