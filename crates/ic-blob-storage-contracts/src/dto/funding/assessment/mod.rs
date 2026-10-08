//! Passive new-intent diagnosis, never a funding command or saved authorization.
use crate::dto::operator::LocalFundingStatus;
use crate::dto::operator::OperatorScope;
use candid::CandidType;
use candid::Deserialize;

/// Explicit proposed identity and complete attachment; no amount is defaulted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingPreparationRequest {
    /// Exact installed service/namespace/Cashier/payer scope.
    pub scope: OperatorScope,
    /// Positive proposed local operation; this query allocates nothing.
    pub operation: u128,
    /// Full positive attachment offer, excluding execution fees.
    pub offered: u128,
    /// Exact optional positive provider target; absence remains absent.
    pub target_balance: Option<u128>,
}
/// Independent current reason preparation is blocked in the unqualified hosts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingPreparationBlocker {
    /// Installed provider/account behavior has not been qualified.
    ProviderUnqualified,
    /// The actual journal retains its restore fence.
    JournalFenced,
    /// Accepted/reserved local attachments remain unresolved or uncredited.
    JournalUncredited,
    /// This exact identity already exists; inspect the original outcome instead.
    IdentityRetained,
    /// Proposed new identity is no greater than retained history.
    IdentityStale,
    /// Lifetime retained intent capacity is exhausted.
    JournalFull,
    /// The full request exceeds local attachment allowance; never a partial offer.
    AllocationReserve {
        /// Current local allowance, not spendable cycles or dispatch authority.
        transferable_cycles: u128,
    },
    /// Independent safe identity/allocation/accounting recovery is unestablished.
    RecoveryUnknown,
    /// Complete account activity across installations/payers is unestablished.
    FundingUnknown,
    /// Spendable cycles after complete holds/liabilities are unestablished.
    SpendabilityUnknown,
}
/// Synchronous observation of local facts and explicitly missing host evidence.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FundingPreparationResponse {
    /// Complete echoed proposed request, including optional target.
    pub request: FundingPreparationRequest,
    /// Maintained local attachment/history facts; never provider credit/liquidity.
    pub journal: LocalFundingStatus,
    /// Independent local and external blockers; this report grants no permission.
    pub blockers: Vec<FundingPreparationBlocker>,
}
/// Rejected assessment leaves all journals unchanged and calls no provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum FundingPreparationFailure {
    /// Zero namespace, operation, offer or present target.
    Invalid,
    /// Actual caller is not the configured operator.
    Denied,
    /// Actual/requested service or installed account scope differs.
    Binding,
    /// Proposed retained identity differs from the original offer/target.
    Conflict,
    /// Retained state or current diagnostic profile is inconsistent.
    Internal,
}
