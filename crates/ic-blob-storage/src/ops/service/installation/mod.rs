//! One immutable installation and its four durable owners, within host-granted memory.
mod configuration;
use super::{
    configuration::{ConfigurationInputError, validate_candidate},
    stores::{ServiceMemories, ServiceStoreConfiguration, ServiceStoreError, ServiceStores},
};
use crate::{
    dto::configuration::ServiceConfigurationInput,
    model::service::{
        installation::{InstallationBindingError, record::ConfigurationRecord},
        read::download::{CaffeineDownloadScope, DownloadScopeError},
        upload::completion::{CompletionAuthority, InvalidCompletionAuthority},
    },
};
use candid::Principal;
use ic_memory::ic_stable_structures::{BTreeMap, Memory, Storable};
use thiserror::Error;

/// Stable key for the host-granted immutable installation record.
pub const INSTALLATION_MEMORY_KEY: &str = "blob.configuration.v1";

/// Explicit installation data plus the host's compiled release identity.
/// No defaults, deployment, allocation or provider qualification are implied.
#[derive(Clone, Copy, Debug)]
pub struct ServiceInstallationCandidate<'a> {
    /// Shared identity, resource and provider-economic configuration.
    pub configuration: ServiceConfigurationInput,
    /// Explicit provisioned project mapping; not inferred from payer/namespace.
    pub project: &'a str,
    /// Trusted whole-content verifier, independent of operator/controller roles.
    pub completion_verifier: Principal,
    /// Host-supplied frozen package release, never an ingress override on restore.
    pub release: &'a str,
}

/// Completely validated installation, produced before a host grants or opens memory.
/// It contains no storage handle or activation authority.
pub struct ValidatedServiceInstallation {
    configuration: ConfigurationRecord,
    limits: ServiceStoreConfiguration,
    download_scope: CaffeineDownloadScope,
    completion: CompletionAuthority,
}
impl ValidatedServiceInstallation {
    /// Validate the complete candidate without memory, registration or provider effects.
    /// The host authenticates installation and supplies its actual platform identity.
    /// # Errors
    /// Rejects invalid limits, bindings, project, verifier or bounded release identity.
    pub fn new(
        actual_service: Principal,
        candidate: ServiceInstallationCandidate<'_>,
    ) -> Result<Self, ServiceInstallationError> {
        let limits = validate_candidate(actual_service, candidate.configuration)?;
        if candidate.release.is_empty()
            || candidate.release.len() > 128
            || candidate.release.trim() != candidate.release
            || candidate.release.chars().any(char::is_control)
        {
            return Err(ServiceInstallationError::ReleaseIdentity);
        }
        let namespace = limits.service().bindings().namespace;
        let download_scope =
            CaffeineDownloadScope::new(actual_service, namespace, candidate.project)?;
        let completion =
            CompletionAuthority::new(actual_service, namespace, candidate.completion_verifier)?;
        let configuration = configuration::record(&candidate);
        if configuration.to_bytes().len() > 16_384 {
            return Err(ServiceInstallationError::RecordBound);
        }
        Ok(Self {
            configuration,
            limits,
            download_scope,
            completion,
        })
    }
}

/// Seventeen distinct exclusive host grants; the library selects no physical IDs.
pub struct ServiceInstallationMemories<M: Memory> {
    /// Immutable current installation record, separate from all service stores.
    pub configuration: M,
    /// Sixteen memories for the four service owners.
    pub stores: ServiceMemories<M>,
}

/// One authoritative installation and its durable service owners.
/// The embedding host retains its memory runtime, supplies caller/time authority,
/// publishes only after synchronous construction and propagates traps for IC rollback.
/// Linking registers no endpoints, grants, bootstrap or lifecycle hooks.
pub struct ServiceInstallation<M: Memory> {
    configuration: ConfigurationRecord,
    download_scope: CaffeineDownloadScope,
    completion: CompletionAuthority,
    stores: ServiceStores<M>,
}
impl<M: Memory> ServiceInstallation<M> {
    /// Install a fully validated candidate only in fresh exclusive memories.
    /// Every grant is preflighted before any service/configuration write. Native
    /// memory provides no whole-operation transaction; IC hosts must not catch traps.
    /// # Errors
    /// Rejects allocated configuration or service memory without replacing any state.
    /// # Panics
    /// Corrupt/aliased grants, allocation or stable-write failures trap.
    pub fn install(
        memories: ServiceInstallationMemories<M>,
        candidate: ValidatedServiceInstallation,
    ) -> Result<Self, ServiceInstallationError> {
        if memories.configuration.size() != 0 {
            return Err(ServiceInstallationError::AlreadyAllocated);
        }
        let stores = ServiceStores::install(memories.stores, candidate.limits)?;
        assert_eq!(
            memories.configuration.size(),
            0,
            "configuration grant aliases service memory"
        );
        let mut records = BTreeMap::new(memories.configuration);
        records.insert(0u8, candidate.configuration.clone());
        Ok(Self {
            configuration: candidate.configuration,
            download_scope: candidate.download_scope,
            completion: candidate.completion,
            stores,
        })
    }

