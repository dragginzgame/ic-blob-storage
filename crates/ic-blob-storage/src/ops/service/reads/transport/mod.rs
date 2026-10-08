//! Normalized host read transport, deliberately independent of provider wire formats.
use crate::model::service::read::session::ReadChunkTarget;
use candid::Principal;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use std::num::NonZeroU32;
/// Original exact target and trusted reply budget, supplied by the shared handler.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadChunkRequest {
    /// Root/reference/object, selected source and complete chunk position.
    pub chunk: ReadChunkTarget,
    /// Bound encoded and decoded buffers before application copies/decoding.
    /// Platform pre-buffering and total transient memory require separate bounds.
    pub max_reply_bytes: NonZeroU32,
}
/// A normalized reply with independently authenticated transport bindings.
/// These fields are host assertions, not proof objects or provider payload claims.
#[derive(Debug)]
pub struct ReadChunkResponse {
    /// Actual authenticated peer, not a principal copied from reply data.
    pub source: Principal,
    /// Root associated with the original transport request.
    pub root: ProviderRootHash,
    /// Chunk associated with that request.
    pub index: u64,
    /// Decoded bytes, bounded before allocation by the host transport.
    pub bytes: Vec<u8>,
}
/// One separately qualified read; no library-selected URL, wire contract or retry.
/// Implementations must authenticate the actual peer and bind the original request,
/// bound response buffering/decoding, and return only after the local call settles.
/// A transport must not silently retry, detach calls or treat an uncertain paid
/// effect as this read-only operation. Provider billing/serving qualification is separate.
pub trait ReadChunkTransport {
    /// Host transport/decode rejection, distinct from content verification.
    type Error;
    /// Send exactly once after durable admission; retain the original target.
    fn read_chunk(
        &self,
        request: ReadChunkRequest,
    ) -> impl Future<Output = Result<ReadChunkResponse, Self::Error>>;
}
