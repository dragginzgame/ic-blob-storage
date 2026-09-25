//! Admission of a new funding intent from independently established observations.
//!
//! This policy neither acquires a lock nor persists an intent. A positive result
//! is not permission to call a provider, resume an attempt or repeat a payment.

use std::num::NonZeroU128;

use crate::{
    model::billing::FundingLimits,
    policy::billing::{FundingDecision, RecoveryState, assess_funding},
};

/// Outstanding funding activity for the bound service/provider account.
///
/// The workflow must obtain this from authoritative state, not a process-local
/// lock. Dropping a future, restarting or expiring evidence cannot clear it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingActivity {
    /// Reconciliation establishes that no funding attempt remains outstanding.
    Clear,
    /// An existing intent is reserved or executing; a second intent must wait.
    InProgress,
    /// A prior effect may have completed and its outcome is not established.
    Uncertain,
}

/// Domain observations for admitting a new intent, not a provider request DTO.
///
/// All fields must refer to the same current service/provider account. The
/// eventual workflow must establish that binding and read/reserve atomically.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingAdmissionObservation {
    /// Cycles remaining after existing reservations and liabilities.
    pub available_cycles: u128,
    /// Independently established identity and accounting recovery state.
    pub recovery: RecoveryState,
    /// Existing funding activity, including uncertain effects after interruption.
    pub activity: FundingActivity,
}

/// First reason a new funding intent cannot be prepared.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingAdmissionBlocker {
    /// Identity allocation or accounting is not reconciled.
    RecoveryFenced,
    /// An earlier funding intent is still outstanding.
    FundingInProgress,
    /// Reconcile the previous effect; do not repeat it or start another intent.
    FundingUncertain,
    /// Validated funding limits are absent.
    NotConfigured,
    /// The complete request does not fit above the reserve.
    ReserveWouldBeViolated {
        /// Exact amount evaluated; a partial top-up is never substituted.
        requested_cycles: NonZeroU128,
        /// Amount above reserve, for diagnosis only.
        transferable_cycles: u128,
    },
}

/// Pure decision about preparing a new funding intent, never an effect permit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingAdmissionDecision {
    /// These observations permit the workflow to prepare an intent.
    ///
    /// Authorization, exact operation identity, atomic reservation and durable
    /// intent persistence must precede effects. No such work happens here.
    PrepareIntent {
        /// Preserve the caller's full requested amount.
        requested_cycles: NonZeroU128,
    },
    /// Do not prepare a new funding intent.
    Blocked(FundingAdmissionBlocker),
}

/// Assess a new request without changing state or authorizing a retry.
///
/// Recovery fencing takes precedence, then outstanding activity, missing
/// configuration and finally reserve arithmetic. Expired or missing outcome
/// evidence must remain `Uncertain` (or recovery `Fenced`); there is deliberately
/// no timeout that admits a replacement payment. Only separate authoritative
/// reconciliation may establish `Clear`.
#[must_use]
pub const fn assess_funding_admission(
    limits: Option<FundingLimits>,
    requested_cycles: NonZeroU128,
    observation: FundingAdmissionObservation,
) -> FundingAdmissionDecision {
    if matches!(observation.recovery, RecoveryState::Fenced) {
        return FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::RecoveryFenced);
    }
    match observation.activity {
        FundingActivity::InProgress => {
            return FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::FundingInProgress);
        }
        FundingActivity::Uncertain => {
            return FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::FundingUncertain);
        }
        FundingActivity::Clear => {}
    }
    let Some(limits) = limits else {
        return FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::NotConfigured);
    };
    match assess_funding(limits, requested_cycles, observation.available_cycles) {
        FundingDecision::FitsReserve { requested_cycles } => {
            FundingAdmissionDecision::PrepareIntent { requested_cycles }
        }
        FundingDecision::ReserveWouldBeViolated {
            requested_cycles,
            transferable_cycles,
        } => FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::ReserveWouldBeViolated {
            requested_cycles,
            transferable_cycles,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn amount(value: u128) -> NonZeroU128 {
        NonZeroU128::new(value).expect("positive test amount")
    }

    #[test]
    fn recovery_fence_dominates_activity_configuration_and_cycle_balance() {
        let limits = FundingLimits::new(1, 10, 100).expect("valid limits");
        for limits in [None, Some(limits)] {
            for activity in [
                FundingActivity::Clear,
                FundingActivity::InProgress,
                FundingActivity::Uncertain,
            ] {
                for available_cycles in [0, u128::MAX] {
                    assert_eq!(
                        assess_funding_admission(
                            limits,
                            amount(1),
                            FundingAdmissionObservation {
                                available_cycles,
                                recovery: RecoveryState::Fenced,
                                activity,
                            },
                        ),
                        FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::RecoveryFenced),
                    );
                }
            }
        }
    }

    #[test]
    fn outstanding_effects_block_new_intents_despite_changed_requests_or_configuration() {
        for (activity, blocker) in [
            (
                FundingActivity::InProgress,
                FundingAdmissionBlocker::FundingInProgress,
            ),
            (
                FundingActivity::Uncertain,
                FundingAdmissionBlocker::FundingUncertain,
            ),
        ] {
            for limits in [
                None,
                Some(FundingLimits::new(1, 1, 1).expect("valid limits")),
                Some(FundingLimits::new(500, 10, 100).expect("valid limits")),
            ] {
                for requested in [1, 500, u128::MAX] {
                    let observation = FundingAdmissionObservation {
                        available_cycles: u128::MAX,
                        recovery: RecoveryState::Reconciled,
                        activity,
                    };
                    for _ in 0..2 {
                        assert_eq!(
                            assess_funding_admission(limits, amount(requested), observation),
                            FundingAdmissionDecision::Blocked(blocker),
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn clear_reconciled_state_still_requires_configuration_and_the_full_reserve() {
        let limits = FundingLimits::new(500, 10, 100).expect("valid limits");
        let observation = FundingAdmissionObservation {
            available_cycles: 1_000,
            recovery: RecoveryState::Reconciled,
            activity: FundingActivity::Clear,
        };
        assert_eq!(
            assess_funding_admission(None, amount(500), observation),
            FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::NotConfigured),
        );
        assert_eq!(
            assess_funding_admission(Some(limits), amount(500), observation),
            FundingAdmissionDecision::PrepareIntent {
                requested_cycles: amount(500)
            },
        );
        for available_cycles in [0, 499, 500, 1_000] {
            assert_eq!(
                assess_funding_admission(
                    Some(limits),
                    amount(501),
                    FundingAdmissionObservation {
                        available_cycles,
                        ..observation
                    },
                ),
                FundingAdmissionDecision::Blocked(
                    FundingAdmissionBlocker::ReserveWouldBeViolated {
                        requested_cycles: amount(501),
                        transferable_cycles: available_cycles.saturating_sub(500),
                    }
                ),
            );
        }
        let limits = FundingLimits::new(1, 1, 1).expect("valid limits");
        assert_eq!(
            assess_funding_admission(
                Some(limits),
                amount(u128::MAX - 1),
                FundingAdmissionObservation {
                    available_cycles: u128::MAX,
                    ..observation
                },
            ),
            FundingAdmissionDecision::PrepareIntent {
                requested_cycles: amount(u128::MAX - 1)
            },
        );
    }
}
