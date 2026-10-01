//! Managed registry borrowing; no store borrow survives a provider await.
use ic_blob_storage::ops::{
    caffeine::gateway::GatewayReplyLimits,
    service::gateways::{StableGatewayRegistry, access::GatewayRegistryAccess},
};

pub(crate) struct GatewayHost;
impl GatewayRegistryAccess for GatewayHost {
    type Memory = ic_blob_storage_canic::memory::ManagedMemory;
    fn with_gateway_registry<R>(
        &self,
        operation: impl FnOnce(&mut StableGatewayRegistry<Self::Memory>) -> R,
    ) -> R {
        super::mutate(|owner| operation(&mut owner.stores_mut().gateways))
    }
}
pub(crate) fn limits() -> GatewayReplyLimits {
    GatewayReplyLimits {
        max_bytes: 65_536.try_into().expect("fixed reply bound"),
        decoding_quota: 500_000.try_into().expect("fixed work bound"),
        skipping_quota: 1000.try_into().expect("fixed skipping bound"),
        max_type_entries: 32.try_into().expect("fixed type bound"),
    }
}
