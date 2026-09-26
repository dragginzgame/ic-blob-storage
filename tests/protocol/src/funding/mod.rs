//! Local cycle-transfer controls; these are not Cashier request DTOs.

use candid::CandidType;
use serde::Deserialize;

/// Controlled response after the receiver accepts the requested cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingReplyMode {
    /// Replay the independent Candid success fixture.
    Success,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
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
    /// This identity was already admitted, including unresolved work.
    AlreadyAdmitted,
    /// A different operation remains unresolved.
    InProgress,
    /// Invalid amount or exhausted lifetime journal capacity.
    Limit,
}
