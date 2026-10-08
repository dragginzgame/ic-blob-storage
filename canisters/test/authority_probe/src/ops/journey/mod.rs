//! Dynamic transient journey over the shared catalog, with substituted completion facts.
use super::{bound, number};
use crate::model::content::ContentRequest;
pub(crate) mod readback;
use crate::model::content::{ContentError, ContentSession, ContentVerdict, manifest_limits};
use blob_test_protocol::journey::{
    JourneyFailure, JourneyManifest, JourneyProgress, JourneyUpload, JourneyVerification,
};
use candid::Principal;
use ic_blob_storage::model::catalog::admission::UploadAdmission;
use ic_blob_storage::model::catalog::admission::UploadCatalog;
use ic_blob_storage::model::catalog::admission::UploadError;
use ic_blob_storage::model::lifecycle::LifecycleChange;
use ic_blob_storage::model::lifecycle::requests::ReferenceRequestOutcome;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::configuration::limits::CatalogLimits;
use ic_blob_storage_contracts::configuration::limits::UploadLimits;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkHash;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkManifest;
use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;
use ic_blob_storage_contracts::upload::binding::UploadObject;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_blob_storage_contracts::upload::binding::UploadRequestId;

pub(crate) struct Journey {
    pub catalog: UploadCatalog,
    pub tenants: [Principal; 2],
    pub(super) requests: Vec<VerifiedUpload>,
    pub(super) reads: crate::model::readback::ReadSlot,
    // Explicit operator-controlled fault injection for real callback rollback.
    pub(super) armed_read_trap: Option<ProviderRootHash>,
    pub(super) trap_read_token: Option<u64>,
    pub(super) read_intent: Option<crate::model::archive::ReadRecord>,
}

// Bounded by catalog lifetime slots. Only leaf hashes and streaming hash state
// survive a message, never file bytes or evidence of provider completion.
pub(super) struct VerifiedUpload {
    pub(super) request: ContentRequest,
    pub(super) content: ContentSession,
    pub(super) manifest_input: JourneyManifest,
}

pub(crate) fn initialize(service: Principal, first: Principal, second: Principal) -> Journey {
    Journey {
        catalog: UploadCatalog::new(
            service,
            CatalogLimits {
                max_objects: bound(8),
                max_tenant_objects: bound(4),
                max_physical_bytes: number(12 * 1024 * 1024),
                max_liability_bytes: number(12 * 1024 * 1024),
                max_tenant_logical_bytes: number(u128::from(
                    manifest_limits().max_content_bytes.get(),
                )),
                max_references_per_object: bound(1),
                max_receipts_per_object: bound(1),
            },
            UploadLimits {
                max_active: bound(8),
                max_tenant_active: bound(4),
            },
        )
        .expect("journey bounds"),
        tenants: [first, second],
        requests: Vec::new(),
        reads: crate::model::readback::ReadSlot::new(),
        armed_read_trap: None,
        trap_read_token: None,
        read_intent: None,
    }
}

fn mutate<T>(f: impl FnOnce(&mut Journey) -> T) -> T {
    super::mutate(|state| f(&mut state.journey))
}

fn error(error: UploadError) -> JourneyFailure {
    match error {
        UploadError::Denied => JourneyFailure::Denied,
        UploadError::UnknownRequest => JourneyFailure::Unknown,
        UploadError::RequestConflict | UploadError::Root(_) => JourneyFailure::Conflict,
        UploadError::InvalidPhase(_) => JourneyFailure::InvalidPhase,
        UploadError::ActiveLimit | UploadError::TenantActiveLimit | UploadError::Catalog(_) => {
            JourneyFailure::Limit
        }
    }
}

pub(crate) fn request(
    service: Principal,
    tenant: Principal,
    input: JourneyUpload,
) -> Result<ContentRequest, JourneyFailure> {
    if input.id == 0 {
        return Err(JourneyFailure::InvalidInput);
    }
    let object = ObjectBinding::new(
        service,
        tenant,
        ObjectIdentity {
            namespace: number(1),
            object: number(u128::from(input.id)),
            incarnation: number(1),
        },
    )
    .map_err(|_| JourneyFailure::InvalidInput)?;
    Ok(ContentRequest {
        upload: UploadRequest {
            id: UploadRequestId::new(number(u128::from(input.id))),
            object: UploadObject {
                root: ProviderRootHash::try_from(input.root.as_slice())
                    .map_err(|_| JourneyFailure::InvalidInput)?,
                bytes: input.bytes,
                first: ReferenceKey::new(object, ReferenceId::new(number(1))),
            },
        },
        content: ContentDigest::try_from(input.digest.as_slice())
            .map_err(|_| JourneyFailure::InvalidInput)?,
    })
}

pub(crate) fn lookup(root: ProviderRootHash) -> Result<ContentRequest, JourneyFailure> {
    super::read(|state| {
        state
            .journey
            .requests
            .iter()
            .find(|r| r.request.upload.object.root == root)
            .map(|r| r.request)
            .ok_or(JourneyFailure::Unknown)
    })
}

