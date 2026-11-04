//! Billing arithmetic and diagnostic readiness, never authority to execute effects.

pub mod admission;
pub mod balance;
pub mod liquidity;
pub mod reconciliation;

use std::num::NonZeroU128;

use ic_blob_storage_contracts::configuration::funding::FundingLimits;

/// Whether the entire positive request fits above the reserve.
///
/// Even a fit requires separate authorization, persisted intent, liability
/// accounting and recovery reconciliation before any provider effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingDecision {
    /// The full request fits; no partial amount is substituted.
    FitsReserve {
        /// Exact amount evaluated.
        requested_cycles: NonZeroU128,
    },
    /// Funding the request would violate the reserve; attach no cycles.
    ReserveWouldBeViolated {
        /// Exact amount evaluated.
        requested_cycles: NonZeroU128,
        /// Maximum available above reserve, for diagnosis only.
        transferable_cycles: u128,
    },
}

/// Evaluate an explicit funding amount without attaching or reserving cycles.
///
/// `available_cycles` must exclude already committed reservations/liabilities.
/// Unknown or stale accounting must be fenced by the eventual workflow; this
/// arithmetic function does not establish freshness or authorize retries.
#[must_use]
pub const fn assess_funding(
    limits: FundingLimits,
    requested_cycles: NonZeroU128,
    available_cycles: u128,
) -> FundingDecision {
    let transferable_cycles = available_cycles.saturating_sub(limits.reserve());
    if requested_cycles.get() > transferable_cycles {
        FundingDecision::ReserveWouldBeViolated {
            requested_cycles,
            transferable_cycles,
        }
    } else {
        FundingDecision::FitsReserve { requested_cycles }
    }
}

/// Result of observing a provider balance; failure is never a zero balance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BalanceObservation {
    /// A successfully decoded balance in cycles.
    Available(u128),
    /// No usable response was obtained.
    Unavailable,
    /// A response could not be represented as a valid balance.
    Malformed,
}

/// Recovery state supplied by the workflow, not established by this policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryState {
    /// Identity allocation or accounting still requires reconciliation.
    Fenced,
    /// The workflow has separately established safe current identity/accounting.
    Reconciled,
}

/// Inputs to a read-only billing diagnosis, not a provider protocol DTO.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BillingObservation {
    /// Number of gateways validated for the configured provider namespace.
    pub gateway_count: usize,
    /// Last balance observation to evaluate.
    pub balance: BalanceObservation,
    /// Local cycles available after existing reservations and liabilities.
    pub available_cycles: u128,
    /// Independently established recovery state.
    pub recovery: RecoveryState,
}

/// Suggested funding need, which never triggers synchronization or funding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingStatus {
    /// Validated billing limits are absent.
    NotConfigured,
    /// Observed balance meets the minimum, even if below the target.
    NotNeeded,
    /// The balance could not be observed.
    BalanceUnavailable,
    /// The observed balance was malformed.
    BalanceMalformed,
    /// Top-up arithmetic result for a balance below the minimum.
    TopUp(FundingDecision),
}

/// Reason the supplied billing observations are insufficient for readiness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BillingBlocker {
    /// Recovery reconciliation is incomplete.
    RecoveryFenced,
    /// Validated billing limits are missing.
    NotConfigured,
    /// No validated gateway is present.
    GatewayPrincipalsMissing,
    /// The balance query did not yield a response.
    BalanceUnavailable,
    /// The balance response was invalid.
    BalanceMalformed,
    /// The provider balance is below the minimum.
    InsufficientBalance,
    /// The entire top-up cannot fit above reserve.
    ReserveWouldBeViolated,
}

/// Diagnostic warning preserved for operator reporting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BillingWarning {
    /// No validated gateway is available.
    GatewayPrincipalSetEmpty,
    /// The balance observation failed.
    BalanceUnavailable,
    /// The balance observation was invalid.
    BalanceMalformed,
}

/// Read-only billing diagnosis. This is not overall service/upload readiness.
///
/// Tenant authority, quota, provider qualification and durable effect state must
/// be evaluated by the service separately. No decision here clears a fence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BillingReadiness {
    funding: FundingStatus,
    blockers: Vec<BillingBlocker>,
    warnings: Vec<BillingWarning>,
}

impl BillingReadiness {
    /// Whether these billing observations have no blockers.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.blockers.is_empty()
    }

    /// Funding diagnosis, including arithmetic while a recovery fence is active.
    #[must_use]
    pub const fn funding(&self) -> FundingStatus {
        self.funding
    }

    /// All applicable billing blockers in deterministic order.
    #[must_use]
    pub fn blockers(&self) -> &[BillingBlocker] {
        &self.blockers
    }

    /// Non-mutating operator warnings.
    #[must_use]
    pub fn warnings(&self) -> &[BillingWarning] {
        &self.warnings
    }
}

/// Diagnose billing without state access, gateway synchronization or top-up.
///
/// Absent limits produce `NotConfigured`; provider observations are not trusted
/// in that case. Recovery fencing is reported regardless of configuration.
#[must_use]
pub fn assess_readiness(
    limits: Option<FundingLimits>,
    observation: BillingObservation,
) -> BillingReadiness {
    let diagnosis = balance::assess_balance(
        limits,
        balance::BalanceContext {
            gateway_count: observation.gateway_count,
            balance: observation.balance,
            recovery: observation.recovery,
        },
    );
    let mut blockers = diagnosis.blockers;
    let funding = match diagnosis.funding {
        balance::FundingNeed::NotConfigured => FundingStatus::NotConfigured,
        balance::FundingNeed::NotNeeded => FundingStatus::NotNeeded,
        balance::FundingNeed::BalanceUnavailable => FundingStatus::BalanceUnavailable,
        balance::FundingNeed::BalanceMalformed => FundingStatus::BalanceMalformed,
        balance::FundingNeed::TopUp(requested) => {
            let Some(limits) = limits else {
                unreachable!("a shortfall requires configured limits")
            };
            let decision = assess_funding(limits, requested, observation.available_cycles);
            if matches!(decision, FundingDecision::ReserveWouldBeViolated { .. }) {
                blockers.push(BillingBlocker::ReserveWouldBeViolated);
            }
            FundingStatus::TopUp(decision)
        }
    };
    BillingReadiness {
        funding,
        blockers,
        warnings: diagnosis.warnings,
    }
}

#[cfg(test)]
mod tests;
