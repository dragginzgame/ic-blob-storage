//! Actual local IC chunk call; this wire is a labelled provider substitute.
use blob_test_protocol::storage::Failure;
use candid::{de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::ops::service::reads::transport::{
    ReadChunkRequest, ReadChunkResponse, ReadChunkTransport,
};
use ic_cdk::call::Call;

pub(crate) struct LocalChunk;
impl ReadChunkTransport for LocalChunk {
    type Error = Failure;
    async fn read_chunk(&self, request: ReadChunkRequest) -> Result<ReadChunkResponse, Failure> {
        let source = request.chunk.target.gateway;
        let root = request.chunk.target.root;
        let index = request.chunk.index;
        let response = Call::bounded_wait(source, "fixture_chunk")
            .with_args(&(root.as_bytes().to_vec(), index))
            .await
            .map_err(|_| Failure::Transport)?;
        // Bounds application decoding, not the platform/CDK's initial buffer.
        let encoded = response.as_ref();
        if encoded.len() > request.max_reply_bytes.get() as usize {
            return Err(Failure::Capacity);
        }
        let mut config = DecoderConfig::new();
        config
            .set_decoding_quota(32_768)
            .set_skipping_quota(64)
            .set_max_type_len(8)
            .set_full_error_message(false);
        let bytes = decode_one_with_config::<serde_bytes::ByteBuf>(encoded, &config)
            .map_err(|_| Failure::Invalid)?
            .into_vec();
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
