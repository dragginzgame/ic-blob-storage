//! Exact retained observations, distinct from provider credit or payment authority.
use crate::dto::funding::FundingPhase;
use crate::dto::operator::OperatorScope;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;

/// Complete original local intent; changed amounts or target options must reject.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingOutcomeRequest {
    /// Installed service, namespace, Cashier and payer.
    pub scope: OperatorScope,
    /// Positive local operation identity, not a provider receipt ID.
    pub operation: u128,
    /// Original positive attachment, excluding execution fees.
    pub offered: u128,
    /// Exact optional positive target balance; no inferred default.
    pub target_balance: Option<u128>,
}
/// Provider-reported balance components, never a credited-amount receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingReportedBalance {
    /// Reported total.
    pub total: u128,
    /// Reported prepaid cycles.
    pub prepaid: u128,
    /// Reported promotional cycles.
    pub promotional: u128,
    /// Reported ledger cycles.
    pub ledger: u128,
}
/// Balance component whose decoding or amount validation failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingBalanceField {
    /// Total balance.
    Total,
    /// Prepaid balance.
    Prepaid,
    /// Promotional balance.
    Promotional,
    /// Ledger balance.
    Ledger,
}
/// Retained structured response, independent of the exact callback refund.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingResponse {
    /// The prepared call was consumed without polling it.
    NotDispatched,
    /// The platform did not enqueue a call.
    NotEnqueued,
    /// Raw IC rejection code, without diagnostic text.
    Rejected(u32),
    /// Validated reported balance, not proof of credit or freshness.
    ReportedSuccess(FundingReportedBalance),
    /// Provider-reported principal, not caller authority.
    NotAuthorized(Principal),
    /// Provider-reported balance overflow; refund accounting stays independent.
    AccountBalanceOverflow,
    /// Provider internal error; diagnostic text is discarded.
    InternalError,
    /// Provider reported no attached cycles; local transport facts stay authoritative.
    TopUpWithoutCycles,
    /// Reply exceeded the configured decoding byte budget.
    ReplyTooLarge,
    /// Malformed, incompatible or over-budget Candid.
    InvalidReply,
    /// Reported balance component could not be represented or validated.
    InvalidBalance(FundingBalanceField),
}
/// Conservative follow-up derived only from retained attachment facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingReconciliation {
    /// Immutable receipt established by the trusted host, not inferred from balance.
    CreditConfirmed {
        /// Exact positive transport-accepted attachment covered by the evidence.
        accepted_cycles: u128,
        /// SHA-256 of preserved provider credit evidence; not a proof token.
        receipt_digest: [u8; 32],
    },
    /// No attached cycles transferred; execution fees are separate.
    NoTransfer,
    /// Exact transport acceptance still requires independent provider-credit evidence.
    CreditRequired(u128),
    /// Full original attachment remains potentially spent.
    TransferUnknown(u128),
}
/// One retained local observation; no field authorizes retry or unfencing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingOutcomeResponse {
    /// Complete original intent checked before disclosure.
    pub request: FundingOutcomeRequest,
    /// Local transport progress.
    pub phase: FundingPhase,
    /// Absent when no structured response was durably retained.
    pub response: Option<FundingResponse>,
    /// Independent diagnosis; a success report cannot clear accepted attachment.
    pub reconciliation: FundingReconciliation,
    /// Immutable local budget authorization increase; zero means no grant.
    /// This is neither refunded cycles nor provider credit.
    pub renewed_allocation: u128,
    /// Restored journal permits inspection only.
    pub fenced: bool,
}
/// Exact inspection refusal; absence is returned separately and never permits retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingOutcomeFailure {
    /// Zero namespace, operation, offer or present target balance.
    Invalid,
    /// Caller is not the configured operator.
    Denied,
    /// Wrong actual service or installed scope.
    Binding,
    /// Retained intent differs from the supplied original amounts/options.
    Conflict,
    /// Retained state cannot be read consistently.
    Internal,
}
