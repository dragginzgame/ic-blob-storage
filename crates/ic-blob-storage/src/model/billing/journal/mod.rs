//! Exact local funding intents. These values supply no permission to spend cycles.
pub mod credit;
pub(crate) mod record;
pub mod renewal;
use candid::Principal;
use std::num::NonZeroU128;
use thiserror::Error;

/// Explicit local journal scope; not proof of complete provider-account activity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingJournalScope {
    /// Actual service holding this journal.
    pub service: Principal,
    /// Installed Cashier candidate, not provider qualification.
    pub cashier: Principal,
    /// Explicit payment account.
    pub account: Principal,
    /// Installed provider namespace.
    pub namespace: NonZeroU128,
}

/// Complete local attachment identity; scope and amount must match on every retry.
/// Operation IDs must increase for new intents. This ordering is not independent
/// freshness authority after restore and must never be used to release a fence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingIntent {
    /// Actual service holding the allocation.
    pub service: Principal,
    /// Configured Cashier candidate, not deployment proof.
    pub cashier: Principal,
    /// Explicit configured payment account.
    pub account: Principal,
    /// Installed provider namespace.
    pub namespace: NonZeroU128,
    /// Externally supplied identity, retained for the installation's lifetime.
    pub operation: NonZeroU128,
    /// Exact positive cycle attachment, separate from execution fees or credit.
    pub offered: NonZeroU128,
    /// Exact optional Cashier target balance; absence is preserved, never defaulted.
    pub target_balance: Option<NonZeroU128>,
}
/// Independently established callback context, never taken from reply payloads.
/// Matching these fields checks correlation; the host must authenticate transport.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingTransportContext {
    /// Actual running service receiving the outcome.
    pub service: Principal,
    /// Actual original call target supplied by trusted transport context.
    pub cashier: Principal,
}
/// Retained local progress, never provider credit or retry authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingIntentState {
    /// Full attachment reserved; no local attempt marker yet.
    Prepared,
    /// Attempt marker persisted; entire attachment remains potentially spent.
    Uncertain,
    /// Trusted host supplied positive proof that the attempt was never enqueued.
    NotEnqueued,
    /// Exact refund from the matching unbounded callback; not evidence of credit.
    Callback {
        /// Cycles refunded, bounded by the original attachment.
        refunded: u128,
    },
}
/// Current exact intent and its local progress, also readable while fenced.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingIntentView {
    /// Original scope, identity and amount.
    pub intent: FundingIntent,
    /// Current local progress.
    pub state: FundingIntentState,
}
/// Independently correlated transport evidence supplied by a trusted host.
/// Missing replies/timeouts must not be converted to either variant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingTransportOutcome {
    /// Positive evidence of enqueue failure, with no transfer or callback.
    NotEnqueued,
    /// Refund captured in the exact unbounded callback before another await.
    Callback {
        /// Exact refunded attachment, independent of reply decoding.
        refunded: u128,
    },
}
/// Local intent admission result. Neither variant authorizes provider dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingIntentAdmission {
    /// A new full attachment was reserved.
    Created,
    /// Exact replay; no new reservation or attempt.
    Existing(FundingIntentState),
}
/// Intent transition rejection, before any durable write.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum FundingIntentError {
    /// Supplied identity has changed scope or amount.
    #[error("funding intent conflict")]
    Conflict,
    /// Only a prepared intent can record its first attempt marker.
    #[error("funding intent already attempted")]
    AlreadyAttempted,
    /// Transport evidence cannot precede the attempt marker.
    #[error("funding intent not attempted")]
    NotAttempted,
    /// A different terminal transport result is already retained.
    #[error("funding outcome conflict")]
    OutcomeConflict,
    /// Exact refund arithmetic is invalid.
    #[error(transparent)]
    Transfer(#[from] super::transfer::FundingTransferError),
}
