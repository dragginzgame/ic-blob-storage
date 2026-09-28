//! Canonical receipt conversion, bounded reply decoding and explicit IC transport.
pub mod client;
pub mod reply;
use crate::{
    dto::reference::{
        ReferenceAction, ReferenceChange, ReferenceCommand, ReferenceFailure,
        ReferenceReceiptResponse, ReferenceTransitionFailure, ReferenceUpload,
    },
    model::{
        catalog::admission::{UploadError, UploadObject, UploadRequest, UploadRequestId},
        identity::ProviderRootHash,
        lifecycle::{
            LifecycleChange, LifecycleError, ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
            requests::{
                ReferenceOperation, ReferenceReceiptView, ReferenceRequest, ReferenceRequestError,
                ReferenceRequestId,
            },
        },
        service::upload::{UploadAdmissionError, UploadContext},
    },
    ops::service::uploads::UploadStoreError,
};
use std::num::NonZeroU128;
/// Canonical query method, also callable through replicated IC execution.
/// Linking the library does not export it.
pub const REFERENCE_RECEIPT_METHOD: &str = "blob_reference_receipt";
/// Canonical update method. The caller must durably retain the exact intent first.
pub const REFERENCE_APPLY_METHOD: &str = "blob_apply_reference";

pub(crate) fn parse(
    context: UploadContext,
    request: ReferenceCommand,
) -> Result<(UploadRequest, ReferenceRequest), ReferenceFailure> {
    let upload = parse_upload(context, request.upload)?;
    let positive = |n| NonZeroU128::new(n).ok_or(ReferenceFailure::Invalid);
    let key = ReferenceKey::new(
        upload.object.first.object(),
        ReferenceId::new(positive(request.reference)?),
    );
    Ok((
        upload,
        ReferenceRequest {
            id: ReferenceRequestId::new(positive(request.operation)?),
            operation: match request.action {
                ReferenceAction::Retain => ReferenceOperation::Retain(key),
                ReferenceAction::Release => ReferenceOperation::Release(key),
            },
        },
    ))
}
pub(crate) fn parse_upload(
    context: UploadContext,
    upload: ReferenceUpload,
) -> Result<UploadRequest, ReferenceFailure> {
    if upload.service != context.service {
        return Err(ReferenceFailure::Binding);
    }
    if upload.tenant != context.actor {
        return Err(ReferenceFailure::Denied);
    }
    if upload.bytes == 0 {
        return Err(ReferenceFailure::Invalid);
    }
    let positive = |n| NonZeroU128::new(n).ok_or(ReferenceFailure::Invalid);
    let object = ObjectBinding::new(
        upload.service,
        upload.tenant,
        ObjectIdentity {
            namespace: positive(upload.namespace)?,
            object: positive(upload.object)?,
            incarnation: positive(upload.incarnation)?,
        },
    )
    .map_err(|_| ReferenceFailure::Invalid)?;
    Ok(UploadRequest {
        id: UploadRequestId::new(positive(upload.upload)?),
        object: UploadObject {
            root: ProviderRootHash::try_from(upload.root.as_slice()).expect("fixed root"),
            bytes: upload.bytes,
            first: ReferenceKey::new(object, ReferenceId::new(positive(upload.first_reference)?)),
        },
    })
}
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
            crate::model::service::tenant::TenantError::NotEnrolled
            | crate::model::service::tenant::TenantError::Suspended,
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
) -> Result<crate::dto::reference::ReferenceMutationResponse, ReferenceFailure> {
    use crate::model::lifecycle::requests::ReferenceRequestOutcome;
    let (replayed, result) = match outcome {
        ReferenceRequestOutcome::Recorded { result } => (false, *result),
        ReferenceRequestOutcome::Replayed { result } => (true, *result),
    };
    Ok(crate::dto::reference::ReferenceMutationResponse {
        receipt: present(request, ReferenceReceiptView { result })?,
        replayed,
    })
}
