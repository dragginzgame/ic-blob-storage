//! Shared checks for the heap owner and durable pending-upload owner.
use super::NonZeroU64;
use super::ObjectBinding;
use super::Principal;
use super::ServiceConfiguration;
use super::TenantError;
use super::UploadAdmissionError;
use super::UploadPermissionView;
use super::UploadPhase;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;

pub(crate) fn tenant(
    config: &ServiceConfiguration,
    context: UploadContext,
    tenant: Principal,
    namespace: std::num::NonZeroU128,
) -> Result<(), UploadAdmissionError> {
    if context.service != config.bindings().service {
        return Err(UploadAdmissionError::WrongService);
    }
    if namespace != config.bindings().namespace {
        return Err(UploadAdmissionError::WrongNamespace);
    }
    if context.actor != tenant {
        return Err(UploadAdmissionError::NotProject);
    }
    Ok(())
}

pub(crate) fn object(
    config: &ServiceConfiguration,
    context: UploadContext,
    object: ObjectBinding,
) -> Result<(), UploadAdmissionError> {
    if context.service != config.bindings().service || object.service() != context.service {
        return Err(UploadAdmissionError::WrongService);
    }
    if object.identity().namespace != config.bindings().namespace {
        return Err(UploadAdmissionError::WrongNamespace);
    }
    Ok(())
}

pub(crate) fn fresh(
    config: &ServiceConfiguration,
    input: UploadPermission,
    now: u64,
) -> Result<(), UploadAdmissionError> {
    if input.uploader == Principal::anonymous()
        || input.uploader == Principal::management_canister()
    {
        return Err(UploadAdmissionError::InvalidUploader);
    }
    if now >= input.expires_at_ns {
        return Err(UploadAdmissionError::Expired);
    }
    if input.request.object.bytes == 0 {
        return Err(UploadAdmissionError::EmptyObject);
    }
    if input.request.object.bytes > config.limits().max_object_bytes.get() {
        return Err(UploadAdmissionError::ObjectTooLarge);
    }
    Ok(())
}

pub(crate) fn uploader(
    context: UploadContext,
    view: &UploadPermissionView,
    generation: Result<NonZeroU64, TenantError>,
    now: u64,
) -> Result<(), UploadAdmissionError> {
    if context.actor != view.permission.uploader {
        return Err(UploadAdmissionError::NotUploader);
    }
    if view.revoked {
        return Err(UploadAdmissionError::Revoked);
    }
    if generation? != view.tenant_generation {
        return Err(TenantError::StalePermission.into());
    }
    if now < view.admitted_at_ns {
        return Err(UploadAdmissionError::ClockReversed);
    }
    if now >= view.permission.expires_at_ns {
        return Err(UploadAdmissionError::Expired);
    }
    if view.phase != UploadPhase::Reserved {
        return Err(UploadAdmissionError::NotReserved(view.phase));
    }
    Ok(())
}
