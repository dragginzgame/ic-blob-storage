//! Current serving descriptor from one active, unfenced durable upload owner.
use super::{
    Memory, ProviderRootHash, ReferenceKey, RetainedUploadDescriptorView, StableUploads,
    UploadContext, UploadStoreError,
};
use crate::model::service::read::download::CaffeineDownloadScope;
use thiserror::Error;

/// A current local observation, not a certified public mapping or publication lease.
/// The caller must authenticate delivery and coordinate its reference lifetime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaffeineDownloadDescriptorView {
    /// Host-supplied owner/project mapping checked against the installed namespace.
    pub scope: CaffeineDownloadScope,
    /// Exact live reference, root, length and first-accepted hash metadata.
    pub content: RetainedUploadDescriptorView,
    /// Relative Caffeine HTTP target; an approved origin is deliberately separate.
    pub request_target: String,
}
/// No operational descriptor can be disclosed for this request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum DownloadDescriptorError {
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
    ) -> Result<CaffeineDownloadDescriptorView, DownloadDescriptorError> {
        let bindings = self.callback_configuration().bindings();
        if scope.owner() != bindings.service || scope.namespace() != bindings.namespace {
            return Err(DownloadDescriptorError::Binding);
        }
        // Operational disclosure requires active enrollment and an unfenced owner.
        // Historical descriptor inspection deliberately keeps its separate contract.
        self.active_read_generation(context, reference)?;
        let retained = self
            .retained_content_descriptor(context, root, reference)?
            .ok_or(DownloadDescriptorError::Unavailable)?;
        Ok(CaffeineDownloadDescriptorView {
            scope: scope.clone(),
            content: retained,
            request_target: crate::ops::caffeine::download::request_target(scope, root),
        })
    }
}
