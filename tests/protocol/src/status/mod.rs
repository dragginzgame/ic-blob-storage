//! Read-only diagnostics for the local fixture, not a production service API.
use crate::authority::{ArchiveCatalog, ArchivePhase, ArchivedReadView};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Summary of one independent fixture catalog; totals never combine its quota
/// with another catalog's independently configured capacity.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct CatalogStatusView {
    /// Owning fixture catalog.
    pub catalog: ArchiveCatalog,
    /// All lifetime operations/objects, including byte-free history.
    pub objects: u64,
    /// Phase counts, including zero counts, in a fixed bounded order.
    pub phases: Vec<PhaseCountView>,
    /// Retained release receipts; settlement does not erase them.
    pub release_receipts: u64,
    /// Charged logical bytes including reservations.
    pub logical: u128,
    /// Charged physical bytes including reservations.
    pub physical: u128,
    /// Charged billing-liability bytes including reservations, not cycle amounts.
    pub liability: u128,
}

/// Count within one catalog, without exposing roots or verifier state.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct PhaseCountView {
    /// Retained local phase.
    pub phase: ArchivePhase,
    /// Number of objects in this phase.
    pub objects: u64,
}

/// Operator-only snapshot of the current active or frozen inspection owner.
/// None-valued economic fields mean unobserved, never zero or reconciled.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct OperatorStatusView {
    /// Actual service binding.
    pub service: Principal,
    /// Bound provider namespace.
    pub namespace: u128,
    /// Permanent restoration fence, if set.
    pub fenced: bool,
    /// Always false for the local provider substitute.
    pub provider_qualified: bool,
    /// Whether this fixture owns installed billing configuration.
    pub billing_configured: bool,
    /// Bound diagnostic limits and shared threshold assessment.
    pub billing: crate::billing::BillingStatusView,
    /// Locally age-bounded simulated report; never spendable funds or provider credit.
    pub provider_balance: Option<u128>,
    /// Scoped local balance history, never provider credit or spendable funds.
    pub balance_observation: crate::balance::BalanceStatusView,
    /// Spendable cycles after authoritative reservations; not the canister balance.
    pub available_funding_cycles: Option<u128>,
    /// Funding journal observation; absent means unknown.
    pub funding_activity: Option<FundingActivityView>,
    /// Retained current gateway principals; a pending sync does not replace them.
    pub gateways: Vec<Principal>,
    /// Exact pending list sequence, without a reusable token.
    pub pending_sync: Option<u64>,
    /// Controlled local list source, not a deployed Cashier binding.
    pub sync_source: Principal,
    /// Last allocated sync sequence, including failed attempts.
    pub last_sync: u64,
    /// Operator-edit revision; absent means exhausted and permanently blocked.
    pub sync_revision: Option<u64>,
    /// Exact pending read, including stale authority and trapped callbacks.
    pub pending_read: Option<ArchivedReadView>,
    /// The three separately bounded fixture catalogs.
    pub catalogs: Vec<CatalogStatusView>,
    /// Diagnosis blockers; this report grants no authority even when empty.
    pub blockers: Vec<OperatorBlockerView>,
    /// Outstanding work and shared billing warnings.
    pub warnings: Vec<OperatorWarningView>,
}

/// Funding journal observations are independent of available balances.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum FundingActivityView {
    /// Observed journal has no outstanding attempt; this says nothing about freshness.
    Clear,
    /// Exact intent is reserved or executing.
    InProgress,
    /// Prior effect is unresolved.
    Uncertain,
}

/// Existing shared billing diagnosis at the boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum BillingBlockerView {
    /// Recovery is fenced.
    RecoveryFenced,
    /// Billing configuration is absent.
    NotConfigured,
    /// No gateway is available.
    GatewayPrincipalsMissing,
    /// No valid observation was obtained.
    BalanceUnavailable,
    /// A balance observation failed validation.
    BalanceMalformed,
    /// Balance is below the configured minimum.
    InsufficientBalance,
    /// The whole funding suggestion cannot fit above reserve.
    ReserveWouldBeViolated,
}

/// Preserves the shared diagnosis without interpreting it as an action permit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum OperatorBlockerView {
    /// No authoritative recovery assessment was supplied.
    RecoveryUnknown,
    /// Recovery must remain fenced.
    RecoveryFenced,
    /// Provider behavior has not been qualified.
    ProviderUnqualified,
    /// No current gateway authority exists.
    GatewaysMissing,
    /// Billing configuration has not been installed.
    BillingNotConfigured,
    /// Required billing/accounting inputs are missing.
    BillingUnavailable,
    /// Local spendable-cycle accounting is unobserved, never zero.
    SpendabilityUnknown,
    /// Blocker from the shared billing policy.
    Billing(BillingBlockerView),
    /// No authoritative funding journal was observed.
    FundingUnknown,
    /// An exact funding intent is pending.
    FundingInProgress,
    /// Earlier funding effects remain unresolved.
    FundingUncertain,
}

/// Warnings from the shared billing diagnosis.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum BillingWarningView {
    /// No gateway is available.
    GatewayPrincipalSetEmpty,
    /// Balance observation is unavailable.
    BalanceUnavailable,
    /// Balance observation was invalid.
    BalanceMalformed,
}

/// Work that remains visible independently of readiness blockers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum OperatorWarningView {
    /// A read-only gateway sync is pending.
    SyncPending,
    /// A source read remains pending.
    ReadPending,
    /// Upload authority may have escaped without known completion.
    UploadCompletionUnknown,
    /// Released objects remain physically charged.
    DeletionPending,
    /// Deleted objects remain economically charged.
    BillingCessationPending,
    /// Warning from existing billing policy.
    Billing(BillingWarningView),
}
