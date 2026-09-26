//! Diagnose provider balance thresholds independently of local spendable funds.
use super::{BalanceObservation, BillingBlocker, BillingWarning, RecoveryState};
use crate::model::billing::FundingLimits;
use std::num::NonZeroU128;

/// Bound balance inputs; no local balance or reserve assumption is required.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BalanceContext {
    /// Current gateways validated for the same service/provider scope.
    pub gateway_count: usize,
    /// Usable scoped report, or why none is available.
    pub balance: BalanceObservation,
    /// Independently established recovery condition; never inferred from a balance.
    pub recovery: RecoveryState,
}

/// Reported balance shortfall only; no variant authorizes a transfer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingNeed {
    /// Validated billing limits are absent.
    NotConfigured,
    /// Balance meets the configured minimum.
    NotNeeded,
    /// No usable current report.
    BalanceUnavailable,
    /// The current report was malformed.
    BalanceMalformed,
    /// Exact difference from the target; affordability remains a separate question.
    TopUp(NonZeroU128),
}

/// Balance diagnosis without any claim about spendability or overall readiness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BalanceDiagnosis {
    pub(super) funding: FundingNeed,
    pub(super) blockers: Vec<BillingBlocker>,
    pub(super) warnings: Vec<BillingWarning>,
}

impl BalanceDiagnosis {
    /// Threshold shortfall, without reserve arithmetic or a payment permit.
    #[must_use]
    pub const fn funding_need(&self) -> FundingNeed {
        self.funding
    }
    /// Configuration, recovery, gateway and balance blockers.
    #[must_use]
    pub fn blockers(&self) -> &[BillingBlocker] {
        &self.blockers
    }
    /// Gateway and balance warnings.
    #[must_use]
    pub fn warnings(&self) -> &[BillingWarning] {
        &self.warnings
    }
}

/// Assess the provider balance without substituting zero for unknown local funds.
/// Complete reserve diagnosis uses [`super::assess_readiness`] when spendability
/// is independently known. Neither function authenticates its supplied observations.
#[must_use]
#[expect(
    clippy::missing_panics_doc,
    reason = "validated target >= minimum > balance proves a positive difference"
)]
pub fn assess_balance(limits: Option<FundingLimits>, context: BalanceContext) -> BalanceDiagnosis {
    let mut result = BalanceDiagnosis {
        funding: FundingNeed::NotConfigured,
        blockers: vec![],
        warnings: vec![],
    };
    if context.recovery == RecoveryState::Fenced {
        result.blockers.push(BillingBlocker::RecoveryFenced);
    }
    let Some(limits) = limits else {
        result.blockers.push(BillingBlocker::NotConfigured);
        return result;
    };
    if context.gateway_count == 0 {
        result
            .blockers
            .push(BillingBlocker::GatewayPrincipalsMissing);
        result
            .warnings
            .push(BillingWarning::GatewayPrincipalSetEmpty);
    }
    result.funding = match context.balance {
        BalanceObservation::Unavailable => {
            result.blockers.push(BillingBlocker::BalanceUnavailable);
            result.warnings.push(BillingWarning::BalanceUnavailable);
            FundingNeed::BalanceUnavailable
        }
        BalanceObservation::Malformed => {
            result.blockers.push(BillingBlocker::BalanceMalformed);
            result.warnings.push(BillingWarning::BalanceMalformed);
            FundingNeed::BalanceMalformed
        }
        BalanceObservation::Available(balance) if balance >= limits.minimum_balance() => {
            FundingNeed::NotNeeded
        }
        BalanceObservation::Available(balance) => {
            result.blockers.push(BillingBlocker::InsufficientBalance);
            FundingNeed::TopUp(
                NonZeroU128::new(limits.target_balance() - balance).expect("positive shortfall"),
            )
        }
    };
    result
}

#[cfg(test)]
mod tests;
