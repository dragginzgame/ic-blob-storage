//! Admission of a new funding intent from independently established observations.
//!
//! This policy neither acquires a lock nor persists an intent. A positive result
//! is not permission to call a provider, resume an attempt or repeat a payment.

pub mod attempt;
pub mod evidence;
pub mod journal;

use std::num::NonZeroU128;

use crate::policy::billing::FundingDecision;
use crate::policy::billing::RecoveryState;
use crate::policy::billing::assess_funding;
use ic_blob_storage_contracts::configuration::funding::FundingLimits;

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
mod tests;
