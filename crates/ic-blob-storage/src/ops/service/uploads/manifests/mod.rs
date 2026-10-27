//! Retained declaration projection from the stable owner.
use super::{Memory, PreparedUploadManifest, StableUploads, UploadRequest, UploadStoreError};
use crate::model::service::upload::UploadManifestState;
use ic_blob_storage_contracts::dto::upload::{
    admission::{UploadAdmissionFailure, UploadAdmissionRequest},
    manifest::{
        UploadManifestDeclaration, UploadManifestFailure, UploadManifestHeader,
        UploadManifestInspection, UploadManifestResponse,
    },
};
pub(crate) fn observation<M: Memory>(
    store: &StableUploads<M>,
    input: UploadAdmissionRequest,
    view: &crate::model::service::upload::UploadPermissionView,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    let permission = view.permission;
    let manifest = if view.manifest == UploadManifestState::Unprepared {
        UploadManifestInspection::Unprepared
    } else {
        let retained = store
            .manifest(permission.request)
            .map_err(|_| UploadManifestFailure::Permission(UploadAdmissionFailure::Internal))?
            .ok_or(UploadManifestFailure::Permission(
                UploadAdmissionFailure::Internal,
            ))?
            .into_view();
        UploadManifestInspection::Prepared(UploadManifestDeclaration {
            chunks: retained.chunks,
            headers: retained
                .headers
                .into_iter()
                .map(|h| UploadManifestHeader {
                    name: h.name,
                    value: h.value,
                })
                .collect(),
        })
    };
    Ok(UploadManifestResponse {
        permission: input,
        manifest,
    })
}

impl<M: Memory> StableUploads<M> {
    // Both parts are committed before binding the permission, in the same IC message.
    // No native-memory transaction guarantee is implied; stable-write traps propagate.
    pub(super) fn store_manifest(
        &mut self,
        request: UploadRequest,
        manifest: PreparedUploadManifest,
    ) {
        for (part, record) in [0_u8, 1].into_iter().zip(manifest.split()) {
            self.manifests.insert((super::key(request), part), record);
        }
    }
    pub(super) fn manifest(
        &self,
        request: UploadRequest,
    ) -> Result<Option<PreparedUploadManifest>, UploadStoreError> {
        match (
            self.manifests.get(&(super::key(request), 0)),
            self.manifests.get(&(super::key(request), 1)),
        ) {
            (None, None) => Ok(None),
            (Some(chunks), Some(headers)) => PreparedUploadManifest::join(chunks, headers)
                .map(Some)
                .ok_or(UploadStoreError::InvalidRecord),
            _ => Err(UploadStoreError::InvalidRecord),
        }
    }
}
