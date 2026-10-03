use super::*;
use crate::native::{execute, upload_inputs::tests as fixture};
use ic_blob_storage::dto::configuration::ServiceInstallationInput;
use std::fs;

struct Trial {
    base: tempfile::TempDir,
    inventory: Value,
    installation: ServiceInstallationInput,
}
impl Trial {
    fn new() -> Self {
        let base = tempfile::tempdir().unwrap();
        let mut files = Vec::new();
        for index in 0..2 {
            let mut binding = fixture::binding();
            binding["upload"] = json!((index + 1).to_string());
            binding["object"] = json!((index + 3).to_string());
            binding["first_reference"] = json!((index + 5).to_string());
            let binding_bytes = serde_json::to_vec(&binding).unwrap();
            let manifest_bytes = serde_json::to_vec(&fixture::manifest()).unwrap();
            let binding_path = format!("binding-{index}.json");
            let manifest_path = format!("manifest-{index}.json");
            let body_path = format!("body-{index}.bin");
            fs::write(base.path().join(&binding_path), &binding_bytes).unwrap();
            fs::write(base.path().join(&manifest_path), &manifest_bytes).unwrap();
            fs::write(base.path().join(&body_path), b"abc").unwrap();
            files.push(
                json!({"binding":binding_path, "binding_sha256":digest(&binding_bytes),
                "manifest":manifest_path, "manifest_sha256":digest(&manifest_bytes),
                "body":body_path, "body_sha256":digest(b"abc")}),
            );
        }
        Self {
            base,
            inventory: json!({"schema":1,"files":files}),
            installation: fixture::installation(&fixture::binding()),
        }
    }
    fn args(&self) -> Vec<String> {
        fs::write(
            self.base.path().join("inventory.json"),
            serde_json::to_vec(&self.inventory).unwrap(),
        )
        .unwrap();
        fs::write(
            self.base.path().join("installation.candid"),
            candid::encode_one(&self.installation).unwrap(),
        )
        .unwrap();
        vec![
            "publish-inputs".into(),
            "--inventory".into(),
            self.base
                .path()
                .join("inventory.json")
                .display()
                .to_string(),
            "--root".into(),
            self.base.path().display().to_string(),
            "--installation".into(),
            self.base
                .path()
                .join("installation.candid")
                .display()
                .to_string(),
            "--max-bytes".into(),
            "10".into(),
            "--max-total-bytes".into(),
            "20".into(),
            "--run-dir".into(),
            self.output().display().to_string(),
        ]
    }
    fn output(&self) -> std::path::PathBuf {
        self.base.path().join("output")
    }
    fn change_binding(&mut self, field: &str, value: Value) {
        let path = self.base.path().join("binding-1.json");
        let mut binding: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        binding[field] = value;
        let bytes = serde_json::to_vec(&binding).unwrap();
        fs::write(path, &bytes).unwrap();
        self.inventory["files"][1]["binding_sha256"] = json!(digest(&bytes));
    }
}

#[test]
fn serial_snapshots_bind_the_inventory_and_survive_source_change_without_overwrite() {
    let trial = Trial::new();
    let args = trial.args();
    let report = execute(&args).unwrap();
    assert_eq!(report["capacity"]["physical_bytes"], "6");
    assert_eq!(report["capacity"]["retained_chunks"], 2);
    assert_eq!(report["remaining_capacity_observed"], false);
    assert_eq!(report["provider_dispatched"], false);
    for name in ["file-0000", "file-0001"] {
        let directory = trial.output().join(name);
        assert_eq!(fs::read(directory.join("body.bin")).unwrap(), b"abc");
        let permission: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest =
            candid::decode_one(&fs::read(directory.join("permission.candid")).unwrap()).unwrap();
        assert_eq!(permission.upload.bytes, 3);
    }
    let summary = fs::read(trial.output().join("summary.json")).unwrap();
    fs::write(trial.base.path().join("body-0.bin"), b"abd").unwrap();
    assert_eq!(execute(&args), Err(Failure::ExistingRun));
    assert_eq!(
        fs::read(trial.output().join("summary.json")).unwrap(),
        summary
    );
    assert_eq!(
        fs::read(trial.output().join("file-0000/body.bin")).unwrap(),
        b"abc"
    );
}

#[test]
fn duplicate_operation_object_reference_and_changed_scope_refuse_before_output() {
    for (field, value) in [
        ("upload", json!("1")),
        ("object", json!("3")),
        ("first_reference", json!("5")),
        ("bucket", json!("other-bucket")),
        (
            "tenant",
            json!(candid::Principal::self_authenticating([9]).to_text()),
        ),
    ] {
        let mut trial = Trial::new();
        trial.change_binding(field, value);
        assert_eq!(execute(&trial.args()), Err(Failure::Binding));
        assert!(!trial.output().exists());
    }
}

