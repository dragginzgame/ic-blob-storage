//! Explicit descriptor bindings and bounded reply interpretation.
pub mod reply;
pub mod scope;
use crate::binding::ObjectBinding;
use crate::binding::ObjectIdentity;
use crate::binding::ReferenceId;
use crate::binding::ReferenceKey;
use crate::dto::download::DownloadFailure;
use crate::dto::download::DownloadRequest;
use crate::identity::ProviderRootHash;
use crate::upload::binding::UploadContext;
use std::num::NonZeroU128;
/// Canonical structural binding; the caller supplies authenticated context.
/// # Errors
/// Rejects malformed or differently bound identities without effects.
/// # Panics
/// The fixed-size wire root must remain 32 bytes; a violated internal width invariant traps.
pub fn parse(
    context: UploadContext,
    request: DownloadRequest,
) -> Result<(ProviderRootHash, ReferenceKey), DownloadFailure> {
    if request.service != context.service {
        return Err(DownloadFailure::Binding);
    }
    if request.tenant != context.actor {
        return Err(DownloadFailure::Denied);
    }
    let positive = |n| NonZeroU128::new(n).ok_or(DownloadFailure::Invalid);
    let object = ObjectBinding::new(
        request.service,
        request.tenant,
        ObjectIdentity {
            namespace: positive(request.namespace)?,
            object: positive(request.object)?,
            incarnation: positive(request.incarnation)?,
        },
    )
    .map_err(|_| DownloadFailure::Invalid)?;
    Ok((
        ProviderRootHash::try_from(request.root.as_slice()).expect("fixed root"),
        ReferenceKey::new(object, ReferenceId::new(positive(request.reference)?)),
    ))
}
