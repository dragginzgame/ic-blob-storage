//! Local diagnostic limits; neither configuration nor diagnosis grants funding authority.
use crate::balance::BalanceScope;
use candid::CandidType;
use serde::Deserialize;

/// Configure thresholds only for an explicitly selected current observation binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BillingLimitsInput {
    /// Exact service, namespace, source and account.
    pub scope: BalanceScope,
    /// Expected balance configuration revision; prevents stale reconfiguration.
    pub revision: u64,
    /// Positive local reserve threshold, not observed local funds.
    pub reserve: u128,
    /// Positive minimum reported provider balance.
    pub minimum: u128,
    /// Positive target, at least the minimum.
    pub target: u128,
}

/// Retained validated limits, without inferring spendable accounting.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BillingLimitsView {
    /// Positive local reserve.
    pub reserve: u128,
    /// Minimum reported provider balance.
    pub minimum: u128,
    /// Target reported provider balance.
    pub target: u128,
}

/// Threshold result, distinct from reserve feasibility and payment permission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingNeedView {
    /// Current scope has no installed limits.
    NotConfigured,
    /// Report meets the minimum.
    NotNeeded,
    /// No usable current report exists.
    BalanceUnavailable,
    /// Current reply failed validation.
    BalanceMalformed,
    /// Exact target shortfall; local spendability remains unknown.
    TopUp(u128),
}

/// Read-only local diagnosis, including limits invalidated by a changed scope.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BillingStatusView {
    /// Retained scope, not automatically rebound when observation configuration changes.
    pub scope: Option<BalanceScope>,
    /// Retained configuration revision.
    pub revision: Option<u64>,
    /// Validated retained amounts.
    pub limits: Option<BillingLimitsView>,
    /// Whether retained limits still belong to the current configuration.
    pub current: bool,
    /// Shared balance-threshold assessment, never a transfer recommendation.
    pub funding_need: FundingNeedView,
}
