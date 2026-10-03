use super::*;

#[test]
fn canonical_values_and_unique_complete_flags() {
    for value in ["0", "01", "+1", "-1", " 1", "1 "] {
        assert_eq!(positive::<u128>(value), Err(Failure::Arguments));
    }
    assert_eq!(positive::<u128>(&u128::MAX.to_string()), Ok(u128::MAX));
    for values in [vec!["--a"], vec!["--a", "1", "--a", "2"]] {
        let values = values.into_iter().map(str::to_owned).collect::<Vec<_>>();
        assert_eq!(flags(&values), Err(Failure::Arguments));
    }
    // Syntax parsing does not grant authority; each command still checks its roles.
    assert_eq!(principal("aaaaa-aa"), Ok(Principal::management_canister()));
    assert_eq!(principal("2vxsx-fae"), Ok(Principal::anonymous()));
    assert_eq!(principal("AAAAA-AA"), Err(Failure::Arguments));
}