#[test]
fn total_bytes_objects_and_retained_leaves_are_checked_against_the_candidate() {
    let setters: [fn(&mut ServiceInstallationInput); 5] = [
        |i| {
            i.configuration.resources.max_objects = 1;
            i.configuration.resources.max_tenant_objects = 1;
        },
        |i| {
            i.configuration.resources.max_chunks = 1;
            i.configuration.resources.max_tenant_chunks = 1;
        },
        |i| i.configuration.resources.max_physical_bytes = 5,
        |i| i.configuration.resources.max_liability_bytes = 5,
        |i| i.configuration.resources.max_tenant_logical_bytes = 5,
    ];
    for setter in setters {
        let mut trial = Trial::new();
        // Keep each object admissible while the aggregate fails.
        trial.installation.configuration.resources.max_object_bytes = 3;
        setter(&mut trial.installation);
        assert_eq!(execute(&trial.args()), Err(Failure::ReplyLimit));
        assert!(!trial.output().exists());
    }
    let trial = Trial::new();
    let mut args = trial.args();
    args[10] = "5".into();
    assert_eq!(execute(&args), Err(Failure::ReplyLimit));
    assert!(!trial.output().exists());
}

#[test]
fn changed_metadata_and_untrusted_paths_refuse_without_output() {
    let trial = Trial::new();
    fs::write(trial.base.path().join("binding-1.json"), b"{}").unwrap();
    assert_eq!(execute(&trial.args()), Err(Failure::Binding));
    assert!(!trial.output().exists());
    for path in ["../body-1.bin", "/etc/passwd", "./body-1.bin"] {
        let mut trial = Trial::new();
        trial.inventory["files"][1]["body"] = json!(path);
        assert_eq!(execute(&trial.args()), Err(Failure::Arguments));
        assert!(!trial.output().exists());
    }
    #[cfg(unix)]
    {
        let mut trial = Trial::new();
        std::os::unix::fs::symlink(
            trial.base.path().join("body-1.bin"),
            trial.base.path().join("link"),
        )
        .unwrap();
        trial.inventory["files"][1]["body"] = json!("link");
        assert_eq!(execute(&trial.args()), Err(Failure::Arguments));
        assert!(!trial.output().exists());
    }
}

#[test]
fn corrupt_later_body_retains_completed_inputs_and_failure_without_a_batch_success() {
    let trial = Trial::new();
    fs::write(trial.base.path().join("body-1.bin"), b"abd").unwrap();
    assert_eq!(execute(&trial.args()), Err(Failure::Content));
    assert_eq!(
        fs::read(trial.output().join("file-0000/body.bin")).unwrap(),
        b"abc"
    );
    assert!(trial.output().join("file-0000/permission.candid").is_file());
    assert_eq!(
        fs::read(trial.output().join("file-0001/body.part")).unwrap(),
        b"abd"
    );
    assert!(!trial.output().join("file-0001/permission.candid").exists());
    assert!(!trial.output().join("summary.json").exists());
    let failure: Value =
        serde_json::from_slice(&fs::read(trial.output().join("failure.json")).unwrap()).unwrap();
    assert_eq!(failure["file"], "file-0001");
    assert_eq!(failure["completed_files"], 1);
    assert_eq!(execute(&trial.args()), Err(Failure::ExistingRun));
}

#[test]
fn inventory_raw_digest_must_match_verified_body_before_usable_requests() {
    let mut trial = Trial::new();
    trial.inventory["files"][0]["body_sha256"] = json!(digest(b"abd"));
    assert_eq!(execute(&trial.args()), Err(Failure::Content));
    assert_eq!(
        fs::read(trial.output().join("file-0000/body.bin")).unwrap(),
        b"abc"
    );
    assert!(!trial.output().join("file-0000/permission.candid").exists());
    assert!(!trial.output().join("summary.json").exists());
}

#[test]
fn bounded_inventory_and_regular_body_requirements_refuse_before_claiming_output() {
    for change in [json!([]), json!([{"body":"missing.bin"}])] {
        let mut trial = Trial::new();
        trial.inventory["files"] = change;
        assert_eq!(execute(&trial.args()), Err(Failure::Arguments));
        assert!(!trial.output().exists());
    }
    let mut trial = Trial::new();
    trial.inventory["files"][0]["body_sha256"] = json!("ABCDEF".repeat(11));
    assert_eq!(execute(&trial.args()), Err(Failure::Arguments));
    assert!(!trial.output().exists());
    let trial = Trial::new();
    fs::write(trial.base.path().join("body-1.bin"), b"ab").unwrap();
    assert_eq!(execute(&trial.args()), Err(Failure::Content));
    assert!(!trial.output().exists());
    let trial = Trial::new();
    let mut args = trial.args();
    args.extend(["--unknown".into(), "value".into()]);
    assert_eq!(execute(&args), Err(Failure::Arguments));
    assert!(!trial.output().exists());
}
