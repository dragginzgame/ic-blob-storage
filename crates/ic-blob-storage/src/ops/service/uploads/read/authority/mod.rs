//! Indexed current read authority, with no manifest or history copies.
use super::{
    Memory, ProviderRootHash, ReferenceKey, StableUploads, UploadContext, UploadStoreError,
    validation,
};
use std::num::NonZeroU64;
impl<M: Memory> StableUploads<M> {
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
