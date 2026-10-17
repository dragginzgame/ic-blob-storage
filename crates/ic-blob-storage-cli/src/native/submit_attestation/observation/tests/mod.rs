use super::*;

#[test]
fn observation_reads_admit_bounded_regular_bytes_and_refuse_aliases() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("statement.cbor");
    std::fs::write(&path, b"abc").unwrap();
    let mut artifacts = Artifacts {
        path: directory.path(),
        hashes: BTreeMap::new(),
    };
    assert_eq!(artifacts.bytes("statement.cbor", 3).unwrap(), b"abc");
    assert_eq!(
        artifacts.hashes["statement.cbor"],
        ContentDigest::compute(b"abc").to_string()
    );
    assert_eq!(
        artifacts.bytes("statement.cbor", 2),
        Err(Failure::Observation)
    );
    std::fs::write(&path, b"").unwrap();
    assert_eq!(
        artifacts.bytes("statement.cbor", 3),
        Err(Failure::Observation)
    );
    std::fs::remove_file(&path).unwrap();
    std::fs::create_dir(&path).unwrap();
    assert_eq!(
        artifacts.bytes("statement.cbor", 3),
        Err(Failure::Observation)
    );
    std::fs::remove_dir(&path).unwrap();
    let target = directory.path().join("target");
    std::fs::write(&target, b"abc").unwrap();
    std::os::unix::fs::symlink(&target, &path).unwrap();
    assert_eq!(
        artifacts.bytes("statement.cbor", 3),
        Err(Failure::Observation)
    );
    assert_eq!(std::fs::read(target).unwrap(), b"abc");
}
