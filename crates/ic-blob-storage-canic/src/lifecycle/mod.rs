//! Synchronous participant operations; the artifact explicitly owns publication.
use crate::{
    arguments::{self, InitializationArgumentFailure},
    memory::{self, ManagedMemory, MemoryAdoptionFailure},
};
use ic_blob_storage::{
    dto::configuration::ServiceConfigurationInput,
    ops::service::installation::{
        ServiceInstallation, ServiceInstallationCandidate, ServiceInstallationError,
        ValidatedServiceInstallation,
    },
};
use thiserror::Error;

/// Shared installation within Canic's already bootstrapped runtime.
pub type ManagedInstallation = ServiceInstallation<ManagedMemory>;

/// Install in the synchronous participant after Canic bootstrap, before activation.
/// Validate all application inputs before opening blob grants. The artifact supplies
/// its compiled release and expected memory authority, publishes only after success
/// and must trap on failure so
/// IC rolls back framework bootstrap and any partial writes. Do not use async setup.
/// # Errors
/// Rejects invalid inputs, missing committed grants or allocated service state.
/// # Panics
/// Stable write failures or aliased/corrupt grants trap; do not catch and continue.
pub fn install(release: &str, authority: &str) -> Result<ManagedInstallation, LifecycleFailure> {
    let size = ic0::msg_arg_data_size();
    if size > arguments::CARRIER_BYTES {
        return Err(InitializationArgumentFailure::CarrierBound.into());
    }
    let mut bytes = vec![0; size];
    ic0::msg_arg_data_copy(&mut bytes, 0);
    let input = arguments::installation_arguments(&bytes)?;
    let actual_service = ic_cdk::api::canister_self();
    let candidate = ValidatedServiceInstallation::new(
        actual_service,
        ServiceInstallationCandidate {
            configuration: ServiceConfigurationInput {
                service: actual_service,
                operator: input.operator,
                payment_account: input.payment_account,
                namespace: input.namespace,
                resources: input.resources,
                billing: input.billing,
                funding: input.funding,
                reads: input.reads,
            },
            project: &input.project,
            completion_verifier: input.completion_verifier,
            release,
        },
    )?;
    Ok(ServiceInstallation::install(
        memory::open(authority)?,
        candidate,
    )?)
}

/// Restore in the synchronous post-upgrade participant before deferred work.
/// Only Canic's Candid unit upgrade argument is accepted. The host supplies its compiled
/// release and expected memory authority; all four owners remain fenced even if
/// Canic returns to Active.
/// # Errors
/// Rejects replacement arguments, missing memory, binding changes or invalid state.
/// # Panics
/// Binary corruption traps. The artifact must propagate failures for IC rollback.
pub fn restore(release: &str, authority: &str) -> Result<ManagedInstallation, LifecycleFailure> {
    let empty = candid::encode_one(()).expect("encode Canic unit upgrade argument");
    if ic0::msg_arg_data_size() != empty.len() || ic_cdk::api::msg_arg_data() != empty {
        return Err(LifecycleFailure::UpgradeArguments);
    }
    Ok(ServiceInstallation::open(
        memory::open(authority)?,
        ic_cdk::api::canister_self(),
        release,
    )?)
}

/// Local lifecycle rejection; never provider or operational recovery authority.
#[derive(Debug, Error)]
pub enum LifecycleFailure {
    /// The owning Canic artifact has no consistent authenticated release authority.
    #[error("managed release authority unavailable")]
    ManagedAuthority,
    /// Managed carrier or explicit typed application input was refused.
    #[error(transparent)]
    Arguments(#[from] InitializationArgumentFailure),
    /// Same-release restoration accepts no replacement configuration.
    #[error("upgrade requires Canic's Candid unit argument")]
    UpgradeArguments,
    /// Framework grants differ from the expected declarations/authority or cannot open.
    #[error(transparent)]
    Memory(#[from] MemoryAdoptionFailure),
    /// Shared installation validation/construction/restoration failed.
    #[error(transparent)]
    Installation(#[from] ServiceInstallationError),
}
