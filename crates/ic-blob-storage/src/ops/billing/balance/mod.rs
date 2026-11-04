//! Validate all balance amounts before exposing an unsigned aggregate value.
//!
//! Provider bindings must supply the amounts from one bound response. This
//! numeric conversion does not define that response's wire schema or interpret
//! its payment/debt mode. Decoder resource limits remain the caller's concern.

use ic_platform::candid::contract::Int;
use thiserror::Error;

use crate::model::billing::balance::BalanceAmounts;

use super::cycles_from_int;

/// Numeric component rejected during whole-balance conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BalanceField {
    /// Reported total.
    Total,
    /// Prepaid cycles.
    Prepaid,
    /// Promotional cycles.
    Promotional,
    /// Ledger cycles.
    Ledger,
}

/// Borrowed balance amounts, without provider identifiers or defaults.
#[derive(Clone, Copy, Debug)]
pub struct BalanceAmountsInput<'a> {
    /// Independently reported total.
    pub total: &'a Int,
    /// Prepaid cycles.
    pub prepaid: &'a Int,
    /// Promotional cycles.
    pub promotional: &'a Int,
    /// Ledger cycles.
    pub ledger: &'a Int,
}

/// A balance component is negative or exceeds the unsigned 128-bit range.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("{field:?} balance is outside the unsigned 128-bit cycle range")]
pub struct BalanceInputError {
    /// First rejected field, in total/prepaid/promotional/ledger order.
    pub field: BalanceField,
}

/// Convert every amount before returning a usable balance value.
///
/// A valid total never hides a malformed component. Values are neither clamped
/// nor summed, and arbitrary-precision integer storage is borrowed, not cloned.
/// No failure produces a partial balance or substitutes zero.
///
/// # Errors
/// Returns the first negative or oversized component as a typed field error.
pub fn balance_amounts_from_candid(
    input: BalanceAmountsInput<'_>,
) -> Result<BalanceAmounts, BalanceInputError> {
    let total = component(BalanceField::Total, input.total)?;
    let prepaid = component(BalanceField::Prepaid, input.prepaid)?;
    let promotional = component(BalanceField::Promotional, input.promotional)?;
    let ledger = component(BalanceField::Ledger, input.ledger)?;
    Ok(BalanceAmounts::new(total, prepaid, promotional, ledger))
}

fn component(field: BalanceField, value: &Int) -> Result<u128, BalanceInputError> {
    cycles_from_int(value).map_err(|_| BalanceInputError { field })
}

#[cfg(test)]
mod tests;
