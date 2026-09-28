//! Direct-to-client serving metadata, with no canister bulk read or provider call.
use crate::{
    model::{
        identity::ProviderRootHash,
        lifecycle::binding::ReferenceKey,
        service::{read::download::CaffeineDownloadScope, upload::UploadContext},
    },
    ops::service::uploads::{
        StableUploads,
        read::download::{CaffeineDownloadDescriptorView, DownloadDescriptorError},
    },
};
use ic_memory::ic_stable_structures::Memory;

/// Shared canonical endpoint handler. The adapter must authenticate the actual
/// caller/service and supply its installed serving scope; linking exports nothing.
/// Bound ingress decoding separately. This synchronous call performs no provider effect.
/// # Errors
/// Returns the maintained boundary refusal without exposing internal record errors.
pub fn handle<M: Memory>(
    uploads: &StableUploads<M>,
    context: UploadContext,
    scope: &CaffeineDownloadScope,
    request: crate::dto::download::DownloadRequest,
) -> Result<crate::dto::download::DownloadResponse, crate::dto::download::DownloadFailure> {
    use crate::ops::service::reads::download;
    let (root, reference) = download::parse(context, request)?;
    let view = describe(uploads, context, scope, root, reference).map_err(download::failure)?;
    Ok(download::present(request, view))
}

/// Observe an operational descriptor for the authenticated tenant's exact live
/// reference. Scope must be trusted host configuration, never an ingress choice.
/// Current storage owner must equal the service; payer and tenant cannot replace it.
/// No body, session slot, effect, reference or receipt is allocated. Copies can
/// become stale immediately; hosts own authenticated delivery, provisioned project
/// mapping, approved origin, serving policy and publication/release coordination.
/// HTTP origin membership is not inferred from the IC gateway-principal registry.
/// # Errors
/// Rejects owner/namespace mismatch, inactive/foreign tenants, unavailable content
/// and the restore fence. Existing passive inspection is unaffected.
pub fn describe<M: Memory>(
    uploads: &StableUploads<M>,
    context: UploadContext,
    scope: &CaffeineDownloadScope,
    root: ProviderRootHash,
    reference: ReferenceKey,
) -> Result<CaffeineDownloadDescriptorView, DownloadDescriptorError> {
    uploads.download_descriptor(context, scope, root, reference)
}
