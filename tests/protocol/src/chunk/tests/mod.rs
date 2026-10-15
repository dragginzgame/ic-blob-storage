use super::*;

#[test]
fn caller_byte_and_work_budgets_remain_independent() {
    let bytes = candid::encode_one(vec![42_u8; 1024]).unwrap();
    assert_eq!(
        decode_reply(&bytes, bytes.len(), 32_768).unwrap(),
        vec![42; 1024]
    );
    assert_eq!(
        decode_reply(&bytes, bytes.len() - 1, 32_768),
        Err(ChunkReplyError::TooLarge)
    );
    assert_eq!(
        decode_reply(&bytes, bytes.len(), 0),
        Err(ChunkReplyError::Invalid)
    );
}

#[test]
fn malformed_and_non_blob_values_do_not_become_chunk_bytes() {
    for encoded in [
        b"not-candid".to_vec(),
        candid::encode_one(vec![1_u16, 2]).unwrap(),
        candid::encode_one(vec![(); 100_000]).unwrap(),
    ] {
        assert_eq!(
            decode_reply(&encoded, 4096, 32_768),
            Err(ChunkReplyError::Invalid)
        );
    }
}
