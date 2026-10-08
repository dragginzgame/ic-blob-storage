//! Compare every computed leaf and both identities with independent JS vectors.
use super::*;
use ic_blob_storage_contracts::identity::caffeine::manifest::builder::CaffeineManifestBuilder;

#[test]
fn prepared_manifests_match_independent_vectors_under_unaligned_frames() {
    for vector in fixture().vectors {
        let limits = CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(32 * 1024 * 1024).unwrap(),
            max_append_bytes: bound(65_537),
            max_headers: bound(8),
            max_header_bytes: bound(1024),
        };
        let mut builder = CaffeineManifestBuilder::new(
            vector.bytes as u64,
            &headers(&vector),
            limits,
            bound(vector.chunk_hashes.len()),
        )
        .unwrap();
        let mut buffer = vec![0; limits.max_append_bytes.get()];
        let mut offset = 0;
        while offset < vector.bytes {
            let count = buffer.len().min(vector.bytes - offset);
            for (index, byte) in buffer[..count].iter_mut().enumerate() {
                *byte = match vector.pattern {
                    Pattern::Abc => b"abc"[offset + index],
                    Pattern::Zeroes => 0,
                    Pattern::LinearMod251 => {
                        u8::try_from(((offset + index) * 31 + 7) % 251).unwrap()
                    }
                };
            }
            builder.append(offset as u64, &buffer[..count]).unwrap();
            offset += count;
        }
        let output = builder.finish().unwrap();
        assert_eq!(
            output.hashes().content_digest.to_string(),
            vector.raw_digest,
            "{}",
            vector.name
        );
        assert_eq!(
            output.hashes().provider_root.to_string(),
            vector.provider_root,
            "{}",
            vector.name
        );
        assert_eq!(output.manifest().content_bytes(), vector.bytes as u64);
        assert_eq!(
            output
                .manifest()
                .chunks()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            vector.chunk_hashes,
            "{}",
            vector.name
        );
        // Same bounded declaration constructor used by service admission accepts it.
        let admitted = CaffeineChunkManifest::new(
            output.hashes().provider_root,
            vector.bytes as u64,
            output.manifest().chunks(),
            &headers(&vector),
            CaffeineManifestLimits {
                max_content_bytes: limits.max_content_bytes,
                max_chunks: bound(vector.chunk_hashes.len()),
                max_headers: limits.max_headers,
                max_header_bytes: limits.max_header_bytes,
            },
        )
        .unwrap();
        assert_eq!(&admitted, output.manifest());
    }
}
