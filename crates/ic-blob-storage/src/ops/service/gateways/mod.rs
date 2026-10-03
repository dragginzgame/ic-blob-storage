//! Durable bounded gateway membership and sync correlation, not callback authority.
pub mod access;
pub(crate) mod callbacks;
pub mod reply;
pub mod revocation;
pub mod sync;
use crate::model::{
    gateway::{
        GatewayListError,
        membership::{GatewayAddOutcome, GatewayMembership},
        registry::{
            GatewayRegistry, GatewayScope, GatewaySyncError, GatewaySyncToken, GatewaySyncView,
            record::{GatewayRegistryRecord, MAX_MEMBERS},
        },
    },
    service::{configuration::ServiceConfiguration, upload::UploadContext},
};
use candid::Principal;
use ic_memory::ic_stable_structures::{BTreeMap, Memory};
use thiserror::Error;

/// Passive operator inspection. Membership alone is not authenticated provider
/// authority, a read-session generation or permission to accept callbacks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayRegistryView {
    /// Exact installed service/namespace/Cashier scope.
    pub scope: GatewayScope,
    /// Current distinct members in retained order, possibly empty after removal.
    pub principals: Vec<Principal>,
    /// All allocated sync identities and any outstanding read-only attempt.
    pub sync: GatewaySyncView,
    /// Restoration keeps the whole owner inspection-only.
    pub fenced: bool,
}
/// One host-granted memory and one bounded v1 record own membership and the
/// pending sync together. The installed membership bound must be at most 1024.
/// Updates reuse the transient registry's validation/revocation rules and write
/// one complete record; their cost is bounded by configured membership, not
/// lifetime sync history. No implicit memory IDs, endpoints or lifecycle hooks.
pub struct StableGatewayRegistry<M: Memory> {
    records: BTreeMap<u8, GatewayRegistryRecord, M>,
    config: ServiceConfiguration,
    fenced: bool,
}
impl<M: Memory> StableGatewayRegistry<M> {
    pub(crate) fn set_recovery_fence(&mut self, fenced: bool) {
        self.fenced = fenced;
    }
    pub(crate) const fn inspection_configuration(&self) -> &ServiceConfiguration {
        &self.config
    }
    /// Install empty membership with no pending sync in fresh exclusive memory.
    /// # Errors
    /// Rejects allocated memory or an unsupported member envelope before writing.
    /// # Panics
    /// Stable allocation/encoding failures trap; hosts must propagate IC rollback.
    pub fn install(memory: M, config: ServiceConfiguration) -> Result<Self, GatewayStoreError> {
        validate_envelope(&config)?;
        if memory.size() != 0 {
            return Err(GatewayStoreError::AlreadyAllocated);
        }
        let scope = Self::scope(&config);
        let registry = GatewayRegistry::new(
            scope,
            GatewayMembership::new(config.billing().gateway_limits()),
        );
        let mut records = BTreeMap::new(memory);
        records.insert(0, GatewayRegistryRecord::new(&config, &registry));
        Ok(Self {
            records,
            config,
            fenced: false,
        })
    }
    /// Restore exact membership and pending history synchronously into a fence.
    /// Never clears a pending token, repairs storage or supplies freshness from
    /// the same backup. Host installation/release identity checks remain required.
    /// # Errors
    /// Rejects missing/mismatched/corrupt records or unsupported bounds.
    /// # Panics
    /// Invalid collection/record bytes trap without initializing replacement state.
    pub fn open(memory: M, config: ServiceConfiguration) -> Result<Self, GatewayStoreError> {
        validate_envelope(&config)?;
        if memory.size() == 0 {
            return Err(GatewayStoreError::Missing);
        }
        let store = Self {
            records: BTreeMap::load(memory),
            config,
            fenced: true,
        };
        store.registry()?;
        Ok(store)
    }
    /// Inspect as the configured operator with an explicit complete scope.
    /// # Errors
    /// Rejects wrong service/operator/scope or invalid retained state.
    pub fn inspect(
        &self,
        context: UploadContext,
        scope: GatewayScope,
    ) -> Result<GatewayRegistryView, GatewayStoreError> {
        self.authorize(context, scope)?;
        let registry = self.registry()?;
        Ok(GatewayRegistryView {
            scope,
            principals: registry.gateways().principals().to_vec(),
            sync: registry.sync_view(),
            fenced: self.fenced,
        })
    }
    /// Persist one pending read-only sync before any host effect.
    /// # Errors
    /// Rejects scope/operator/fence, overlapping sync or sequence exhaustion.
    /// # Panics
    /// Stable writes must propagate IC rollback.
    pub fn begin_sync(
        &mut self,
        context: UploadContext,
        scope: GatewayScope,
    ) -> Result<GatewaySyncToken, GatewayStoreError> {
        self.mutate(context, scope, |r| Ok(r.begin_sync()?))
    }
    /// Cancel only the exact outstanding read-only sync; paid effects are separate.
    /// # Errors
    /// Rejects wrong authority/fence or stale token without changing history.
    /// # Panics
    /// Stable writes must propagate IC rollback.
    pub fn cancel_sync(
        &mut self,
        context: UploadContext,
        scope: GatewayScope,
        token: GatewaySyncToken,
    ) -> Result<(), GatewayStoreError> {
        self.mutate(context, scope, |r| Ok(r.cancel_sync(token)?))
    }
    /// Add one member, invalidating an earlier sync even on an already-present add.
    /// # Errors
    /// Rejects wrong authority/fence, invalid principal or capacity before writing.
    /// # Panics
    /// Stable writes must propagate IC rollback.
    pub fn add(
        &mut self,
        context: UploadContext,
        scope: GatewayScope,
        principal: Principal,
    ) -> Result<GatewayAddOutcome, GatewayStoreError> {
        self.mutate(context, scope, |r| Ok(r.add(principal)?))
    }
    /// Revoke a member, including the final member; always invalidate an earlier
    /// pending sync, even when absent. This remains possible at sync exhaustion.
    /// # Errors
    /// Rejects wrong authority/fence or corrupt storage.
    /// # Panics
    /// Stable writes must propagate IC rollback.
    pub fn remove(
        &mut self,
        context: UploadContext,
        scope: GatewayScope,
        principal: Principal,
    ) -> Result<bool, GatewayStoreError> {
        self.mutate(context, scope, |r| Ok(r.remove(principal)))
    }
    fn mutate<R, E: From<GatewayStoreError>>(
        &mut self,
        context: UploadContext,
        scope: GatewayScope,
        change: impl FnOnce(&mut GatewayRegistry) -> Result<R, E>,
    ) -> Result<R, E> {
        self.authorize(context, scope)?;
        if self.fenced {
            return Err(GatewayStoreError::Fenced.into());
        }
        let mut registry = self.registry()?;
        let result = change(&mut registry)?;
        self.records
            .insert(0, GatewayRegistryRecord::new(&self.config, &registry));
        Ok(result)
    }
    fn authorize(
        &self,
        context: UploadContext,
        scope: GatewayScope,
    ) -> Result<(), GatewayStoreError> {
        if context.service != self.config.bindings().service || scope != Self::scope(&self.config) {
            return Err(GatewayStoreError::Binding);
        }
        if context.actor != self.config.bindings().operator {
            return Err(GatewayStoreError::NotOperator);
        }
        Ok(())
    }
    fn registry(&self) -> Result<GatewayRegistry, GatewayStoreError> {
        if self.records.len() != 1 {
            return Err(GatewayStoreError::InvalidRecord);
        }
        let record = self
            .records
            .get(&0)
            .ok_or(GatewayStoreError::InvalidRecord)?;
        if !record.matches(&self.config) {
            return Err(GatewayStoreError::Binding);
        }
        record
            .registry(&self.config)
            .ok_or(GatewayStoreError::InvalidRecord)
    }
    fn scope(config: &ServiceConfiguration) -> GatewayScope {
        GatewayScope::new(
            config.bindings().service,
            config.bindings().namespace,
            config.billing().cashier(),
        )
        .expect("validated service configuration")
    }
}

