use super::*;
use std::fs;

fn input(base: &Path, sources: &[&str]) -> PathBuf {
    let files: Vec<_> = sources
        .iter()
        .enumerate()
        .map(|(i, source)| {
            serde_json::json!({
                "asset": format!("asset-{i}"), "source": source,
                "declaration": { "bytes": 3, "headers": [
                    { "name": "Content-Length", "value": "3" },
                    { "name": "Content-Type", "value": "text/plain" }
                ] }
            })
        })
        .collect();
    let path = base.join("input.json");
    fs::write(
        &path,
        serde_json::to_vec(&serde_json::json!({
            "limits": { "files": 4, "file_bytes": 3, "file_chunks": 1,
                "total_bytes": 12, "total_chunks": 4 },
            "files": files
        }))
        .unwrap(),
    )
    .unwrap();
    path
}

#[test]
fn saved_bytes_match_independent_root_and_survive_source_replacement() {
    let source = tempfile::tempdir().unwrap();
    let destination = tempfile::tempdir().unwrap();
    fs::write(source.path().join("body"), b"abc").unwrap();
    let input = input(source.path(), &["body", "body"]);
    let saved = run(&input, destination.path()).unwrap();
    fs::write(source.path().join("body"), b"xyz").unwrap();
    let expected = "0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd";
    let bodies = saved.directory.join("bodies");
    let entries: Vec<_> = fs::read_dir(&bodies).unwrap().map(Result::unwrap).collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(fs::read(bodies.join(expected)).unwrap(), b"abc");
    let report: serde_json::Value =
        serde_json::from_slice(&fs::read(saved.directory.join("inventory.json")).unwrap()).unwrap();
    assert_eq!(report, serde_json::to_value(&saved.inventory).unwrap());
    assert_eq!(report["assets"][0]["root"], format!("sha256:{expected}"));
    assert_eq!(report["assets"][1]["root"], report["assets"][0]["root"]);
    assert_ne!(report["assets"][0]["asset"], report["assets"][1]["asset"]);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(&saved.directory).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            fs::metadata(bodies.join(expected))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    // A second run never resumes or modifies the first snapshot.
    let again = run(&input, destination.path()).unwrap();
    assert_ne!(saved.directory, again.directory);
    assert_eq!(fs::read(bodies.join(expected)).unwrap(), b"abc");
}

#[test]
fn failure_after_a_saved_body_removes_only_this_attempt() {
    let source = tempfile::tempdir().unwrap();
    let destination = tempfile::tempdir().unwrap();
    fs::write(source.path().join("body"), b"abc").unwrap();
    let valid = run(&input(source.path(), &["body"]), destination.path()).unwrap();
    fs::write(destination.path().join("unrelated"), b"keep").unwrap();
    let failed = run(
        &input(source.path(), &["body", "missing"]),
        destination.path(),
    );
    assert!(
        matches!(failed, Err(inventory::InventoryError::SourceFile { asset, .. }) if asset == "asset-1")
    );
    let mut remaining: Vec<_> = fs::read_dir(destination.path())
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    remaining.sort();
    let mut expected = vec![
        valid.directory.clone(),
        destination.path().join("unrelated"),
    ];
    expected.sort();
    assert_eq!(remaining, expected);
    assert!(valid.directory.join("inventory.json").is_file());
    assert_eq!(
        fs::read(destination.path().join("unrelated")).unwrap(),
        b"keep"
    );
}
