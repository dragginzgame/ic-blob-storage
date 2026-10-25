//! Shared pure validators own permission/metadata/tree invariants before dispatch.
use super::{
    Failure, Input, Kind, Payload, Request, admission_error, manifest_error, manifest_reply_limits,
};
use ic_blob_storage_contracts::upload::admission::reply as admission;
use ic_blob_storage_contracts::upload::manifests::reply as manifest;
use ic_platform::Principal;

pub(super) fn load(input: &Input, actor: Principal) -> Result<Request, Failure> {
    let bytes = crate::native::read(
        &input.request,
        if input.kind == Kind::Prepare {
            super::MANIFEST_DOCUMENT_BYTES as u64
        } else {
            super::PERMISSION_DOCUMENT_BYTES as u64
        },
    )?;
    let directory = || input.directory.clone().ok_or(Failure::Arguments);
    let permission = || crate::native::exact_wire::decode(&bytes, super::PERMISSION_DOCUMENT_BYTES);
    let payload = match input.kind {
        Kind::Prepare => Payload::Prepare(
            crate::native::exact_wire::decode(&bytes, super::MANIFEST_DOCUMENT_BYTES)?,
            directory()?,
        ),
        Kind::Admit => Payload::Admit(permission()?, directory()?),
        Kind::Revoke => Payload::Revoke(permission()?, directory()?),
        Kind::Permission => Payload::Permission(permission()?),
        Kind::Manifest => Payload::Manifest(permission()?),
    };
    let permission = payload.permission();
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
    if let Payload::Prepare(r, _) = &payload {
        let limits = manifest_reply_limits(
            super::MAX_CONTENT_BYTES
                .try_into()
                .expect("positive content bound"),
        );
        manifest::validate_declaration(permission, &r.declaration, limits.declaration)
            .map_err(manifest_error)?;
    }
    let argument = match &payload {
        Payload::Prepare(manifest, _) => {
            ic_blob_storage_contracts::wire::encode_request(payload.kind().method(), manifest)
        }
        _ => ic_blob_storage_contracts::wire::encode_request(payload.kind().method(), permission),
    }
    .map_err(|_| Failure::Arguments)?;
    Ok(Request { payload, argument })
}
