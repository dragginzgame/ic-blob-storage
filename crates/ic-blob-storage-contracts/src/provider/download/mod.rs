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
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for b in value.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(b));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[usize::from(b >> 4)]));
            encoded.push(char::from(HEX[usize::from(b & 15)]));
        }
    }
    encoded
}
