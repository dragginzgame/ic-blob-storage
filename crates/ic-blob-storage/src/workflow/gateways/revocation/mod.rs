//! One synchronous operator revocation through the shared durable owner.
use crate::{
    dto::gateway::{GatewayRevocationFailure, GatewayRevocationRequest, GatewayRevocationResponse},
    model::service::upload::UploadContext,
    ops::service::gateways::{StableGatewayRegistry, revocation},
};
use ic_memory::ic_stable_structures::Memory;

/// Revoke local membership and invalidate pending sync/read observations together.
/// The host supplies actual service/caller and commits synchronously before replying.
/// An absent member still invalidates older observations. Future explicitly
/// authorized additions/syncs are separate decisions; this is not a denylist.
/// No provider call, physical deletion, billing settlement or read-slot release
/// occurs. A missing acknowledgment does not authorize an automatic repeat.
/// # Errors
/// Rejects invalid input, scope/actor, corrupt state or restored mutation fences.
/// # Panics
/// Stable-write traps must propagate for IC transaction rollback.
pub fn revoke<M: Memory>(
    store: &mut StableGatewayRegistry<M>,
    context: UploadContext,
    input: GatewayRevocationRequest,
) -> Result<GatewayRevocationResponse, GatewayRevocationFailure> {
    revocation::revoke(store, context, input)
}
