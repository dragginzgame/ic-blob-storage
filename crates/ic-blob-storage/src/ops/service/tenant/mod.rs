//! Incremental stable tenant enrollment, using the shared model transition.
//!
//! This component does not persist uploads, references or provider obligations.
//! Hosts own memory grants and lifecycle. Reopening is always inspection-only;
//! it cannot establish freshness after an older backup or qualify service recovery.

use crate::model::service::{
    configuration::ServiceConfiguration,
    tenant::{
        TenantEnrollmentView, TenantError, TenantUpdate, next_enrollment,
        record::{TenantEnrollmentRecord, TenantStoreMetadataRecord, TenantStoreRecord},
    },
    upload::UploadContext,
};
use candid::Principal;
use ic_memory::ic_stable_structures::{BTreeMap, Memory};
use thiserror::Error;

/// One host-granted memory holding a bounded lifetime enrollment map.
///
/// The reserved management-principal key owns the store binding; that principal
/// cannot enroll. Records are updated individually in the caller's synchronous
/// IC message transaction. Storage traps must propagate, never be caught and
/// converted into successful updates. Memory must have one live owning handle.
pub struct StableTenantEnrollments<M: Memory> {
    entries: BTreeMap<Principal, TenantStoreRecord, M>,
    config: ServiceConfiguration,
    fenced: bool,
}

impl<M: Memory> StableTenantEnrollments<M> {
    pub(crate) fn set_recovery_fence(&mut self, fenced: bool) {
        self.fenced = fenced;
    }
    /// Install in a fresh host-granted memory. This never initializes over old data.
    /// # Errors
    /// Rejects any allocated memory, even if its schema or records are missing.
    /// # Panics
    /// Traps on stable-memory allocation or encoding failure; the IC rolls back
    /// the message. Native callers do not receive that platform guarantee.
    pub fn install(memory: M, config: ServiceConfiguration) -> Result<Self, TenantStoreError> {
        if memory.size() != 0 {
            return Err(TenantStoreError::AlreadyAllocated);
        }
        let mut entries = BTreeMap::new(memory);
        entries.insert(
            Principal::management_canister(),
            TenantStoreRecord::Metadata(TenantStoreMetadataRecord::new(&config)),
        );
        Ok(Self {
            entries,
            config,
            fenced: false,
        })
    }

    /// Reopen existing state synchronously into an enforced mutation fence.
    ///
    /// Only enrollment-specific configuration is compared: service, operator,
    /// namespace and lifetime tenant limit. Other service stores must independently
    /// validate their own configuration. The host must check installation/release
    /// identity before opening; this component does not permit cross-release
    /// transitions. No migration, reset or unfence exists.
    /// # Errors
    /// Rejects missing memory, mismatched binding, capacity or invalid records.
    /// # Panics
    /// Corrupt stable collection headers or encoded records trap without seeding
    /// replacement data. The host must propagate restoration failure.
    pub fn open(memory: M, config: ServiceConfiguration) -> Result<Self, TenantStoreError> {
        if memory.size() == 0 {
            return Err(TenantStoreError::Missing);
        }
        // `init` can overwrite unrecognized memory; restoration must use `load`.
        let entries = BTreeMap::load(memory);
        let expected = TenantStoreRecord::Metadata(TenantStoreMetadataRecord::new(&config));
        if entries.get(&Principal::management_canister()) != Some(expected) {
            return Err(TenantStoreError::Binding);
        }
        if entries.len() - 1 > config.limits().max_tenants.get() as u64 {
            return Err(TenantStoreError::InvalidRecord);
        }
        let store = Self {
            entries,
            config,
            fenced: true,
        };
        // Bounded by the configured lifetime tenant count. Restore validates every
        // enrollment before exposing even inspection, without copying the map.
        for entry in store.entries.iter() {
            if *entry.key() != Principal::management_canister() {
                enrollment(*entry.key(), entry.value())?;
            }
        }
        Ok(store)
    }

