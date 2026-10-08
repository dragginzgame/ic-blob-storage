//! Passive exact reference binding and bounded authenticated reply admission.
pub mod binding;
pub mod reply;
pub mod status;
use crate::binding::ObjectBinding;
use crate::binding::ObjectIdentity;
use crate::binding::ReferenceId;
use crate::binding::ReferenceKey;
use crate::dto::reference::ReferenceAction;
use crate::dto::reference::ReferenceCommand;
use crate::dto::reference::ReferenceFailure;
use crate::dto::reference::ReferenceUpload;
use crate::identity::ProviderRootHash;
use crate::reference::binding::ReferenceOperation;
use crate::reference::binding::ReferenceRequest;
use crate::reference::binding::ReferenceRequestId;
use crate::upload::binding::UploadContext;
use crate::upload::binding::UploadObject;
use crate::upload::binding::UploadRequest;
use crate::upload::binding::UploadRequestId;
use std::num::NonZeroU128;
/// Canonical structural binding; the caller supplies authenticated context.
/// # Errors
/// Rejects malformed or differently bound identities without effects.
pub fn parse(
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
/// Canonical structural binding; the caller supplies authenticated context.
/// # Errors
/// Rejects malformed or differently bound identities without effects.
pub fn parse_upload(
    context: UploadContext,
    upload: ReferenceUpload,
) -> Result<UploadRequest, ReferenceFailure> {
    if upload.service != context.service {
        return Err(ReferenceFailure::Binding);
    }
    if upload.tenant != context.actor {
        return Err(ReferenceFailure::Denied);
    }
    parse_upload_binding(context.service, upload)
}
/// Canonical structural binding; the caller supplies authenticated context.
/// # Errors
/// Rejects malformed or differently bound identities without effects.
/// # Panics
/// The fixed-size wire root must remain 32 bytes; a violated internal width invariant traps.
pub fn parse_upload_binding(
    service: candid::Principal,
    upload: ReferenceUpload,
) -> Result<UploadRequest, ReferenceFailure> {
    if upload.service != service {
        return Err(ReferenceFailure::Binding);
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
