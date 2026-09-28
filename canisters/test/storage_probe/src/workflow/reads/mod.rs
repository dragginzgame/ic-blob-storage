//! Authority-only fixture across an actual local IC await; not a blob read API.
use crate::ops::{gateways::callbacks, read::authority};
use blob_test_protocol::storage::{Failure, gateways::ReadAuthorityInput};
use ic_blob_storage::{
    model::service::upload::UploadContext,
    workflow::reads::{ReadAuthorityError, capture, recheck},
};
fn failure(error: ReadAuthorityError) -> Failure {
    match error {
        ReadAuthorityError::Registry(e) => callbacks::registry_failure(e),
        ReadAuthorityError::Uploads(e) => callbacks::upload_failure(e),
        ReadAuthorityError::Gateway(e) => callbacks::access_failure(e),
        ReadAuthorityError::Unavailable => Failure::Unknown,
        ReadAuthorityError::Stale => Failure::Conflict,
        ReadAuthorityError::Exhausted => Failure::Capacity,
    }
}
pub(crate) async fn run(context: UploadContext, input: ReadAuthorityInput) -> Result<(), Failure> {
    let (scope, target) = authority::parse(context, input)?;
    let original = callbacks::with_owners(|registry, uploads| {
        capture(registry, uploads, context, scope, target)
    })
    .map_err(failure)?;
    authority::enter()?;
    let waited = authority::wait_source(scope).await;
    authority::leave();
    waited?;
    callbacks::with_owners(|registry, uploads| recheck(registry, uploads, context, &original))
        .map_err(failure)
}
