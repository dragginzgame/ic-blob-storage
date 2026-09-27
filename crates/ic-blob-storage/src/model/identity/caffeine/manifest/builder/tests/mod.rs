use super::*;
use crate::model::identity::caffeine::CaffeineHashLimit;
use std::num::NonZeroU64;

fn limits() -> CaffeineHashLimits {
    CaffeineHashLimits {
        max_content_bytes: NonZeroU64::new(3 * CAFFEINE_CHUNK_BYTES as u64).unwrap(),
        max_append_bytes: NonZeroUsize::new(2 * CAFFEINE_CHUNK_BYTES).unwrap(),
        max_headers: NonZeroUsize::new(2).unwrap(),
        max_header_bytes: NonZeroUsize::new(128).unwrap(),
    }
}

#[test]
fn content_and_leaf_limits_reject_before_preparation() {
    let one = NonZeroUsize::new(1).unwrap();
    assert!(matches!(
        CaffeineManifestBuilder::new(0, &[], limits(), one),
        Err(CaffeineManifestBuilderError::Hash(
            CaffeineHashError::EmptyContentUnqualified
        ))
    ));
    assert!(matches!(
        CaffeineManifestBuilder::new(u64::MAX, &[], limits(), one),
        Err(CaffeineManifestBuilderError::Hash(
            CaffeineHashError::Limit(CaffeineHashLimit::ContentBytes)
        ))
    ));
    assert!(matches!(
        CaffeineManifestBuilder::new(CAFFEINE_CHUNK_BYTES as u64 + 1, &[], limits(), one),
        Err(CaffeineManifestBuilderError::TooManyChunks)
    ));
    let headers = [
        CaffeineHeader {
            name: "x",
            value: "a",
        },
        CaffeineHeader {
            name: "x",
            value: "b",
        },
    ];
    assert!(matches!(
        CaffeineManifestBuilder::new(1, &headers, limits(), one),
        Err(CaffeineManifestBuilderError::Hash(
            CaffeineHashError::DuplicateHeader
        ))
    ));
}

#[test]
fn one_frame_can_finish_multiple_leaves_and_rejections_preserve_collected_leaves() {
    let body = vec![7; 2 * CAFFEINE_CHUNK_BYTES];
    let length = body.len() as u64 + 1;
    let mut builder =
        CaffeineManifestBuilder::new(length, &[], limits(), NonZeroUsize::new(3).unwrap()).unwrap();
    builder.append(0, &body).unwrap();
    assert_eq!(builder.chunks.len(), 2);
    let leaves = builder.chunks.clone();
    let offset = builder.received_bytes();
    assert_eq!(
        builder.append(0, &[9]),
        Err(CaffeineHashError::UnexpectedOffset {
            expected: offset,
            actual: 0
        })
    );
    assert_eq!(
        builder.append(offset, &[9, 9]),
        Err(CaffeineHashError::ExceedsLength)
    );
    let excessive = vec![0; limits().max_append_bytes.get() + 1];
    assert_eq!(
        builder.append(offset, &excessive),
        Err(CaffeineHashError::Limit(CaffeineHashLimit::AppendBytes))
    );
    assert_eq!(builder.received_bytes(), offset);
    assert_eq!(builder.remaining_bytes(), 1);
    assert_eq!(builder.chunks, leaves);
    builder.append(offset, &[9]).unwrap();
    let output = builder.finish().unwrap();
    assert_eq!(output.manifest().chunk_count(), 3);
    assert_eq!(&output.manifest().chunks()[..2], leaves);
    output
        .manifest()
        .verify_chunk(0, &body[..CAFFEINE_CHUNK_BYTES])
        .unwrap();
    output
        .manifest()
        .verify_chunk(1, &body[CAFFEINE_CHUNK_BYTES..])
        .unwrap();
    output.manifest().verify_chunk(2, &[9]).unwrap();
}

#[test]
fn incomplete_finish_returns_no_partial_manifest() {
    let mut builder =
        CaffeineManifestBuilder::new(3, &[], limits(), NonZeroUsize::new(1).unwrap()).unwrap();
    builder.append(0, b"ab").unwrap();
    assert_eq!(
        builder.finish(),
        Err(CaffeineHashError::Incomplete { remaining: 1 })
    );
}
