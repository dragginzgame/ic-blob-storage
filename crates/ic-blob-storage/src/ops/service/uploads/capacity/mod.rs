//! Shared admission-capacity boundary for the durable owner and heap model.
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::UploadAdmissions;
use crate::model::service::upload::planning::AdmissionCapacityLookup;
use crate::model::service::upload::planning::AdmissionCapacityView;
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::UploadStoreError;
use candid::Principal;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollment;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU128;

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
