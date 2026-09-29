//! Shared admission-capacity boundary for the durable owner and heap model.
use crate::{
    dto::{
        tenant::{TenantEnrollment, TenantScope},
        upload::capacity::{UploadCapacityFailure, UploadCapacityResponse},
    },
    model::service::{
        tenant::TenantError,
        upload::{
            UploadAdmissionError, UploadAdmissions, UploadContext,
            planning::{AdmissionCapacityLookup, AdmissionCapacityView},
        },
    },
    ops::service::uploads::{StableUploads, UploadStoreError},
};
use candid::Principal;
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU128;

/// Canonical passive tenant query; linking exports no endpoint.
pub const UPLOAD_CAPACITY_METHOD: &str = "blob_upload_capacity";

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
impl UploadCapacitySource for UploadAdmissions {
    fn capacity_view(
        &self,
        context: UploadContext,
        scope: AdmissionCapacityLookup,
    ) -> Result<UploadCapacityView, UploadStoreError> {
        // The heap owner has no restore path. Hosts must not recreate it from an old snapshot.
        Ok(UploadCapacityView {
            headroom: self.admission_capacity(context, scope)?,
            fenced: false,
        })
    }
}
fn failure(error: UploadStoreError) -> UploadCapacityFailure {
    use UploadAdmissionError as A;
    use UploadCapacityFailure as F;
    match error {
        UploadStoreError::Binding
        | UploadStoreError::Admission(A::WrongService | A::WrongNamespace) => F::Binding,
        UploadStoreError::Admission(A::NotProject) => F::Denied,
        UploadStoreError::Admission(A::Tenant(TenantError::NotEnrolled)) => F::NotEnrolled,
        _ => F::Internal,
    }
}
pub(crate) fn inspect<S: UploadCapacitySource>(
    source: &S,
    context: UploadContext,
    scope: TenantScope,
) -> Result<UploadCapacityResponse, UploadCapacityFailure> {
    use UploadCapacityFailure as F;
    if scope.service != context.service {
        return Err(F::Binding);
    }
    if scope.tenant != context.actor {
        return Err(F::Denied);
    }
    if [Principal::anonymous(), Principal::management_canister()].contains(&scope.tenant) {
        return Err(F::Invalid);
    }
    let namespace = NonZeroU128::new(scope.namespace).ok_or(F::Invalid)?;
    let UploadCapacityView {
        headroom: view,
        fenced,
    } = source
        .capacity_view(
            context,
            AdmissionCapacityLookup {
                tenant: scope.tenant,
                namespace,
            },
        )
        .map_err(failure)?;
    let count = |value| u64::try_from(value).map_err(|_| F::Internal);
    Ok(UploadCapacityResponse {
        scope,
        enrollment: TenantEnrollment {
            generation: view.enrollment.generation.get(),
            active: view.enrollment.active,
        },
        max_object_bytes: view.max_object_bytes,
        max_headers: count(view.max_headers)?,
        max_header_bytes: count(view.max_header_bytes)?,
        remaining_objects: count(view.remaining_objects)?,
        remaining_active_uploads: count(view.remaining_active_uploads)?,
        remaining_manifest_chunks: view.remaining_manifest_chunks,
        remaining_bytes: view.remaining_bytes,
        fenced,
    })
}
