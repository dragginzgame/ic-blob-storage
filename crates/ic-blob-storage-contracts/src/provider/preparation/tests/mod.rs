use crate::provider::preparation::*;
use std::num::NonZeroU64;

// Independent Caffeine 1.1.2 abc/text vector, also retained in caffeine-hashing/vectors.json.
const ROOT: &str = "sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd";
const LEAF: &str = "sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";
fn limits() -> PreparedManifestLimits {
    PreparedManifestLimits {
        max_json_bytes: NonZeroUsize::new(4096).unwrap(),
        manifest: CaffeineManifestLimits {
            max_content_bytes: NonZeroU64::new(10).unwrap(),
            max_chunks: NonZeroUsize::MIN,
            max_headers: NonZeroUsize::new(4).unwrap(),
            max_header_bytes: NonZeroUsize::new(256).unwrap(),
        },
    }
}
fn manifest() -> serde_json::Value {
    serde_json::json!({ "tree_type": "DSBMTWH", "chunk_hashes": [LEAF],
        "tree": { "hash": ROOT }, "headers": ["Content-Length: 3", "Content-Type: text/plain"] })
}
fn decode(value: &serde_json::Value) -> Result<UploadManifestDeclaration, PreparedManifestError> {
    decode_prepared_manifest(
        ROOT.parse().unwrap(),
        3,
        &serde_json::to_vec(value).unwrap(),
        limits(),
    )
}

#[test]
fn upstream_metadata_and_leaves_become_the_existing_service_declaration() {
    let result = decode(&manifest()).unwrap();
    assert_eq!(
        result.chunks,
        vec![*LEAF.parse::<CaffeineChunkHash>().unwrap().as_bytes()]
    );
    assert_eq!(
        result.headers,
        vec![
            UploadManifestHeader {
                name: "Content-Length".into(),
                value: "3".into()
            },
            UploadManifestHeader {
                name: "Content-Type".into(),
                value: "text/plain".into()
            },
        ]
    );
    let mut reordered = manifest();
    reordered["headers"].as_array_mut().unwrap().reverse();
    let reversed = decode(&reordered).unwrap();
    assert_eq!(reversed.headers[0], result.headers[1]);
    assert_eq!(reversed.chunks, result.chunks);
}

#[test]
fn json_and_declaration_budgets_precede_conversion() {
    let body = serde_json::to_vec(&manifest()).unwrap();
    let mut bounded = limits();
    bounded.max_json_bytes = NonZeroUsize::new(body.len() - 1).unwrap();
    assert_eq!(
        decode_prepared_manifest(ROOT.parse().unwrap(), 3, &body, bounded),
        Err(PreparedManifestError::JsonLimit)
    );
    bounded.max_json_bytes = NonZeroUsize::new(body.len()).unwrap();
    assert!(decode_prepared_manifest(ROOT.parse().unwrap(), 3, &body, bounded).is_ok());
    bounded.manifest.max_header_bytes = NonZeroUsize::MIN;
    assert_eq!(
        decode_prepared_manifest(ROOT.parse().unwrap(), 3, &body, bounded),
        Err(PreparedManifestError::DeclarationLimit)
    );
    let mut many = manifest();
    many["chunk_hashes"] = serde_json::json!([LEAF, LEAF]);
    assert_eq!(decode(&many), Err(PreparedManifestError::DeclarationLimit));
    many = manifest();
    many["headers"] = serde_json::json!(["a: 1", "b: 2", "c: 3", "d: 4", "e: 5"]);
    assert_eq!(decode(&many), Err(PreparedManifestError::DeclarationLimit));
}

#[test]
fn wrong_roots_leaves_lengths_and_metadata_do_not_become_permissions() {
    let mut value = manifest();
    value["tree"]["hash"] = LEAF.into();
    assert_eq!(decode(&value), Err(PreparedManifestError::Root));
    value = manifest();
    value["chunk_hashes"][0] = ROOT.into();
    assert_eq!(
        decode(&value),
        Err(PreparedManifestError::Manifest(
            CaffeineManifestError::RootMismatch
        ))
    );
    value = manifest();
    value["headers"][0] = "Content-Length: 4".into();
    assert_eq!(
        decode(&value),
        Err(PreparedManifestError::Metadata(
            UploadMetadataError::LengthMismatch
        ))
    );
    value["headers"][0] = "Content-Length: 03".into();
    assert_eq!(
        decode(&value),
        Err(PreparedManifestError::Metadata(
            UploadMetadataError::NonCanonicalLength
        ))
    );
    value["headers"][0] = "Content-Length:3".into();
    assert_eq!(decode(&value), Err(PreparedManifestError::Header));
    value = manifest();
    value["headers"]
        .as_array_mut()
        .unwrap()
        .push("content-type: text/plain".into());
    assert_eq!(
        decode(&value),
        Err(PreparedManifestError::Metadata(
            UploadMetadataError::DuplicateHeader
        ))
    );
}

#[test]
fn malformed_ambiguous_or_unsupported_json_is_refused() {
    for body in [
        b"".as_slice(),
        b"{}",
        b"{} {}",
        br#"{"tree_type":"DSBMTWH","tree_type":"DSBMTWH"}"#,
    ] {
        assert_eq!(
            decode_prepared_manifest(ROOT.parse().unwrap(), 3, body, limits()),
            Err(PreparedManifestError::Json)
        );
    }
    let mut value = manifest();
    value["tree_type"] = "unknown".into();
    assert_eq!(decode(&value), Err(PreparedManifestError::Json));
    value = manifest();
    value["chunk_hashes"][0] = "not-a-hash".into();
    assert!(matches!(
        decode(&value),
        Err(PreparedManifestError::Hash(_))
    ));
}
