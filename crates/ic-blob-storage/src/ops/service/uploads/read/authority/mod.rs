//! Indexed current read authority, with no manifest or history copies.
use super::{
    Memory, ProviderRootHash, ReferenceKey, StableUploads, UploadContext, UploadStoreError,
    validation,
};
use std::num::NonZeroU64;
impl<M: Memory> StableUploads<M> {
    pub(crate) fn read_chunk_bytes(
        &self,
        context: UploadContext,
        target: crate::model::service::read::ReadTarget,
        index: u64,
    ) -> Result<Option<u64>, UploadStoreError> {
        let Some(view) = self.retained_content(context, target.root, target.reference)? else {
            return Ok(None);
        };
        let bytes = view.request.object.bytes;
        let chunk = crate::model::identity::caffeine::CAFFEINE_CHUNK_BYTES as u64;
        let Some(offset) = index.checked_mul(chunk).filter(|offset| *offset < bytes) else {
            return Ok(None);
        };
        Ok(Some((bytes - offset).min(chunk)))
    }
    pub(crate) fn read_authority_generation(
        &self,
        context: UploadContext,
        root: ProviderRootHash,
        reference: ReferenceKey,
    ) -> Result<Option<NonZeroU64>, UploadStoreError> {
        let object = reference.object();
        validation::object(&self.config, context, object)?;
        validation::tenant(
            &self.config,
            context,
            object.tenant(),
            object.identity().namespace,
        )?;
        self.mutable()?;
        let generation = self.generation(object.tenant())?;
        Ok(self
            .retained_content(context, root, reference)?
            .map(|_| generation))
    }
}
