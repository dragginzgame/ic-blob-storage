//! Tenant-bound conversion over the maintained reference/receipt counters.
use crate::model::lifecycle::requests::ReferenceCapacityView;
use crate::model::service::upload::content::ContentLookup;
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::Memory;

/// One synchronous owner observation, without a reservation or lifetime-history scan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceCapacityObservation {
    /// Absent for roots unavailable to this tenant or not yet confirmed.
    pub headroom: Option<ReferenceCapacityView>,
    /// The same owner's restore fence, independent of content visibility.
    pub fenced: bool,
}
/// Read access to one owner. Hosts keep the complete observation under one borrow.
pub trait ReferenceCapacitySource {
    /// Authenticate context/scope and inspect retained counters without writing.
    /// # Errors
    /// Rejects foreign scope/caller or inconsistent retained state.
    fn reference_capacity_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<ReferenceCapacityObservation, UploadStoreError>;
}
impl<M: Memory> ReferenceCapacitySource for StableUploads<M> {
    fn reference_capacity_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<ReferenceCapacityObservation, UploadStoreError> {
        Ok(ReferenceCapacityObservation {
            headroom: self.reference_capacity(context, input)?,
            fenced: self.is_fenced(),
        })
    }
}
