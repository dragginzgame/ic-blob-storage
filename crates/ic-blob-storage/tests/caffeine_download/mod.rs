//! Independent client roots verified under arbitrary transport framing, without
//! reading the fixture's raw digest or chunk manifest as verifier inputs.
use super::*;
use ic_blob_storage::model::identity::caffeine::verification::CaffeineRootVerifier;

#[test]
fn streamed_client_vectors_verify_without_a_manifest_or_expected_raw_digest() {
    for vector in fixture().vectors {
        let mut state = CaffeineRootVerifier::new(
            vector.provider_root.parse().unwrap(),
            u64::try_from(vector.bytes).unwrap(),
            &headers(&vector),
            CaffeineHashLimits {
                max_content_bytes: NonZeroU64::new(32 * 1024 * 1024).unwrap(),
                max_append_bytes: bound(65_537),
                max_headers: bound(8),
                max_header_bytes: bound(1024),
            },
        )
        .unwrap();
        // Fixed-size receive frame; its prime length crosses provider leaves.
        let mut frame = vec![0; 65_537];
        let mut offset = 0;
        while offset < vector.bytes {
            let take = frame.len().min(vector.bytes - offset);
            for (index, byte) in frame[..take].iter_mut().enumerate() {
                *byte = match vector.pattern {
                    Pattern::Abc => b"abc"[offset + index],
                    Pattern::Zeroes => 0,
                    Pattern::LinearMod251 => {
                        u8::try_from(((offset + index) * 31 + 7) % 251).unwrap()
                    }
                };
            }
            state
                .append(u64::try_from(offset).unwrap(), &frame[..take])
                .unwrap();
            offset += take;
        }
        let actual = state.finish().unwrap();
        assert_eq!(
            actual.provider_root.to_string(),
            vector.provider_root,
            "{}",
            vector.name
        );
        // Independent comparison only after completion; this was never trusted
        // by the verifier or derived from a leaf manifest supplied by the test.
        assert_eq!(
            actual.content_digest.to_string(),
            vector.raw_digest,
            "{}",
            vector.name
        );
    }
}
