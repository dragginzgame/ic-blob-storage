//! Exact fixture identity construction and typed error conversion.

use blob_test_protocol::admission::{Failure, Phase, Request};
use ic_blob_storage::model::{
    catalog::admission::{UploadObject, UploadPhase, UploadRequest, UploadRequestId},
    identity::ProviderRootHash,
    lifecycle::{
        ReferenceId,
        binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
    },
    service::{
        tenant::TenantError,
        upload::{UploadAdmissionError, UploadManifestLimit},
    },
};
use std::num::NonZeroU128;

pub(super) fn request(input: Request) -> Result<UploadRequest, Failure> {
    let id = NonZeroU128::new(input.id).ok_or(Failure::InvalidInput)?;
    let namespace = NonZeroU128::new(input.namespace).ok_or(Failure::InvalidInput)?;
    let binding = ObjectBinding::new(
        input.service,
        input.tenant,
        ObjectIdentity {
            namespace,
            object: id,
            incarnation: NonZeroU128::MIN,
        },
    )
    .map_err(|_| Failure::InvalidInput)?;
    Ok(UploadRequest {
        id: UploadRequestId::new(id),
        object: UploadObject {
            root: ProviderRootHash::try_from(input.root.as_slice()).expect("fixed hash width"),
            bytes: input.bytes,
            first: ReferenceKey::new(binding, ReferenceId::new(NonZeroU128::MIN)),
        },
    })
}

pub(super) const fn phase(phase: UploadPhase) -> Phase {
    match phase {
        UploadPhase::Reserved => Phase::Reserved,
        UploadPhase::ExposurePossible => Phase::ExposurePossible,
        UploadPhase::Confirmed => Phase::Confirmed,
        UploadPhase::Cancelled => Phase::Cancelled,
    }
}

pub(super) const fn failure(error: UploadAdmissionError) -> Failure {
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
