use super::*;
const TOO_LARGE: &str = "340282366920938463463374607431768211456";
#[test]
fn candid_numbers_preserve_zero_and_full_unsigned_range() {
    for value in [0, 1, u64::MAX.into(), u128::MAX] {
        assert_eq!(
            cycles_from_nat(BillingField::ObservedBalance, &Nat::from(value)),
            Ok(value)
        );
        assert_eq!(cycles_from_int(&Int::from(value)), Ok(value));
    }
    for text in ["-1", "-340282366920938463463374607431768211456", TOO_LARGE] {
        let value = Int::parse(text.as_bytes()).expect("integer fixture");
        assert_eq!(
            cycles_from_int(&value),
            Err(BillingInputError::OutOfRange {
                field: BillingField::ObservedBalance,
            })
        );
    }
    let oversized = Nat::parse(TOO_LARGE.as_bytes()).expect("nat fixture");
    assert_eq!(
        cycles_from_nat(BillingField::ObservedBalance, &oversized),
        Err(BillingInputError::OutOfRange {
            field: BillingField::ObservedBalance,
        })
    );
}
