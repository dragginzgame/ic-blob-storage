use super::*;
use crate::native::{arguments::Command, execute, upload_inputs::tests as fixture};
use ic_host_fs::testing::FixtureDirectory;
use std::fs;

#[test]
fn preparation_summary_preserves_shape_and_refusal_classification() {
    assert_eq!(
        PreparationSummaryView::validate(br#"{"prepared":true,"state":"prepared","extra":1}"#),
        Ok(())
    );
    assert_eq!(
        PreparationSummaryView::validate(b"{"),
        Err(Failure::Binding)
    );
    for malformed in [
        br#"{"prepared":"true"} trailing"#.as_slice(),
        br#"{"prepared":true,"state":"prepared"} trailing"#.as_slice(),
    ] {
        assert_eq!(
            PreparationSummaryView::validate(malformed),
            Err(Failure::Binding)
        );
    }
    for report in [
        json!(null),
        json!([]),
        json!({}),
        json!({"prepared":"true","state":"prepared"}),
        json!({"prepared":true,"state":"unknown"}),
        json!({"prepared":false,"state":"prepared"}),
        json!({"prepared":true,"state":"blocked"}),
    ] {
        assert_eq!(
            PreparationSummaryView::validate(&serde_json::to_vec(&report).unwrap()),
            Err(Failure::Unprepared)
        );
    }
}

fn arguments(directory: &std::path::Path) -> Vec<String> {
    [
        "publish-prepare",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io/",
        "--identity",
        "missing-tenant.pem",
        "--actor",
        fixture::binding()["tenant"].as_str().unwrap(),
        "--uploader-identity",
        "missing-uploader.pem",
        "--service",
        fixture::binding()["service"].as_str().unwrap(),
        "--namespace",
        "7",
        "--inputs",
        directory.to_str().unwrap(),
        "--file-index",
        "0",
        "--max-bytes",
        "10",
        "--max-total-bytes",
        "20",
        "--timeout-seconds",
        "30",
        "--run-dir",
        directory.join("run").to_str().unwrap(),
    ]
    .map(str::to_owned)
    .to_vec()
}
#[test]
fn indexed_prepare_requires_canonical_index_explicit_roles_and_original_recovery_journal() {
    let directory = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let args = arguments(directory.path());
    assert!(Options::parse(&args).is_ok());
    for value in ["00", "-1", "4096"] {
        let mut changed = args.clone();
        let index = changed.iter().position(|a| a == "--file-index").unwrap();
        changed[index + 1] = value.into();
        assert!(matches!(Options::parse(&changed), Err(Failure::Arguments)));
    }
    let mut resume = args;
    resume[0] = "publish-prepare-resume".into();
    assert!(matches!(Options::parse(&resume), Err(Failure::Arguments)));
    assert_eq!(execute(&resume), Err(Failure::Arguments));
    assert!(!directory.path().join("run").exists());
}

#[test]
fn batch_prepare_requires_all_file_selection_without_recovery_or_index_flags() {
    let directory = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let mut args = arguments(directory.path());
    args[0] = "publish-prepare-batch".into();
    let index = args.iter().position(|a| a == "--file-index").unwrap();
    args.drain(index..=index + 1);
    assert!(matches!(
        Options::parse(&args).unwrap().0,
        Command::PublishPrepareBatch(_)
    ));
    for extra in ["--file-index", "--source-run"] {
        let mut changed = args.clone();
        changed.extend([extra.into(), "0".into()]);
        assert!(matches!(Options::parse(&changed), Err(Failure::Arguments)));
    }
}
#[test]
fn original_batch_binding_refuses_changed_manifest_permission_and_index() {
    let directory = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let selected = crate::native::upload_inputs::PreparedInput::load(
        crate::native::upload_inputs::InputFiles {
            binding: serde_json::to_vec(&fixture::binding()).unwrap(),
            manifest: serde_json::to_vec(&fixture::manifest()).unwrap(),
            installation: crate::native::upload_inputs::installation::Installation::decode(
                ic_blob_storage_contracts::wire::encode_request(
                    "blob_install",
                    fixture::installation(&fixture::binding()),
                )
                .unwrap(),
                10.try_into().unwrap(),
            )
            .unwrap(),
            body: directory.path().join("body.bin").into(),
        },
    )
    .unwrap();
    let expected = json!({"operation":"publish_prepare","file_index":0});
    let (permission, manifest) = selected.setup_requests();
    fs::write(directory.path().join("intent.json"), expected.to_string()).unwrap();
    fs::write(directory.path().join("permission.wire"), permission).unwrap();
    fs::write(directory.path().join("manifest.wire"), manifest).unwrap();
    journal::validate(
        &journal::RecoveryDirectory::open(directory.path()).unwrap(),
        &expected,
        &selected,
    )
    .unwrap();
    let changed = json!({"operation":"publish_prepare","file_index":1});
    assert_eq!(
        journal::validate(
            &journal::RecoveryDirectory::open(directory.path()).unwrap(),
            &changed,
            &selected
        ),
        Err(Failure::Binding)
    );
    fs::write(directory.path().join("manifest.wire"), b"changed").unwrap();
    assert_eq!(
        journal::validate(
            &journal::RecoveryDirectory::open(directory.path()).unwrap(),
            &expected,
            &selected
        ),
        Err(Failure::Binding)
    );
    fs::write(directory.path().join("manifest.wire"), manifest).unwrap();
    fs::write(directory.path().join("permission.wire"), b"changed").unwrap();
    assert_eq!(
        journal::validate(
            &journal::RecoveryDirectory::open(directory.path()).unwrap(),
            &expected,
            &selected
        ),
        Err(Failure::Binding)
    );
}
#[test]
fn partial_attempt_is_claimed_and_symlink_claim_is_refused() {
    let directory = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    assert!(!journal::claimed(directory.path(), journal::Step::Preparation).unwrap());
    fs::create_dir(directory.path().join("preparation")).unwrap();
    assert!(journal::claimed(directory.path(), journal::Step::Preparation).unwrap());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(
            directory.path().join("preparation"),
            directory.path().join("admission"),
        )
        .unwrap();
        assert_eq!(
            journal::claimed(directory.path(), journal::Step::Admission),
            Err(Failure::Binding)
        );
    }
}
