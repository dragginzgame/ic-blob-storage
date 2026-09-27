use super::*;
use crate::{Header, UploadMetadataError};
use std::fs;

fn limits() -> InventoryLimits {
    InventoryLimits {
        files: NonZeroUsize::new(4).unwrap(),
        file_bytes: NonZeroU64::new(10).unwrap(),
        file_chunks: NonZeroUsize::new(1).unwrap(),
        total_bytes: NonZeroU64::new(12).unwrap(),
        total_chunks: NonZeroU64::new(4).unwrap(),
    }
}

fn entry(asset: &str, path: &str) -> InventoryFile {
    InventoryFile {
        asset: asset.into(),
        source: path.into(),
        declaration: Declaration {
            bytes: 3,
            headers: vec![
                Header {
                    name: "Content-Length".into(),
                    value: "3".into(),
                },
                Header {
                    name: "Content-Type".into(),
                    value: "text/plain".into(),
                },
            ],
        },
    }
}

#[test]
fn duplicates_share_a_blob_but_preserve_asset_mappings_and_source_work() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a"), b"abc").unwrap();
    fs::write(dir.path().join("b"), b"abc").unwrap();
    let first = entry("image-a", "a");
    let mut second = entry("image-b", "b");
    second.declaration.headers.reverse();
    let mut third = entry("binary-c", "a");
    third.declaration.headers[1].value = "application/octet-stream".into();
    let report = prepare_inventory(
        Inventory {
            limits: limits(),
            files: vec![first, second, third],
        },
        dir.path(),
    )
    .unwrap();
    assert_eq!(report.totals.assets, 3);
    assert_eq!(report.totals.source_bytes, 9);
    assert_eq!(report.totals.source_chunks, 3);
    assert_eq!(report.totals.distinct_blobs, 2);
    assert_eq!(report.totals.distinct_bytes, 6);
    assert_eq!(report.totals.distinct_chunks, 2);
    assert_eq!(report.assets[0].root, report.assets[1].root);
    assert_ne!(report.assets[0].asset, report.assets[1].asset);
    assert_ne!(report.assets[0].root, report.assets[2].root);
    let text = report
        .blobs
        .iter()
        .find(|b| b.claim.root == report.assets[0].root)
        .unwrap();
    assert_eq!(
        text.claim.root,
        "sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd"
    );
    assert_eq!(text.claim.headers[0].name, "Content-Length");
    assert_eq!(
        text.chunk_hashes,
        ["sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7"]
    );
}

#[test]
fn entire_inventory_is_validated_before_opening_any_source() {
    let dir = tempfile::tempdir().unwrap();
    // Neither source exists. A later declaration/budget error must take precedence.
    let mut inventory = Inventory {
        limits: limits(),
        files: vec![entry("a", "missing-a"), entry("b", "missing-b")],
    };
    inventory.files[1].declaration.headers[0].value = "4".into();
    assert!(matches!(
        prepare_inventory(inventory, dir.path()),
        Err(InventoryError::Preparation {
            error: PreparationError::Metadata(UploadMetadataError::LengthMismatch),
            ..
        })
    ));
    for limit in [
        InventoryLimit::Files,
        InventoryLimit::FileBytes,
        InventoryLimit::FileChunks,
        InventoryLimit::TotalBytes,
        InventoryLimit::TotalChunks,
    ] {
        let mut inventory = Inventory {
            limits: limits(),
            files: vec![entry("a", "missing-a"), entry("b", "missing-b")],
        };
        match limit {
            InventoryLimit::Files => inventory.limits.files = NonZeroUsize::new(1).unwrap(),
            InventoryLimit::FileBytes => inventory.limits.file_bytes = NonZeroU64::new(2).unwrap(),
            InventoryLimit::FileChunks => {
                let file = &mut inventory.files[0];
                file.declaration.bytes = CAFFEINE_CHUNK_BYTES as u64 + 1;
                file.declaration.headers[0].value = file.declaration.bytes.to_string();
                inventory.limits.file_bytes = NonZeroU64::new(file.declaration.bytes).unwrap();
            }
            InventoryLimit::TotalBytes => {
                inventory.limits.total_bytes = NonZeroU64::new(5).unwrap();
            }
            InventoryLimit::TotalChunks => {
                inventory.limits.total_chunks = NonZeroU64::new(1).unwrap();
            }
        }
        assert!(
            matches!(prepare_inventory(inventory, dir.path()), Err(InventoryError::Limit(actual)) if actual == limit)
        );
    }
    assert!(matches!(
        add(u64::MAX, 1, u64::MAX, InventoryLimit::TotalBytes),
        Err(InventoryError::Limit(InventoryLimit::TotalBytes))
    ));
}

