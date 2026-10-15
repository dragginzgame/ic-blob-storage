//! Bounded declarations for direct uploads; no file bytes or streaming hash state.

use crate::model::lifecycle::LifecycleChange;
use crate::model::service::upload::UploadAdmissionError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadRequest;

use super::{UploadAdmissions, key};
use crate::model::service::upload::manifest::UploadManifest;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkManifest;

/// One retained declaration per lifetime permission, never a byte buffer.
#[derive(Debug)]
pub(super) struct PreparedManifest {
    pub(super) identity: CaffeineChunkManifest,
    pub(super) headers: Vec<crate::model::service::upload::download::ContentHeader>,
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
        let manifest =
            crate::model::service::upload::manifest::validate(&self.config, request, input)?;
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
            .map(
                |header| crate::model::service::upload::download::ContentHeader {
                    name: header.name.to_owned(),
                    value: header.value.to_owned(),
                },
            )
            .collect();
        permission.manifest = Some(PreparedManifest {
            identity: manifest,
            headers,
        });
        Ok(LifecycleChange::Changed)
    }
}
