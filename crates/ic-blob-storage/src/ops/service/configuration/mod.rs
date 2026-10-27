//! Convert the shared checked resource profile into service-owned accounting.
use super::stores::{ServiceStoreConfiguration, ServiceStoreError};
use crate::model::billing::allocation::FundingAllocation;
use ic_blob_storage_contracts::configuration;
use ic_blob_storage_contracts::configuration::ConfigurationInputError;
use ic_blob_storage_contracts::dto::configuration::ServiceConfigurationInput;
use ic_platform::Principal;
use std::num::{NonZeroU128, NonZeroUsize};
/// Validate and convert immutable inputs; allocation accounting remains service-owned.
/// # Errors
/// Rejects the canonical pure input or the current stable-owner envelope.
pub fn validate_candidate(
    actual_service: Principal,
    input: ServiceConfigurationInput,
) -> Result<ServiceStoreConfiguration, ServiceConfigurationAdapterError> {
    let profile = configuration::validate_candidate(actual_service, input)?;
    Ok(from_validated(&profile)?)
}

pub(super) fn from_validated(
    profile: &configuration::ValidatedConfiguration,
) -> Result<ServiceStoreConfiguration, ServiceStoreError> {
    let f = profile.funding();
    let funding = FundingAllocation::new(
        f.allocated,
        NonZeroU128::new(f.reserve).expect("validated reserve"),
        NonZeroUsize::new(f.max_attempts as usize).expect("validated attempts"),
    )
    .expect("validated allocation")
    .with_renewal_ceiling(f.renewal_ceiling)
    .expect("validated ceiling");
    ServiceStoreConfiguration::new(profile.service(), funding)
}
/// Service construction rejects pure input and retained owner envelopes separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ServiceConfigurationAdapterError {
    /// Passive candidate validation failed.
    #[error(transparent)]
    Input(#[from] ConfigurationInputError),
    /// Current stable-owner codec or resource envelope is unsupported.
    #[error(transparent)]
    Stores(#[from] ServiceStoreError),
}
#[cfg(test)]
pub(super) mod tests;
