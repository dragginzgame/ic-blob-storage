//! Standalone receipt recovery uses independent retained upload identities.
use super::*;
use crate::reference_cli::{command, inspect, intent};
use std::{fs, path::Path};

#[test]
fn standalone_saved_intent_preserves_unknown_unconfirmed_and_restored_refusals() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let mut permission = f.manifest().permission;
    permission.upload.upload = u128::MAX;
    permission.upload.object = u128::MAX - 1;
    permission.upload.incarnation = u128::MAX - 2;
    permission.upload.first_reference = u128::MAX - 3;
    let input = ReferenceCommand {
        upload: permission.upload,
        reference: u128::MAX - 4,
        operation: u128::MAX - 5,
        action: ReferenceAction::Retain,
    };
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
    let observe = || inspect(&f.harness.pic, f.service, f.tenant, saved, 3);
    let before = f.harness.pic.get_stable_memory(f.service);
    let unknown = observe();
    assert_eq!(unknown["observation"]["status"], "service_refusal");
    assert_eq!(unknown["observation"]["failure"], "Unknown");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap();
    admitted.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let unconfirmed = observe();
    assert_eq!(unconfirmed["observation"]["failure"], "Unconfirmed");
    assert_eq!(unconfirmed["intent"]["object"], (u128::MAX - 1).to_string());
    assert_eq!(
        unconfirmed["intent"]["first_reference"],
        (u128::MAX - 3).to_string()
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(observe(), unconfirmed);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, "blob_reference_receipt", bytes)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
    }
}
