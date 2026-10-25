use super::*;
use ic_host_fs::testing::FixtureDirectory;
use std::os::unix::fs::symlink;

#[test]
fn confined_claim_reads_keep_limits_missing_errors_and_reject_links_without_writes() {
    let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let root = base.path();
    std::fs::create_dir(root.join("admission")).unwrap();
    std::fs::write(root.join("admission/request"), b"original").unwrap();
    std::fs::write(root.join("empty"), b"").unwrap();
    symlink("admission", root.join("alias")).unwrap();
    symlink("admission/request", root.join("link")).unwrap();
    assert_eq!(read_at(root, "admission/request", 8).unwrap(), b"original");
    for path in ["missing", "empty", "admission"] {
        assert!(matches!(read_at(root, path, 8), Err(Failure::File)));
    }
    assert!(matches!(
        read_at(root, "admission/request", 7),
        Err(Failure::File)
    ));
    for path in ["alias/request", "link", "../outside"] {
        assert!(matches!(read_at(root, path, 8), Err(Failure::Binding)));
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
    std::fs::write(path.join("intent.json"), b"original").unwrap();
    let held = RecoveryDirectory::open(&path).unwrap();
    std::fs::rename(&path, base.path().join("retained")).unwrap();
    std::fs::create_dir(&path).unwrap();
    std::fs::write(path.join("intent.json"), b"replacement").unwrap();
    assert_eq!(held.read("intent.json", 32).unwrap(), b"original");
    assert_eq!(read_at(&path, "intent.json", 32).unwrap(), b"replacement");
}
