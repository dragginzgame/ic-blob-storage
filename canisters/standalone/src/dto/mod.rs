//! Standalone installation observations, without deployment defaults or readiness claims.
use candid::{CandidType, Deserialize};
use ic_blob_storage::dto::configuration::ServiceConfigurationInput;

/// Operator-only installed configuration and local restore state.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct HostConfigurationView {
    /// Exact installed configuration; observation performs no provider calls.
    pub configuration: ServiceConfigurationInput,
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
