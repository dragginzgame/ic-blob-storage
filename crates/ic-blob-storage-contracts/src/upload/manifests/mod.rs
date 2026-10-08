//! Bounded original declarations and exact authenticated replies.
pub mod reply;
use crate::dto::upload::manifest::UploadManifestDeclaration;
use crate::dto::upload::manifest::UploadManifestFailure;
use crate::identity::caffeine::CaffeineHeader;
use crate::identity::caffeine::manifest::CaffeineManifestLimits;
/// Check chunk count, header count and total header bytes against explicit limits.
/// # Errors
/// Rejects declarations that exceed any supplied count or byte limit.
pub fn bounds(
    declaration: &UploadManifestDeclaration,
    limits: CaffeineManifestLimits,
) -> Result<(), UploadManifestFailure> {
    if declaration.chunks.len() > limits.max_chunks.get()
        || declaration.headers.len() > limits.max_headers.get()
    {
        return Err(UploadManifestFailure::Limit);
    }
    let mut remaining = limits.max_header_bytes.get();
    for h in &declaration.headers {
        for len in [h.name.len(), h.value.len(), 3] {
            remaining = remaining
                .checked_sub(len)
                .ok_or(UploadManifestFailure::Limit)?;
        }
    }
    Ok(())
}
/// Borrow original header strings for content hashing, preserving order and duplicates.
/// This conversion performs no validation; check declaration bounds separately.
#[must_use]
pub fn headers(declaration: &UploadManifestDeclaration) -> Vec<CaffeineHeader<'_>> {
    declaration
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect()
}
