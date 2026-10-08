//! Shared indexed tenant content discovery.
use crate::ops::service::uploads::discovery;
use crate::ops::service::uploads::discovery::UploadDiscoverySource;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
/// Discover complete original identities and current local state under one owner borrow.
/// Suspension and restoration preserve passive inspection. Unknown and foreign roots
/// share absence; neither absence nor confirmed history authorizes upload or serving.
/// # Errors
/// Rejects invalid/foreign scope, unrelated callers or inconsistent retained state.
pub fn inspect<S: UploadDiscoverySource>(
    source: &S,
    context: UploadContext,
    request: UploadDiscoveryRequest,
) -> Result<UploadDiscoveryResponse, UploadDiscoveryFailure> {
    discovery::inspect(source, context, request)
}
