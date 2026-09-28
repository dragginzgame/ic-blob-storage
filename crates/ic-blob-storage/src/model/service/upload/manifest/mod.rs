//! Bounded declarations for direct uploads; no file bytes or streaming hash state.
mod metadata;
pub use metadata::{UploadMetadataError, validate_upload_metadata};

use super::{
    LifecycleChange, UploadAdmissionError, UploadAdmissions, UploadContext, UploadRequest, key,
};
use crate::model::identity::caffeine::{
    CaffeineHeader,
    manifest::{CaffeineChunkHash, CaffeineChunkManifest},
};

/// One retained declaration per lifetime permission, never a byte buffer.
#[derive(Debug)]
pub(super) struct PreparedManifest {
    pub(super) identity: CaffeineChunkManifest,
    pub(super) headers: Vec<super::download::ContentHeader>,
}

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

impl UploadAdmissions {
    /// Bind a manifest for a direct browser-to-provider upload without file bytes.
    /// Exact retries preserve the original declaration and reservation. No provider
    /// completion, verified length or raw-digest claim follows from this operation.
    /// # Errors
    /// Rejects invalid uploader/activation/time/phase, invalid manifest or a changed
    /// declaration. All rejection paths preserve existing state and accounting.
    pub fn prepare_manifest(
        &mut self,
        context: UploadContext,
        request: UploadRequest,
        input: UploadManifest<'_>,
        now_ns: u64,
    ) -> Result<LifecycleChange, UploadAdmissionError> {
        self.check_uploader(context, request, now_ns)?;
        let manifest = validate(&self.config, request, input)?;
        let permission = self
            .permissions
            .get_mut(&key(request))
            .ok_or(UploadAdmissionError::UnknownPermission)?;
        if let Some(existing) = &permission.manifest {
            if existing.identity != manifest {
                return Err(UploadAdmissionError::PermissionConflict);
            }
            return Ok(LifecycleChange::Unchanged);
        }
        // Copy only on the first successful binding. Limits and the root were
        // checked above; reordered retries preserve the first accepted spelling
        // and order without another retained allocation.
        let headers = input
            .headers
            .iter()
            .map(|header| super::download::ContentHeader {
                name: header.name.to_owned(),
                value: header.value.to_owned(),
            })
            .collect();
        permission.manifest = Some(PreparedManifest {
            identity: manifest,
            headers,
        });
        Ok(LifecycleChange::Changed)
    }
}

pub(crate) fn validate(
    config: &super::ServiceConfiguration,
    request: UploadRequest,
    input: UploadManifest<'_>,
) -> Result<CaffeineChunkManifest, UploadAdmissionError> {
    let limits = config.manifest_limits();
    validate_upload_metadata(
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
