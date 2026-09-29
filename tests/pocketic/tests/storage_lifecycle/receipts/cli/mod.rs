//! A saved local intent recovers maintained durable receipts without dispatch.
use super::*;
use crate::reference_cli::{command, inspect, intent};
use std::{fs, path::Path};

#[test]
fn saved_reference_intent_recovers_history_through_release_settlement_and_restore() {
    let f = Fixture::new();
    let permission = f.exposed();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let retain = reference(permission.request, u128::MAX, u128::MAX - 1, true);
    let input = receipt_request(retain);
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source.json");
    let journal = dir.path().join("journal");
    fs::create_dir(&journal).unwrap();
    intent(&source, input);
    let stored = command(
        &[
            "save".into(),
            "--intent".into(),
            source.to_str().unwrap().into(),
            "--journal".into(),
            journal.to_str().unwrap().into(),
        ],
        0,
    );
    let saved = Path::new(stored["saved"].as_str().unwrap());
    fs::remove_file(source).unwrap();
    let observe = |code| inspect(&f.harness.pic, f.service, f.tenant, saved, code);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(observe(4)["observation"]["status"], "absent");
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.reference(retain).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let recovered = observe(0);
    assert_eq!(recovered["observation"]["status"], "recorded_success");
    assert_eq!(recovered["current_liveness"], "not_assessed");
    assert_eq!(recovered["dispatch"], "not_performed");
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    f.reference(reference(permission.request, 2, retain.reference, false))
        .unwrap();
    f.reference(reference(permission.request, 3, 1, false))
        .unwrap();
    f.fact(permission.request, ProviderFact::Deleted).unwrap();
    f.fact(permission.request, ProviderFact::Settled).unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(observe(0), recovered);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.reference(retain), Err(ReferenceFailure::Fenced));
}

#[test]
fn reference_cli_distinguishes_recorded_failure_from_conflict_and_wrong_caller() {
    let f = Fixture::new();
    let permission = f.exposed();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let release = reference(permission.request, u128::MAX, 9, false);
    f.reference(release).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("failed.json");
    let input = receipt_request(release);
    intent(&path, input);
    let before = f.harness.pic.get_stable_memory(f.service);
    let observed = inspect(&f.harness.pic, f.service, f.tenant, &path, 4);
    assert_eq!(observed["observation"]["status"], "recorded_failure");
    assert_eq!(observed["observation"]["failure"], "UnknownReference");
    intent(
        &path,
        ReferenceCommand {
            action: ReferenceAction::Retain,
            ..input
        },
    );
    let refused = inspect(&f.harness.pic, f.service, f.tenant, &path, 3);
    assert_eq!(refused["observation"]["status"], "service_refusal");
    assert_eq!(refused["observation"]["failure"], "Conflict");
    assert_eq!(
        inspect(&f.harness.pic, f.service, f.operator, &path, 3)["error"],
        "binding_mismatch"
    );
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, REFERENCE_RECEIPT_METHOD, bytes)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}
