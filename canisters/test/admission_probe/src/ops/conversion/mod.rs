//! Exact fixture identity construction and typed error conversion.

use blob_test_protocol::admission::{Failure, Phase, Request};
use ic_blob_storage::model::catalog::admission::UploadPhase;
use ic_blob_storage::model::service::upload::UploadAdmissionError;
use ic_blob_storage::model::service::upload::UploadManifestLimit;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::upload::binding::UploadRequest;

pub(super) fn request(input: Request) -> Result<UploadRequest, Failure> {
    blob_test_protocol::admission::conversion::upload_request(input).ok_or(Failure::InvalidInput)
}

pub(super) const fn phase(phase: UploadPhase) -> Phase {
    match phase {
        UploadPhase::Reserved => Phase::Reserved,
        UploadPhase::Registered => Phase::Registered,
        UploadPhase::Cancelled => Phase::Cancelled,
    }
}

pub(super) fn failure(error: ic_blob_storage::ops::service::uploads::UploadStoreError) -> Failure {
    use ic_blob_storage::ops::service::uploads::UploadStoreError;
    match error {
        UploadStoreError::Admission(error) => admission_failure(error),
        UploadStoreError::Tenants(error) => {
            use ic_blob_storage::ops::service::tenant::TenantStoreError as T;
            match error {
                T::NotOperator => Failure::NotOperator,
                T::NotObserver => Failure::NotObserver,
                T::WrongService => Failure::WrongService,
                T::Tenant(error) => admission_failure(UploadAdmissionError::Tenant(error)),
                _ => Failure::Catalog,
            }
        }
        _ => Failure::Catalog,
    }
}
pub(super) const fn admission_failure(error: UploadAdmissionError) -> Failure {
    match error {
        UploadAdmissionError::NotOperator => Failure::NotOperator,
        UploadAdmissionError::NotProject => Failure::NotProject,
        UploadAdmissionError::NotUploader => Failure::NotUploader,
        UploadAdmissionError::NotObserver => Failure::NotObserver,
        UploadAdmissionError::WrongService => Failure::WrongService,
        UploadAdmissionError::WrongNamespace => Failure::WrongNamespace,
        UploadAdmissionError::Tenant(TenantError::NotEnrolled) => Failure::NotEnrolled,
        UploadAdmissionError::Tenant(TenantError::Suspended) => Failure::Suspended,
        UploadAdmissionError::Tenant(TenantError::StalePermission) => Failure::StalePermission,
        UploadAdmissionError::PermissionConflict
        | UploadAdmissionError::Tenant(TenantError::Conflict) => Failure::Conflict,
        UploadAdmissionError::UnknownPermission => Failure::Unknown,
        UploadAdmissionError::Revoked => Failure::Revoked,
        UploadAdmissionError::Expired => Failure::Expired,
        UploadAdmissionError::ClockReversed => Failure::ClockReversed,
        UploadAdmissionError::ObjectTooLarge
        | UploadAdmissionError::Tenant(TenantError::Capacity | TenantError::GenerationExhausted) => {
            Failure::Capacity
        }
        UploadAdmissionError::ManifestCapacity(UploadManifestLimit::Global) => {
            Failure::ManifestCapacity
        }
        UploadAdmissionError::ManifestCapacity(UploadManifestLimit::Tenant) => {
            Failure::TenantManifestCapacity
        }
        UploadAdmissionError::NotReserved(_) => Failure::Phase,
        UploadAdmissionError::ManifestNotPrepared => Failure::Unprepared,
        UploadAdmissionError::Manifest(_) => Failure::Manifest,
        UploadAdmissionError::Metadata(_) => Failure::Metadata,
        UploadAdmissionError::EmptyObject
        | UploadAdmissionError::InvalidUploader
        | UploadAdmissionError::Tenant(TenantError::InvalidTenant) => Failure::InvalidInput,
        UploadAdmissionError::Reference(_) | UploadAdmissionError::Catalog(_) => Failure::Catalog,
    }
}
