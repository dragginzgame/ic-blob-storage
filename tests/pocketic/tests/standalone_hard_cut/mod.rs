//! Actual old allocation bytes must refuse upgrade without losing the old owner.
use super::*;
use ic_blob_storage_contracts::identity::ContentDigest;
use std::path::PathBuf;

// Frozen producer for retired installation contracts, used only to install their
// pinned test images. Production accepts the current DTO exclusively.
pub(super) fn frozen_installation(f: &Fixture) -> Vec<u8> {
    let mut args = candid::IDLArgs::from_bytes(&installation(&f.config)).unwrap();
    let candid::IDLValue::Record(fields) = &mut args.args[0] else {
        panic!("installation record")
    };
    fields.push(candid::types::value::IDLField {
        id: candid::types::Label::Named("trusted_uploader".into()),
        val: candid::IDLValue::Principal(f.uploader),
    });
    fields.sort_by_key(|field| field.id.get_id());
    args.to_bytes().unwrap()
}

// Project only unchanged upload counters from the pinned image's raw reply.
// Production decoders accept exclusively the current mandatory funding schema.
#[derive(candid::CandidType, serde::Deserialize)]
struct UploadAccountingView {
    uploads: ic_blob_storage_contracts::dto::operator::LocalUploadStatus,
}
// Project only the unchanged release field; preserve and compare the entire raw
// configuration reply rather than decode a historical schema as the current DTO.
#[derive(candid::CandidType, serde::Deserialize)]
struct ConfigurationView {
    release: String,
}
fn raw_configuration(f: &Fixture) -> Vec<u8> {
    f.harness
        .pic
        .query_call(
            f.service,
            f.operator,
            "blob_configuration",
            candid::encode_args(()).unwrap(),
        )
        .unwrap()
}
fn raw_status(f: &Fixture) -> Vec<u8> {
    f.harness
        .pic
        .query_call(
            f.service,
            f.operator,
            "blob_local_status",
            candid::encode_one(f.operator_scope()).unwrap(),
        )
        .unwrap()
}

fn pinned_image() -> (serde_json::Value, Vec<u8>, String) {
    let pinned: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/standalone-pre-cut/manifest.json"
    ))
    .unwrap();
    let old = std::fs::read(fixture_path("BLOB_PRE_CUT_STANDALONE_WASM")).unwrap();
    let old_hash = ContentDigest::compute(&old).to_string();
    assert_eq!(
        old_hash,
        format!("sha256:{}", pinned["wasm_sha256"].as_str().unwrap())
    );
    (pinned, old, old_hash)
}

fn obligations(f: &Fixture) -> (UploadAdmissionRequest, UploadAdmissionRequest) {
    standalone_lifecycle::confirm_and_release_locally(f);
    let released = f.small_manifest().permission;
    let mut pending = f.manifest();
    pending.permission.upload.upload = 2;
    pending.permission.upload.object = 4;
    pending.permission.upload.first_reference = 5;
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (pending.permission,),
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &pending).unwrap();
    (released, pending.permission)
}

