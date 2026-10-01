use super::*;
use ic_blob_storage::ops::caffeine::preparation::{
    PreparedManifestLimits, decode_prepared_manifest,
};
const ROOT: &str = "sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd";
fn declaration() -> UploadManifestDeclaration {
    let bytes = br#"{"tree_type":"DSBMTWH","chunk_hashes":["sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7"],"tree":{"hash":"sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd"},"headers":["Content-Length: 3","Content-Type: text/plain"]}"#;
    decode_prepared_manifest(
        ROOT.parse().unwrap(),
        3,
        bytes,
        PreparedManifestLimits {
            max_json_bytes: 4096.try_into().unwrap(),
            manifest:
                ic_blob_storage::model::identity::caffeine::manifest::CaffeineManifestLimits {
                    max_content_bytes: 3.try_into().unwrap(),
                    max_chunks: 1.try_into().unwrap(),
                    max_headers: 2.try_into().unwrap(),
                    max_header_bytes: 128.try_into().unwrap(),
                },
        },
    )
    .unwrap()
}

#[test]
fn source_changes_after_open_cannot_produce_verified_partial_or_excess_bytes() {
    let base = tempfile::tempdir().unwrap();
    let path = base.path().join("body");
    for changed in [b"abd".as_slice(), b"ab", b"abcd"] {
        std::fs::write(&path, b"abc").unwrap();
        let body = LocalBody::open(&path, 3).unwrap();
        std::fs::write(&path, changed).unwrap();
        let mut sink = Vec::new();
        assert!(matches!(
            body.verify(
                ROOT.parse().unwrap(),
                &declaration(),
                3.try_into().unwrap(),
                &mut sink
            ),
            Err(Failure::Content)
        ));
        assert!(sink.len() <= 3);
    }
}

#[cfg(unix)]
#[test]
fn replacing_source_path_does_not_change_the_selected_handle_or_snapshot_bytes() {
    let base = tempfile::tempdir().unwrap();
    let path = base.path().join("body");
    std::fs::write(&path, b"abc").unwrap();
    let body = LocalBody::open(&path, 3).unwrap();
    std::fs::rename(&path, base.path().join("old-body")).unwrap();
    std::fs::write(&path, b"abd").unwrap();
    let mut sink = Vec::new();
    body.verify(
        ROOT.parse().unwrap(),
        &declaration(),
        3.try_into().unwrap(),
        &mut sink,
    )
    .unwrap();
    assert_eq!(sink, b"abc");
    assert_eq!(std::fs::read(&path).unwrap(), b"abd");
}
