//! Synchronous host access; no registry borrow crosses transport awaits.
use super::{Memory, StableGatewayRegistry};

/// Access the same installed registry before dispatch and on completion.
/// Invoke each closure exactly once, synchronously, release the borrow on return,
/// and propagate traps. Never substitute a different installation or clear fences.
pub trait GatewayRegistryAccess {
    /// Exclusive host-granted memory implementation.
    type Memory: Memory;
    /// Perform one synchronous registry operation.
    fn with_gateway_registry<R>(
        &self,
        operation: impl FnOnce(&mut StableGatewayRegistry<Self::Memory>) -> R,
    ) -> R;
}
