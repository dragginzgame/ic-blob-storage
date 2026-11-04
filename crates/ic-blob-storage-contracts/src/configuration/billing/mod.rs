//! Validated local billing configuration, without installation or provider proof.

use std::num::NonZeroUsize;

use ic_platform_types::Principal;
use thiserror::Error;

use crate::configuration::funding::FundingLimits;
use crate::configuration::limits::GatewayListLimits;

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
mod tests;
