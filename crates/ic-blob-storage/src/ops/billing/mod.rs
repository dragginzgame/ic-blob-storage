//! Convert billing inputs before applying model invariants or pure policy.
//!
//! These helpers neither define a provider wire contract nor perform effects.
//! Candid decoding/resource limits remain the caller's responsibility. Conversion
//! borrows arbitrary-precision values without cloning their integer storage.

use std::num::NonZeroU128;

use candid::{Int, Nat};
use thiserror::Error;

use crate::model::billing::{FundingLimits, FundingLimitsError};

pub mod balance;
pub mod configuration;

/// Numeric field rejected at a billing boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BillingField {
    /// Local cycle reserve.
    Reserve,
    /// Provider balance threshold.
    MinimumBalance,
    /// Desired provider balance after funding.
    TargetBalance,
    /// Explicit positive funding amount.
    RequestedFunding,
    /// One observed balance component; its provider provenance is separate.
    ObservedBalance,
}

/// Borrowed numeric configuration inputs, without provider identity or defaults.
#[derive(Clone, Copy, Debug)]
pub struct FundingLimitsInput<'a> {
    /// Local cycle reserve.
    pub reserve: &'a Nat,
    /// Minimum provider balance.
    pub minimum_balance: &'a Nat,
    /// Target provider balance.
    pub target_balance: &'a Nat,
}

