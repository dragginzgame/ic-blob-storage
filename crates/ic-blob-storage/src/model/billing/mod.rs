//! Validated numeric billing limits, independent of provider configuration.

use thiserror::Error;

/// Positive reserve and upload-balance thresholds, with minimum <= target.
///
/// These limits contain no provider identity, credentials or persisted state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingLimits {
    reserve: u128,
    minimum_balance: u128,
    target_balance: u128,
}

impl FundingLimits {
    /// Validate cycle amounts before they can participate in funding decisions.
    ///
    /// # Errors
    /// Rejects zero amounts and a minimum greater than the target.
    pub const fn new(
        reserve: u128,
        minimum_balance: u128,
        target_balance: u128,
    ) -> Result<Self, FundingLimitsError> {
        if reserve == 0 {
            return Err(FundingLimitsError::ZeroReserve);
        }
        if minimum_balance == 0 {
            return Err(FundingLimitsError::ZeroMinimumBalance);
        }
        if target_balance == 0 {
            return Err(FundingLimitsError::ZeroTargetBalance);
        }
        if minimum_balance > target_balance {
            return Err(FundingLimitsError::MinimumExceedsTarget);
        }
        Ok(Self {
            reserve,
            minimum_balance,
            target_balance,
        })
    }

    /// Cycles that funding must leave in the service's available balance.
    #[must_use]
    pub const fn reserve(self) -> u128 {
        self.reserve
    }

    /// Provider balance at or above which no automatic top-up is suggested.
    #[must_use]
    pub const fn minimum_balance(self) -> u128 {
        self.minimum_balance
    }

    /// Desired provider balance when a balance below the minimum is observed.
    #[must_use]
    pub const fn target_balance(self) -> u128 {
        self.target_balance
    }
}

/// Invalid numeric funding configuration.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FundingLimitsError {
    /// A positive local cycle reserve is required.
    #[error("cycle reserve must be positive")]
    ZeroReserve,
    /// The minimum provider balance must be positive.
    #[error("minimum balance must be positive")]
    ZeroMinimumBalance,
    /// The target provider balance must be positive.
    #[error("target balance must be positive")]
    ZeroTargetBalance,
    /// The target cannot be below the admission threshold.
    #[error("minimum balance exceeds target")]
    MinimumExceedsTarget,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_reject_each_invalid_boundary() {
        for (values, error) in [
            ((0, 10, 100), FundingLimitsError::ZeroReserve),
            ((1, 0, 100), FundingLimitsError::ZeroMinimumBalance),
            ((1, 10, 0), FundingLimitsError::ZeroTargetBalance),
            ((1, 101, 100), FundingLimitsError::MinimumExceedsTarget),
        ] {
            assert_eq!(FundingLimits::new(values.0, values.1, values.2), Err(error));
        }
    }

    #[test]
    fn limits_accept_equal_thresholds_and_maximum_cycle_values() {
        let limits = FundingLimits::new(u128::MAX, u128::MAX, u128::MAX).expect("valid limits");
        assert_eq!(limits.reserve(), u128::MAX);
        assert_eq!(limits.minimum_balance(), u128::MAX);
        assert_eq!(limits.target_balance(), u128::MAX);
    }
}
