//! Bounded root resolution with actual uploader authority and retained index checks.
use super::{StableUploads, UploadPermissionRecord, admission, exposure};
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_contracts::dto::upload::certificate::UploadCertificateAssessmentResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure as E;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_memory::Memory;

pub(crate) fn resolve<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<UploadPermissionRecord, E> {
    if context.service != store.config.bindings().service {
        return Err(E::Permission(A::Binding));
    }
    let root: ProviderRootHash = root.parse().map_err(|_| E::Permission(A::Invalid))?;
    let object = store
        .roots
        .binding_for_owner(root)
        .map_err(|e| E::Permission(admission::failure(e.into())))?
        .ok_or(E::Permission(A::Denied))?;
    let record = store
        .indexed_permission_record(root, object)
        .map_err(exposure::failure)?;
    let view = record.view().ok_or(E::Permission(A::Internal))?;
    // The root is only a locator. Neither tenant nor controller status grants issuance.
    if context.actor != view.permission.uploader {
        return Err(E::Permission(A::Denied));
    }
    store
        .check_exposure_record(context, &record, now)
        .map_err(exposure::failure)?;
    Ok(record)
}

pub(crate) fn input(permission: UploadPermission) -> UploadAdmissionRequest {
    UploadAdmissionRequest {
        upload: admission::upload(permission.request),
        uploader: permission.uploader,
        expires_at_ns: permission.expires_at_ns,
    }
}

pub(crate) fn response(permission: UploadPermission) -> CaffeineUploadCertificateResponse {
    CaffeineUploadCertificateResponse {
        method: "upload".into(),
        blob_hash: permission.request.object.root.to_string(),
    }
}

pub(crate) fn assessment(
    permission: UploadPermission,
    now: u64,
) -> UploadCertificateAssessmentResponse {
    UploadCertificateAssessmentResponse {
        permission: input(permission),
        assessed_at_ns: now,
    }
}

/// Inspect current uploader eligibility without creating a reusable issuance permit.
pub(crate) fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<UploadCertificateAssessmentResponse, E> {
    let record = resolve(store, context, root, now)?;
    let permission = record.view().ok_or(E::Permission(A::Internal))?.permission;
    Ok(assessment(permission, now))
}
/// Resolve current authority and commit exposure before returning the provider payload.
/// No borrow or authorization crosses an await; traps retain IC atomic rollback.
pub(crate) fn issue<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<CaffeineUploadCertificateResponse, E> {
    let record = resolve(store, context, root, now)?;
    let permission = record.view().ok_or(E::Permission(A::Internal))?.permission;
    exposure::commit(store, record)?;
    Ok(response(permission))
}
