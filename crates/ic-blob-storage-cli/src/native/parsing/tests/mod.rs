use super::*;

#[test]
fn canonical_values() {
    for value in ["0", "01", "+1", "-1", " 1", "1 "] {
        assert_eq!(positive::<u128>(value), Err(Failure::Arguments));
    }
    assert_eq!(positive::<u128>(&u128::MAX.to_string()), Ok(u128::MAX));
    // Syntax parsing does not grant authority; each command still checks its roles.
    assert_eq!(principal("aaaaa-aa"), Ok(Principal::management_canister()));
    assert_eq!(principal("2vxsx-fae"), Ok(Principal::anonymous()));
    assert_eq!(principal("AAAAA-AA"), Err(Failure::Arguments));
}
