//! Exact fixture identities and observable typed failure categories.
use blob_test_protocol::{
    admission::{Phase, Request},
    storage::Failure,
};
use ic_blob_storage::{
    model::{
        catalog::admission::{UploadObject, UploadPhase, UploadRequest, UploadRequestId},
        identity::ProviderRootHash,
        lifecycle::{
            ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        },
        service::{tenant::TenantError, upload::UploadAdmissionError},
    },
    ops::service::{tenant::TenantStoreError, uploads::UploadStoreError},
};
use std::num::NonZeroU128;
pub(super) fn request(input: Request) -> Result<UploadRequest, Failure> {
    let id = NonZeroU128::new(input.id).ok_or(Failure::Invalid)?;
    let namespace = NonZeroU128::new(input.namespace).ok_or(Failure::Invalid)?;
    let object = ObjectBinding::new(
        input.service,
        input.tenant,
        ObjectIdentity {
            namespace,
            object: id,
            incarnation: NonZeroU128::MIN,
        },
    )
    .map_err(|_| Failure::Invalid)?;
    Ok(UploadRequest {
        id: UploadRequestId::new(id),
        object: UploadObject {
            root: ProviderRootHash::try_from(input.root.as_slice()).expect("fixed root"),
            bytes: input.bytes,
            first: ReferenceKey::new(object, ReferenceId::new(NonZeroU128::MIN)),
        },
    })
}
pub(super) const fn phase(phase: UploadPhase) -> Phase {
    match phase {
        UploadPhase::Reserved => Phase::Reserved,
        UploadPhase::ExposurePossible => Phase::ExposurePossible,
        UploadPhase::Cancelled => Phase::Cancelled,
        UploadPhase::Confirmed => Phase::Confirmed,
    }
}
pub(super) const fn failure(error: UploadStoreError) -> Failure {
    match error {
        UploadStoreError::CursorScope => Failure::CursorScope,
        UploadStoreError::Reference(
            ic_blob_storage::model::lifecycle::requests::ReferenceRequestError::ReceiptLimitReached,
        ) => Failure::ReceiptCapacity,
        UploadStoreError::Reference(
            ic_blob_storage::model::lifecycle::requests::ReferenceRequestError::RequestConflict,
        ) => Failure::Conflict,
        UploadStoreError::Lifecycle(_)
        | UploadStoreError::Admission(
            UploadAdmissionError::Catalog(
                ic_blob_storage::model::catalog::admission::UploadError::InvalidPhase(_),
            )
            | UploadAdmissionError::NotReserved(_),
        ) => Failure::Phase,
        UploadStoreError::Fenced | UploadStoreError::Tenants(TenantStoreError::Fenced) => {
            Failure::Fenced
        }
        UploadStoreError::Tenants(
            TenantStoreError::NotOperator | TenantStoreError::NotObserver,
        )
        | UploadStoreError::Admission(
            UploadAdmissionError::NotOperator
            | UploadAdmissionError::NotProject
            | UploadAdmissionError::NotUploader
            | UploadAdmissionError::NotObserver,
        ) => Failure::Denied,
        UploadStoreError::Admission(UploadAdmissionError::UnknownPermission) => Failure::Unknown,
        UploadStoreError::Admission(UploadAdmissionError::ManifestNotPrepared) => {
            Failure::Unprepared
        }
        UploadStoreError::Admission(UploadAdmissionError::Revoked) => Failure::Revoked,
        UploadStoreError::Admission(
            UploadAdmissionError::WrongService | UploadAdmissionError::WrongNamespace,
        ) => Failure::Binding,
        UploadStoreError::Admission(UploadAdmissionError::PermissionConflict)
        | UploadStoreError::Tenants(TenantStoreError::Tenant(TenantError::Conflict)) => {
            Failure::Conflict
        }
        UploadStoreError::Admission(UploadAdmissionError::Tenant(
            TenantError::NotEnrolled | TenantError::Suspended | TenantError::StalePermission,
        )) => Failure::Inactive,
        UploadStoreError::Admission(
            UploadAdmissionError::ManifestCapacity(_) | UploadAdmissionError::ObjectTooLarge,
        )
        | UploadStoreError::Tenants(TenantStoreError::Tenant(TenantError::Capacity)) => {
            Failure::Capacity
        }
        _ => Failure::Invalid,
    }
}
