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
mod tests;
