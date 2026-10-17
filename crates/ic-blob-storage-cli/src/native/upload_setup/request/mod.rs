//! Shared pure validators own permission/metadata/tree invariants before dispatch.
use super::{
    Failure, Input, Kind, Payload, Request, admission_error, manifest_error, manifest_reply_limits,
};
use candid::Principal;
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
    let payload = if input.kind == Kind::Prepare {
        Payload::Manifest(crate::native::exact_wire::decode(&bytes, 65536)?)
    } else {
        Payload::Permission(crate::native::exact_wire::decode(&bytes, 65536)?)
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
    if let Payload::Manifest(r) = &payload {
        let limits = manifest_reply_limits(
            (1024 * 1024 * 1024)
                .try_into()
                .expect("positive content bound"),
        );
        manifest::validate_declaration(permission, &r.declaration, limits.declaration)
            .map_err(manifest_error)?;
    }
    let argument = match &payload {
        Payload::Manifest(manifest) => ic_blob_storage_contracts::wire::encode(manifest),
        Payload::Permission(permission) => ic_blob_storage_contracts::wire::encode(permission),
    }
    .map_err(|_| Failure::Arguments)?;
    Ok(Request { payload, argument })
}
