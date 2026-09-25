//! Validate a complete configuration candidate without installing it.

use candid::Principal;
use thiserror::Error;

use crate::model::billing::configuration::{BillingConfiguration, BillingConfigurationError};

use super::{BillingInputError, FundingLimitsInput, funding_limits_from_candid};

/// Passive configuration inputs, without a provider wire or stable-state schema.
#[derive(Clone, Copy, Debug)]
pub struct BillingConfigurationInput<'a> {
    /// Cashier principal selected by the caller.
    pub cashier: Principal,
    /// Borrowed arbitrary-precision numeric funding fields.
    pub funding: FundingLimitsInput<'a>,
    /// Maximum gateway input entries, including duplicates.
    pub max_gateway_entries: u64,
    /// Maximum distinct gateway principals.
    pub max_gateway_unique: u64,
}

/// Failure to construct a complete configuration candidate.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BillingConfigurationInputError {
    /// Funding number conversion or threshold invariant failed.
    #[error(transparent)]
    Funding(#[from] BillingInputError),
    /// Cashier identity or gateway bound failed model validation.
    #[error(transparent)]
    Configuration(#[from] BillingConfigurationError),
}

/// Convert and validate every field before returning a configuration candidate.
///
/// Success grants no authority to install or use the configuration. No current
/// configuration or gateway list is accessed or mutated.
///
/// # Errors
/// Returns typed funding conversion, Cashier identity or gateway bound failures.
pub fn configuration_from_candid(
    input: BillingConfigurationInput<'_>,
) -> Result<BillingConfiguration, BillingConfigurationInputError> {
    let funding = funding_limits_from_candid(input.funding)?;
    Ok(BillingConfiguration::new(
        input.cashier,
        funding,
        input.max_gateway_entries,
        input.max_gateway_unique,
    )?)
}

#[cfg(test)]
mod tests {
    use candid::Nat;

    use super::*;
    use crate::{model::billing::configuration::GatewayLimitField, ops::billing::BillingField};

    #[test]
    fn conversion_preserves_typed_errors_from_both_validation_layers() {
        let reserve = Nat::from(100_u8);
        let minimum = Nat::from(10_u8);
        let target = Nat::from(50_u8);
        let input = BillingConfigurationInput {
            cashier: Principal::from_slice(&[1, 1]),
            funding: FundingLimitsInput {
                reserve: &reserve,
                minimum_balance: &minimum,
                target_balance: &target,
            },
            max_gateway_entries: 8,
            max_gateway_unique: 4,
        };
        let oversized = Nat::parse(b"340282366920938463463374607431768211456").expect("large Nat");
        assert_eq!(
            configuration_from_candid(BillingConfigurationInput {
                funding: FundingLimitsInput {
                    target_balance: &oversized,
                    ..input.funding
                },
                ..input
            }),
            Err(BillingConfigurationInputError::Funding(
                BillingInputError::OutOfRange {
                    field: BillingField::TargetBalance
                }
            ))
        );
        assert_eq!(
            configuration_from_candid(BillingConfigurationInput {
                cashier: Principal::anonymous(),
                ..input
            }),
            Err(BillingConfigurationInputError::Configuration(
                BillingConfigurationError::InvalidCashier {
                    principal: Principal::anonymous()
                }
            ))
        );
        assert_eq!(
            configuration_from_candid(BillingConfigurationInput {
                max_gateway_entries: u64::MAX,
                ..input
            }),
            Err(BillingConfigurationInputError::Configuration(
                BillingConfigurationError::GatewayLimitOutOfRange {
                    field: GatewayLimitField::Entries
                }
            ))
        );
        let valid = configuration_from_candid(input).expect("valid input after failures");
        assert_eq!(valid.cashier(), input.cashier);
        assert_eq!(valid.funding_limits().reserve(), 100);
        assert_eq!(valid.funding_limits().minimum_balance(), 10);
        assert_eq!(valid.funding_limits().target_balance(), 50);
    }
}
