//! Direct-to-client serving metadata, with no canister bulk read or provider call.
use crate::{
    model::service::{read::download::CaffeineDownloadScope, upload::UploadContext},
    ops::service::uploads::StableUploads,
};
use ic_memory::ic_stable_structures::Memory;

/// Shared canonical endpoint handler. The adapter must authenticate the actual
/// caller/service and supply its installed serving scope; linking exports nothing.
/// Bound ingress decoding separately. This synchronous call performs no provider effect.
/// Only an active tenant's exact live reference on an unfenced owner is served;
/// historical descriptor inspection has a separate contract. Installed owner and
/// namespace must match the service, independently of payer or tenant identity.
/// Copies can become stale immediately. Hosts own authenticated delivery, approved
/// origin/serving policy and publication/release coordination; clients build HTTP targets.
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
    let view = uploads
        .download_descriptor(context, scope, root, reference)
        .map_err(download::failure)?;
    Ok(download::present(request, scope, view))
}
