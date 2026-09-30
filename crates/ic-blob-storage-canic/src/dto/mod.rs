//! Explicit managed installation policy; platform and compiled release bind identity.
use candid::{CandidType, Deserialize, Principal};
use ic_blob_storage::dto::configuration::{
    ServiceBillingInput, ServiceFundingInput, ServiceReadInput, ServiceResourceInput,
};

/// Complete managed application input, encoded as one Candid value inside Canic's
/// application bytes. No service principal or release can be supplied here: the
/// lifecycle binds actual platform identity and Canic's validated release authority.
/// Every policy value, provider project and verifier must be explicitly selected.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ManagedInstallationInput {
    /// Operator identity; controller/Root status never supplies this role.
    pub operator: Principal,
    /// Explicit payer account, independent of tenant and operator authority.
    pub payment_account: Principal,
    /// Local provider namespace; this does not provision a provider assignment.
    pub namespace: u128,
    /// Shared bounded service/tenant object and metadata budgets.
    pub resources: ServiceResourceInput,
    /// Shared Cashier, reserve, balance and gateway policy.
    pub billing: ServiceBillingInput,
    /// Shared allocated cycles, reserve and durable attempt budget.
    pub funding: ServiceFundingInput,
    /// Shared read-session and reply limits.
    pub reads: ServiceReadInput,
    /// Explicit Caffeine project mapped to the local namespace.
    pub project: String,
    /// Explicit trusted whole-content completion verifier.
    pub completion_verifier: Principal,
}
