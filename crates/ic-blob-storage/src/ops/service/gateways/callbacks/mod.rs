//! Private current-state bridge for synchronous gateway workflows.
use super::{
    GatewayRegistry, GatewayScope, GatewayStoreError, Memory, ServiceConfiguration,
    StableGatewayRegistry,
};

/// Internal read only; never expose or retain this as a callback permit.
pub(crate) struct GatewayCallbackView {
    pub registry: GatewayRegistry,
    pub fenced: bool,
}
impl<M: Memory> StableGatewayRegistry<M> {
    pub(crate) fn callback_view(
        &self,
        expected: &ServiceConfiguration,
        scope: GatewayScope,
    ) -> Result<GatewayCallbackView, GatewayStoreError> {
        if self.config != *expected || scope != Self::scope(&self.config) {
            return Err(GatewayStoreError::Binding);
        }
        Ok(GatewayCallbackView {
            registry: self.registry()?,
            fenced: self.fenced,
        })
    }
}
