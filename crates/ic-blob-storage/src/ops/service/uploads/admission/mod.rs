//! Canonical permission conversion and bounded replicated transport.
pub mod client;
pub mod reply;
use crate::{
    dto::{
        reference::ReferenceFailure,
        upload::admission::{
            UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
            UploadAdmissionResponse,
        },
    },
    model::{
        catalog::{
            CatalogError,
            admission::{UploadAdmission, UploadError},
        },
        lifecycle::roots::RootClaimError,
        service::{
            tenant::TenantError,
            upload::{UploadAdmissionError, UploadContext, UploadPermission, UploadPermissionView},
        },
    },
    ops::service::{references, uploads::UploadStoreError},
};
/// Canonical admission update. Linking the library exports no endpoint.
pub const UPLOAD_ADMISSION_METHOD: &str = "blob_admit_upload";
/// Read-only exact permission query, also callable through replicated execution.
pub const UPLOAD_ADMISSION_STATUS_METHOD: &str = "blob_upload_admission";
pub(crate) fn parse(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadPermission, UploadAdmissionFailure> {
    let request = references::parse_upload(context, input.upload).map_err(|e| match e {
        ReferenceFailure::Denied => UploadAdmissionFailure::Denied,
        ReferenceFailure::Binding => UploadAdmissionFailure::Binding,
        _ => UploadAdmissionFailure::Invalid,
    })?;
    if [
        candid::Principal::anonymous(),
        candid::Principal::management_canister(),
    ]
    .contains(&input.uploader)
    {
        return Err(UploadAdmissionFailure::Invalid);
    }
    Ok(UploadPermission {
        request,
        uploader: input.uploader,
        expires_at_ns: input.expires_at_ns,
    })
}
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
