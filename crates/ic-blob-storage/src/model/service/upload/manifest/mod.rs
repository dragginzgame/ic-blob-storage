//! Bounded declarations for direct uploads; no file bytes or streaming hash state.

use super::UploadAdmissionError;
use super::UploadRequest;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkHash;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkManifest;

/// Local declaration state. Neither variant establishes stored or verified content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadManifestState {
    /// No manifest bound to this operation.
    Unprepared,
    /// Bounded leaves and metadata match the admitted root and declared length.
    /// The declaration is not proof that the bytes exist or have this length.
    Bound,
}

/// Borrowed bounded declaration. No successful verdict is supplied by the caller.
#[derive(Clone, Copy, Debug)]
pub struct UploadManifest<'a> {
    /// Ordered leaf identities expected to produce the admitted provider root.
    pub chunks: &'a [CaffeineChunkHash],
    /// Canonical Content-Length must match admission; names are unique ignoring
    /// ASCII case and values exclude controls/surrounding whitespace. Consistency
    /// does not establish actual length, MIME safety or provider enforcement.
    pub headers: &'a [CaffeineHeader<'a>],
}
/// Owned original declaration recovered from one bounded immutable record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UploadManifestView {
    pub(crate) chunks: Vec<[u8; 32]>,
    pub(crate) headers: Vec<super::download::ContentHeader>,
}

pub(crate) fn validate(
    config: &super::ServiceConfiguration,
    request: UploadRequest,
    input: UploadManifest<'_>,
) -> Result<CaffeineChunkManifest, UploadAdmissionError> {
    let limits = config.manifest_limits();
    ic_blob_storage_contracts::upload::metadata::validate_upload_metadata(
        input.headers,
        request.object.bytes,
        limits.max_headers.get(),
        limits.max_header_bytes.get(),
    )?;
    Ok(CaffeineChunkManifest::new(
        request.object.root,
        request.object.bytes,
        input.chunks,
        input.headers,
        limits,
    )?)
}
