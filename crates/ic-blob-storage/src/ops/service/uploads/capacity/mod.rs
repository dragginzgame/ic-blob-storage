//! Shared admission-capacity boundary for the durable owner and heap model.
use crate::model::service::upload::planning::{AdmissionCapacityLookup, AdmissionCapacityView};
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::Memory;

/// One synchronous local observation, without reservation or restore authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadCapacityView {
    /// Maintained global/tenant capacity dimensions and enrollment.
    pub headroom: AdmissionCapacityView,
    /// The same owner is inspection-only after restoration.
    pub fenced: bool,
}

/// Synchronous read access to one owner; implementations must authenticate context
/// and read maintained counters without reserving or scanning operation history.
pub trait UploadCapacitySource {
    /// Read capacity and the same owner's restore fence under one host borrow.
    /// # Errors
    /// Rejects scope, caller, missing enrollment or inconsistent retained state.
    fn capacity_view(
        &self,
        context: UploadContext,
        scope: AdmissionCapacityLookup,
    ) -> Result<UploadCapacityView, UploadStoreError>;
}
impl<M: Memory> UploadCapacitySource for StableUploads<M> {
    fn capacity_view(
        &self,
        context: UploadContext,
        scope: AdmissionCapacityLookup,
    ) -> Result<UploadCapacityView, UploadStoreError> {
        Ok(UploadCapacityView {
            headroom: self.admission_capacity(context, scope)?,
            fenced: self.is_fenced(),
        })
    }
}
