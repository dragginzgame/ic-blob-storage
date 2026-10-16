use super::*;
#[test]
fn admitted_root_survives_namespace_replacement_for_metadata_and_body_reads() {
    let base = tempfile::tempdir().unwrap();
    let original = base.path().join("input");
    std::fs::create_dir(&original).unwrap();
    std::fs::write(original.join("body"), b"original").unwrap();
    let root = Root::new(&original).unwrap();
    let body = root.source("body").unwrap();
    std::fs::rename(&original, base.path().join("retained")).unwrap();
    std::fs::create_dir(&original).unwrap();
    std::fs::write(original.join("body"), b"replacement").unwrap();
    assert_eq!(root.read("body", 8).unwrap(), b"original");
    let mut file = body.file().unwrap();
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut bytes).unwrap();
    assert_eq!(bytes, b"original");
    assert_eq!(root.read("body", 7), Err(Failure::File));
}
