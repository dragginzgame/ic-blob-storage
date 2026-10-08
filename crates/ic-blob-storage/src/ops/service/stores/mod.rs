//! Synchronous assembly of durable owners with host-granted exclusive memory.
pub mod grants;
use super::funding;
use super::funding::FundingJournalError;
use super::funding::FundingMemories;
use super::funding::StableFundingJournal;
use super::gateways;
use super::gateways::GatewayStoreError;
use super::gateways::StableGatewayRegistry;
use super::reads::ReadSessionMemories;
use super::reads::StableReadSessions;
use super::uploads;
use super::uploads::StableUploads;
use super::uploads::UploadMemories;
use super::uploads::UploadStoreError;
use crate::model::billing::allocation::FundingAllocation;
use crate::model::service::read::session::ReadSessionError;
use crate::model::service::read::session::ReadSessionLimits;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_memory::ic_stable_structures::Memory;
use thiserror::Error;

/// Validated limits for all maintained stable owners; no deployment or effect authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceStoreConfiguration {
    service: ServiceConfiguration,
    funding: FundingAllocation,
    reads: ReadSessionLimits,
}
impl ServiceStoreConfiguration {
    /// Check every store's current codec/resource envelope without touching memory.
    /// # Errors
    /// Rejects unsupported upload, gateway, funding or read-session limits.
    pub fn new(
        service: ServiceConfiguration,
        funding: FundingAllocation,
        reads: ReadSessionLimits,
    ) -> Result<Self, ServiceStoreError> {
        uploads::validate_envelope(&service)?;
        funding::validate_envelope(&service, funding)?;
        gateways::validate_envelope(&service)?;
        reads.validate()?;
        Ok(Self {
            service,
            funding,
            reads,
        })
    }
    /// Shared service bindings, tenant policy and upload/billing limits.
    #[must_use]
    pub const fn service(self) -> ServiceConfiguration {
        self.service
    }
    /// Separate local funding allocation, not provider balance or spendability.
    #[must_use]
    pub const fn funding(self) -> FundingAllocation {
        self.funding
    }
    /// Bounded concurrent read occupancy.
    #[must_use]
    pub const fn reads(self) -> ReadSessionLimits {
        self.reads
    }
}

/// Sixteen distinct exclusive memories supplied by the host's `ic-memory` runtime.
/// The library never chooses memory IDs or registers grants implicitly.
pub struct ServiceMemories<M: Memory> {
    /// Ten upload, reference and accounting memories.
    pub uploads: UploadMemories<M>,
    /// Two funding accounting/history memories.
    pub funding: FundingMemories<M>,
    /// One gateway registry memory.
    pub gateways: M,
    /// Three read occupancy memories.
    pub reads: ReadSessionMemories<M>,
}
impl<M: Memory> ServiceMemories<M> {
    fn all(&self) -> [&M; 16] {
        [
            &self.uploads.tenants,
            &self.uploads.roots,
            &self.uploads.objects,
            &self.uploads.permissions,
            &self.uploads.usage,
            &self.uploads.manifests,
            &self.uploads.confirmed,
            &self.uploads.references,
            &self.uploads.receipts,
            &self.uploads.root_requests,
            &self.funding.accounting,
            &self.funding.intents,
            &self.gateways,
            &self.reads.journal,
            &self.reads.sessions,
            &self.reads.tenants,
        ]
    }
}

/// Complete owner assembly; hosts publish it only after synchronous construction succeeds.
/// Existing shared handlers borrow these owners; assembly adds no parallel workflows.
pub struct ServiceStores<M: Memory> {
    /// Tenant admission, manifests, references and continuing obligations.
    pub uploads: StableUploads<M>,
    /// Exact retained funding attempts and allocation accounting.
    pub funding: StableFundingJournal<M>,
    /// Gateway membership and pending sync identity.
    pub gateways: StableGatewayRegistry<M>,
    /// Concurrent read intents and reserved buffers.
    pub reads: StableReadSessions<M>,
}
impl<M: Memory> ServiceStores<M> {
    pub(crate) fn set_recovery_fence(&mut self, fenced: bool) {
        self.uploads.set_recovery_fence(fenced);
        self.funding.set_recovery_fence(fenced);
        self.gateways.set_recovery_fence(fenced);
        self.reads.set_recovery_fence(fenced);
    }
    /// Install all owners in one synchronous IC message under validated limits.
    /// The host must grant distinct exclusive memories and propagate traps for IC
    /// rollback. This function provides no native-memory transaction guarantee.
    /// # Errors
    /// Rejects any allocated memory before writing to any store.
    /// # Panics
    /// Aliased grants or allocation/write failures trap; never catch and continue.
    pub fn install(
        memory: ServiceMemories<M>,
        config: ServiceStoreConfiguration,
    ) -> Result<Self, ServiceStoreError> {
        if memory.all().iter().any(|m| m.size() != 0) {
            return Err(ServiceStoreError::AlreadyAllocated);
        }
        // After whole-set preflight, an allocation rejection indicates broken
        // exclusivity or a platform failure. Trap so IC rolls back earlier writes.
        let uploads = StableUploads::install(memory.uploads, config.service)
            .expect("validated fresh exclusive upload memories");
        let funding = StableFundingJournal::install(memory.funding, config.service, config.funding)
            .expect("validated fresh exclusive funding memories");
        let gateways = StableGatewayRegistry::install(memory.gateways, config.service)
            .expect("validated fresh exclusive gateway memory");
        let reads = StableReadSessions::install(memory.reads, config.service, config.reads)
            .expect("validated fresh exclusive read memories");
        Ok(Self {
            uploads,
            funding,
            gateways,
            reads,
        })
    }
    /// Load every owner synchronously without initialization, repair or unfencing.
    /// Hosts must reject a failed restore and separately verify release/installation
    /// identity. No timers, endpoints, callbacks or deferred work are registered.
    /// # Errors
    /// Rejects missing memories, changed configuration or inconsistent retained state.
    /// # Panics
    /// Corrupt binary records or collection headers trap.
    pub fn open(
        memory: ServiceMemories<M>,
        config: ServiceStoreConfiguration,
    ) -> Result<Self, ServiceStoreError> {
        if memory.all().iter().any(|m| m.size() == 0) {
            return Err(ServiceStoreError::Missing);
        }
        let uploads = StableUploads::open(memory.uploads, config.service)?;
        let funding = StableFundingJournal::open(memory.funding, config.service, config.funding)?;
        let gateways = StableGatewayRegistry::open(memory.gateways, config.service)?;
        let reads = StableReadSessions::open(memory.reads, config.service, config.reads)?;
        Ok(Self {
            uploads,
            funding,
            gateways,
            reads,
        })
    }
}

/// Assembly/configuration failure; binary corruption and stable writes trap separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ServiceStoreError {
    /// Fresh install never replaces any allocated memory in the grant set.
    #[error("service memory already allocated")]
    AlreadyAllocated,
    /// Restore never creates any missing member of the grant set.
    #[error("service memory missing")]
    Missing,
    /// Upload codec, binding or retained obligation validation failed.
    #[error(transparent)]
    Uploads(#[from] UploadStoreError),
    /// Funding envelope, binding or accounting validation failed.
    #[error(transparent)]
    Funding(#[from] FundingJournalError),
    /// Gateway envelope, binding or retained sync validation failed.
    #[error(transparent)]
    Gateways(#[from] GatewayStoreError),
    /// Read limits, binding or retained occupancy validation failed.
    #[error(transparent)]
    Reads(#[from] ReadSessionError),
}

#[cfg(test)]
mod tests;
