use super::*;
use ic_platform::candid::contract::{encode_args, encode_one};
#[test]
fn callback_input_bounds_preserve_binary_roots_order_and_duplicates() {
    let roots = vec![vec![1u8; 32], vec![2; 32], vec![1; 32]];
    assert_eq!(decode_roots(&encode_one(&roots).unwrap()), Ok(roots));
    assert_eq!(
        decode_roots(&encode_one(Vec::<Vec<u8>>::new()).unwrap()),
        Ok(vec![])
    );
    assert_eq!(decode_empty(&encode_args(()).unwrap()), Ok(()));
    for malformed in [vec![], vec![1u8; 31], vec![1; 33]] {
        assert_eq!(
            decode_roots(&encode_one(vec![malformed]).unwrap()),
            Err(GcInputError::MalformedRoot)
        );
    }
    let maximum = vec![vec![1u8; 32]; MAX_ROOTS];
    assert_eq!(decode_roots(&encode_one(&maximum).unwrap()), Ok(maximum));
    assert!(matches!(
        decode_roots(&encode_one(vec![vec![1u8; 32]; MAX_ROOTS + 1]).unwrap()),
        Err(GcInputError::Batch(_))
    ));
    assert_eq!(
        decode_roots(&vec![0; MAX_REQUEST_BYTES + 1]),
        Err(GcInputError::TooLarge)
    );
    assert_eq!(decode_roots(b"bad"), Err(GcInputError::Invalid));
    assert!(decode_empty(&encode_one(123u64).unwrap()).is_err());
    assert!(decode_roots(&encode_args((vec![vec![1u8; 32]], 5u64)).unwrap()).is_err());
}
