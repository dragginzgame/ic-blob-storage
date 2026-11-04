use crate::identity::verification::*;

#[test]
fn restore_preserves_buffered_bytes_at_sha_padding_and_block_boundaries() {
    let bytes: Vec<u8> = (0..=255).cycle().take(4097).collect();
    let expected = ContentDigest::compute(&bytes);
    for offset in [0, 1, 55, 56, 63, 64, 65, 119, 120, 127, 128, 4096, 4097] {
        let mut original = ContentVerifier::new(expected, 4097).unwrap();
        original.append(0, &bytes[..offset]).unwrap();
        let mut restored = original.checkpoint_state().restore().expect("valid state");
        assert_eq!(restored.received_bytes(), u64::try_from(offset).unwrap());
        restored
            .append(u64::try_from(offset).unwrap(), &bytes[offset..])
            .unwrap();
        assert_eq!(restored.finish(), Ok(expected));
    }
}
