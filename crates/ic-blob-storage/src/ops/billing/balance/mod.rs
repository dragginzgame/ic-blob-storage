//! Validate all balance amounts before exposing an unsigned aggregate value.
//!
//! Provider bindings must supply the amounts from one bound response. This
//! numeric conversion does not define that response's wire schema or interpret
//! its payment/debt mode. Decoder resource limits remain the caller's concern.

use candid::Int;
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
mod tests {
    use super::*;

    #[test]
    fn preserves_each_amount_without_summing_or_narrowing() {
        let total = Int::from(10);
        let prepaid = Int::from(u128::MAX);
        let promotional = Int::from(0);
        let ledger = Int::from(3);
        let amounts = balance_amounts_from_candid(BalanceAmountsInput {
            total: &total,
            prepaid: &prepaid,
            promotional: &promotional,
            ledger: &ledger,
        })
        .expect("independently bounded amounts");
        assert_eq!(amounts.total(), 10);
        assert_eq!(amounts.prepaid(), u128::MAX);
        assert_eq!(amounts.promotional(), 0);
        assert_eq!(amounts.ledger(), 3);
    }

    #[test]
    fn every_malformed_component_rejects_the_whole_balance() {
        let valid = Int::from(u128::MAX);
        for invalid in [
            Int::from(-1),
            Int::parse(b"340282366920938463463374607431768211456").expect("oversized integer"),
        ] {
            for (index, field) in [
                BalanceField::Total,
                BalanceField::Prepaid,
                BalanceField::Promotional,
                BalanceField::Ledger,
            ]
            .into_iter()
            .enumerate()
            {
                let mut values = [&valid; 4];
                values[index] = &invalid;
                assert_eq!(
                    balance_amounts_from_candid(BalanceAmountsInput {
                        total: values[0],
                        prepaid: values[1],
                        promotional: values[2],
                        ledger: values[3],
                    }),
                    Err(BalanceInputError { field }),
                );
            }
        }
    }
}
