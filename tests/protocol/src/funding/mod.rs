//! Local cycle-transfer controls; these are not Cashier request DTOs.

use crate::status::{FundingActivityView, OperatorBlockerView, OperatorWarningView};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Read-only current journal diagnosis for the local funding experiment.
/// No gateway/account binding, provider credit or recovery qualification is implied.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingOperatorStatusView {
    /// Actual canister answering this query.
    pub service: Principal,
    /// Journal-bound local transfer peer, never a qualified Cashier account.
    pub peer: Principal,
    /// Permanent inspection-only fence persisted by every successful restore.
    pub fenced: bool,
    /// Always false for this controlled substitute.
    pub provider_qualified: bool,
    /// Always false: attachment controls are not provider billing configuration.
    pub billing_configured: bool,
    /// Unobserved; decoded canned success replies do not supply account balances.
    pub provider_balance: Option<u128>,
    /// Unobserved; gross canister cycles do not establish spendable reservations.
    pub available_funding_cycles: Option<u128>,
    /// Complete outgoing journal diagnosis, independent of callback completion.
    pub funding_activity: FundingActivityView,
    /// Bounded lifetime attempts, including no-transfer and unresolved history.
    pub attempts: Vec<FundingAttemptStatusView>,
    /// This canister's incoming local acceptance receipts, never provider credit.
    pub receipts: Vec<FundingReceiptRecord>,
    /// Shared diagnostic blockers; these do not control experimental admission.
    pub blockers: Vec<OperatorBlockerView>,
    /// Shared diagnostic warnings, not retry advice.
    pub warnings: Vec<OperatorWarningView>,
}

/// Exact outgoing intent and transport facts, separately from provider credit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingAttemptStatusView {
    /// Lifetime operation identity within the bound service/peer journal.
    pub id: u64,
    /// Original attachment, including the whole uncertain amount after rollback.
    pub offered: u128,
    /// Exact callback refund; absent after missing callback or enqueue failure.
    pub refunded: Option<u128>,
    /// Known accepted cycles; enqueue failure has known zero, missing reply does not.
    pub transport_accepted: Option<u128>,
    /// Retained response classification, absent if callback completion rolled back.
    pub outcome: Option<FundingOutcome>,
    /// Independent provider credit remains unobserved, including after success.
    pub provider_credit: Option<u128>,
    /// Recomputed from the retained transport facts through shared policy.
    pub reconciliation: FundingReconciliationView,
}

/// Controlled response after the receiver accepts the requested cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingReplyMode {
    /// Replay the independent Candid success fixture.
    Success,
    /// Commit acceptance, then delay success across bounded actual IC rounds.
    DelayedSuccess,
    /// Replay the independent Candid `InternalError` fixture.
    ProviderError,
    /// Return non-Candid bytes.
    Malformed,
    /// Explicitly reject after accepting cycles.
    Reject,
    /// Trap after acceptance and the journal write, rolling both back.
    Trap,
}

/// Same-release fixture lifecycle controls, not a production upgrade interface.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingUpgradeArgs {
    /// Fail the real upgrade after restoring the journal synchronously.
    pub trap_after_restore: bool,
}

/// Exact identity and parameters of a local experiment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingRequest {
    /// Fixture operation identity; never reused or automatically retried.
    pub id: u64,
    /// Cycles attached to the inter-canister call.
    pub offered: u128,
    /// Cycles the controlled receiver attempts to accept.
    pub accept: u128,
    /// Controlled reply behavior.
    pub reply: FundingReplyMode,
    /// Trap in the sender callback after observing the refund.
    pub trap_callback: bool,
}

/// Reply classification independent of the platform's cycle refund.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum FundingOutcome {
    /// The current production decoder accepts the balance report.
    ReportedSuccess,
    /// The current production decoder reports the provider's `InternalError`.
    ProviderError,
    /// Candid decoding or balance validation failed.
    InvalidReply,
    /// Actual reject code returned by the IC.
    Rejected(u32),
    /// The CDK did not enqueue a call, so there is no callback refund.
    NotEnqueued,
}

/// Original call facts; enqueue failure has no callback refund to capture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingObservation {
    /// Exact platform refund, absent when no call was enqueued.
    pub refunded: Option<u128>,
    /// Known transport acceptance, zero for proven enqueue failure; never credit.
    pub transport_accepted: Option<u128>,
    /// Independent interpretation of the response bytes or transport failure.
    pub outcome: FundingOutcome,
    /// Shared policy's diagnosis; no variant authorizes another payment.
    pub reconciliation: FundingReconciliationView,
}

/// Passive projection of shared reconciliation policy for the experiment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingReconciliationView {
    /// No attached cycles were transferred; execution fees remain separate.
    NoTransfer,
    /// Exact accepted amount still needs provider credit evidence.
    CreditRequired(u128),
    /// The full attachment remains potentially spent.
    TransferUnknown(u128),
}

/// Persisted local experiment entry; not a production service schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingAttemptRecord {
    /// Original immutable operation.
    pub request: FundingRequest,
    /// None retains uncertainty, including after a callback trap.
    pub observation: Option<FundingObservation>,
}

/// Independently recorded receiver-side acceptance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingReceiptRecord {
    /// Original operation identity.
    pub id: u64,
    /// Cycles available in this call.
    pub available: u128,
    /// Actual return value of the platform accept operation.
    pub accepted: u128,
}

/// Admission failures happen before an external effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingFailure {
    /// Caller is not the fixture driver.
    Denied,
    /// Restored journals cannot send or receive further funding effects.
    Fenced,
    /// This identity was already admitted, including unresolved work.
    AlreadyAdmitted,
    /// A different operation remains unresolved.
    InProgress,
    /// Invalid amount or exhausted lifetime journal capacity.
    Limit,
}
