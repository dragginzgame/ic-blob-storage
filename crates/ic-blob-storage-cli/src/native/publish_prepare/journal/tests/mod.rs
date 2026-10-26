use super::*;
use ic_host_fs::testing::FixtureDirectory;
use std::os::unix::fs::symlink;

#[test]
fn confined_claim_reads_keep_limits_missing_errors_and_reject_links_without_writes() {
    let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let root = base.path();
    let reader = RecoveryDirectory::open(root).unwrap();
    std::fs::create_dir(root.join("admission")).unwrap();
    std::fs::write(root.join("admission/request"), b"original").unwrap();
    std::fs::write(root.join("empty"), b"").unwrap();
    symlink("admission", root.join("alias")).unwrap();
    symlink("admission/request", root.join("link")).unwrap();
    assert_eq!(reader.read("admission/request", 8).unwrap(), b"original");
    for path in ["missing", "empty", "admission"] {
        assert!(matches!(reader.read(path, 8), Err(Failure::File)));
    }
    assert!(matches!(
        reader.read("admission/request", 7),
        Err(Failure::File)
    ));
    for path in ["alias/request", "link", "../outside"] {
        assert!(matches!(reader.read(path, 8), Err(Failure::Binding)));
    }
    assert_eq!(
        std::fs::read(root.join("admission/request")).unwrap(),
        b"original"
    );
}

#[test]
fn recovery_packets_stay_with_the_opened_directory_after_path_replacement() {
    let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let path = base.path().join("original");
    std::fs::create_dir(&path).unwrap();
    let frozen = crate::native::publish_check::tests::frozen();
    let batch = crate::native::publish_check::tests::batch(&frozen);
    let selected = &batch.files[0].input;
    let expected = json!({"operation":"publish_prepare","file_index":0});
    let (permission, manifest) = selected.setup_requests();
    std::fs::write(path.join("intent.json"), expected.to_string()).unwrap();
    std::fs::write(path.join("permission.wire"), permission).unwrap();
    std::fs::write(path.join("manifest.wire"), manifest).unwrap();
    for step in [Step::Admission, Step::Preparation] {
        std::fs::create_dir(path.join(name(step))).unwrap();
        std::fs::write(path.join(name(step)).join("request.wire"), b"original").unwrap();
    }
    std::fs::write(path.join("summary.json"), b"original summary").unwrap();
    let held = RecoveryDirectory::open(&path).unwrap();
    validate(&held, &expected, selected).unwrap();
    std::fs::rename(&path, base.path().join("retained")).unwrap();
    std::fs::create_dir(&path).unwrap();
    std::fs::write(path.join("intent.json"), b"{}").unwrap();
    std::fs::write(path.join("summary.json"), b"replacement summary").unwrap();
    validate(&held, &expected, selected).unwrap();
    for step in [Step::Admission, Step::Preparation] {
        assert_eq!(
            held.read(&format!("{}/request.wire", name(step)), 32)
                .unwrap(),
            b"original"
        );
    }
    assert_eq!(held.read("summary.json", 32).unwrap(), b"original summary");
    assert_eq!(
        validate(
            &RecoveryDirectory::open(&path).unwrap(),
            &expected,
            selected
        ),
        Err(Failure::Binding)
    );
}
