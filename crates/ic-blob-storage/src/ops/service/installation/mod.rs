//! One immutable installation and its four durable owners, within host-granted memory.
use super::configuration::ServiceConfigurationAdapterError;
use super::stores::ServiceMemories;
use super::stores::ServiceStoreConfiguration;
use super::stores::ServiceStoreError;
use super::stores::ServiceStores;
use crate::model::service::installation::InstallationBindingError;
use crate::model::service::installation::record::ConfigurationRecord;
use candid::Principal;
use ic_blob_storage_contracts::configuration::ServiceInstallationCandidate;
use ic_blob_storage_contracts::download::scope::CaffeineDownloadScope;
use ic_blob_storage_contracts::download::scope::DownloadScopeError;
use ic_blob_storage_contracts::dto::configuration::ServiceConfigurationInput;
use ic_blob_storage_contracts::upload::completion::CompletionAuthority;
use ic_blob_storage_contracts::upload::completion::InvalidCompletionAuthority;
use ic_blob_storage_contracts::upload::issuer::InvalidUploadIssuerAuthority;
use ic_blob_storage_contracts::upload::issuer::UploadIssuerAuthority;
use ic_memory::ic_stable_structures::{BTreeMap, Memory, Storable};
use ic_memory::{MemoryRequest, SchemaMetadata, StaticMemoryDeclarationError};
use thiserror::Error;

mod configuration;

/// Stable allocation key for the host-granted immutable installation record.
/// It identifies the slot, not the frozen record layout or release. Preserve the
/// slot when refusing an incompatible record; never replace its retained state.
pub const INSTALLATION_MEMORY_KEY: &str = "blob.configuration.v1";

/// Build all seventeen installation memory requests under the host's authority.
/// Configuration and store keys share one inventory; no registration, physical
/// ID selection, bootstrap, allocation, lifecycle hook or stable write occurs.
/// # Errors
/// Rejects an invalid authority using the memory runtime's declaration error.
pub fn requests(authority: &str) -> Result<Vec<MemoryRequest>, StaticMemoryDeclarationError> {
    let mut requests = vec![MemoryRequest::new(
        authority,
        INSTALLATION_MEMORY_KEY,
        SchemaMetadata::default(),
    )?];
    requests.extend(super::stores::grants::requests(authority)?);
    Ok(requests)
}

