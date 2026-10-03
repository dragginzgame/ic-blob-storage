//! Current serving descriptor from one active, unfenced durable upload owner.
use super::{
    Memory, ProviderRootHash, ReferenceKey, RetainedUploadDescriptorView, StableUploads,
    UploadContext, UploadStoreError,
};
use crate::model::service::read::download::CaffeineDownloadScope;
use thiserror::Error;

/// No operational descriptor can be disclosed for this request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub(crate) enum DownloadDescriptorError {
    /// Owner or local namespace differs from the installed service.
    #[error("download scope does not match installation")]
    Binding,
    /// No confirmed object with exactly this live reference.
    #[error("download unavailable")]
    Unavailable,
    /// Tenant authority, restore fence or stored invariant failed.
    #[error(transparent)]
    Store(#[from] UploadStoreError),
}
impl<M: Memory> StableUploads<M> {
    pub(crate) fn download_descriptor(
        &self,
        context: UploadContext,
        scope: &CaffeineDownloadScope,
        root: ProviderRootHash,
        reference: ReferenceKey,
    ) -> Result<RetainedUploadDescriptorView, DownloadDescriptorError> {
        let bindings = self.callback_configuration().bindings();
        if scope.owner() != bindings.service || scope.namespace() != bindings.namespace {
            return Err(DownloadDescriptorError::Binding);
        }
        // Operational disclosure requires active enrollment and an unfenced owner.
        // Historical descriptor inspection deliberately keeps its separate contract.
        self.active_read_generation(context, reference)?;
        self.retained_content_descriptor(context, root, reference)?
            .ok_or(DownloadDescriptorError::Unavailable)
    }
}
