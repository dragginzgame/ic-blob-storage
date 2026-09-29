//! Standalone installation observations, without deployment defaults or readiness claims.
use candid::{CandidType, Deserialize};
use ic_blob_storage::dto::configuration::ServiceConfigurationInput;

/// Complete standalone installation, with an explicit provider project mapping.
/// Validation checks representation only; it does not provision a Caffeine project.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct HostInstallationInput {
    /// Shared service identity, tenant/resource bounds and provider economics.
    pub configuration: ServiceConfigurationInput,
    /// Provider project mapped to this service's local namespace; never inferred.
    pub project: String,
    /// Trusted external whole-content verifier; no controller or operator default.
    pub completion_verifier: candid::Principal,
}

/// Operator-only installed configuration and local restore state.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct HostConfigurationView {
    /// Exact installed configuration; observation performs no provider calls.
    pub configuration: ServiceConfigurationInput,
    /// Immutable installed provider project, independent of the payer and tenants.
    pub project: String,
    /// Explicit installed verifier, independent of gateway membership.
    pub completion_verifier: candid::Principal,
    /// Package release bound to this installation, not a module hash.
    pub release: String,
    /// Restored stores allow inspection only; false is not provider readiness.
    pub fenced: bool,
}
/// Inspection authority rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum HostFailure {
    /// Caller is not the configured operator; controller status is insufficient.
    Denied,
}
