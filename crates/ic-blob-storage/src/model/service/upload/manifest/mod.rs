//! Bounded declarations for direct uploads; no file bytes or streaming hash state.
mod metadata;
pub use metadata::UploadMetadataError;

use super::{
    LifecycleChange, UploadAdmissionError, UploadAdmissions, UploadContext, UploadRequest, key,
};
use crate::model::identity::caffeine::{
    CaffeineHeader,
    manifest::{CaffeineChunkHash, CaffeineChunkManifest},
};

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
        let limits = self.config.manifest_limits();
        metadata::validate(
            input.headers,
            request.object.bytes,
            limits.max_headers.get(),
            limits.max_header_bytes.get(),
        )?;
        let manifest = CaffeineChunkManifest::new(
            request.object.root,
            request.object.bytes,
            input.chunks,
            input.headers,
            limits,
        )?;
        let permission = self
            .permissions
            .get_mut(&key(request))
            .ok_or(UploadAdmissionError::UnknownPermission)?;
        if let Some(existing) = &permission.manifest {
            if existing != &manifest {
                return Err(UploadAdmissionError::PermissionConflict);
            }
            return Ok(LifecycleChange::Unchanged);
        }
        permission.manifest = Some(manifest);
        Ok(LifecycleChange::Changed)
    }
}
