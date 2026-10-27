//! Bounded v1 registry snapshot, preserving sync identity without granting freshness.
use super::{GatewayRegistry, GatewayScope, GatewaySyncToken};
use crate::model::gateway::membership::GatewayMembership;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_platform::principal::ExternalPrincipal;
use serde::Deserialize;
use serde::Serialize;

pub(crate) use ic_blob_storage_contracts::configuration::envelope::MAX_GATEWAY_MEMBERS as MAX_MEMBERS;
const MAX_BYTES: u32 = 65_536;
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct GatewayRegistryRecord {
    version: u8,
    service: ExternalPrincipal,
    operator: ExternalPrincipal,
    cashier: ExternalPrincipal,
    namespace: u128,
    max_entries: u64,
    max_unique: u64,
    principals: Vec<ExternalPrincipal>,
    last_sequence: u64,
    pending_sequence: Option<u64>,
}
impl GatewayRegistryRecord {
    pub(crate) fn new(config: &ServiceConfiguration, registry: &GatewayRegistry) -> Self {
        let scope = registry.scope();
        let limits = registry.gateways().limits();
        let sync = registry.sync_view();
        Self {
            version: 1,
            service: scope.service().into(),
            operator: config.bindings().operator.into(),
            cashier: scope.cashier().into(),
            namespace: scope.namespace().get(),
            max_entries: limits.max_entries.get() as u64,
            max_unique: limits.max_unique.get() as u64,
            principals: registry
                .gateways()
                .principals()
                .iter()
                .copied()
                .map(Into::into)
                .collect(),
            last_sequence: sync.last_sequence,
            pending_sequence: sync.pending_sequence,
        }
    }
    pub(crate) fn matches(&self, config: &ServiceConfiguration) -> bool {
        let b = config.bindings();
        let l = config.billing().gateway_limits();
        self.version == 1
            && self.service == b.service.into()
            && self.operator == b.operator.into()
            && self.cashier == config.billing().cashier().into()
            && self.namespace == b.namespace.get()
            && self.max_entries == l.max_entries.get() as u64
            && self.max_unique == l.max_unique.get() as u64
    }
    pub(crate) fn registry(&self, config: &ServiceConfiguration) -> Option<GatewayRegistry> {
        if !self.matches(config)
            || self.principals.len() > MAX_MEMBERS
            || self.principals.len() > config.billing().gateway_limits().max_unique.get()
        {
            return None;
        }
        if self
            .pending_sequence
            .is_some_and(|n| n == 0 || n != self.last_sequence)
        {
            return None;
        }
        let scope = GatewayScope::new(
            self.service.into(),
            config.bindings().namespace,
            self.cashier.into(),
        )
        .ok()?;
        let gateways = GatewayMembership::from_retained(
            &self
                .principals
                .iter()
                .copied()
                .map(Into::into)
                .collect::<Vec<_>>(),
            config.billing().gateway_limits(),
        )?;
        Some(GatewayRegistry {
            scope,
            gateways,
            last_sequence: self.last_sequence,
            pending: self
                .pending_sequence
                .map(|sequence| GatewaySyncToken { scope, sequence }),
        })
    }
}
crate::model::record_codec::storable!(
    GatewayRegistryRecord, id: b"BLGATE01", bytes: MAX_BYTES,
    layout: ic_storage::adapter::storable::Bound::Bounded {
        max_size: MAX_BYTES, is_fixed_size: false,
    },
);
