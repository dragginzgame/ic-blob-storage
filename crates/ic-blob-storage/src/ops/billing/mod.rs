//! Bounded provider-number conversion; installation inputs use Contracts validation.
use ic_platform::candid::contract::{Int, Nat};
use thiserror::Error;

pub mod balance;

/// Numeric field rejected at the provider boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BillingField {
    /// One observed balance component; its provider provenance is separate.
    ObservedBalance,
}

/// A provider number cannot be represented without truncation or clamping.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BillingInputError {
    /// A negative integer or a value exceeding the internal cycle range.
    #[error("{field:?} is outside the unsigned 128-bit cycle range")]
    OutOfRange {
        /// Numeric input that could not be represented.
        field: BillingField,
    },
}

/// Convert an unsigned Candid field without truncation or clamping.
///
/// # Errors
/// Returns a field-specific error if the number exceeds `u128::MAX`.
pub fn cycles_from_nat(field: BillingField, value: &Nat) -> Result<u128, BillingInputError> {
    u128::try_from(&value.0).map_err(|_| BillingInputError::OutOfRange { field })
}

/// Convert a signed Candid balance component without accepting debt as zero.
///
/// This conversion establishes neither balance freshness nor transfer completion.
/// # Errors
/// Rejects negative balances and values exceeding `u128::MAX`.
pub fn cycles_from_int(value: &Int) -> Result<u128, BillingInputError> {
    u128::try_from(&value.0).map_err(|_| BillingInputError::OutOfRange {
        field: BillingField::ObservedBalance,
    })
}

#[cfg(test)]
mod tests {
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
}
