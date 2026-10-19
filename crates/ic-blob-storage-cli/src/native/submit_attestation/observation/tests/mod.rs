use super::*;
use ic_host_fs::testing::FixtureDirectory;

#[test]
fn observation_reads_admit_bounded_regular_bytes_and_refuse_aliases() {
    let directory = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let path = directory.path().join("statement.wire");
    std::fs::write(&path, b"abc").unwrap();
    let mut artifacts = Artifacts {
        path: directory.path(),
        hashes: BTreeMap::new(),
    };
    assert_eq!(artifacts.bytes("statement.wire", 3).unwrap(), b"abc");
    assert_eq!(
        artifacts.hashes["statement.wire"],
        ContentDigest::compute(b"abc").to_string()
    );
    assert_eq!(
        artifacts.bytes("statement.wire", 2),
        Err(Failure::Observation)
    );
    std::fs::write(&path, b"").unwrap();
    assert_eq!(
        artifacts.bytes("statement.wire", 3),
        Err(Failure::Observation)
    );
    std::fs::remove_file(&path).unwrap();
    std::fs::create_dir(&path).unwrap();
    assert_eq!(
        artifacts.bytes("statement.wire", 3),
        Err(Failure::Observation)
    );
    std::fs::remove_dir(&path).unwrap();
    let target = directory.path().join("target");
    std::fs::write(&target, b"abc").unwrap();
    std::os::unix::fs::symlink(&target, &path).unwrap();
    assert_eq!(
        artifacts.bytes("statement.wire", 3),
        Err(Failure::Observation)
    );
    assert_eq!(std::fs::read(target).unwrap(), b"abc");
}