/// Completely validated installation, produced before a host grants or opens memory.
/// It contains no storage handle or activation authority.
pub struct ValidatedServiceInstallation {
    configuration: ConfigurationRecord,
    limits: ServiceStoreConfiguration,
    download_scope: CaffeineDownloadScope,
    completion: CompletionAuthority,
    issuer: UploadIssuerAuthority,
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
        let input = ic_blob_storage_contracts::configuration::ValidatedInstallationInput::new(
            actual_service,
            candidate,
        )?;
        let limits = super::configuration::from_validated(input.configuration())?;
        let completion = input.completion();
        let issuer = input.issuer();
        let download_scope = input.into_download_scope();
        let configuration = configuration::record(&candidate);
        if configuration.to_bytes().len() > 16_384 {
            return Err(ServiceInstallationError::RecordBound);
        }
        Ok(Self {
            configuration,
            limits,
            download_scope,
            completion,
            issuer,
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
    issuer: UploadIssuerAuthority,
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
            issuer: candidate.issuer,
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
                trusted_uploader: record.trusted_uploader,
                release,
                platform_installation_version: record.platform_installation_version,
            },
        )?;
        let stores = ServiceStores::open(memories.stores, candidate.limits)?;
        Ok(Self {
            configuration: record,
            download_scope: candidate.download_scope,
            completion: candidate.completion,
            issuer: candidate.issuer,
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
    pub fn configuration_view(
        &self,
    ) -> ic_blob_storage_contracts::dto::configuration::HostConfigurationView {
        ic_blob_storage_contracts::dto::configuration::HostConfigurationView {
            configuration: self.configuration(),
            project: self.download_scope.project().to_owned(),
            completion_verifier: self.completion.verifier(),
            trusted_uploader: self.issuer.uploader(),
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
    /// Immutable trusted-uploader authority; does not qualify provider behavior.
    #[must_use]
    pub const fn issuer_authority(&self) -> UploadIssuerAuthority {
        self.issuer
    }

    /// Derive current restricted-contract facts from the installed owner and limits.
    /// No facts are supplied by ingress; local bindings do not prove provisioning.
    #[must_use]
    pub fn certificate_evidence(
        &self,
        permission: ic_blob_storage_contracts::upload::binding::UploadPermission,
        now: u64,
        durable_commit: bool,
    ) -> crate::policy::upload::exposure::UploadExposureHostEvidence {
        use crate::policy::upload::exposure::UploadExposureHostEvidence;
        // Admission and manifest preparation already enforce this installation's
        // object, tenant, global and lifetime quotas before exposure is possible.
        let object = permission.request.object.first.object();
        UploadExposureHostEvidence {
            permission,
            observed_at_ns: now,
            namespace_binding: object.service() == self.download_scope.owner()
                && object.identity().namespace == self.download_scope.namespace(),
            trusted_uploader: self.issuer.permits(permission),
            current_owner: !self.stores.uploads.is_fenced(),
            durable_commit,
        }
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
    /// Immutable platform anchor captured by the installing host. It becomes
    /// useful only with independently obtained complete IC management history.
    #[must_use]
    pub const fn platform_installation_version(&self) -> u64 {
        self.configuration.platform_installation_version
    }
    /// Enter an inspection-only fence for every owner. This never changes
    /// reservations, pending work, history, accounting or external obligations.
    pub fn fence(&mut self) {
        self.stores.set_recovery_fence(true);
    }
    /// Resume the exact current installation after an independently obtained IC
    /// continuity proof, in the same replicated callback and without an await.
    /// Uncertain effects and occupied sessions remain held; this authorizes no retry.
    /// # Errors
    /// Rejects proofs from another service, anchor or platform execution version.
    pub fn resume_current_instance(
        &mut self,
        proof: super::recovery::CurrentInstanceProof,
    ) -> Result<(), super::recovery::CurrentInstanceRecoveryError> {
        proof.check(
            self.configuration.service,
            self.configuration.platform_installation_version,
        )?;
        self.stores.set_recovery_fence(false);
        Ok(())
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
    /// Retained service/release/layout binding differs from the running host.
    #[error(transparent)]
    Binding(#[from] InstallationBindingError),
    /// Shared candidate identity/resource validation failed.
    #[error(transparent)]
    Configuration(#[from] ServiceConfigurationAdapterError),
    /// Explicit provider serving identity is invalid.
    #[error(transparent)]
    Project(#[from] DownloadScopeError),
    /// Explicit verifier identity is invalid.
    #[error(transparent)]
    Verifier(#[from] InvalidCompletionAuthority),
    /// Explicit trusted certificate uploader is invalid.
    #[error(transparent)]
    Issuer(#[from] InvalidUploadIssuerAuthority),
    /// A shared store could not be installed or restored under the immutable limits.
    #[error(transparent)]
    Stores(#[from] ServiceStoreError),
}

impl From<ic_blob_storage_contracts::configuration::InstallationInputError>
    for ServiceInstallationError
{
    fn from(error: ic_blob_storage_contracts::configuration::InstallationInputError) -> Self {
        use ic_blob_storage_contracts::configuration::InstallationInputError;
        match error {
            InstallationInputError::Configuration(error) => Self::Configuration(error.into()),
            InstallationInputError::Download(error) => Self::Project(error),
            InstallationInputError::Completion(error) => Self::Verifier(error),
            InstallationInputError::Issuer(error) => Self::Issuer(error),
            InstallationInputError::ReleaseIdentity => Self::ReleaseIdentity,
        }
    }
}

#[cfg(test)]
mod tests;
