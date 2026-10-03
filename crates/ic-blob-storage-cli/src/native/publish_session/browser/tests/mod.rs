use super::*;
use crate::native::{publish_check::tests as fixture, publish_prepare};
use serde_json::json;
use std::num::NonZeroU64;

#[test]
fn selection_binds_fresh_paths_batch_and_original_recovery_without_allocating_output() {
    let directory = fixture::frozen();
    let batch = fixture::batch(&directory);
    let path = directory.path().join("browser.json");
    let mut input = Input {
        prepare: publish_prepare::Input {
            service: batch.files[0].input.permission.upload.service,
            namespace: 1,
            index: 0,
            max_bytes: NonZeroU64::new(10).unwrap(),
            max_total_bytes: NonZeroU64::new(20).unwrap(),
            timeout_seconds: 30,
            inputs: directory.path().join("batch"),
            directory: directory.path().join("session"),
            uploader_identity: directory.path().join("key"),
            source: None,
        },
        operator_identity: directory.path().join("key"),
        verifier_identity: None,
        source_session: None,
        browser_selection: Some(path.clone()),
        gateway: "https://example.com/".parse().unwrap(),
        max_steps: 9,
    };
    let selected = json!({"format":"ic-blob-storage/browser-selection", "session":input.prepare.directory,
        "profile":directory.path().join("profile"),"project":batch.files[0].input.project(),
        "bucket":batch.files[0].input.bucket(),"asset_port":45000,"signer_sha256":"1".repeat(64),
        "host_sha256":"2".repeat(64),"worker_sha256":"3".repeat(64),"database":"selected", "max_slots":1});
    let save = |value: &serde_json::Value| {
        std::fs::write(&path, serde_json::to_vec(value).unwrap()).unwrap();
    };
    save(&selected);
    assert!(selection(&input, &batch).unwrap().is_some());
    assert!(!input.prepare.directory.exists());
    for (key, value) in [
        ("format", json!("unknown")),
        ("asset_port", json!(0)),
        ("signer_sha256", json!("not-a-hash")),
        ("database", json!("")),
        ("max_slots", json!(0)),
        ("project", json!("foreign")),
        ("bucket", json!("foreign")),
        ("session", json!(directory.path().join("other"))),
        ("profile", json!(input.prepare.directory)),
    ] {
        let mut changed = selected.clone();
        changed[key] = value;
        save(&changed);
        assert!(matches!(selection(&input, &batch), Err(Failure::Binding)));
    }
    save(&selected);
    std::fs::create_dir(directory.path().join("profile")).unwrap();
    assert!(matches!(selection(&input, &batch), Err(Failure::Binding)));
    input.source_session = Some(input.prepare.directory.clone());
    assert!(selection(&input, &batch).is_err());
    std::fs::create_dir(&input.prepare.directory).unwrap();
    input.prepare.directory = directory.path().join("resumed");
    assert!(selection(&input, &batch).unwrap().is_some());
    assert!(!input.prepare.directory.exists());
}
