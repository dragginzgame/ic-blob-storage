//! Actual uploader authority, bounded declaration conversion and immutable recovery.
pub mod client;
pub mod reply;
use super::{StableUploads, UploadStoreError, admission, key};
use crate::{
    dto::upload::{
        admission::{UploadAdmissionFailure, UploadAdmissionRequest},
        manifest::{
            UploadManifestDeclaration, UploadManifestFailure, UploadManifestHeader,
            UploadManifestInspection, UploadManifestMutation, UploadManifestRequest,
            UploadManifestResponse,
        },
    },
    model::{
        identity::caffeine::{
            CaffeineHeader,
            manifest::{CaffeineChunkHash, CaffeineManifestLimits},
        },
        lifecycle::LifecycleChange,
        service::{
            tenant::TenantError,
            upload::{
                UploadAdmissionError, UploadContext, UploadManifestState, UploadPermission,
                manifest::UploadManifest,
            },
        },
    },
};
use ic_memory::ic_stable_structures::Memory;
/// Canonical preparation update. Linking exports no endpoint.
pub const UPLOAD_MANIFEST_PREPARE_METHOD: &str = "blob_prepare_upload";
/// Canonical exact permission manifest query, including after fencing/revocation.
pub const UPLOAD_MANIFEST_INSPECT_METHOD: &str = "blob_upload_manifest";
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
    admission::parse_binding(context.service, input).map_err(UploadManifestFailure::Permission)
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
pub(crate) fn bounds(
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
pub(crate) fn headers(declaration: &UploadManifestDeclaration) -> Vec<CaffeineHeader<'_>> {
    declaration
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect()
}
pub(crate) fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    let permission = parse(context, input, false)?;
    let view = exact(store, context, permission)?;
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
