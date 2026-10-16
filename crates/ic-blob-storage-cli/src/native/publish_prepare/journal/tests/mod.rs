use super::*;
use std::os::unix::fs::symlink;

#[test]
fn confined_claim_reads_keep_limits_missing_errors_and_reject_links_without_writes() {
    let base = tempfile::tempdir().unwrap();
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
