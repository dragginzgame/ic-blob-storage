use super::*;
use crate::fixture_wire;

#[test]
fn caller_byte_budget_and_method_binding_admit_bulk_leaf() {
    let value = ic_wire::ByteBuf::from(vec![42_u8; 1024]);
    let bytes = fixture_wire::encode_reply("fixture_chunk", value.clone()).unwrap();
    assert_eq!(decode_reply(&bytes, bytes.len()).unwrap(), value.into_vec());
    assert_eq!(
        decode_reply(&bytes, bytes.len() - 1),
        Err(ChunkReplyError::TooLarge)
    );
    let wrong =
        fixture_wire::encode_reply("configure_read", ic_wire::ByteBuf::from(vec![42; 1024]))
            .unwrap();
    assert_eq!(
        decode_reply(&wrong, wrong.len()),
        Err(ChunkReplyError::Invalid)
    );
}

#[test]
fn malformed_wrong_shape_and_old_candid_never_become_chunk_bytes() {
    for encoded in [
        b"not-wire".to_vec(),
        ic_platform::candid::contract::encode_one(vec![1_u8, 2]).unwrap(),
        fixture_wire::encode_reply("fixture_chunk", ()).unwrap(),
        fixture_wire::encode_one("fixture_chunk", ic_wire::ByteBuf::from(vec![1, 2])).unwrap(),
        fixture_wire::response("fixture_chunk", (ic_wire::ByteBuf::from(vec![1, 2]), true))
            .unwrap(),
    ] {
        assert_eq!(decode_reply(&encoded, 4096), Err(ChunkReplyError::Invalid));
    }
}
