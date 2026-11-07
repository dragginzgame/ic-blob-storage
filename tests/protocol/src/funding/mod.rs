//! Controlled receiver behavior and projections of production funding observations.
use ic_platform::principal::ExternalPrincipal;

use serde::{Deserialize, Serialize};

/// Controlled response after the receiver accepts the requested cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum FundingReplyMode {
    /// Replay the independent Candid success fixture.
    Success,
    /// Commit acceptance, then delay success across bounded actual IC rounds.
    DelayedSuccess,
    /// Replay the independent Candid `InternalError` fixture.
    InternalError,
    /// Replay the independent Candid authorization failure fixture.
    NotAuthorized,
    /// Replay the independent Candid balance-overflow fixture.
    AccountBalanceOverflow,
    /// Replay the independent Candid missing-cycles fixture.
    TopUpWithoutCycles,
    /// Replay a ledger notification on the direct-top-up route; must reject.
    LedgerReport,
    /// Replay an unknown provider error; must remain unusable.
    UnknownError,
    /// Return non-Candid bytes.
    Malformed,
    /// Explicitly reject after accepting cycles.
    Reject,
    /// Trap after acceptance and the journal write, rolling both back.
    Trap,
}

/// Same-release fixture lifecycle controls, not a production upgrade interface.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct FundingUpgradeArgs {
    /// Fail the real upgrade after restoring the journal synchronously.
    pub trap_after_restore: bool,
}

/// Exact incoming attachment and scripted receiver behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct FundingRequest {
    /// Fixture operation identity; never reused or automatically retried.
    pub id: u64,
    /// Cycles attached to the inter-canister call.
    pub offered: u128,
    /// Cycles the controlled receiver attempts to accept.
    pub accept: u128,
    /// Controlled reply behavior.
    pub reply: FundingReplyMode,
}

/// Reply classification independent of the platform's cycle refund.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum FundingOutcome {
    /// The current production decoder accepts the balance report.
    ReportedSuccess,
    /// Advertised provider failure, independent of actual accepted/refunded cycles.
    ProviderError(FundingProviderErrorView),
    /// Candid decoding or balance validation failed.
    InvalidReply,
    /// Actual reject code returned by the IC.
    Rejected(u32),
    /// The CDK did not enqueue a call, so there is no callback refund.
    NotEnqueued,
    /// The post-persistence liquidity guard refused dispatch; no callback exists.
    LiquidityBlocked,
}

/// Passive projection of the library decoder's advertised top-up failures.
/// These observations never establish refunds, credit or authority to retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum FundingProviderErrorView {
    /// The provider-reported principal, not inferred caller or account authority.
    NotAuthorized(ExternalPrincipal),
    /// Reported balance overflow, not evidence that no cycles were accepted.
    AccountBalanceOverflow,
    /// Internal failure with diagnostic text discarded by the shared decoder.
    InternalError,
    /// Reported absence of cycles; actual callback facts remain authoritative.
    TopUpWithoutCycles,
}

/// Passive projection of shared reconciliation policy for the experiment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub enum FundingReconciliationView {
    /// Matching successful provider callback, without independent credit proof.
    ProviderAcknowledged(u128),
    /// Exact receipt retained by the trusted host, separate from balance reports.
    CreditConfirmed {
        /// Exact known positive attachment accepted by this operation.
        accepted_cycles: u128,
        /// Fingerprint of uniquely attributed synthetic host evidence.
        receipt_digest: [u8; 32],
    },
    /// No attached cycles were transferred; execution fees remain separate.
    NoTransfer,
    /// Exact accepted amount still needs provider credit evidence.
    CreditRequired(u128),
    /// The full attachment remains potentially spent.
    TransferUnknown(u128),
}

/// Independently recorded receiver-side acceptance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct FundingReceiptRecord {
    /// Original operation identity.
    pub id: u64,
    /// Cycles available in this call.
    pub available: u128,
    /// Actual return value of the platform accept operation.
    pub accepted: u128,
}

/// Admission failures happen before an external effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub enum FundingFailure {
    /// Caller is not the fixture driver.
    Denied,
    /// Invalid expected argument bytes or exhausted receiver control capacity.
    Limit,
}

/// Funding journal observations are independent of available balances.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum FundingActivityView {
    /// Observed journal has no outstanding attempt; this says nothing about freshness.
    Clear,
    /// Exact intent is reserved or executing.
    InProgress,
    /// Prior effect is unresolved.
    Uncertain,
}
