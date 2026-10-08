//! Bounded root resolution with actual uploader authority and retained index checks.
use super::{StableUploads, admission, exposure};
use crate::policy::upload::exposure::UploadExposureAssessment;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_contracts::dto::upload::certificate::UploadCertificateAssessmentResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure as E;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_memory::ic_stable_structures::Memory;

pub(crate) fn resolve<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    root: &str,
    now: u64,
) -> Result<UploadPermission, E> {
    if context.service != store.config.bindings().service {
        return Err(E::Permission(A::Binding));
    }
    let root: ProviderRootHash = root.parse().map_err(|_| E::Permission(A::Invalid))?;
    let object = store
        .roots
        .binding_for_owner(root)
        .map_err(|e| E::Permission(admission::failure(e.into())))?
        .ok_or(E::Permission(A::Denied))?;
    let view = store
        .indexed_permission(root, object)
        .map_err(exposure::failure)?;
    // The root is only a locator. Neither tenant nor controller status grants issuance.
    if context.actor != view.permission.uploader {
        return Err(E::Permission(A::Denied));
    }
    store
        .exposure_record(context, view.permission.request, now)
        .map_err(exposure::failure)?;
    Ok(view.permission)
}

pub(crate) fn input(permission: UploadPermission) -> UploadAdmissionRequest {
    let r = permission.request;
    let object = r.object.first.object();
    UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: object.service(),
            tenant: object.tenant(),
            namespace: object.identity().namespace.get(),
            upload: r.id.get().get(),
            object: object.identity().object.get(),
            incarnation: object.identity().incarnation.get(),
            first_reference: r.object.first.reference().get().get(),
            root: *r.object.root.as_bytes(),
            bytes: r.object.bytes,
        },
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
    assessment: UploadExposureAssessment,
) -> UploadCertificateAssessmentResponse {
    UploadCertificateAssessmentResponse {
        permission: input(permission),
        assessed_at_ns: now,
        blockers: exposure::blockers(assessment),
    }
}