    /// True for every reopened store, regardless of counters in the same backup.
    #[must_use]
    pub const fn is_fenced(&self) -> bool {
        self.fenced
    }

    /// Read one enrollment as its tenant or the explicitly configured operator.
    /// # Errors
    /// Rejects the wrong running service, unrelated caller or invalid record.
    pub fn inspect(
        &self,
        context: UploadContext,
        tenant: Principal,
    ) -> Result<Option<TenantEnrollmentView>, TenantStoreError> {
        self.check_service(context)?;
        if context.actor != tenant && context.actor != self.config.bindings().operator {
            return Err(TenantStoreError::NotObserver);
        }
        self.read(tenant)
    }

    /// Commit an operator's exact enrollment transition before returning success.
    /// # Errors
    /// Rejects wrong service/operator, all reopened stores, invalid tenants,
    /// stale preconditions, exhausted generation or lifetime capacity.
    /// # Panics
    /// Stable write failures trap and must roll back the containing IC message.
    pub fn update(
        &mut self,
        context: UploadContext,
        update: TenantUpdate,
    ) -> Result<TenantEnrollmentView, TenantStoreError> {
        self.check_service(context)?;
        if context.actor != self.config.bindings().operator {
            return Err(TenantStoreError::NotOperator);
        }
        if self.fenced {
            return Err(TenantStoreError::Fenced);
        }
        let current = self.read(update.tenant)?;
        let next = next_enrollment(
            update,
            current,
            self.entries.len() - 1 < self.config.limits().max_tenants.get() as u64,
        )?;
        self.entries.insert(
            update.tenant,
            TenantStoreRecord::Enrollment(TenantEnrollmentRecord::new(next)),
        );
        Ok(next)
    }

    fn check_service(&self, context: UploadContext) -> Result<(), TenantStoreError> {
        if context.service != self.config.bindings().service {
            return Err(TenantStoreError::WrongService);
        }
        Ok(())
    }

    fn read(&self, tenant: Principal) -> Result<Option<TenantEnrollmentView>, TenantStoreError> {
        if tenant == Principal::management_canister() {
            return Ok(None);
        }
        self.entries
            .get(&tenant)
            .map(|record| enrollment(tenant, record))
            .transpose()
    }

    pub(super) fn enrollment_for_owner(
        &self,
        tenant: Principal,
    ) -> Result<Option<TenantEnrollmentView>, TenantStoreError> {
        self.read(tenant)
    }
}

fn enrollment(
    tenant: Principal,
    record: TenantStoreRecord,
) -> Result<TenantEnrollmentView, TenantStoreError> {
    match record {
        TenantStoreRecord::Enrollment(record) if tenant != Principal::anonymous() => {
            record.view().ok_or(TenantStoreError::InvalidRecord)
        }
        _ => Err(TenantStoreError::InvalidRecord),
    }
}

/// Typed enrollment-store rejection. Corrupt binary storage traps separately.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TenantStoreError {
    /// Installation must never overwrite any allocated memory.
    #[error("tenant memory is already allocated")]
    AlreadyAllocated,
    /// Reopening cannot initialize a missing store.
    #[error("tenant memory is missing")]
    Missing,
    /// Retained metadata differs from the host's exact enrollment configuration.
    #[error("tenant store binding mismatch")]
    Binding,
    /// Invalid or over-capacity persisted enrollment.
    #[error("invalid tenant record")]
    InvalidRecord,
    /// Actual running service differs from installed service.
    #[error("wrong tenant service")]
    WrongService,
    /// Mutation requires the configured operator, never controller inference.
    #[error("caller is not the tenant operator")]
    NotOperator,
    /// Inspection requires the tenant or configured operator.
    #[error("caller cannot inspect tenant")]
    NotObserver,
    /// Reopened stores never regain mutation authority from their own records.
    #[error("tenant store is fenced")]
    Fenced,
    /// Shared model transition rejected without modifying storage.
    #[error(transparent)]
    Tenant(#[from] TenantError),
}

#[cfg(test)]
pub(super) mod tests;
