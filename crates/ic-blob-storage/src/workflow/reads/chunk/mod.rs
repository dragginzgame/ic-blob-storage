//! One durable session, one host read and one authority-checked verified chunk.
use super::sessions::{ReadSessionWorkflowError, begin, complete};
use crate::model::gateway::registry::GatewayScope;
use crate::model::service::read::session::ReadChunkTarget;
use crate::ops::service::reads::access::ReadSessionAccess;
use crate::ops::service::reads::transport::ReadChunkRequest;
use crate::ops::service::reads::transport::ReadChunkTransport;
use crate::ops::service::uploads::read::verification::ReadVerificationError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
/// Verified bytes for exactly one admitted chunk; not whole-file completion or
/// provider durability/billing proof. The root/reference remain in the caller's request.
#[derive(Debug, Eq, PartialEq)]
pub struct VerifiedReadChunk {
    /// Original zero-based chunk position.
    pub index: u64,
    /// Exact offset in the declared content.
    pub offset: u64,
    /// Exact length and leaf-hash-checked bytes; the transport buffer is moved here.
    pub bytes: Vec<u8>,
}
/// A read refused disclosure. Ordinary callback errors consume only that exact
/// returned call's slot; traps and restore fences preserve its reservation.
#[derive(Debug, Eq, PartialEq)]
pub enum ReadChunkError<E> {
    /// Admission or exact callback authority/occupancy failed.
    Session(ReadSessionWorkflowError),
    /// The single host transport or bounded decoding failed.
    Transport(E),
    /// Authenticated peer or request correlation differs from the original.
    Binding,
    /// Host returned more decoded bytes than its reserved budget.
    ReplyTooLarge,
    /// Immutable manifest or returned content did not verify.
    Verification(ReadVerificationError),
}
/// Admit on first poll, call once without any owner borrow, then settle and verify
/// synchronously before returning bytes. The original context and request remain
/// captured across the await. Wrong source, bad bytes, failed transport and lost
/// authority never disclose bytes; the exact settled call's reservation is released.
/// Session finalization and verification share one callback: a trap rolls back its
/// writes. Dropping a pending future does not clear the durable reservation.
///
/// Hosts must authenticate context/source, enforce actual update execution, propagate
/// traps and qualify the transport's provider protocol and costs. This function
/// installs no endpoint, runtime, retry or background task. It is not provider qualification.
/// # Errors
/// Rejects admission, transport/correlation, callback authority or content verification.
/// # Panics
/// Stable traps propagate for whole-message IC rollback.
pub async fn read_chunk<H: ReadSessionAccess, T: ReadChunkTransport>(
    host: &H,
    transport: &T,
    context: UploadContext,
    scope: GatewayScope,
    chunk: ReadChunkTarget,
) -> Result<VerifiedReadChunk, ReadChunkError<T::Error>> {
    let (ticket, max_reply_bytes) = host
        .with_read_sessions(|registry, uploads, sessions| {
            let ticket = begin(registry, uploads, sessions, context, scope, chunk)?;
            Ok((
                ticket,
                sessions
                    .limits()
                    .max_reply_bytes()
                    .try_into()
                    .expect("validated reply budget"),
            ))
        })
        .map_err(ReadChunkError::Session)?;
    let request = ReadChunkRequest {
        chunk: ticket.chunk(),
        max_reply_bytes,
    };
    let response = transport.read_chunk(request).await;
    host.with_read_sessions(|registry, uploads, sessions| {
        complete(registry, uploads, sessions, context, &ticket).map_err(ReadChunkError::Session)?;
        let response = response.map_err(ReadChunkError::Transport)?;
        if response.source != chunk.target.gateway
            || response.root != chunk.target.root
            || response.index != chunk.index
        {
            return Err(ReadChunkError::Binding);
        }
        if response.bytes.len() > max_reply_bytes.get() as usize {
            return Err(ReadChunkError::ReplyTooLarge);
        }
        let range = uploads
            .verify_read_chunk(context, chunk.target, chunk.index, &response.bytes)
            .map_err(ReadChunkError::Verification)?;
        Ok(VerifiedReadChunk {
            index: range.index,
            offset: range.offset,
            bytes: response.bytes,
        })
    })
}
