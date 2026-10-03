//! Late verified completion retains obligations; release and attestation replay stay separate.
use super::*;
use ic_blob_storage::dto::upload::{
    admission::{UploadAdmissionFailure, UploadRevocationResponse},
    completion::{
        UploadAttestationFailure, UploadAttestationLookup, UploadAttestationMutation,
        UploadAttestationResponse,
    },
};
use ic_testkit::pic::CandidCallExt;

fn retained_bytes(trial: &Trial, logical: u128) {
    let status = trial
        .f
        .local_status(trial.f.operator, trial.f.operator_scope())
        .unwrap();
    assert_eq!(status.uploads.logical_bytes, logical);
    assert_eq!(status.uploads.physical_bytes, 1024);
    assert_eq!(status.uploads.liability_bytes, 1024);
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_lost_chunk_reply_verifies_after_stop_start_without_redispatch() {
    let mut trial = Trial::new("lost-final");
    let (driver, input, uploaded) = upload(&trial, Scenario::LostFinal);
    let exposure = trial.f.admission(input.permission);
    let configuration = trial.f.configuration(trial.f.operator).unwrap();
    retained_bytes(&trial, 1024);
    trial.f.harness.pic.stop_progress();
    trial
        .f
        .harness
        .pic
        .stop_canister(trial.f.service, Some(trial.f.controller))
        .unwrap();
    trial
        .f
        .harness
        .pic
        .start_canister(trial.f.service, Some(trial.f.controller))
        .unwrap();
    trial.f.resume(trial.f.operator).unwrap();
    assert_eq!(
        trial.f.configuration(trial.f.operator).unwrap(),
        configuration
    );
    assert_eq!(trial.f.admission(input.permission), exposure);
    trial.f.harness.pic.auto_progress();
    let observation = trial.observe(&input, &uploaded.gateway, 0);
    assert_eq!(
        observation["content_digest"],
        ContentDigest::compute(&[42; 1024]).to_string()
    );
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
    retained_bytes(&trial, 0);
    let browser = finish(&trial, driver);
    assert_eq!(
        browser["journal"]["gateway"]["requests"][1]["phase"],
        "uncertain"
    );
    trial.record("summary.json", &serde_json::json!({
        "schema":1, "evidence":"actual_standalone_lost_response_local_gateway_substitute",
        "browser_upload_acknowledged":false, "certificate_recovered_without_reissue":true,
        "final_gateway_request_phase":"uncertain", "owner_stop_start_preserved":true,
        "independent_observation":true, "attestation_accepted":true, "tenant_download_verified":true,
        "reference_released":true, "physical_bytes_retained":1024, "liability_bytes_retained":1024,
        "provider_puts":browser["puts"], "provider_gets":browser["gets"],
        "provider_deletion_established":false, "billing_cessation_established":false,
        "live_provider_requests":0, "paid_effects":0,
    }));
    trial.f.harness.pic.stop_live();
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_withdrawal_and_late_completion_preserve_release() {
    let mut trial = Trial::new("withdrawn");
    let (driver, input, uploaded) = upload(&trial, Scenario::Withdrawn);
    let observation = trial.observe(&input, &uploaded.gateway, 0);
    assert_eq!(
        observation["content_digest"],
        ContentDigest::compute(&[42; 1024]).to_string()
    );
    let original = std::fs::read(trial.report.join("observation/statement.candid")).unwrap();
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = trial
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
    assert_eq!(trial.attest(0, "attestation")["outcome"], "accepted");
    let result: Result<UploadAttestationMutation, UploadAttestationFailure> = candid::decode_one(
        &std::fs::read(trial.report.join("observation/attestation/response.candid")).unwrap(),
    )
    .unwrap();
    let receipt = result.unwrap().receipt;
    assert_eq!(candid::encode_one(receipt.request).unwrap(), original);
    let outcome: serde_json::Value = serde_json::from_slice(
        &std::fs::read(trial.report.join("observation/attestation/outcome.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(outcome["outcome"], "accepted");
    assert_eq!(outcome["retry_authorized"], false);
    assert_eq!(
        trial.attest(3, "attestation-repeat-refused")["error"],
        "submission_already_claimed"
    );
    assert_eq!(
        std::fs::read(trial.report.join("observation/statement.candid")).unwrap(),
        original
    );
    let lookup: Result<UploadAttestationResponse, UploadAttestationFailure> = trial
        .f
        .harness
        .pic
        .query_candid_as(
            trial.f.service,
            trial.f.tenant,
            "blob_upload_attestation",
            (input.permission,),
        )
        .unwrap();
    assert_eq!(
        lookup.unwrap().attestation,
        UploadAttestationLookup::Found(receipt)
    );
    let state = trial.f.admission(input.permission);
    assert!(state.revoked);
    assert_eq!(state.state, UploadState::Confirmed);
    trial.release(&input);
    let replay: Result<UploadAttestationMutation, UploadAttestationFailure> = trial
        .f
        .harness
        .pic
        .update_candid_as(
            trial.f.service,
            receipt.verifier,
            "blob_attest_upload",
            (receipt.request,),
        )
        .unwrap();
    let replay = replay.unwrap();
    assert!(!replay.changed);
    assert_eq!(replay.receipt, receipt);
    trial.download(&input, &uploaded.gateway, "after-withdrawal", 3);
    retained_bytes(&trial, 0);
    let browser = finish(&trial, driver);
    assert_eq!(browser["journal"]["cancelled"], true);
    assert_eq!(
        browser["journal"]["gateway"]["requests"][1]["phase"],
        "uncertain"
    );
    trial.record("summary.json", &serde_json::json!({
        "schema":1, "evidence":"actual_standalone_withdrawal_after_verified_observation_local_substitute",
        "browser_upload_acknowledged":false, "whole_body_observed":true, "permission_revoked":true,
        "late_attestation_accepted_for_reconciliation":true, "authoritative_receipt_preserved":true,
        "second_submission_blocked":true, "original_statement_preserved":true,
        "tenant_download_unavailable":true, "browser_cancelled":true,
        "reference_explicitly_released":true, "attestation_replay_leaves_reference_inactive":true,
        "final_gateway_request_phase":"uncertain", "state":"Confirmed", "logical_bytes":0,
        "physical_bytes_retained":1024, "liability_bytes_retained":1024,
        "provider_puts":browser["puts"], "provider_gets":browser["gets"],
        "provider_deletion_established":false, "billing_cessation_established":false,
        "live_provider_requests":0, "paid_effects":0,
    }));
    trial.f.harness.pic.stop_live();
}
