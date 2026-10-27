//! Actual local IC chunk call over the private Wire protocol.
use blob_test_protocol::chunk::{ChunkReplyError, decode_reply};
use blob_test_protocol::storage::Failure;
use ic_blob_storage::ops::service::reads::transport::{
    ReadChunkRequest, ReadChunkResponse, ReadChunkTransport,
};
use ic_platform::cdk::call::Call;

pub(crate) struct LocalChunk;
impl ReadChunkTransport for LocalChunk {
    type Error = Failure;
    async fn read_chunk(&self, request: ReadChunkRequest) -> Result<ReadChunkResponse, Failure> {
        let source = request.chunk.target.gateway;
        let root = request.chunk.target.root;
        let index = request.chunk.index;
        let response = Call::bounded_wait(source, "fixture_chunk")
            .take_raw_args(
                blob_test_protocol::fixture_wire::encode_args(
                    "fixture_chunk",
                    (root.as_bytes().to_vec(), index),
                )
                .expect("fixture chunk arguments"),
            )
            .await
            .map_err(|_| Failure::Transport)?;
        // Bounds application decoding, not the platform/CDK's initial buffer.
        let encoded = response.as_ref();
        let bytes =
            decode_reply(encoded, request.max_reply_bytes.get() as usize).map_err(|error| {
                match error {
                    ChunkReplyError::TooLarge => Failure::Capacity,
                    ChunkReplyError::Invalid => Failure::Invalid,
                }
            })?;
        // The actual call target authenticates the peer; correlation comes from
        // this captured call, never from a peer's untrusted payload claims.
        Ok(ReadChunkResponse {
            source,
            root,
            index,
            bytes,
        })
    }
}
