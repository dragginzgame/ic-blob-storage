//! Tenant capacity inspection using maintained accounting, never reservation authority.
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::planning::AdmissionCapacityLookup;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollment;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::upload::binding::UploadContext;

use crate::ops::service::uploads::capacity::{UploadCapacitySource, UploadCapacityView};

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
/// Inspect one tenant under actual caller/service context. Hosts bound ingress and
/// keep one synchronous owner borrow. Positive headroom can change immediately;
/// it establishes no identity freshness, provider funds or safe upload/retry authority.
/// Suspension and restore preserve inspection and are reported independently.
/// # Errors
/// Rejects invalid inputs, foreign scope/caller, missing enrollment or inconsistent state.
pub fn inspect<S: UploadCapacitySource>(
    source: &S,
    context: UploadContext,
    scope: TenantScope,
) -> Result<UploadCapacityResponse, UploadCapacityFailure> {
    use UploadCapacityFailure as F;
    let namespace = crate::policy::tenant::assess_scope(
        context.service,
        context.actor,
        scope.service,
        scope.tenant,
        scope.namespace,
    )
    .map_err(|error| match error {
        crate::policy::tenant::TenantScopeError::Binding => F::Binding,
        crate::policy::tenant::TenantScopeError::Denied => F::Denied,
        crate::policy::tenant::TenantScopeError::Invalid => F::Invalid,
    })?;
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
        remaining_logical_bytes: view.remaining_logical_bytes,
        remaining_physical_bytes: view.remaining_physical_bytes,
        remaining_liability_bytes: view.remaining_liability_bytes,
        remaining_bytes: view.remaining_bytes,
        fenced,
    })
}
