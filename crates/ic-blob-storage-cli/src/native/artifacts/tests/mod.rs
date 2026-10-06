use super::*;

#[test]
fn records_publish_complete_bytes_without_replacing_existing_evidence() {
    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("run");
    let run = Run::create(&path).unwrap();
    run.bytes("request.candid", b"original").unwrap();
    assert!(matches!(
        run.bytes("request.candid", b"replacement"),
        Err(Failure::File)
    ));
    assert_eq!(
        std::fs::read(path.join("request.candid")).unwrap(),
        b"original"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        assert_eq!(
            std::fs::metadata(path.join("request.candid"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
        let target = parent.path().join("target");
        std::fs::write(&target, b"outside").unwrap();
        std::os::unix::fs::symlink(&target, path.join("response.candid")).unwrap();
        assert!(matches!(
            run.bytes("response.candid", b"replacement"),
            Err(Failure::File)
        ));
        assert_eq!(std::fs::read(target).unwrap(), b"outside");
    }
}

#[test]
fn removed_or_substituted_run_cannot_be_recreated_by_record_publication() {
    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("run");
    let run = Run::create(&path).unwrap();
    std::fs::remove_dir(&path).unwrap();
    assert!(matches!(
        run.bytes("request.candid", b"request"),
        Err(Failure::File)
    ));
    assert!(!path.exists());
    #[cfg(unix)]
    {
        let substitute = parent.path().join("substitute");
        std::fs::create_dir(&substitute).unwrap();
        std::os::unix::fs::symlink(&substitute, &path).unwrap();
        assert!(matches!(
            run.bytes("request.candid", b"request"),
            Err(Failure::File)
        ));
        assert!(!substitute.join("request.candid").exists());
    }
}
