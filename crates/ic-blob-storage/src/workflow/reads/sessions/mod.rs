//! Persist bounded occupancy before a read and settle only its original callback.
use super::{ReadAuthority, ReadAuthorityError, capture, recheck};
use crate::{
    model::{
        gateway::registry::GatewayScope,
        service::{
            read::session::{
                ReadChunkTarget, ReadSessionError, ReadSessionIntent, ReadSessionTicket,
            },
            upload::UploadContext,
        },
    },
    ops::service::{
        gateways::{GatewayStoreError, StableGatewayRegistry},
        reads::StableReadSessions,
        uploads::StableUploads,
    },
};
use ic_memory::ic_stable_structures::Memory;
use thiserror::Error;
/// Session admission/completion failure; an invalidated callback can release its
/// exact slot while returning `Authority`, so hosts must reject byte disclosure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ReadSessionWorkflowError {
    /// Durable capacity, identity, restoration or configuration rejection.
    #[error(transparent)]
    Session(#[from] ReadSessionError),
    /// Current tenant/reference/gateway authority refused.
    #[error(transparent)]
    Authority(#[from] ReadAuthorityError),
}
/// Reserve exact bounded occupancy after current authority and chunk-range checks.
/// The host must commit this intent before its single separately qualified call.
/// No call is sent here and dropping a ticket does not release its reservation.
/// # Errors
/// Rejects authority, any owner fence, mismatched configuration, range or capacity.
/// # Panics
/// Stable traps must roll back the complete synchronous IC update.
pub fn begin<G: Memory, U: Memory, S: Memory>(
    registry: &StableGatewayRegistry<G>,
    uploads: &StableUploads<U>,
    sessions: &mut StableReadSessions<S>,
    context: UploadContext,
    scope: GatewayScope,
    chunk: ReadChunkTarget,
) -> Result<ReadSessionTicket, ReadSessionWorkflowError> {
    sessions.bind(uploads.callback_configuration())?;
    let authority = capture(registry, uploads, context, scope, chunk.target)?;
    let bytes = uploads
        .read_chunk_bytes(context, chunk.target, chunk.index)
        .map_err(ReadAuthorityError::from)?
        .ok_or(ReadSessionError::Chunk)?;
    if bytes > u64::from(sessions.limits().max_reply_bytes()) {
        return Err(ReadSessionError::Capacity.into());
    }
    Ok(sessions.reserve(&ReadSessionIntent {
        context,
        scope,
        chunk,
        gateway_generation: authority.gateway_generation,
        tenant_generation: authority.tenant_generation,
    })?)
}
/// Consume exactly the original callback's occupancy and recheck disclosure authority.
/// Original authenticated context and ticket must be retained by the host, never
/// supplied by ingress. Invoke only once the original local call has completed;
/// this does not authenticate its source, inspect its bytes or grant retry authority.
/// Ordinary authority loss still releases that exact returned call's capacity.
/// Any restore fence or configuration mismatch retains capacity. Hosts must treat
/// an error as refusal to disclose bytes, even when occupancy was released.
/// # Errors
/// Rejects wrong/stale callbacks, any fence, binding or lost disclosure authority.
/// # Panics
/// Propagate write traps so session removal and all counters roll back together.
pub fn complete<G: Memory, U: Memory, S: Memory>(
    registry: &StableGatewayRegistry<G>,
    uploads: &StableUploads<U>,
    sessions: &mut StableReadSessions<S>,
    context: UploadContext,
    ticket: &ReadSessionTicket,
) -> Result<(), ReadSessionWorkflowError> {
    sessions.bind(uploads.callback_configuration())?;
    sessions.check(context, ticket)?;
    let current = registry
        .callback_view(uploads.callback_configuration(), ticket.intent.scope)
        .map_err(ReadAuthorityError::from)?;
    if current.fenced {
        return Err(ReadAuthorityError::Registry(GatewayStoreError::Fenced).into());
    }
    uploads
        .check_callback_active()
        .map_err(ReadAuthorityError::from)?;
    let original = ReadAuthority {
        context: ticket.intent.context,
        scope: ticket.intent.scope,
        target: ticket.intent.chunk.target,
        gateway_generation: ticket.intent.gateway_generation,
        tenant_generation: ticket.intent.tenant_generation,
    };
    let authority = recheck(registry, uploads, context, &original);
    // Exact locally completed read occupancy is releasable even if its authority
    // disappeared. There is no paid-effect liability or upload-byte refund here.
    sessions.complete(context, ticket)?;
    Ok(authority?)
}