pub(crate) fn validate_envelope(config: &ServiceConfiguration) -> Result<(), GatewayStoreError> {
    if config.billing().gateway_limits().max_unique.get() > MAX_MEMBERS {
        return Err(GatewayStoreError::UnsupportedEnvelope);
    }
    Ok(())
}
/// Typed rejection; binary or stable-write failures trap separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum GatewayStoreError {
    /// Fresh installation cannot overwrite allocated memory.
    #[error("gateway memory already allocated")]
    AlreadyAllocated,
    /// Restoration cannot initialize absent memory.
    #[error("gateway memory missing")]
    Missing,
    /// Wrong service/operator configuration, provider scope or installed limits.
    #[error("gateway binding mismatch")]
    Binding,
    /// The actual caller is not the configured operator.
    #[error("gateway operator required")]
    NotOperator,
    /// The configured membership cannot fit the bounded record.
    #[error("unsupported gateway membership envelope")]
    UnsupportedEnvelope,
    /// Missing, extra or inconsistent retained state.
    #[error("invalid gateway record")]
    InvalidRecord,
    /// Restored owners remain inspection-only.
    #[error("gateway registry fenced")]
    Fenced,
    /// Invalid membership edit.
    #[error(transparent)]
    List(#[from] GatewayListError),
    /// Invalid local sync transition or complete replacement.
    #[error(transparent)]
    Sync(#[from] GatewaySyncError),
}
#[cfg(test)]
mod tests;
