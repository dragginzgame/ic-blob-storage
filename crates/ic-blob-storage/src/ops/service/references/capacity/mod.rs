//! Tenant-bound conversion over the maintained reference/receipt counters.
use crate::{
    dto::reference::capacity::{
        ReferenceCapacityFailure as F, ReferenceCapacityRequest, ReferenceCapacityResponse,
        ReferenceHeadroom,
    },
    model::{
        identity::ProviderRootHash,
        lifecycle::requests::ReferenceCapacityView,
        service::upload::{
            UploadAdmissionError, UploadAdmissions, UploadContext, content::ContentLookup,
        },
    },
    ops::service::uploads::{StableUploads, UploadStoreError},
};
use candid::Principal;
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU128;

/// Canonical tenant query; linking the library exports no endpoint.
pub const REFERENCE_CAPACITY_METHOD: &str = "blob_reference_capacity";

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
impl ReferenceCapacitySource for UploadAdmissions {
    fn reference_capacity_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<ReferenceCapacityObservation, UploadStoreError> {
        // The heap owner has no restore path; recreation is not operational recovery.
        Ok(ReferenceCapacityObservation {
            headroom: self.reference_capacity(context, input)?,
            fenced: false,
        })
    }
}
fn failure(error: UploadStoreError) -> F {
    match error {
        UploadStoreError::Binding
        | UploadStoreError::Admission(
            UploadAdmissionError::WrongService | UploadAdmissionError::WrongNamespace,
        ) => F::Binding,
        UploadStoreError::Admission(UploadAdmissionError::NotProject) => F::Denied,
        _ => F::Internal,
    }
}
fn present(view: ReferenceCapacityView) -> Result<ReferenceHeadroom, F> {
    let count = |v| u64::try_from(v).map_err(|_| F::Internal);
    Ok(ReferenceHeadroom {
        reference_slots: count(view.reference_slots)?,
        unreserved_receipts: count(view.unreserved_receipts)?,
        release_reserved_receipts: count(view.release_reserved_receipts)?,
        fresh_retains: count(view.fresh_retains)?,
    })
}
pub(crate) fn inspect<S: ReferenceCapacitySource>(
    source: &S,
    context: UploadContext,
    request: ReferenceCapacityRequest,
) -> Result<ReferenceCapacityResponse, F> {
    let scope = request.scope;
    if scope.service != context.service {
        return Err(F::Binding);
    }
    if scope.tenant != context.actor {
        return Err(F::Denied);
    }
    if [Principal::anonymous(), Principal::management_canister()].contains(&scope.tenant) {
        return Err(F::Invalid);
    }
    let input = ContentLookup {
        tenant: scope.tenant,
        namespace: NonZeroU128::new(scope.namespace).ok_or(F::Invalid)?,
        root: ProviderRootHash::try_from(request.root.as_slice()).expect("fixed root width"),
    };
    let observed = source
        .reference_capacity_view(context, input)
        .map_err(failure)?;
    Ok(ReferenceCapacityResponse {
        request,
        headroom: observed.headroom.map(present).transpose()?,
        fenced: observed.fenced,
    })
}