    /// Load the current installation synchronously, checking actual service and
    /// the host's compiled release before assembling any owner. No replacement
    /// configuration, repair, initialization, effect or unfencing is permitted.
    /// # Errors
    /// Rejects missing/invalid installation, changed bindings or inconsistent stores.
    /// # Panics
    /// Corrupt binary state or collection headers trap; IC hosts must propagate them.
    pub fn open(
        memories: ServiceInstallationMemories<M>,
        actual_service: Principal,
        release: &str,
    ) -> Result<Self, ServiceInstallationError> {
        if memories.configuration.size() == 0 {
            return Err(ServiceInstallationError::Missing);
        }
        let records: BTreeMap<u8, ConfigurationRecord, M> = BTreeMap::load(memories.configuration);
        if records.len() != 1 {
            return Err(ServiceInstallationError::RecordCount);
        }
        let record = records.get(&0).ok_or(ServiceInstallationError::Missing)?;
        record.check_binding(actual_service, release)?;
        let candidate = ValidatedServiceInstallation::new(
            actual_service,
            ServiceInstallationCandidate {
                configuration: configuration::input(&record),
                project: &record.project,
                completion_verifier: record.completion_verifier,
                release,
            },
        )?;
        let stores = ServiceStores::open(memories.stores, candidate.limits)?;
        Ok(Self {
            configuration: record,
            download_scope: candidate.download_scope,
            completion: candidate.completion,
            stores,
        })
    }

    /// Exact installed service/resource configuration; endpoint authentication is host-owned.
    #[must_use]
    pub fn configuration(&self) -> ServiceConfigurationInput {
        configuration::input(&self.configuration)
    }
    /// Convert retained installation state for an already-authorized observer.
    /// Endpoint consumers should use the shared installation inspection workflow.
    #[must_use]
    pub fn configuration_view(&self) -> crate::dto::configuration::HostConfigurationView {
        crate::dto::configuration::HostConfigurationView {
            configuration: self.configuration(),
            project: self.download_scope.project().to_owned(),
            completion_verifier: self.completion.verifier(),
            release: self.configuration.release.clone(),
            fenced: self.stores.uploads.is_fenced(),
        }
    }
    /// Immutable provider project mapping, not evidence of namespace provisioning.
    #[must_use]
    pub fn download_scope(&self) -> &CaffeineDownloadScope {
        &self.download_scope
    }
    /// Immutable verifier trust; this grants no operator/controller delegation.
    #[must_use]
    pub const fn completion_authority(&self) -> CompletionAuthority {
        self.completion
    }
    /// Frozen installed package release; not an artifact hash or freshness authority.
    #[must_use]
    pub fn release(&self) -> &str {
        &self.configuration.release
    }
    /// Borrow all owners for shared read handlers; no implicit endpoint is exported.
    #[must_use]
    pub const fn stores(&self) -> &ServiceStores<M> {
        &self.stores
    }
    /// Borrow all owners for synchronous shared transitions, preserving each owner's fence.
    pub fn stores_mut(&mut self) -> &mut ServiceStores<M> {
        &mut self.stores
    }
}

/// Installation/restore rejection. Platform binary corruption and stable writes trap separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ServiceInstallationError {
    /// Fresh install must not overwrite the immutable configuration grant.
    #[error("installation memory already allocated")]
    AlreadyAllocated,
    /// Restore must not create a missing installation record or memory.
    #[error("installation memory or record missing")]
    Missing,
    /// The current installation has exactly one immutable configuration row.
    #[error("invalid installation record count")]
    RecordCount,
    /// Compiled release identity is empty, unbounded or contains invalid text.
    #[error("invalid installation release identity")]
    ReleaseIdentity,
    /// Encoded immutable installation exceeds its maintained envelope.
    #[error("installation record exceeds byte bound")]
    RecordBound,
    /// Retained service/release/schema binding differs from the running host.
    #[error(transparent)]
    Binding(#[from] InstallationBindingError),
    /// Shared candidate identity/resource validation failed.
    #[error(transparent)]
    Configuration(#[from] ConfigurationInputError),
    /// Explicit provider serving identity is invalid.
    #[error(transparent)]
    Project(#[from] DownloadScopeError),
    /// Explicit verifier identity is invalid.
    #[error(transparent)]
    Verifier(#[from] InvalidCompletionAuthority),
    /// A shared store could not be installed or restored under the immutable limits.
    #[error(transparent)]
    Stores(#[from] ServiceStoreError),
}

#[cfg(test)]
mod tests;
