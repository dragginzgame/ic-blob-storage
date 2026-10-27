use super::*;

#[test]
fn positional_requests_and_responses_bind_method_direction_and_arity() {
    let bytes = encode_args("configure", (7_u64, "fixture".to_owned())).unwrap();
    assert_eq!(
        request::<(u64, String)>("configure", &bytes, MAX_BYTES).unwrap(),
        (7, "fixture".into())
    );
    assert!(request::<(u64, String)>("resume", &bytes, MAX_BYTES).is_err());
    assert!(decode_args::<(u64, String)>("configure", &bytes, MAX_BYTES).is_err());
    let reply = encode_reply("configure", true).unwrap();
    assert!(decode_one::<bool>("configure", &reply, MAX_BYTES).unwrap());
    assert!(request::<(bool,)>("configure", &reply, MAX_BYTES).is_err());
    let empty = encode_args("resume", ()).unwrap();
    request::<()>("resume", &empty, MAX_BYTES).unwrap();
    assert!(request::<()>("resume", &[], MAX_BYTES).is_err());
    assert!(request::<()>("resume", b"DIDL\0\0", MAX_BYTES).is_err());
}

#[test]
fn malformed_trailing_and_over_budget_frames_refuse_without_fallback() {
    let bytes = encode_one("configure", 7_u64).unwrap();
    for end in 0..bytes.len() {
        assert!(request::<(u64,)>("configure", &bytes[..end], MAX_BYTES).is_err());
    }
    assert!(request::<(u64,)>("configure", &bytes, bytes.len() - 1).is_err());
    let mut trailing = bytes;
    trailing.push(0);
    assert!(request::<(u64,)>("configure", &trailing, MAX_BYTES).is_err());
    let extra = response("configure", (true, 7_u64)).unwrap();
    assert!(decode_one::<bool>("configure", &extra, MAX_BYTES).is_err());
    let value = vec![(); 262_145];
    assert!(encode_one("configure", value).is_err());
}

#[test]
fn lifecycle_identities_are_separate_from_rpc_and_each_other() {
    let bytes = encode_args("fixture_init", (7_u64,)).unwrap();
    assert!(request::<(u64,)>("fixture_post_upgrade", &bytes, MAX_BYTES).is_err());
    assert!(request::<(u64,)>("configure", &bytes, MAX_BYTES).is_err());
}

#[test]
fn entrypoint_hooks_bind_static_identity_without_message_context() {
    const CONFIGURE: u64 = identity("configure");
    let input = encode_one("configure", 7_u64).unwrap();
    assert_eq!(hooks::one::<u64, CONFIGURE>(input), 7);
    let reply = hooks::reply::<_, CONFIGURE>(true);
    assert!(decode_one::<bool>("configure", &reply, MAX_BYTES).unwrap());
    hooks::unit::<CONFIGURE>(encode_args("configure", ()).unwrap());
    assert_eq!(
        hooks::init_one::<u64>(encode_one("fixture_init", 7_u64).unwrap()),
        7
    );
    assert_eq!(
        hooks::upgrade_one::<u64>(encode_one("fixture_post_upgrade", 7_u64).unwrap()),
        7
    );
}

#[test]
fn bulk_bytes_keep_exact_encoding_and_admit_a_complete_fixture_leaf() {
    let small = vec![0_u8, 127, 128, 255];
    assert_eq!(
        encode_one("journey_append", &small).unwrap(),
        encode_one(
            "journey_append",
            ic_wire::BytesRef::<16>::new(&small).unwrap()
        )
        .unwrap()
    );
    let bytes = vec![0xa5; 1024 * 1024];
    let frame = encode_one(
        "journey_append",
        ic_wire::BytesRef::<{ 1024 * 1024 }>::new(&bytes).unwrap(),
    )
    .unwrap();
    let (decoded,): (ic_wire::ByteBuf,) = request("journey_append", &frame, MAX_BYTES).unwrap();
    assert_eq!(decoded.as_slice(), bytes);
    assert!(request::<(ic_wire::ByteBuf,)>("journey_append", &frame, frame.len() - 1).is_err());
}
