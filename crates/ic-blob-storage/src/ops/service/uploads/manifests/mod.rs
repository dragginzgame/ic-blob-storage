//! Retained declaration projection from the stable owner.
use super::{StableUploads, key};
use crate::model::service::upload::UploadManifestState;
use ic_blob_storage_contracts::dto::upload::{
    admission::{UploadAdmissionFailure, UploadAdmissionRequest},
    manifest::{
        UploadManifestDeclaration, UploadManifestFailure, UploadManifestHeader,
        UploadManifestInspection, UploadManifestResponse,
    },
};
use ic_memory::Memory;
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
            .manifests
            .get(&key(permission.request))
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
