//! Controlled balance observations, not a production account or Cashier API.
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Explicit local observation binding; no default project/account is inferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BalanceScope {
    /// Authority fixture answering status.
    pub service: Principal,
    /// Namespace owned by that authority fixture.
    pub namespace: u128,
    /// Controlled local source, never an implicitly selected Cashier.
    pub source: Principal,
    /// Account expected inside the independent Candid reply bytes.
    pub account: Principal,
}

/// Exact local read admission. Reusing a consumed sequence cannot dispatch again.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BalanceRefreshRequest {
    /// All expected observation bindings.
    pub scope: BalanceScope,
    /// Exact current configuration revision.
    pub revision: u64,
    /// Expected next lifetime attempt number, starting at one.
    pub sequence: u64,
}

/// Admission or observation failure; errors never stand in for zero cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum BalanceFailure {
    /// Caller lacks explicit operator authority.
    Denied,
    /// Restored inspection owner cannot refresh or configure.
    Fenced,
    /// Invalid or mismatched scope.
    Binding,
    /// No configured observation scope.
    NotConfigured,
    /// Numeric billing limits failed validation.
    InvalidLimits,
    /// An earlier read is still pending, including invalidated reads.
    Busy,
    /// Lifetime attempts or configuration sequence exhausted.
    Limit,
    /// Actual local call rejected or could not be enqueued.
    Transport,
    /// Response exceeded the byte budget.
    Oversized,
    /// Invalid Candid or invalid signed/oversized amounts.
    Malformed,
    /// Returned account differs from the captured request.
    AccountMismatch,
    /// Structured provider-shaped failure from the substitute.
    AccountNotFound,
    /// Structured internal failure from the substitute.
    ProviderInternal,
    /// Expected revision/sequence is stale, or configuration changed during the call.
    Stale,
}

/// All independently reported components; total is not recomputed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BalanceAmountsView {
    /// Reported total cycles.
    pub total: u128,
    /// Reported prepaid cycles.
    pub prepaid: u128,
    /// Reported promotional cycles.
    pub promotional: u128,
    /// Reported ledger cycles.
    pub ledger: u128,
}

/// Retained outcome; reports do not prove payment credit or provider freshness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum BalanceOutcomeView {
    /// Exact validated response and local receive time in nanoseconds.
    Reported(BalanceAmountsView, u64),
    /// Failed observation, preserved independently of prior reports.
    Failed(BalanceFailure),
}

/// One immutable intent and its optional terminal result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BalanceAttemptView {
    /// Lifetime read identity, never reused.
    pub sequence: u64,
    /// Configuration revision captured before dispatch.
    pub revision: u64,
    /// Exact source/account/service/namespace context.
    pub scope: BalanceScope,
    /// Local dispatch time, in nanoseconds.
    pub started_at: u64,
    /// Absent means pending, including restored callback uncertainty.
    pub outcome: Option<BalanceOutcomeView>,
}

/// Why the latest retained report is or is not usable for local display.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum BalanceUsability {
    /// No attempt under this configuration.
    Unobserved,
    /// Most recent call has no completion.
    Pending,
    /// Valid local reply within the fixture age bound; not provider freshness proof.
    Observed,
    /// Local age bound exceeded or the clock precedes dispatch/receipt.
    Expired,
    /// Configuration changed since the latest attempt.
    Invalidated,
    /// Latest observation failed.
    Failed,
    /// Inspection-only restoration denies use of retained reports.
    Fenced,
}

/// Read-only bounded history of simulated balance observations.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BalanceStatusView {
    /// Current explicit configuration, if any.
    pub configured: Option<BalanceScope>,
    /// Current configuration revision.
    pub revision: u64,
    /// Local fixture observation age limit, in nanoseconds.
    pub max_age_ns: u64,
    /// Current use classification, without altering retained history.
    pub usability: BalanceUsability,
    /// At most sixteen lifetime attempts; no recycling or reset.
    pub attempts: Vec<BalanceAttemptView>,
}

/// Driver-installed raw response, independent of the production decoder schema.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BalanceSourceConfig {
    /// Account the incoming request must explicitly name.
    pub account: Principal,
    /// Independent Candid fixture bytes; up to 4,097 for the oversize case.
    pub bytes: Vec<u8>,
    /// Reject instead of returning the bytes.
    pub reject: bool,
    /// Hold across actual IC rounds until explicitly resumed or bounded exhaustion.
    pub hold: bool,
}

/// Driver-only source scheduling evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct BalanceSourceView {
    /// Restored source cannot answer or resume.
    pub fenced: bool,
    /// Lifetime received calls, bounded to 64.
    pub requests: u64,
    /// Exact held account request, if any.
    pub pending: Option<Principal>,
}
