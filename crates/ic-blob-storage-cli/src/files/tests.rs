use super::*;
use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
    path::PathBuf,
};

fn fixture() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/host-custody-test")
        .join(format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&path).unwrap();
    path.canonicalize().unwrap()
}

#[test]
fn held_publication_does_not_follow_a_replaced_directory_name() {
    let root = fixture();
    let original = root.join("custody");
    let held = claim(&original).unwrap();
    fs::rename(&original, root.join("retained")).unwrap();
    fs::create_dir(&original).unwrap();
    publish(&held, "request.cbor".as_ref(), b"original request").unwrap();
    assert!(!original.join("request.cbor").exists());
    assert_eq!(
        fs::read(root.join("retained/request.cbor")).unwrap(),
        b"original request"
    );
    assert_eq!(
        fs::metadata(root.join("retained/request.cbor"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
}

#[test]
fn existing_and_partial_claims_cannot_be_overwritten_or_retried() {
    let root = fixture();
    let held = directory(&root, false).unwrap();
    publish(&held, "dispatch.claim".as_ref(), b"original id").unwrap();
    assert!(publish(&held, "dispatch.claim".as_ref(), b"replacement").is_err());
    assert_eq!(
        read_at(&held, "dispatch.claim".as_ref(), 32).unwrap(),
        b"original id"
    );
    assert!(!root.join(".dispatch.claim.pending").exists());
    fs::write(root.join(".request.cbor.pending"), []).unwrap();
    assert!(publish(&held, "request.cbor".as_ref(), b"new request").is_err());
    assert!(!root.join("request.cbor").exists());
    assert_eq!(fs::read(root.join(".request.cbor.pending")).unwrap(), b"");
    assert!(claim(&root).is_err());
}

#[test]
fn bounded_reads_and_directory_admission_refuse_links_and_traversal() {
    let root = fixture();
    let held = directory(&root, false).unwrap();
    fs::write(root.join("input"), b"four").unwrap();
    assert!(read_at(&held, "input".as_ref(), 3).is_err());
    symlink("input", root.join("alias")).unwrap();
    assert!(read_at(&held, "alias".as_ref(), 4).is_err());
    // Operator inputs retain their existing path semantics; custody reads do not.
    assert_eq!(crate::read(&root.join("alias"), 4).unwrap(), b"four");
    fs::create_dir(root.join("child")).unwrap();
    assert_eq!(
        crate::read(&root.join("child/../input"), 4).unwrap(),
        b"four"
    );
    symlink(&root, root.join("other")).unwrap();
    assert!(directory(&root.join("other/missing"), true).is_err());
    assert!(!root.join("missing").exists());
    assert!(directory(&root.join("new/../escape"), true).is_err());
    assert!(!root.join("new").exists());
    assert_eq!(read_at(&held, "input".as_ref(), 4).unwrap(), b"four");
}
