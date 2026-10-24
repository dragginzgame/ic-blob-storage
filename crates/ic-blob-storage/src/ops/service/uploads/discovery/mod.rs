//! Tenant-bound indexed discovery over the maintained upload owner.
use crate::model::service::upload::content::{ContentLookup, TenantContentView};
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::Memory;

/// One synchronous owner observation, without an allocation or lifetime-history scan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadDiscoveryObservation {
    /// Absent for roots unavailable to this tenant.
    pub content: Option<TenantContentView>,
    /// The same owner's restore fence, independent of content visibility.
    pub fenced: bool,
}
/// Read access to one owner. Hosts keep the complete observation under one borrow.
pub trait UploadDiscoverySource {
    /// Authenticate context/scope and inspect retained content without writing.
    /// # Errors
    /// Rejects foreign scope/caller or inconsistent retained state.
    fn discovery_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<UploadDiscoveryObservation, UploadStoreError>;
}
impl<M: Memory> UploadDiscoverySource for StableUploads<M> {
    fn discovery_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<UploadDiscoveryObservation, UploadStoreError> {
        Ok(UploadDiscoveryObservation {
            content: self.lookup_content(context, input)?,
            fenced: self.is_fenced(),
        })
    }
}
