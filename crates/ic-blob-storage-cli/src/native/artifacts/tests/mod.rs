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
