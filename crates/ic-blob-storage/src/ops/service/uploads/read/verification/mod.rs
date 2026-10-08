//! Verify one leaf from its already admitted immutable manifest; no tree rebuild.
use super::{Memory, StableUploads, UploadContext, UploadStoreError, key};
use crate::model::service::read::ReadTarget;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkRange;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineManifestError;
use thiserror::Error;
/// Trusted manifest state or exact returned bytes were rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ReadVerificationError {
    /// Stored target/reference/manifest is absent or inconsistent.
    #[error(transparent)]
    Store(#[from] UploadStoreError),
    /// Length or domain-separated leaf hash differs from the declaration.
    #[error(transparent)]
    Content(#[from] CaffeineManifestError),
}
impl<M: Memory> StableUploads<M> {
    pub(crate) fn verify_read_chunk(
        &self,
        execution: UploadContext,
        target: ReadTarget,
        index: u64,
        bytes: &[u8],
    ) -> Result<CaffeineChunkRange, ReadVerificationError> {
        let content = self
            .retained_content(execution, target.root, target.reference)?
            .ok_or(UploadStoreError::InvalidRecord)?;
        let record = self
            .manifests
            .get(&key(content.request))
            .ok_or(UploadStoreError::InvalidRecord)?;
        let (range, leaf) = record
            .read_leaf(content.request.object.bytes, index)
            .ok_or(UploadStoreError::InvalidRecord)?;
        ic_blob_storage_contracts::identity::caffeine::manifest::verify_leaf(
            leaf,
            range.bytes,
            bytes,
        )?;
        Ok(range)
    }
}
