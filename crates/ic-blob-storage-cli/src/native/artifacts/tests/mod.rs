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
    run.json("summary.json", &serde_json::json!({"complete": true}))
        .unwrap();
    assert!(matches!(
        run.json("summary.json", &serde_json::json!({"complete": false})),
        Err(Failure::File)
    ));
    assert_eq!(
        std::fs::read(path.join("summary.json")).unwrap(),
        b"{\n  \"complete\": true\n}"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        for name in ["request.candid", "summary.json"] {
            assert_eq!(
                std::fs::metadata(path.join(name))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
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
    assert!(matches!(
        run.json("summary.json", &serde_json::json!({"complete": true})),
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
        assert!(matches!(
            run.json("summary.json", &serde_json::json!({"complete": true})),
            Err(Failure::File)
        ));
        assert!(!substitute.join("request.candid").exists());
        assert!(!substitute.join("summary.json").exists());
    }
}

#[test]
fn failed_json_producer_keeps_existing_evidence_without_publishing_a_prefix() {
    struct PartialRecord;
    impl Serialize for PartialRecord {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::{Error as _, SerializeStruct as _};
            let mut record = serializer.serialize_struct("PartialRecord", 2)?;
            record.serialize_field("complete", &false)?;
            Err(S::Error::custom("record production interrupted"))
        }
    }

    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("run");
    let run = Run::create(&path).unwrap();
    run.bytes("request.candid", b"original").unwrap();
    assert!(matches!(
        run.json("summary.json", &PartialRecord),
        Err(Failure::File)
    ));
    assert!(!path.join("summary.json").exists());
    assert_eq!(
        std::fs::read(path.join("request.candid")).unwrap(),
        b"original"
    );
    assert_eq!(std::fs::read_dir(&path).unwrap().count(), 1);
}

#[test]
fn a_replacement_directory_cannot_inherit_the_claimed_run() {
    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("run");
    let original = parent.path().join("original");
    let run = Run::create(&path).unwrap();
    run.bytes("request.candid", b"original request").unwrap();
    let body = run.open_body().unwrap();
    std::fs::rename(&path, &original).unwrap();
    std::fs::create_dir(&path).unwrap();
    assert!(matches!(
        run.bytes("response.candid", b"wrong owner"),
        Err(Failure::File)
    ));
    assert!(matches!(
        run.json("summary.json", &serde_json::json!({"complete": true})),
        Err(Failure::File)
    ));
    assert!(matches!(run.open_body(), Err(Failure::File)));
    assert!(matches!(run.publish_body(body), Err(Failure::File)));
    assert!(std::fs::read_dir(&path).unwrap().next().is_none());
    assert_eq!(std::fs::read_dir(&original).unwrap().count(), 2);
    assert_eq!(
        std::fs::read(original.join("request.candid")).unwrap(),
        b"original request"
    );
}

#[test]
fn record_names_cannot_escape_the_claimed_directory() {
    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("run");
    let run = Run::create(&path).unwrap();
    for name in ["", ".", "..", "../outside", "nested/record"] {
        assert!(matches!(run.bytes(name, b"wrong name"), Err(Failure::File)));
        assert!(matches!(
            run.json(name, &serde_json::json!({"complete": true})),
            Err(Failure::File)
        ));
    }
    assert!(std::fs::read_dir(path).unwrap().next().is_none());
    assert!(!parent.path().join("outside").exists());
}

#[cfg(unix)]
#[test]
fn replaced_partial_body_cannot_publish_under_the_verified_handle() {
    use std::io::Write as _;
    use std::os::unix::fs::PermissionsExt as _;

    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("run");
    let run = Run::create(&path).unwrap();
    let mut body = run.open_body().unwrap();
    body.write_all(b"verified original").unwrap();
    assert_eq!(body.metadata().unwrap().permissions().mode() & 0o777, 0o600);
    std::fs::rename(path.join("body.part"), path.join("original.part")).unwrap();
    std::fs::write(path.join("body.part"), b"substitute").unwrap();

    assert_eq!(run.publish_body(body), Err(Failure::File));
    assert!(!path.join("body.bin").exists());
    assert_eq!(
        std::fs::read(path.join("original.part")).unwrap(),
        b"verified original"
    );
    assert_eq!(
        std::fs::read(path.join("body.part")).unwrap(),
        b"substitute"
    );
}

#[test]
fn private_run_creation_preserves_occupied_claims_and_refuses_control_paths() {
    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("run with spaces");
    let run = Run::create(&path).unwrap();
    run.bytes("request.candid", b"original").unwrap();
    assert!(matches!(Run::create(&path), Err(Failure::ExistingRun)));
    assert_eq!(
        std::fs::read(path.join("request.candid")).unwrap(),
        b"original"
    );
    let occupied = parent.path().join("occupied");
    std::fs::write(&occupied, b"outside").unwrap();
    assert!(matches!(Run::create(&occupied), Err(Failure::ExistingRun)));
    assert_eq!(std::fs::read(&occupied).unwrap(), b"outside");
    for name in ["run\nname", "run\rname"] {
        let refused = parent.path().join(name);
        assert!(matches!(Run::create(&refused), Err(Failure::File)));
        assert!(!refused.exists());
    }
    #[cfg(unix)]
    {
        use std::os::unix::{fs::PermissionsExt as _, fs::symlink};
        assert_eq!(
            std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o700
        );
        let physical = parent.path().join("physical\nparent");
        std::fs::create_dir(&physical).unwrap();
        let alias = parent.path().join("alias");
        symlink(&physical, &alias).unwrap();
        assert!(matches!(
            Run::create(&alias.join("run")),
            Err(Failure::File)
        ));
        assert!(!physical.join("run").exists());
    }
}