/// Typed rejection of a billing input; no invalid value becomes zero.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BillingInputError {
    /// Empty operator funding input.
    #[error("funding amount is empty")]
    EmptyDecimal,
    /// Operator input contains a byte other than an ASCII decimal digit.
    #[error("funding amount contains a non-decimal byte at offset {byte_offset}")]
    InvalidDecimal {
        /// Byte offset of the first non-digit, including whitespace or signs.
        byte_offset: usize,
    },
    /// A negative integer or a value exceeding the internal cycle range.
    #[error("{field:?} is outside the unsigned 128-bit cycle range")]
    OutOfRange {
        /// Numeric input that could not be represented.
        field: BillingField,
    },
    /// Zero is valid for an observed balance, but never for a funding request.
    #[error("funding amount must be positive")]
    ZeroFunding,
    /// Representable configuration numbers violate model invariants.
    #[error(transparent)]
    InvalidLimits(#[from] FundingLimitsError),
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

/// Convert all numeric fields, then apply the model's configuration invariants.
///
/// No configuration is installed or mutated by this function.
/// # Errors
/// Rejects unrepresentable numbers and invalid reserve/minimum/target values.
pub fn funding_limits_from_candid(
    input: FundingLimitsInput<'_>,
) -> Result<FundingLimits, BillingInputError> {
    let reserve = cycles_from_nat(BillingField::Reserve, input.reserve)?;
    let minimum = cycles_from_nat(BillingField::MinimumBalance, input.minimum_balance)?;
    let target = cycles_from_nat(BillingField::TargetBalance, input.target_balance)?;
    Ok(FundingLimits::new(reserve, minimum, target)?)
}

/// Convert a Candid funding amount into the positive input required by policy.
///
/// # Errors
/// Rejects zero or a number exceeding `u128::MAX`.
pub fn funding_from_nat(value: &Nat) -> Result<NonZeroU128, BillingInputError> {
    positive_funding(cycles_from_nat(BillingField::RequestedFunding, value)?)
}

/// Parse the operator's positive, unsigned ASCII decimal cycle amount.
///
/// Leading zeros are accepted, as in Canic. Signs, whitespace, separators,
/// fractional/exponent notation and non-ASCII digits are rejected. Input length
/// is bounded by the caller's command/request limit, not an arbitrary digit cap.
/// # Errors
/// Rejects empty/non-decimal input, overflow and zero funding.
pub fn parse_funding_cycles(value: &str) -> Result<NonZeroU128, BillingInputError> {
    if value.is_empty() {
        return Err(BillingInputError::EmptyDecimal);
    }
    if let Some(byte_offset) = value.bytes().position(|byte| !byte.is_ascii_digit()) {
        return Err(BillingInputError::InvalidDecimal { byte_offset });
    }
    let cycles = value
        .parse::<u128>()
        .map_err(|_| BillingInputError::OutOfRange {
            field: BillingField::RequestedFunding,
        })?;
    positive_funding(cycles)
}

fn positive_funding(value: u128) -> Result<NonZeroU128, BillingInputError> {
    NonZeroU128::new(value).ok_or(BillingInputError::ZeroFunding)
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

    #[test]
    fn configuration_reports_each_oversized_field_before_model_validation() {
        let one = Nat::from(1_u8);
        let oversized = Nat::parse(TOO_LARGE.as_bytes()).expect("nat fixture");
        for (reserve, minimum_balance, target_balance, field) in [
            (&oversized, &one, &one, BillingField::Reserve),
            (&one, &oversized, &one, BillingField::MinimumBalance),
            (&one, &one, &oversized, BillingField::TargetBalance),
        ] {
            assert_eq!(
                funding_limits_from_candid(FundingLimitsInput {
                    reserve,
                    minimum_balance,
                    target_balance,
                }),
                Err(BillingInputError::OutOfRange { field })
            );
        }
    }

    #[test]
    fn converted_configuration_uses_model_invariants() {
        for (reserve, minimum, target, expected) in [
            (
                0,
                10,
                100,
                Err(BillingInputError::InvalidLimits(
                    FundingLimitsError::ZeroReserve,
                )),
            ),
            (
                1,
                0,
                100,
                Err(BillingInputError::InvalidLimits(
                    FundingLimitsError::ZeroMinimumBalance,
                )),
            ),
            (
                1,
                10,
                0,
                Err(BillingInputError::InvalidLimits(
                    FundingLimitsError::ZeroTargetBalance,
                )),
            ),
            (
                1,
                101,
                100,
                Err(BillingInputError::InvalidLimits(
                    FundingLimitsError::MinimumExceedsTarget,
                )),
            ),
            (
                u128::MAX,
                u128::MAX,
                u128::MAX,
                Ok(FundingLimits::new(u128::MAX, u128::MAX, u128::MAX).expect("valid limits")),
            ),
        ] {
            assert_eq!(
                funding_limits_from_candid(FundingLimitsInput {
                    reserve: &Nat::from(reserve),
                    minimum_balance: &Nat::from(minimum),
                    target_balance: &Nat::from(target),
                }),
                expected
            );
        }
    }

    #[test]
    fn funding_text_and_candid_agree_at_positive_boundaries() {
        for value in [1, 10, u64::MAX.into(), u128::MAX] {
            let expected = NonZeroU128::new(value).expect("positive fixture");
            assert_eq!(funding_from_nat(&Nat::from(value)), Ok(expected));
            assert_eq!(parse_funding_cycles(&value.to_string()), Ok(expected));
            assert_eq!(parse_funding_cycles(&format!("000{value}")), Ok(expected));
        }
        assert_eq!(
            funding_from_nat(&Nat::from(0_u8)),
            Err(BillingInputError::ZeroFunding)
        );
        for text in ["0", "00000"] {
            assert_eq!(
                parse_funding_cycles(text),
                Err(BillingInputError::ZeroFunding)
            );
        }
        let expected = Err(BillingInputError::OutOfRange {
            field: BillingField::RequestedFunding,
        });
        assert_eq!(parse_funding_cycles(TOO_LARGE), expected);
        assert_eq!(
            funding_from_nat(&Nat::parse(TOO_LARGE.as_bytes()).expect("nat fixture")),
            expected
        );
    }

    #[test]
    fn operator_amounts_reject_non_decimal_syntax_with_byte_offsets() {
        assert_eq!(
            parse_funding_cycles(""),
            Err(BillingInputError::EmptyDecimal)
        );
        for (text, byte_offset) in [
            ("+1", 0),
            ("-1", 0),
            (" 1", 0),
            ("1 ", 1),
            ("1\n", 1),
            ("1\t", 1),
            ("1\0", 1),
            ("1_000", 1),
            ("1,000", 1),
            ("1.0", 1),
            ("1e3", 1),
            ("0xff", 1),
            ("１２", 0),
            ("1٢", 1),
        ] {
            assert_eq!(
                parse_funding_cycles(text),
                Err(BillingInputError::InvalidDecimal { byte_offset })
            );
        }
    }
}
