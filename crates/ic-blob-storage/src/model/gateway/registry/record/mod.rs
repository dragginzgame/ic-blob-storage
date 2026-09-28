//! Bounded v1 registry snapshot, preserving sync identity without granting freshness.
use super::{GatewayRegistry, GatewayScope, GatewaySyncToken};
use crate::model::{
    gateway::membership::GatewayMembership, service::configuration::ServiceConfiguration,
};
use candid::{CandidType, DecoderConfig, Deserialize, Principal, decode_one_with_config};
use ic_memory::ic_stable_structures::{Storable, storable::Bound};
use std::borrow::Cow;

pub(crate) const MAX_MEMBERS: usize = 1024;
const MAX_BYTES: u32 = 65_536;
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
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
            pending: self
                .pending_sequence
                .map(|sequence| GatewaySyncToken { scope, sequence }),
        })
    }
}
impl Storable for GatewayRegistryRecord {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.clone().into_bytes())
    }
    fn into_bytes(self) -> Vec<u8> {
        let bytes = candid::encode_one(self).expect("bounded gateway registry encoding");
        assert!(
            bytes.len() <= MAX_BYTES as usize,
            "gateway record byte bound"
        );
        bytes
    }
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        assert!(
            bytes.len() <= MAX_BYTES as usize,
            "gateway record byte bound"
        );
        let mut config = DecoderConfig::new();
        config
            .set_decoding_quota(500_000)
            .set_skipping_quota(1000)
            .set_max_type_len(32)
            .set_max_header_len(MAX_BYTES as usize)
            .set_full_error_message(false);
        decode_one_with_config(&bytes, &config).expect("valid same-release gateway record")
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_BYTES,
        is_fixed_size: false,
    };
}
