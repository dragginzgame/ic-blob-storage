//! Synchronous access to the same exclusive owners around one host read await.
use super::StableReadSessions;
use crate::ops::service::{gateways::StableGatewayRegistry, uploads::StableUploads};
use ic_memory::ic_stable_structures::Memory;
/// Host bridge; invoke each closure once, synchronously, without substituting
/// owners or catching traps. Release all borrows before returning. Actual endpoint
/// authentication, installation identity and update execution remain host duties.
pub trait ReadSessionAccess {
    /// Host-granted memory implementation shared by these owners.
    type Memory: Memory;
    /// Borrow the authoritative owners for one synchronous operation.
    fn with_read_sessions<R>(
        &self,
        operation: impl FnOnce(
            &StableGatewayRegistry<Self::Memory>,
            &StableUploads<Self::Memory>,
            &mut StableReadSessions<Self::Memory>,
        ) -> R,
    ) -> R;
}
