//! Actual uploader authority, bounded declaration conversion and immutable recovery.
pub mod client;
use super::{StableUploads, UploadStoreError, admission, key};
use crate::model::lifecycle::LifecycleChange;
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::UploadManifestState;
use crate::model::service::upload::manifest::UploadManifest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestDeclaration;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestHeader;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestInspection;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestMutation;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkHash;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_blob_storage_contracts::upload::manifests::bounds;
use ic_blob_storage_contracts::upload::manifests::headers;

use ic_memory::ic_stable_structures::Memory;
fn parse(
    context: UploadContext,
    input: UploadAdmissionRequest,
    mutation: bool,
) -> Result<UploadPermission, UploadManifestFailure> {
    let uploader = context.actor == input.uploader;
    let tenant_observer = !mutation && context.actor == input.upload.tenant;
    if !uploader && !tenant_observer {
        return Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Denied,
        ));
    }
    ic_blob_storage_contracts::upload::admission::parse_binding(context.service, input)
        .map_err(UploadManifestFailure::Permission)
}
fn exact<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    permission: UploadPermission,
) -> Result<crate::model::service::upload::UploadPermissionView, UploadManifestFailure> {
    let view = store.lookup(context, permission.request).map_err(failure)?;
    if view.permission != permission {
        return Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Conflict,
        ));
    }
    Ok(view)
}

pub(crate) fn prepare<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    input: &UploadManifestRequest,
    now: u64,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    let permission = parse(context, input.permission, true)?;
    exact(store, context, permission)?;
    bounds(&input.declaration, store.config.manifest_limits())?;
    let chunks = input
        .declaration
        .chunks
        .iter()
        .map(|c| CaffeineChunkHash::try_from(c.as_slice()).expect("fixed leaf"))
        .collect::<Vec<_>>();
    let headers = headers(&input.declaration);
    let change = store
        .prepare_manifest(
            context,
            permission.request,
            UploadManifest {
                chunks: &chunks,
                headers: &headers,
            },
            now,
        )
        .map_err(failure)?;
    Ok(UploadManifestMutation {
        observation: inspect(store, context, input.permission)?,
        changed: change == LifecycleChange::Changed,
    })
}

pub(crate) fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    let permission = parse(context, input, false)?;
    let view = exact(store, context, permission)?;
    observation(store, input, &view)
}
pub(super) fn observation<M: Memory>(
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
fn failure(error: UploadStoreError) -> UploadManifestFailure {
    use UploadAdmissionError as A;
    match error {
        UploadStoreError::Admission(A::NotUploader | A::NotObserver) => {
            UploadManifestFailure::Permission(UploadAdmissionFailure::Denied)
        }
        UploadStoreError::Admission(A::Revoked) => UploadManifestFailure::Revoked,
        UploadStoreError::Admission(A::NotReserved(_) | A::ClockReversed) => {
            UploadManifestFailure::Phase
        }
        UploadStoreError::Admission(A::Tenant(TenantError::StalePermission)) => {
            UploadManifestFailure::Permission(UploadAdmissionFailure::Inactive)
        }
        UploadStoreError::Admission(A::Manifest(_) | A::Metadata(_)) => {
            UploadManifestFailure::Declaration
        }
        other => UploadManifestFailure::Permission(admission::failure(other)),
    }
}
