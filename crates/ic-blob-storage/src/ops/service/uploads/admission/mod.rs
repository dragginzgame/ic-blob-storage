//! Canonical permission conversion and bounded replicated transport.
pub mod client;
use crate::model::catalog::CatalogError;
use crate::model::catalog::admission::UploadAdmission;
use crate::model::catalog::admission::UploadError;
use crate::model::lifecycle::roots::RootClaimError;
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::UploadPermissionView;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::tenant::TenantError;

pub(crate) fn revocation(
    admission: UploadAdmissionResponse,
    change: crate::model::lifecycle::LifecycleChange,
) -> ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse {
    ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse {
        admission,
        changed: change == crate::model::lifecycle::LifecycleChange::Changed,
    }
}

// Conversion without actor substitution; the calling boundary owns role checks.

pub(crate) fn present(
    input: UploadAdmissionRequest,
    view: &UploadPermissionView,
) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
    // The owner's exact lookup already binds every upload field. Permission recovery
    // additionally binds uploader/deadline; no later retry may renew either field.
    if view.permission.uploader != input.uploader
        || view.permission.expires_at_ns != input.expires_at_ns
    {
        return Err(UploadAdmissionFailure::Conflict);
    }
    let status = super::status::present(input.upload, view.phase, view.revoked);
    Ok(UploadAdmissionResponse {
        permission: input,
        state: status.state,
        revoked: status.revoked,
    })
}
pub(crate) fn mutation(
    admission: UploadAdmissionResponse,
    outcome: UploadAdmission,
) -> UploadAdmissionMutation {
    UploadAdmissionMutation {
        admission,
        replayed: outcome != UploadAdmission::Reserved,
    }
}
pub(crate) fn failure(error: UploadStoreError) -> UploadAdmissionFailure {
    use UploadAdmissionError as A;
    use UploadAdmissionFailure as F;
    match error {
        UploadStoreError::Fenced => F::Fenced,
        UploadStoreError::Binding
        | UploadStoreError::Admission(A::WrongService | A::WrongNamespace) => F::Binding,
        UploadStoreError::Admission(A::NotProject | A::NotObserver) => F::Denied,
        UploadStoreError::Admission(A::UnknownPermission) => F::Unknown,
        UploadStoreError::Admission(
            A::PermissionConflict
            | A::Catalog(
                UploadError::RequestConflict
                | UploadError::Root(
                    RootClaimError::RootAlreadyClaimed | RootClaimError::ObjectAlreadyClaimed,
                ),
            ),
        ) => F::Conflict,
        UploadStoreError::Admission(A::Tenant(
            TenantError::NotEnrolled | TenantError::Suspended,
        )) => F::Inactive,
        UploadStoreError::Admission(A::Expired) => F::Expired,
        UploadStoreError::Admission(
            A::ObjectTooLarge
            | A::ManifestCapacity(_)
            | A::Catalog(
                UploadError::ActiveLimit
                | UploadError::TenantActiveLimit
                | UploadError::Catalog(CatalogError::Capacity(_)),
            ),
        ) => F::Capacity,
        UploadStoreError::Admission(A::InvalidUploader | A::EmptyObject) => F::Invalid,
        _ => F::Internal,
    }
}
