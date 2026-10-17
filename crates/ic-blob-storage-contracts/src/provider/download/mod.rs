//! Current Caffeine direct-blob request target, without transport or origin selection.
use crate::download::scope::CaffeineDownloadScope;
use crate::identity::ProviderRootHash;

#[cfg(test)]
mod tests;

/// Encode the maintained `/v1/blob/` path and its three explicit query fields.
/// Owner/project/root come from trusted application metadata, never response headers.
/// The host must bind this target to its separately approved HTTPS origin, handle
/// redirect/credential/CORS policy and verify the body before application use.
/// Producing a string grants no tenant authority or provider availability guarantee.
#[must_use]
pub fn request_target(scope: &CaffeineDownloadScope, root: ProviderRootHash) -> String {
    format!(
        "/v1/blob/?blob_hash={}&owner_id={}&project_id={}",
        component(&root.to_string()),
        scope.owner().to_text(),
        component(scope.project())
    )
}
fn component(value: &str) -> String {
    const QUERY_COMPONENT: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');
    percent_encoding::utf8_percent_encode(value, QUERY_COMPONENT).to_string()
}