#[test]
fn duplicate_asset_ids_unsafe_paths_and_nonregular_sources_reject() {
    let dir = tempfile::tempdir().unwrap();
    let inventory = Inventory {
        limits: limits(),
        files: vec![entry("same", "a"), entry("same", "b")],
    };
    assert!(matches!(
        prepare_inventory(inventory, dir.path()),
        Err(InventoryError::Asset)
    ));
    for path in ["", "..", "../outside", "/absolute", "a/../../outside"] {
        let inventory = Inventory {
            limits: limits(),
            files: vec![entry("a", path)],
        };
        assert!(matches!(
            prepare_inventory(inventory, dir.path()),
            Err(InventoryError::Source)
        ));
    }
    fs::create_dir(dir.path().join("folder")).unwrap();
    let inventory = Inventory {
        limits: limits(),
        files: vec![entry("a", "folder")],
    };
    assert!(matches!(
        prepare_inventory(inventory, dir.path()),
        Err(InventoryError::SourceFile { asset, reason, .. }) if asset == "a" && matches!(*reason, InventoryError::Source)
    ));
    fs::write(dir.path().join("short"), b"ab").unwrap();
    let inventory = Inventory {
        limits: limits(),
        files: vec![entry("a", "short")],
    };
    assert!(matches!(
        prepare_inventory(inventory, dir.path()),
        Err(InventoryError::SourceFile { asset, path, reason }) if asset == "a" && path == Path::new("short") && matches!(*reason, InventoryError::SourceLength)
    ));
}

#[cfg(unix)]
#[test]
fn symlinks_in_leaf_or_parent_components_are_rejected() {
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("real")).unwrap();
    fs::write(dir.path().join("real/file"), b"abc").unwrap();
    symlink("real/file", dir.path().join("leaf")).unwrap();
    symlink("real", dir.path().join("parent")).unwrap();
    for path in ["leaf", "parent/file"] {
        let inventory = Inventory {
            limits: limits(),
            files: vec![entry("a", path)],
        };
        assert!(matches!(
            prepare_inventory(inventory, dir.path()),
            Err(InventoryError::SourceFile { reason, .. }) if matches!(*reason, InventoryError::Source)
        ));
    }
}

#[test]
fn file_boundary_bounds_json_and_rejects_ambiguous_input() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("inventory.json");
    fs::write(
        &input,
        vec![b' '; usize::try_from(INVENTORY_BYTES).unwrap() + 1],
    )
    .unwrap();
    assert!(matches!(run(&input), Err(InventoryError::EncodedLimit)));
    let limits = serde_json::json!({ "files": 1, "file_bytes": 3, "file_chunks": 1, "total_bytes": 3, "total_chunks": 1 });
    fs::write(
        &input,
        serde_json::to_vec(&serde_json::json!({ "limits": limits, "files": [] })).unwrap(),
    )
    .unwrap();
    assert!(matches!(run(&input), Err(InventoryError::Empty)));
    fs::write(
        &input,
        serde_json::to_vec(&serde_json::json!({ "limits": limits, "files": [], "surprise": true }))
            .unwrap(),
    )
    .unwrap();
    assert!(matches!(run(&input), Err(InventoryError::Json(_))));
}
