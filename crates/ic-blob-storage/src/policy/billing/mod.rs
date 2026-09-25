//! Billing arithmetic and diagnostic readiness, never authority to execute effects.

pub mod admission;

use std::num::NonZeroU128;

use crate::model::billing::FundingLimits;

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
#[expect(
    clippy::missing_panics_doc,
    reason = "validated FundingLimits and the balance guard prove target >= minimum > balance"
)]
pub fn assess_readiness(
    limits: Option<FundingLimits>,
    observation: BillingObservation,
) -> BillingReadiness {
    let mut result = BillingReadiness {
        funding: FundingStatus::NotConfigured,
        blockers: Vec::new(),
        warnings: Vec::new(),
    };
    if observation.recovery == RecoveryState::Fenced {
        result.blockers.push(BillingBlocker::RecoveryFenced);
    }
    let Some(limits) = limits else {
        result.blockers.push(BillingBlocker::NotConfigured);
        return result;
    };
    if observation.gateway_count == 0 {
        result
            .blockers
            .push(BillingBlocker::GatewayPrincipalsMissing);
        result
            .warnings
            .push(BillingWarning::GatewayPrincipalSetEmpty);
    }
    result.funding = match observation.balance {
        BalanceObservation::Unavailable => {
            result.blockers.push(BillingBlocker::BalanceUnavailable);
            result.warnings.push(BillingWarning::BalanceUnavailable);
            FundingStatus::BalanceUnavailable
        }
        BalanceObservation::Malformed => {
            result.blockers.push(BillingBlocker::BalanceMalformed);
            result.warnings.push(BillingWarning::BalanceMalformed);
            FundingStatus::BalanceMalformed
        }
        BalanceObservation::Available(balance) if balance >= limits.minimum_balance() => {
            FundingStatus::NotNeeded
        }
        BalanceObservation::Available(balance) => {
            result.blockers.push(BillingBlocker::InsufficientBalance);
            // Validated target >= minimum > balance: subtraction is positive and safe.
            let requested = NonZeroU128::new(limits.target_balance() - balance)
                .expect("target exceeds a balance below minimum");
            let decision = assess_funding(limits, requested, observation.available_cycles);
            if matches!(decision, FundingDecision::ReserveWouldBeViolated { .. }) {
                result.blockers.push(BillingBlocker::ReserveWouldBeViolated);
            }
            FundingStatus::TopUp(decision)
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn amount(value: u128) -> NonZeroU128 {
        NonZeroU128::new(value).expect("positive test amount")
    }

    fn observation(balance: BalanceObservation) -> BillingObservation {
        BillingObservation {
            gateway_count: 1,
            balance,
            available_cycles: 1_000,
            recovery: RecoveryState::Reconciled,
        }
    }

    #[test]
    fn funding_requires_full_amount_and_preserves_exact_reserve() {
        let limits = FundingLimits::new(500, 10, 100).expect("valid limits");
        assert_eq!(
            assess_funding(limits, amount(500), 1_000),
            FundingDecision::FitsReserve {
                requested_cycles: amount(500)
            }
        );
        assert_eq!(
            assess_funding(limits, amount(501), 1_000),
            FundingDecision::ReserveWouldBeViolated {
                requested_cycles: amount(501),
                transferable_cycles: 500
            }
        );
        for available in [0, 499, 500] {
            assert_eq!(
                assess_funding(limits, amount(1), available),
                FundingDecision::ReserveWouldBeViolated {
                    requested_cycles: amount(1),
                    transferable_cycles: 0
                }
            );
        }
    }

    #[test]
    fn funding_conserves_cycles_at_small_and_extreme_boundaries() {
        let boundaries = [0, 1, 2, 9, 10, 99, 100, 101, u128::MAX - 1, u128::MAX];
        for reserve in boundaries.into_iter().filter(|value| *value > 0) {
            let limits = FundingLimits::new(reserve, 1, 1).expect("valid limits");
            for available in boundaries {
                for requested in boundaries.into_iter().filter_map(NonZeroU128::new) {
                    match assess_funding(limits, requested, available) {
                        FundingDecision::FitsReserve { requested_cycles } => {
                            assert_eq!(requested_cycles, requested);
                            let remaining = available
                                .checked_sub(requested.get())
                                .expect("no overspend");
                            assert!(remaining >= reserve);
                            assert_eq!(remaining.checked_add(requested.get()), Some(available));
                        }
                        FundingDecision::ReserveWouldBeViolated {
                            requested_cycles,
                            transferable_cycles,
                        } => {
                            assert_eq!(requested_cycles, requested);
                            assert!(
                                available
                                    .checked_sub(requested.get())
                                    .is_none_or(|remaining| remaining < reserve)
                            );
                            assert!(transferable_cycles < requested.get());
                            assert!(transferable_cycles <= available);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn readiness_uses_minimum_threshold_then_full_target_top_up() {
        let limits = FundingLimits::new(100, 10, 100).expect("valid limits");
        for balance in [10, 11, 99, 100, u128::MAX] {
            let result = assess_readiness(
                Some(limits),
                observation(BalanceObservation::Available(balance)),
            );
            assert!(result.is_ready());
            assert_eq!(result.funding(), FundingStatus::NotNeeded);
            assert!(result.blockers().is_empty());
            assert!(result.warnings().is_empty());
        }
        for (balance, requested) in [(0, 100), (1, 99), (9, 91)] {
            let result = assess_readiness(
                Some(limits),
                observation(BalanceObservation::Available(balance)),
            );
            assert!(!result.is_ready());
            assert_eq!(
                result.funding(),
                FundingStatus::TopUp(FundingDecision::FitsReserve {
                    requested_cycles: amount(requested)
                })
            );
            assert_eq!(result.blockers(), &[BillingBlocker::InsufficientBalance]);
        }
    }

    #[test]
    fn readiness_preserves_all_simultaneous_blockers() {
        let limits = FundingLimits::new(950, 10, 100).expect("valid limits");
        let input = BillingObservation {
            gateway_count: 0,
            recovery: RecoveryState::Fenced,
            ..observation(BalanceObservation::Available(9))
        };
        let result = assess_readiness(Some(limits), input);
        assert!(!result.is_ready());
        assert_eq!(
            result.blockers(),
            &[
                BillingBlocker::RecoveryFenced,
                BillingBlocker::GatewayPrincipalsMissing,
                BillingBlocker::InsufficientBalance,
                BillingBlocker::ReserveWouldBeViolated,
            ]
        );
        assert_eq!(
            result.warnings(),
            &[BillingWarning::GatewayPrincipalSetEmpty]
        );
        assert_eq!(
            result.funding(),
            FundingStatus::TopUp(FundingDecision::ReserveWouldBeViolated {
                requested_cycles: amount(91),
                transferable_cycles: 50
            })
        );
        assert_eq!(assess_readiness(Some(limits), input), result);
    }

    #[test]
    fn observation_failures_never_become_zero_balance_top_ups() {
        let limits = FundingLimits::new(1, 10, 100).expect("valid limits");
        for (balance, funding, blocker, warning) in [
            (
                BalanceObservation::Unavailable,
                FundingStatus::BalanceUnavailable,
                BillingBlocker::BalanceUnavailable,
                BillingWarning::BalanceUnavailable,
            ),
            (
                BalanceObservation::Malformed,
                FundingStatus::BalanceMalformed,
                BillingBlocker::BalanceMalformed,
                BillingWarning::BalanceMalformed,
            ),
        ] {
            let result = assess_readiness(Some(limits), observation(balance));
            assert!(!result.is_ready());
            assert_eq!(result.funding(), funding);
            assert_eq!(result.blockers(), &[blocker]);
            assert_eq!(result.warnings(), &[warning]);
        }
        let recovered =
            assess_readiness(Some(limits), observation(BalanceObservation::Available(10)));
        assert!(recovered.is_ready());
    }

    #[test]
    fn missing_configuration_and_recovery_fences_fail_closed() {
        let limits = FundingLimits::new(1, 10, 100).expect("valid limits");
        let input = observation(BalanceObservation::Available(u128::MAX));
        let missing = assess_readiness(None, input);
        assert!(!missing.is_ready());
        assert_eq!(missing.funding(), FundingStatus::NotConfigured);
        assert_eq!(missing.blockers(), &[BillingBlocker::NotConfigured]);
        let fenced_input = BillingObservation {
            recovery: RecoveryState::Fenced,
            ..input
        };
        let fenced = assess_readiness(Some(limits), fenced_input);
        assert!(!fenced.is_ready());
        assert_eq!(fenced.funding(), FundingStatus::NotNeeded);
        assert_eq!(fenced.blockers(), &[BillingBlocker::RecoveryFenced]);
        assert_eq!(
            assess_readiness(None, fenced_input).blockers(),
            &[
                BillingBlocker::RecoveryFenced,
                BillingBlocker::NotConfigured
            ]
        );
    }

    #[test]
    fn maximum_target_and_equal_thresholds_do_not_overflow() {
        let limits = FundingLimits::new(1, u128::MAX, u128::MAX).expect("valid limits");
        for (balance, expected) in [
            (
                0,
                FundingDecision::ReserveWouldBeViolated {
                    requested_cycles: amount(u128::MAX),
                    transferable_cycles: u128::MAX - 1,
                },
            ),
            (
                1,
                FundingDecision::FitsReserve {
                    requested_cycles: amount(u128::MAX - 1),
                },
            ),
            (
                u128::MAX - 1,
                FundingDecision::FitsReserve {
                    requested_cycles: amount(1),
                },
            ),
        ] {
            let result = assess_readiness(
                Some(limits),
                BillingObservation {
                    available_cycles: u128::MAX,
                    ..observation(BalanceObservation::Available(balance))
                },
            );
            assert!(!result.is_ready());
            assert_eq!(result.funding(), FundingStatus::TopUp(expected));
        }
    }
}
