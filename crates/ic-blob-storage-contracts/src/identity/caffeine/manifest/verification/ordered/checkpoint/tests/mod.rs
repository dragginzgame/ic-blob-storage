use crate::identity::caffeine::CaffeineHeader;
use crate::identity::caffeine::manifest::CaffeineChunkHash;
use crate::identity::caffeine::manifest::CaffeineManifestLimits;
use crate::identity::caffeine::manifest::verification::ordered::checkpoint::*;
use std::num::NonZeroU64;
use std::num::NonZeroUsize;

fn vector(name: &str) -> (CaffeineChunkManifest, ContentDigest, Vec<u8>) {
    let all: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-hashing/vectors.json"
    )))
    .unwrap();
    let v = all["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == name)
        .unwrap();
    let len = v["bytes"].as_u64().unwrap();
    let bytes = (0..usize::try_from(len).unwrap())
        .map(|i| match v["pattern"].as_str().unwrap() {
            "abc" => b"abc"[i],
            "linear_mod251" => u8::try_from((i * 31 + 7) % 251).unwrap(),
            _ => panic!("selected independent pattern"),
        })
        .collect();
    let chunks: Vec<CaffeineChunkHash> = v["chunk_hashes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| h.as_str().unwrap().parse().unwrap())
        .collect();
    let headers: Vec<_> = v["headers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| CaffeineHeader {
            name: h["name"].as_str().unwrap(),
            value: h["value"].as_str().unwrap(),
        })
        .collect();
    let manifest = CaffeineChunkManifest::new(
        v["provider_root"].as_str().unwrap().parse().unwrap(),
        len,
        &chunks,
        &headers,
        CaffeineManifestLimits {
            max_content_bytes: NonZeroU64::new(6 * 1024 * 1024).unwrap(),
            max_chunks: NonZeroUsize::new(6).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
    )
    .unwrap();
    (
        manifest,
        v["raw_digest"].as_str().unwrap().parse().unwrap(),
        bytes,
    )
}

fn reload(
    verifier: CaffeineOrderedChunkVerifier,
    expected: ContentDigest,
) -> CaffeineOrderedChunkVerifier {
    let bytes = verifier.checkpoint().as_bytes().to_vec();
    let manifest = verifier.manifest().clone();
    drop(verifier);
    let record = CaffeineVerificationCheckpointRecord::try_from(bytes.as_slice()).unwrap();
    CaffeineOrderedChunkVerifier::from_checkpoint(manifest, expected, &record).unwrap()
}

#[test]
fn independent_vectors_resume_at_every_leaf_and_final_buffer_boundary() {
    for name in [
        "abc-text",
        "ecmascript-metadata",
        "pattern-1048576",
        "pattern-1048577",
        "uneven-tree-with-metadata",
    ] {
        let (manifest, digest, bytes) = vector(name);
        let root = manifest.root();
        let mut verifier = reload(CaffeineOrderedChunkVerifier::new(manifest, digest), digest);
        for (index, chunk) in bytes.chunks(CAFFEINE_CHUNK_BYTES).enumerate() {
            let before = verifier.checkpoint();
            let mut corrupt = chunk.to_vec();
            corrupt[0] ^= 1;
            assert!(
                verifier
                    .append_chunk(u64::try_from(index).unwrap(), &corrupt)
                    .is_err()
            );
            assert_eq!(verifier.checkpoint(), before);
            verifier
                .append_chunk(u64::try_from(index).unwrap(), chunk)
                .unwrap();
            verifier = reload(verifier, digest);
            assert_eq!(verifier.next_chunk(), u64::try_from(index + 1).unwrap());
            assert!(
                verifier
                    .append_chunk(u64::try_from(index).unwrap(), chunk)
                    .is_err()
            );
        }
        let result = verifier.finish().unwrap();
        assert_eq!(result.content_digest, digest);
        assert_eq!(result.provider_root, root);
    }
}

#[test]
fn raw_digest_and_manifest_bindings_cannot_be_replaced_during_restore() {
    let (manifest, digest, bytes) = vector("abc-text");
    let (other, _, _) = vector("abc-binary");
    let wrong = ContentDigest::compute(b"wrong");
    let mut verifier = CaffeineOrderedChunkVerifier::new(manifest.clone(), wrong);
    verifier.append_chunk(0, &bytes).unwrap();
    let record = verifier.checkpoint();
    assert_eq!(
        CaffeineOrderedChunkVerifier::from_checkpoint(manifest.clone(), digest, &record).err(),
        Some(CaffeineCheckpointError::WrongDeclaration)
    );
    assert_eq!(
        CaffeineOrderedChunkVerifier::from_checkpoint(other, wrong, &record).err(),
        Some(CaffeineCheckpointError::WrongDeclaration)
    );
    let restored = CaffeineOrderedChunkVerifier::from_checkpoint(manifest, wrong, &record).unwrap();
    assert!(matches!(
        restored.finish(),
        Err(super::super::CaffeineOrderedVerificationError::Content(
            crate::identity::verification::ContentVerificationError::DigestMismatch { .. }
        ))
    ));
}

fn reseal(bytes: &mut [u8]) {
    let checksum = Sha256::digest(&bytes[..CHECKSUM_OFFSET]);
    bytes[CHECKSUM_OFFSET..].copy_from_slice(&checksum);
}

#[test]
fn bounded_decoding_rejects_damage_and_inconsistent_counts_even_with_a_new_checksum() {
    let (manifest, digest, bytes) = vector("abc-text");
    let mut verifier = CaffeineOrderedChunkVerifier::new(manifest, digest);
    verifier.append_chunk(0, &bytes).unwrap();
    let good = verifier.checkpoint().as_bytes().to_vec();
    for size in [0, good.len() - 1, good.len() + 1] {
        assert_eq!(
            CaffeineVerificationCheckpointRecord::try_from(vec![0; size].as_slice()).err(),
            Some(CaffeineCheckpointError::InvalidLength)
        );
    }
    for (at, error) in [
        (0, CaffeineCheckpointError::InvalidFormat),
        (8, CaffeineCheckpointError::WrongRelease),
        (40, CaffeineCheckpointError::ChecksumMismatch),
        (HASH_OFFSET, CaffeineCheckpointError::ChecksumMismatch),
    ] {
        let mut bad = good.clone();
        bad[at] ^= 1;
        assert_eq!(
            CaffeineVerificationCheckpointRecord::try_from(bad.as_slice()).err(),
            Some(error)
        );
    }
    for (range, value) in [
        (104..112, u64::MAX),
        (112..120, 4),
        (HASH_OFFSET + 32..HASH_OFFSET + 40, u64::MAX),
    ] {
        let mut bad = good.clone();
        bad[range].copy_from_slice(&value.to_le_bytes());
        reseal(&mut bad);
        assert_eq!(
            CaffeineVerificationCheckpointRecord::try_from(bad.as_slice()).err(),
            Some(CaffeineCheckpointError::InvalidState)
        );
    }
    for (at, value) in [(HASH_OFFSET + 40, 64), (HASH_OFFSET + 44, 1)] {
        let mut bad = good.clone();
        bad[at] = value;
        reseal(&mut bad);
        assert_eq!(
            CaffeineVerificationCheckpointRecord::try_from(bad.as_slice()).err(),
            Some(CaffeineCheckpointError::InvalidState)
        );
    }
    // A mathematically valid partial SHA block is not a checked complete leaf.
    let mut partial = good;
    partial[104..112].copy_from_slice(&4_u64.to_le_bytes());
    reseal(&mut partial);
    assert_eq!(
        CaffeineVerificationCheckpointRecord::try_from(partial.as_slice()).err(),
        Some(CaffeineCheckpointError::InvalidState)
    );
}

#[test]
fn same_root_does_not_allow_a_different_declared_length() {
    let (manifest, digest, _) = vector("abc-no-metadata");
    let longer = CaffeineChunkManifest::new(
        manifest.root(),
        4,
        &[CaffeineChunkHash::try_from(manifest.root().as_bytes().as_slice()).unwrap()],
        &[],
        CaffeineManifestLimits {
            max_content_bytes: NonZeroU64::new(4).unwrap(),
            max_chunks: NonZeroUsize::new(1).unwrap(),
            max_headers: NonZeroUsize::new(1).unwrap(),
            max_header_bytes: NonZeroUsize::new(1).unwrap(),
        },
    )
    .unwrap();
    let checkpoint = CaffeineOrderedChunkVerifier::new(manifest, digest).checkpoint();
    assert_eq!(
        CaffeineOrderedChunkVerifier::from_checkpoint(longer, digest, &checkpoint).err(),
        Some(CaffeineCheckpointError::WrongDeclaration)
    );
}
