//! Canonical receipt conversion, bounded reply decoding and explicit IC transport.
pub mod capacity;
pub mod client;
pub mod status;
use crate::model::catalog::admission::UploadError;
use crate::model::lifecycle::LifecycleChange;
use crate::model::lifecycle::LifecycleError;
use crate::model::lifecycle::requests::ReferenceReceiptView;
use crate::model::lifecycle::requests::ReferenceRequestError;
use crate::model::service::upload::UploadAdmissionError;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::dto::reference::ReferenceChange;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::ReferenceReceiptResponse;
use ic_blob_storage_contracts::dto::reference::ReferenceTransitionFailure;

// Conversion only. Each boundary authenticates its own tenant/uploader role first.

pub(crate) fn present(
    request: ReferenceCommand,
    view: ReferenceReceiptView,
) -> Result<ReferenceReceiptResponse, ReferenceFailure> {
    let result = match view.result {
        Ok(LifecycleChange::Changed) => Ok(ReferenceChange::Changed),
        Ok(LifecycleChange::Unchanged) => Ok(ReferenceChange::Unchanged),
        Err(LifecycleError::UnknownReference) => Err(ReferenceTransitionFailure::UnknownReference),
        Err(LifecycleError::ReferenceReleased) => Err(ReferenceTransitionFailure::Released),
        Err(LifecycleError::ReferenceLimitReached) => Err(ReferenceTransitionFailure::Limit),
        Err(LifecycleError::DeletionAlreadyQueued) => {
            Err(ReferenceTransitionFailure::DeletionQueued)
        }
        Err(_) => return Err(ReferenceFailure::Internal),
    };
    Ok(ReferenceReceiptResponse { request, result })
}
pub(crate) fn failure(error: UploadStoreError) -> ReferenceFailure {
    match error {
        UploadStoreError::Fenced => ReferenceFailure::Fenced,
        UploadStoreError::Reference(ReferenceRequestError::ReceiptLimitReached) => {
            ReferenceFailure::Capacity
        }
        UploadStoreError::Admission(UploadAdmissionError::Tenant(
            ic_blob_storage_contracts::tenant::TenantError::NotEnrolled
            | ic_blob_storage_contracts::tenant::TenantError::Suspended,
        )) => ReferenceFailure::Inactive,
        UploadStoreError::Admission(UploadAdmissionError::NotProject) => ReferenceFailure::Denied,
        UploadStoreError::Binding
        | UploadStoreError::Admission(
            UploadAdmissionError::WrongService | UploadAdmissionError::WrongNamespace,
        )
        | UploadStoreError::Reference(ReferenceRequestError::BindingMismatch(_)) => {
            ReferenceFailure::Binding
        }
        UploadStoreError::Admission(UploadAdmissionError::UnknownPermission) => {
            ReferenceFailure::Unknown
        }
        UploadStoreError::Admission(UploadAdmissionError::PermissionConflict)
        | UploadStoreError::Reference(ReferenceRequestError::RequestConflict) => {
            ReferenceFailure::Conflict
        }
        UploadStoreError::Admission(UploadAdmissionError::Catalog(UploadError::InvalidPhase(
            _,
        ))) => ReferenceFailure::Unconfirmed,
        _ => ReferenceFailure::Internal,
    }
}

pub(crate) fn mutation(
    request: ReferenceCommand,
    outcome: &crate::model::lifecycle::requests::ReferenceRequestOutcome,
) -> Result<ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse, ReferenceFailure>
{
    use crate::model::lifecycle::requests::ReferenceRequestOutcome;
    let (replayed, result) = match outcome {
        ReferenceRequestOutcome::Recorded { result } => (false, *result),
        ReferenceRequestOutcome::Replayed { result } => (true, *result),
    };
    Ok(
        ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse {
            receipt: present(request, ReferenceReceiptView { result })?,
            replayed,
        },
    )
}
