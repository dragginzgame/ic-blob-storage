//! Distinct Chromium signing identities share one service and retain separate references.
use super::{BrowserDriver, Trial, UploadState};
use crate::Envelope;
use crate::browser_driver::BrowserPreparation;
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::identity::ContentDigest;
use serde_json::json;
use std::path::Path;

// Fixed test-only seed [45; 32], distinct from project [43] and verifier [44].
const SECOND_UPLOADER_PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIC0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0t\n-----END PRIVATE KEY-----\n";

fn user_directory(trial: &mut Trial, parent: &Path, seed: u8) {
    trial.report = parent.join(format!("user-{seed}"));
    std::fs::create_dir(&trial.report).unwrap();
    for name in ["tenant.pem", "verifier.pem", "root.der", "uploader.pem"] {
        std::fs::copy(parent.join(name), trial.report.join(name)).unwrap();
    }
    if seed == 45 {
        std::fs::write(trial.report.join("uploader.pem"), SECOND_UPLOADER_PEM).unwrap();
    }
    trial.f.uploader = BasicIdentity::from_raw_key(&[seed; 32]).sender().unwrap();
}

fn upload(trial: &Trial, seed: u8, byte: u8, id: u128) -> UploadManifestRequest {
    let body = [byte; 1024];
    let mut input = trial.f.manifest_body(&body, "image/png", None);
    input.permission.upload.upload = id;
    input.permission.upload.object = id;
    let config = json!({
        "url": trial.url, "service": trial.f.service.to_text(),
        "rootKey": trial.f.harness.pic.root_key().unwrap(), "project": super::client::PROJECT,
        "identitySeed": seed, "otherUploaderSeed": if seed == 42 { 45 } else { 42 },
        "bodyByte": byte, "providerRootCertificate": trial.report.join("gateway-ca.pem"),
    });
    let mut driver = BrowserDriver::start(&config, "standalone.mjs");
    let plan: BrowserPreparation = driver.read(8192);
    let generated = trial.inputs(&input, &plan, &body);
    trial.setup(&generated);
    driver.send(&trial.transfer_inputs());
    let uploaded: super::client::UploadReport = driver.read(8192);
    assert!(uploaded.uploaded && uploaded.recovered);
    assert!(!uploaded.provider_completion);
    assert_eq!(uploaded.root, plan.hash);
    assert_eq!(uploaded.puts.len(), 2);
    assert_eq!(
        trial.f.admission(input.permission).state,
        UploadState::ExposurePossible
    );
    trial.download(
        &input,
        input.permission.upload.first_reference,
        &uploaded.gateway,
        "unconfirmed",
        3,
    );
    let observed = trial.observe(&input, &uploaded.gateway, 0);
    assert_eq!(
        observed["content_digest"],
        ContentDigest::compute(&body).to_string()
    );
    assert_eq!(trial.attest(0, "attestation")["outcome"], "accepted");
    let delivered = trial.download(
        &input,
        input.permission.upload.first_reference,
        &uploaded.gateway,
        "download",
        0,
    );
    assert_eq!(delivered["content_digest"], observed["content_digest"]);
    assert_eq!(
        std::fs::read(trial.report.join("download/body.bin")).unwrap(),
        body
    );
    driver.send(&json!({"finish": true}));
    let browser = driver.finish();
    assert_eq!(browser["outcome"], "passed");
    assert_eq!(browser["puts"].as_array().unwrap().len(), 2);
    assert_eq!(browser["gets"].as_array().unwrap().len(), 2);
    trial.record("browser-result.json", &browser);
    trial.record(
        "summary.json",
        &json!({
            "uploader": input.permission.uploader.to_text(), "root": uploaded.root,
            "body_sha256": observed["content_digest"], "project_grant_signed": true,
            "browser_identity_conflict_refused_before_ingress": true,
            "independent_verification": true, "tenant_download_verified": true,
            "provider": "local HTTP/2 substitute",
        }),
    );
    input
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_two_users_verify_distinct_uploads_and_release_independently() {
    let mut trial = Trial::with_envelope("two-users", Envelope::Regular);
    let parent = trial.report.clone();
    let files = [(42, 42, 1), (45, 43, 2)].map(|(seed, byte, id)| {
        user_directory(&mut trial, &parent, seed);
        upload(&trial, seed, byte, id)
    });
    assert_ne!(files[0].permission.uploader, files[1].permission.uploader);
    assert_ne!(
        files[0].permission.upload.root,
        files[1].permission.upload.root
    );
    let both = trial
        .f
        .local_status(trial.f.operator, trial.f.operator_scope())
        .unwrap();
    assert_eq!(both.uploads.logical_bytes, 2048);
    trial.report = parent.join("user-42");
    trial.release(&files[0]);
    let one = trial
        .f
        .local_status(trial.f.operator, trial.f.operator_scope())
        .unwrap();
    assert_eq!(one.uploads.logical_bytes, 1024);
    assert_eq!(one.uploads.physical_bytes, 2048);
    assert_eq!(one.uploads.liability_bytes, 2048);
    assert_eq!(
        trial.f.admission(files[1].permission).state,
        UploadState::Confirmed
    );
    trial.report = parent.join("user-45");
    trial.release(&files[1]);
    let released = trial
        .f
        .local_status(trial.f.operator, trial.f.operator_scope())
        .unwrap();
    assert_eq!(released.uploads.logical_bytes, 0);
    assert_eq!(released.uploads.physical_bytes, 2048);
    assert_eq!(released.uploads.liability_bytes, 2048);
    trial.report = parent;
    trial.record(
        "summary.json",
        &json!({
            "evidence_class": "actual_standalone_chromium_native_local_provider_substitute",
            "service": trial.f.service.to_text(), "tenant": trial.f.tenant.to_text(),
            "distinct_signed_browser_uploaders": 2, "distinct_roots": 2,
            "provider_puts": 4, "provider_gets": 4,
            "releasing_first_preserves_second_logical_bytes": one.uploads.logical_bytes,
            "final_logical_bytes": released.uploads.logical_bytes,
            "retained_physical_bytes": released.uploads.physical_bytes,
            "retained_liability_bytes": released.uploads.liability_bytes,
            "application_login_or_deployed_provider_qualified": false,
        }),
    );
}
