//! Validated local billing configuration, without installation or provider proof.

use std::num::NonZeroUsize;

use candid::Principal;
use thiserror::Error;

use crate::model::{billing::FundingLimits, gateway::GatewayListLimits};

/// Which gateway collection bound was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GatewayLimitField {
    /// Raw entries, including duplicates.
    Entries,
    /// Distinct principals retained after normalization.
    Unique,
}

/// Validated configuration values ready for a separately authorized workflow.
///
/// The Cashier principal is only a configured candidate: validation rejects
/// anonymous/management principals but proves neither deployment nor ownership.
/// Account/service/provider-namespace bindings, persistence, configuration-change
/// fences and operator authorization remain the workflow's responsibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BillingConfiguration {
    cashier: Principal,
    funding: FundingLimits,
    gateways: GatewayListLimits,
}

impl BillingConfiguration {
    /// Validate identity and gateway bounds around already validated funding limits.
    ///
    /// Bounds must fit a 32-bit Wasm collection index even on a 64-bit host. This
    /// is a representability ceiling, not a recommended resource budget. No
    /// collection is allocated here, and no deployment defaults are supplied.
    ///
    /// # Errors
    /// Rejects anonymous/management Cashier principals, zero gateway limits and
    /// limits exceeding `u32::MAX` or the current target's `usize` range.
    pub fn new(
        cashier: Principal,
        funding: FundingLimits,
        max_gateway_entries: u64,
        max_gateway_unique: u64,
    ) -> Result<Self, BillingConfigurationError> {
        if cashier == Principal::anonymous() || cashier == Principal::management_canister() {
            return Err(BillingConfigurationError::InvalidCashier { principal: cashier });
        }
        let max_entries = gateway_limit(max_gateway_entries, GatewayLimitField::Entries)?;
        let max_unique = gateway_limit(max_gateway_unique, GatewayLimitField::Unique)?;
        Ok(Self {
            cashier,
            funding,
            gateways: GatewayListLimits {
                max_entries,
                max_unique,
            },
        })
    }

    /// Configured Cashier candidate, not authenticated provider evidence.
    #[must_use]
    pub const fn cashier(&self) -> Principal {
        self.cashier
    }

    /// Validated reserve and balance thresholds for pure billing policy.
    #[must_use]
    pub const fn funding_limits(&self) -> FundingLimits {
        self.funding
    }

    /// Bounds to apply before accepting a gateway-list replacement.
    #[must_use]
    pub const fn gateway_limits(&self) -> GatewayListLimits {
        self.gateways
    }
}

fn gateway_limit(
    value: u64,
    field: GatewayLimitField,
) -> Result<NonZeroUsize, BillingConfigurationError> {
    let portable = u32::try_from(value)
        .map_err(|_| BillingConfigurationError::GatewayLimitOutOfRange { field })?;
    let native = usize::try_from(portable)
        .map_err(|_| BillingConfigurationError::GatewayLimitOutOfRange { field })?;
    NonZeroUsize::new(native).ok_or(BillingConfigurationError::ZeroGatewayLimit { field })
}

/// A rejected configuration cannot become a validated model value.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BillingConfigurationError {
    /// Anonymous and management principals cannot identify the configured Cashier.
    #[error("invalid Cashier principal {principal}")]
    InvalidCashier {
        /// Rejected candidate.
        principal: Principal,
    },
    /// Both processing and distinct membership bounds must be positive.
    #[error("gateway {field:?} limit must be positive")]
    ZeroGatewayLimit {
        /// Rejected bound.
        field: GatewayLimitField,
    },
    /// The bound must be representable on both this target and 32-bit Wasm.
    #[error("gateway {field:?} limit exceeds the supported index range")]
    GatewayLimitOutOfRange {
        /// Rejected bound.
        field: GatewayLimitField,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn funding() -> FundingLimits {
        FundingLimits::new(1, 10, 100).expect("valid funding limits")
    }

    #[test]
    fn special_cashier_principals_reject_without_claiming_other_principals_are_deployed() {
        for principal in [Principal::anonymous(), Principal::management_canister()] {
            assert_eq!(
                BillingConfiguration::new(principal, funding(), 8, 4),
                Err(BillingConfigurationError::InvalidCashier { principal })
            );
        }
        let candidate = Principal::self_authenticating(b"candidate identity");
        let config =
            BillingConfiguration::new(candidate, funding(), 8, 4).expect("valid candidate");
        assert_eq!(config.cashier(), candidate);
        assert_eq!(config.funding_limits(), funding());
        assert_eq!(config.gateway_limits().max_entries.get(), 8);
        assert_eq!(config.gateway_limits().max_unique.get(), 4);
    }

    #[test]
    fn both_gateway_bounds_reject_zero_and_host_only_sizes() {
        let cashier = Principal::from_slice(&[1, 1]);
        for (entries, unique, field) in [
            (0, 4, GatewayLimitField::Entries),
            (8, 0, GatewayLimitField::Unique),
        ] {
            assert_eq!(
                BillingConfiguration::new(cashier, funding(), entries, unique),
                Err(BillingConfigurationError::ZeroGatewayLimit { field })
            );
        }
        for oversized in [u64::from(u32::MAX) + 1, u64::MAX] {
            for (entries, unique, field) in [
                (oversized, 4, GatewayLimitField::Entries),
                (8, oversized, GatewayLimitField::Unique),
            ] {
                assert_eq!(
                    BillingConfiguration::new(cashier, funding(), entries, unique),
                    Err(BillingConfigurationError::GatewayLimitOutOfRange { field })
                );
            }
        }
    }

    #[test]
    fn maximum_representable_configuration_needs_no_collection_allocation() {
        let funding = FundingLimits::new(u128::MAX, u128::MAX, u128::MAX).expect("valid limits");
        let max = u64::from(u32::MAX);
        let config = BillingConfiguration::new(Principal::from_slice(&[1, 1]), funding, max, max)
            .expect("32-bit bounds");
        assert_eq!(config.funding_limits(), funding);
        assert_eq!(
            config.gateway_limits().max_entries.get(),
            usize::try_from(max).expect("target width")
        );
        assert_eq!(
            config.gateway_limits().max_unique.get(),
            usize::try_from(max).expect("target width")
        );
    }
}
