//! Shared pure validators own permission/metadata/tree invariants before dispatch.
use super::{
    Failure, Input, Kind, Request, admission_error, manifest_error, manifest_reply_limits,
};
use candid::Principal;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::upload::admission::reply as admission;
use ic_blob_storage_contracts::upload::manifests::reply as manifest;

pub(super) fn load(input: &Input, actor: Principal) -> Result<Request, Failure> {
    let bytes = crate::native::read(
        &input.request,
        if input.kind == Kind::Prepare {
            65536
        } else {
            4096
        },
    )?;
    let manifest: Option<UploadManifestRequest> = if input.kind == Kind::Prepare {
        Some(crate::native::exact_candid::decode(
            &bytes, 65536, 64, 2_000_000,
        )?)
    } else {
        None
    };
    let permission: UploadAdmissionRequest = match &manifest {
        Some(r) => r.permission,
        None => crate::native::exact_candid::decode(&bytes, 65536, 64, 2_000_000)?,
    };
    admission::validate_request(permission).map_err(admission_error)?;
    if permission.upload.service != input.service || permission.upload.namespace != input.namespace
    {
        return Err(Failure::Binding);
    }
    let allowed = match input.kind {
        Kind::Prepare => actor == permission.uploader,
        Kind::Manifest => actor == permission.upload.tenant || actor == permission.uploader,
        Kind::Admit | Kind::Revoke | Kind::Permission => actor == permission.upload.tenant,
    };
    if !allowed {
        return Err(Failure::Binding);
    }
    if let Some(r) = &manifest {
        let limits = manifest_reply_limits(
            (1024 * 1024 * 1024)
                .try_into()
                .expect("positive content bound"),
        );
        manifest::validate_declaration(permission, &r.declaration, limits.declaration)
            .map_err(manifest_error)?;
    }
    let argument = if let Some(r) = &manifest {
        candid::encode_one(r)
    } else {
        candid::encode_one(permission)
    }
    .map_err(|_| Failure::Arguments)?;
    Ok(Request {
        permission,
        manifest,
        argument,
    })
}
