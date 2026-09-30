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

/// Shared installed-configuration observations used by both endpoint adapters.
pub use ic_blob_storage::dto::configuration::{HostConfigurationView, HostFailure};
