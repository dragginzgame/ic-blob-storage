//! Passive provider reports; no spendability, credit or configuration authority.
use crate::dto::funding::outcome::FundingReportedBalance;
use crate::dto::operator::OperatorScope;
use ic_platform::Principal;
use ic_platform::candid::contract::Int;
use ic_platform::candid::contract::Nat;
use serde::Deserialize;

/// Explicit observation kind; account and owner come from the installed scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum AccountInspectionKind {
    /// Reported payer balances.
    Balance,
    /// Reported relationship between the service owner and installed payer.
    PaymentRelationship,
}
/// Operator-selected read, never arbitrary provider arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccountInspectionRequest {
    /// Exact installed scope checked before dispatch and again after the await.
    pub scope: OperatorScope,
    /// One independently requested observation, not an atomic account snapshot.
    pub kind: AccountInspectionKind,
}
/// Provider relationship figures retained without arithmetic or sentinel interpretation.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccountRelationshipView {
    /// Exact paid canister, checked against the service.
    pub paid_canister: Principal,
    /// Exact expected payer, checked independently.
    pub payment_account: Principal,
    /// Signed daily limit, not assumed to be an allowance.
    #[serde(with = "crate::wire::decimal")]
    pub spending_limit_per_day: Int,
    /// Signed reported spend.
    #[serde(with = "crate::wire::decimal")]
    pub current_period_spent: Int,
    /// Uninterpreted provider period start.
    pub current_period_start: u64,
    /// Uninterpreted provider creation timestamp.
    pub added_timestamp: u64,
    /// Optional provider expiry; absence grants no perpetual authority.
    pub expiration_timestamp: Option<u64>,
    /// Arbitrary-width reported uploaded baseline.
    #[serde(with = "crate::wire::decimal")]
    pub bandwidth_baseline_uploaded: Nat,
    /// Arbitrary-width reported downloaded baseline.
    #[serde(with = "crate::wire::decimal")]
    pub bandwidth_baseline_downloaded: Nat,
    /// Provider baseline timestamp.
    pub bandwidth_baseline_ts_ns: u64,
}
/// Account reports and structured provider errors remain distinct from transport failure.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum AccountObservation {
    /// Independently reported balance components, not proof of payment credit.
    Balance(FundingReportedBalance),
    /// Provider reported no balance account; never a zero balance.
    AccountNotFound,
    /// Present relationship matching both expected principals.
    Relationship(Box<AccountRelationshipView>),
    /// No compatible relationship returned; optional subtyping can also cause this.
    NoRelationshipReported,
    /// Provider-reported missing relationship; diagnostic principal is not authority.
    RelationshipNotFound(Principal),
    /// Provider-reported authorization error.
    NotAuthorized(Principal),
    /// Provider rejected the request, without exposing diagnostic text.
    InvalidRequest,
    /// Provider internal failure, without exposing diagnostic text.
    ProviderInternalError,
}
/// Reply bound to this exact invocation; not persisted or usable as a funding permit.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccountInspectionResponse {
    /// Exact request, including installed scope and selected observation kind.
    pub request: AccountInspectionRequest,
    /// Independently validated provider observation.
    pub observation: AccountObservation,
}
/// Refusal or unusable observation; none supplies fallback account data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum AccountInspectionFailure {
    /// Invalid positive bounds or principal.
    Invalid,
    /// Actual caller is not the operator.
    Denied,
    /// Installed scope, response account or source mismatch.
    Binding,
    /// A restored owner cannot initiate or complete provider observations.
    Fenced,
    /// Inconsistent local state.
    Internal,
    /// Platform did not enqueue the read.
    NotEnqueued,
    /// Raw platform rejection code.
    Rejected(u32),
    /// Reply exceeds the application byte budget.
    ReplyTooLarge,
    /// Malformed, over-budget or invalid provider figures.
    InvalidReply,
}
