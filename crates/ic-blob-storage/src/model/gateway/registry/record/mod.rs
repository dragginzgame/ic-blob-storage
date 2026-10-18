//! Bounded v1 registry snapshot, preserving sync identity without granting freshness.
use super::{GatewayRegistry, GatewayScope, GatewaySyncToken};
use crate::model::gateway::membership::GatewayMembership;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_platform::Principal;
use ic_platform::candid::contract::CandidType;
use ic_platform::candid::contract::serde::Deserialize;

pub(crate) use ic_blob_storage_contracts::configuration::envelope::MAX_GATEWAY_MEMBERS as MAX_MEMBERS;
const MAX_BYTES: u32 = 65_536;
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(crate) struct GatewayRegistryRecord {
    version: u8,
    service: Principal,
    operator: Principal,
    cashier: Principal,
    namespace: u128,
    max_entries: u64,
    max_unique: u64,
    principals: Vec<Principal>,
    last_sequence: u64,
    pending_sequence: Option<u64>,
    read_generation: super::generation::GatewayReadGeneration,
}
impl GatewayRegistryRecord {
    pub(crate) fn new(config: &ServiceConfiguration, registry: &GatewayRegistry) -> Self {
        let scope = registry.scope();
        let limits = registry.gateways().limits();
        let sync = registry.sync_view();
        Self {
            version: 1,
            service: scope.service(),
            operator: config.bindings().operator,
            cashier: scope.cashier(),
            namespace: scope.namespace().get(),
            max_entries: limits.max_entries.get() as u64,
            max_unique: limits.max_unique.get() as u64,
            principals: registry.gateways().principals().to_vec(),
            last_sequence: sync.last_sequence,
            pending_sequence: sync.pending_sequence,
            read_generation: registry.read_generation,
        }
    }
    pub(crate) fn matches(&self, config: &ServiceConfiguration) -> bool {
        let b = config.bindings();
        let l = config.billing().gateway_limits();
        self.version == 1
            && self.service == b.service
            && self.operator == b.operator
            && self.cashier == config.billing().cashier()
            && self.namespace == b.namespace.get()
            && self.max_entries == l.max_entries.get() as u64
            && self.max_unique == l.max_unique.get() as u64
    }
    pub(crate) fn registry(&self, config: &ServiceConfiguration) -> Option<GatewayRegistry> {
        if !self.matches(config)
            || !self.read_generation.valid()
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
        let scope =
            GatewayScope::new(self.service, config.bindings().namespace, self.cashier).ok()?;
        let gateways =
            GatewayMembership::from_retained(&self.principals, config.billing().gateway_limits())?;
        Some(GatewayRegistry {
            scope,
            gateways,
            last_sequence: self.last_sequence,
            read_generation: self.read_generation,
            pending: self
                .pending_sequence
                .map(|sequence| GatewaySyncToken { scope, sequence }),
        })
    }
}
crate::model::record_codec::storable!(
    GatewayRegistryRecord, bytes: MAX_BYTES, work: 500_000, skipping: 1000,
    types: 32, header: MAX_BYTES,
    layout: ic_storage::adapter::storable::Bound::Bounded {
        max_size: MAX_BYTES, is_fixed_size: false,
    },
);
