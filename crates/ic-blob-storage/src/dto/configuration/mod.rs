//! Explicit local service configuration inputs, not a provider wire or stable schema.
use candid::{CandidType, Deserialize, Principal};

/// Operator-only installed configuration and local restore state.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct HostConfigurationView {
    /// Exact installed configuration; observation performs no provider calls.
    pub configuration: ServiceConfigurationInput,
    /// Immutable installed provider project, independent of the payer and tenants.
    pub project: String,
    /// Explicit installed verifier, independent of gateway membership.
    pub completion_verifier: Principal,
    /// Host-validated release identity, not a module hash or freshness authority.
    pub release: String,
    /// Restored stores allow inspection only; false is not provider readiness.
    pub fenced: bool,
}

/// Installed-configuration inspection authority rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum HostFailure {
    /// Caller or service differs from the installed binding; controllers gain no authority.
    Denied,
}

/// Candidate configuration for a separately authorized host installation.
/// No defaults, deployment authorization, provider namespace proof or mutation is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ServiceConfigurationInput {
    /// Exact intended canister, checked against the host's actual identity.
    pub service: Principal,
    /// Explicit operator; controllers do not gain tenant authority.
    pub operator: Principal,
    /// Selected payer account; a different payer needs separate relationship proof.
    pub payment_account: Principal,
    /// Positive local namespace identity, not a provisioned gateway project/bucket.
    pub namespace: u128,
    /// Explicit upload/catalog envelope; counts are portable to Wasm32.
    pub resources: ServiceResourceInput,
    /// Candidate Cashier and local funding policy; no payment is performed.
    pub billing: ServiceBillingInput,
    /// Separate local attachment allocation and retained funding history bound.
    pub funding: ServiceFundingInput,
    /// Concurrent read occupancy and response buffer budgets.
    pub reads: ServiceReadInput,
}

/// Local attachment budget, not platform liquidity or provider credit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ServiceFundingInput {
    /// Total lifetime attachment allocation in cycles.
    pub allocated: u128,
    /// Positive amount kept out of attachment offers within this allocation.
    pub reserve: u128,
    /// Positive lifetime journal capacity, including uncertain and completed attempts.
    pub max_attempts: u32,
}

/// Explicit concurrent read bounds; abandoned sessions continue consuming capacity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ServiceReadInput {
    /// Global simultaneous session slots.
    pub sessions: u32,
    /// Simultaneous slots per tenant.
    pub tenant_sessions: u32,
    /// Maximum response buffer reserved by one session.
    pub reply_bytes: u32,
    /// Global reserved response buffer bytes.
    pub bytes: u64,
    /// Reserved response buffer bytes per tenant.
    pub tenant_bytes: u64,
}

/// Positive resource limits; conversion also checks all existing model relationships.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ServiceResourceInput {
    /// Lifetime tenant slots, including suspended entries.
    pub max_tenants: u32,
    /// Largest admitted object in bytes.
    pub max_object_bytes: u64,
    /// Metadata entries per manifest.
    pub max_headers: u32,
    /// Framed metadata bytes per manifest.
    pub max_header_bytes: u32,
    /// Global lifetime retained leaf slots.
    pub max_chunks: u32,
    /// Lifetime retained leaf slots per tenant.
    pub max_tenant_chunks: u32,
    /// Global lifetime object slots.
    pub max_objects: u32,
    /// Lifetime object slots per tenant.
    pub max_tenant_objects: u32,
    /// Global stored-byte budget, including unreconciled obligations.
    pub max_physical_bytes: u128,
    /// Global byte budget for continuing billing obligations.
    pub max_liability_bytes: u128,
    /// Tenant logical-byte budget.
    pub max_tenant_logical_bytes: u128,
    /// Lifetime reference slots per object.
    pub max_references_per_object: u32,
    /// Lifetime receipt slots per object, including reserved cleanup capacity.
    pub max_receipts_per_object: u32,
    /// Global concurrent upload reservations.
    pub max_active: u32,
    /// Concurrent upload reservations per tenant.
    pub max_tenant_active: u32,
}

/// Explicit provider identity and local cycle thresholds, with no service defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ServiceBillingInput {
    /// Configured Cashier candidate; validation does not prove deployed behavior.
    pub cashier: Principal,
    /// Positive local cycle reserve.
    pub reserve: u128,
    /// Positive minimum provider balance.
    pub minimum_balance: u128,
    /// Positive target balance, at least the minimum.
    pub target_balance: u128,
    /// Raw gateway-list entries including duplicates.
    pub max_gateway_entries: u32,
    /// Distinct gateway principals retained after validation.
    pub max_gateway_unique: u32,
}