#[test]
#[ignore = "Requires the pinned pre-cut Wasm and a fresh report directory; run make test-hard-cut"]
fn older_allocation_ledger_upgrade_preserves_bytes_and_obligations_on_refusal() {
    let (pinned, old, old_hash) = pinned_image();
    let report = PathBuf::from(std::env::var_os("BLOB_HARD_CUT_REPORT").unwrap());
    std::fs::create_dir(&report).expect("fresh owned hard-cut report directory");
    // Replace only an empty, local fixture before introducing the old obligations.
    let f = Fixture::new();
    f.harness
        .pic
        .reinstall_canister(f.service, old, frozen_installation(&f), Some(f.controller))
        .unwrap();
    let configuration = raw_configuration(&f);
    let installed = candid::decode_one::<Result<ConfigurationView, HostFailure>>(&configuration)
        .unwrap()
        .unwrap();
    assert_eq!(
        installed.release,
        pinned["compiled_release"].as_str().unwrap()
    );
    let (released, pending) = obligations(&f);
    let raw = raw_status(&f);
    let status = candid::decode_one::<
        Result<UploadAccountingView, ic_blob_storage_contracts::dto::operator::LocalStatusFailure>,
    >(&raw)
    .unwrap()
    .unwrap();
    let held_bytes = 1024 + u128::from(pending.upload.bytes);
    assert_eq!(status.uploads.physical_bytes, held_bytes);
    assert_eq!(status.uploads.liability_bytes, held_bytes);
    assert_eq!(
        status.uploads.reserved_bytes,
        u128::from(pending.upload.bytes)
    );
    let released_before = f.admission(released);
    let pending_before = f.admission(pending);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert!(before.len() <= 32 * 1024 * 1024);
    std::fs::write(report.join("before.bin"), &before).unwrap();
    for (name, bytes) in [
        ("configuration.candid", configuration.clone()),
        ("status.candid", raw.clone()),
        (
            "released-permission.candid",
            candid::encode_one(released).unwrap(),
        ),
        (
            "pending-permission.candid",
            candid::encode_one(pending).unwrap(),
        ),
        (
            "released-admission.candid",
            candid::encode_one(released_before).unwrap(),
        ),
        (
            "pending-admission.candid",
            candid::encode_one(pending_before).unwrap(),
        ),
    ] {
        std::fs::write(report.join(name), bytes).unwrap();
    }
    let target = wasm();
    let target_hash = ContentDigest::compute(&target).to_string();
    let failure = f
        .harness
        .pic
        .upgrade_canister(
            f.service,
            target,
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap_err();
    std::fs::write(report.join("upgrade-refusal.txt"), &failure.reject_message).unwrap();
    assert_eq!(failure.reject_code, RejectCode::CanisterError);
    let after = f.harness.pic.get_stable_memory(f.service);
    std::fs::write(report.join("after.bin"), &after).unwrap();
    unchanged(&after, &before);
    assert_eq!(raw_configuration(&f), configuration);
    assert_eq!(raw_status(&f), raw);
    assert_eq!(f.admission(released), released_before);
    assert_eq!(f.admission(pending), pending_before);
    std::fs::write(
        report.join("summary.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "evidence_class":"local PocketIC; synthetic verifier; no provider effects",
            "old_release":installed.release,"old_memory":pinned["ic_memory"],
            "target_compiled_release":ic_blob_storage::LIBRARY_VERSION,
            "old_wasm":old_hash,"target_wasm":target_hash,
            "stable_before":ContentDigest::compute(&before).to_string(),
            "stable_after":ContentDigest::compute(&after).to_string(),
            "physical_bytes":status.uploads.physical_bytes.to_string(),
            "liability_bytes":status.uploads.liability_bytes.to_string(),
            "reserved_bytes":status.uploads.reserved_bytes.to_string(),
            "upgrade_refused":true,"old_owner_and_exact_admissions_preserved":true,
            "cross_release_upgrade_supported":false
        }))
        .unwrap(),
    )
    .unwrap();
}

#[test]
#[ignore = "Requires the pinned single-uploader Wasm and a fresh report directory"]
fn single_uploader_installation_upgrade_refuses_without_losing_owner_or_obligations() {
    let report = PathBuf::from(std::env::var_os("BLOB_UPLOAD_AUTHORITY_CUT_REPORT").unwrap());
    std::fs::create_dir(&report).expect("fresh owned hard-cut report directory");
    let old = std::fs::read(fixture_path("BLOB_SINGLE_UPLOADER_WASM")).unwrap();
    let hash = ContentDigest::compute(&old).to_string();
    // Frozen 0.18.4 artifact qualified under its original graph/source record.
    assert_eq!(
        hash,
        "sha256:50429e44ecbd974213ad844b51a36aeb61b6a7a9a441f33f2406de9cd6897a4e"
    );
    let f = Fixture::new();
    let init = frozen_installation(&f);
    f.harness
        .pic
        .reinstall_canister(f.service, old, init.clone(), Some(f.controller))
        .unwrap();
    let configuration = raw_configuration(&f);
    let installed = candid::decode_one::<Result<ConfigurationView, HostFailure>>(&configuration)
        .unwrap()
        .unwrap();
    assert_eq!(installed.release, "0.18.4");
    let (released, pending) = obligations(&f);
    let admissions = [f.admission(released), f.admission(pending)];
    let status = raw_status(&f);
    let before = f.harness.pic.get_stable_memory(f.service);
    let failure = f.upgrade(candid::encode_args(()).unwrap()).unwrap_err();
    assert_eq!(failure.reject_code, RejectCode::CanisterError);
    let after = f.harness.pic.get_stable_memory(f.service);
    unchanged(&after, &before);
    assert_eq!(raw_configuration(&f), configuration);
    assert_eq!(raw_status(&f), status);
    assert_eq!([f.admission(released), f.admission(pending)], admissions);
    for (name, bytes) in [
        ("installation.candid", init),
        ("configuration.candid", configuration),
        ("status.candid", status),
        ("before.bin", before.clone()),
        ("after.bin", after.clone()),
    ] {
        std::fs::write(report.join(name), bytes).unwrap();
    }
    std::fs::write(report.join("upgrade-refusal.txt"), failure.reject_message).unwrap();
    std::fs::write(
        report.join("summary.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "evidence_class":"local PocketIC; synthetic verifier; no provider effects",
            "old_compiled_release":installed.release,"old_wasm":hash,
            "old_evidence":"docs/evidence/host071-tooling0185.md",
            "target_compiled_release":ic_blob_storage::LIBRARY_VERSION,
            "target_wasm":ContentDigest::compute(&wasm()).to_string(),
            "stable_before":ContentDigest::compute(&before).to_string(),
            "stable_after":ContentDigest::compute(&after).to_string(),
            "upgrade_refused":true,"old_owner_configuration_and_admissions_preserved":true
        }))
        .unwrap(),
    )
    .unwrap();
}
