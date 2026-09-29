//! Shared indexed tenant content discovery.
use crate::{
    dto::upload::discovery::{
        UploadDiscoveryFailure, UploadDiscoveryRequest, UploadDiscoveryResponse,
    },
    model::service::upload::UploadContext,
    ops::service::uploads::discovery::{self, UploadDiscoverySource},
};
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
