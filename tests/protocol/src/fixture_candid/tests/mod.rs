use super::*;

#[test]
fn positional_decoding_and_reply_arity_preserve_existing_fixture_bytes() {
    let bytes = contract::encode_args((7_u64, "fixture".to_owned())).unwrap();
    assert_eq!(decode_args::<(u64, String)>(bytes), (7, "fixture".into()));
    let bytes = contract::encode_one(9_u64).unwrap();
    assert_eq!(decode_one::<u64>(bytes), 9);
    assert_eq!(decode_args::<()>(vec![]), ());
    assert_eq!(decode_one::<Option<u64>>(vec![]), None);
    assert_eq!(encode_one(9_u64), contract::encode_one(9_u64).unwrap());
    assert_eq!(encode_unit(), contract::encode_one(()).unwrap());
    assert_ne!(encode_unit(), contract::encode_args(()).unwrap());
    ignore_arguments(vec![0xff]);
}

#[test]
fn malformed_or_over_skipping_budget_arguments_trap_before_dispatch() {
    assert!(std::panic::catch_unwind(|| decode_one::<u64>(vec![0xff])).is_err());
    let bytes = contract::encode_args((7_u64, vec![1_u64; 20_000])).unwrap();
    assert!(std::panic::catch_unwind(|| decode_one::<u64>(bytes)).is_err());
}