pub(super) fn manifest(
    request: ContentRequest,
    input: &JourneyManifest,
) -> Result<CaffeineChunkManifest, JourneyFailure> {
    // Bound conversion work before allocating intermediate boundary values.
    let limits = manifest_limits();
    if input.chunks.len() > limits.max_chunks.get()
        || input.headers.len() > limits.max_headers.get()
    {
        return Err(JourneyFailure::InvalidInput);
    }
    let chunks: Vec<_> = input
        .chunks
        .iter()
        .map(|bytes| CaffeineChunkHash::try_from(bytes.as_slice()).expect("fixed hash length"))
        .collect();
    let headers: Vec<_> = input
        .headers
        .iter()
        .map(|(name, value)| CaffeineHeader { name, value })
        .collect();
    let manifest = CaffeineChunkManifest::new(
        request.upload.object.root,
        request.upload.object.bytes,
        &chunks,
        &headers,
        limits,
    )
    .map_err(|_| JourneyFailure::InvalidInput)?;
    Ok(manifest)
}

pub(crate) fn reserve(
    actor: Principal,
    request: ContentRequest,
    input: &JourneyManifest,
) -> Result<(), JourneyFailure> {
    let manifest = manifest(request, input)?;
    mutate(|state| {
        check_content_request(state, request)?;
        if state
            .requests
            .iter()
            .any(|entry| entry.request == request && entry.content.manifest() != &manifest)
        {
            return Err(JourneyFailure::Conflict);
        }
        let result = state
            .catalog
            .reserve(actor, request.upload)
            .map_err(error)?;
        if result == UploadAdmission::Reserved {
            // Each successful fresh reservation consumed a bounded lifetime slot.
            state.requests.push(VerifiedUpload {
                request,
                content: ContentSession::new(manifest, request.content),
                manifest_input: input.clone(),
            });
        }
        Ok(())
    })
}

pub(crate) fn expose(actor: Principal, request: ContentRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        if !state
            .requests
            .iter()
            .any(|r| r.request == request && r.content.progress().2 == ContentVerdict::Verified)
        {
            return Err(JourneyFailure::InvalidPhase);
        }
        match state
            .catalog
            .mark_exposure_possible(actor, request.upload)
            .map_err(error)?
        {
            LifecycleChange::Changed => Ok(()),
            LifecycleChange::Unchanged => Err(JourneyFailure::InvalidPhase),
        }
    })
}

pub(crate) fn append(
    request: ContentRequest,
    index: u64,
    bytes: &[u8],
) -> Result<(), JourneyFailure> {
    mutate(|state| {
        use ic_blob_storage::model::catalog::admission::UploadPhase;
        if state
            .catalog
            .phase(
                request.upload.object.first.object().tenant(),
                request.upload,
            )
            .map_err(error)?
            != UploadPhase::Reserved
        {
            return Err(JourneyFailure::InvalidPhase);
        }
        let entry = state
            .requests
            .iter_mut()
            .find(|r| r.request == request)
            .ok_or(JourneyFailure::Unknown)?;
        entry
            .content
            .append(index, bytes)
            .map_err(|error| match error {
                ContentError::OutOfOrder => JourneyFailure::OutOfOrder,
                ContentError::Mismatch => JourneyFailure::ContentMismatch,
            })
    })
}

pub(crate) fn progress(request: ContentRequest) -> Result<JourneyProgress, JourneyFailure> {
    super::read(|state| {
        let entry = state
            .journey
            .requests
            .iter()
            .find(|r| r.request == request)
            .ok_or(JourneyFailure::Unknown)?;
        let (next_chunk, verified_bytes, verdict) = entry.content.progress();
        Ok(JourneyProgress {
            next_chunk,
            verified_bytes,
            verification: match verdict {
                ContentVerdict::Pending => JourneyVerification::Pending,
                ContentVerdict::Verified => JourneyVerification::Verified,
                ContentVerdict::Rejected => JourneyVerification::Rejected,
            },
        })
    })
}

pub(crate) fn cancel(actor: Principal, request: ContentRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        check_content_request(state, request)?;
        state
            .catalog
            .cancel(actor, request.upload)
            .map(|_| ())
            .map_err(error)
    })
}

pub(crate) fn complete(request: ContentRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        check_content_request(state, request)?;
        state
            .catalog
            .confirm_upload(request.upload)
            .map(|_| ())
            .map_err(error)
    })
}

pub(crate) fn release(actor: Principal, request: ContentRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        match state.catalog.apply_reference(
            request.upload.object.root,
            actor,
            ReferenceRequest {
                id: ReferenceRequestId::new(number(1)),
                operation: ReferenceOperation::Release(request.upload.object.first),
            },
        ) {
            Ok(
                ReferenceRequestOutcome::Recorded { result: Ok(_) }
                | ReferenceRequestOutcome::Replayed { result: Ok(_) },
            ) => Ok(()),
            _ => Err(JourneyFailure::InvalidPhase),
        }
    })
}

pub(crate) fn deleted(request: ContentRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        state
            .catalog
            .confirm_provider_deleted(
                request.upload.object.root,
                request.upload.object.first.object(),
            )
            .map(|_| ())
            .map_err(|_| JourneyFailure::InvalidPhase)
    })
}

pub(crate) fn settled(request: ContentRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        state
            .catalog
            .confirm_billing_stopped(
                request.upload.object.root,
                request.upload.object.first.object(),
            )
            .map(|_| ())
            .map_err(|_| JourneyFailure::InvalidPhase)
    })
}

// The integrity experiment binds its independent raw digest outside the catalog.
fn check_content_request(state: &Journey, request: ContentRequest) -> Result<(), JourneyFailure> {
    if state
        .requests
        .iter()
        .any(|entry| entry.request.upload == request.upload && entry.request != request)
    {
        return Err(JourneyFailure::Conflict);
    }
    Ok(())
}
