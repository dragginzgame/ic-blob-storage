//! Dynamic transient journey over the shared catalog, with substituted completion facts.
use super::{bound, number};
pub(crate) mod readback;
use crate::model::content::{ContentError, ContentSession, ContentVerdict};
use blob_test_protocol::journey::{
    JourneyFailure, JourneyManifest, JourneyProgress, JourneyUpload, JourneyVerification,
};
use candid::Principal;
use ic_blob_storage::model::{
    catalog::{
        CatalogLimits,
        admission::{
            UploadAdmission, UploadCatalog, UploadError, UploadLimits, UploadObject, UploadRequest,
            UploadRequestId,
        },
    },
    identity::{
        ContentDigest, ProviderRootHash,
        caffeine::{
            CaffeineHeader,
            manifest::{CaffeineChunkHash, CaffeineChunkManifest, CaffeineManifestLimits},
        },
    },
    lifecycle::{
        LifecycleChange, ReferenceId,
        binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        requests::{
            ReferenceOperation, ReferenceRequest, ReferenceRequestId, ReferenceRequestOutcome,
        },
    },
};

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
    pub(super) request: UploadRequest,
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
                max_tenant_logical_bytes: number(6 * 1024 * 1024),
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
) -> Result<UploadRequest, JourneyFailure> {
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
    Ok(UploadRequest {
        id: UploadRequestId::new(number(u128::from(input.id))),
        object: UploadObject {
            root: ProviderRootHash::try_from(input.root.as_slice())
                .map_err(|_| JourneyFailure::InvalidInput)?,
            bytes: input.bytes,
            first: ReferenceKey::new(object, ReferenceId::new(number(1))),
        },
        content: ContentDigest::try_from(input.digest.as_slice())
            .map_err(|_| JourneyFailure::InvalidInput)?,
    })
}

pub(crate) fn lookup(root: ProviderRootHash) -> Result<UploadRequest, JourneyFailure> {
    super::read(|state| {
        state
            .journey
            .requests
            .iter()
            .find(|r| r.request.object.root == root)
            .map(|r| r.request)
            .ok_or(JourneyFailure::Unknown)
    })
}

pub(crate) fn reserve(
    actor: Principal,
    request: UploadRequest,
    input: &JourneyManifest,
) -> Result<(), JourneyFailure> {
    // Bound conversion work before allocating intermediate boundary values.
    if input.chunks.len() > 6 || input.headers.len() > 8 {
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
        request.object.root,
        request.object.bytes,
        &chunks,
        &headers,
        CaffeineManifestLimits {
            max_content_bytes: std::num::NonZeroU64::new(6 * 1024 * 1024).expect("bound"),
            max_chunks: bound(6),
            max_headers: bound(8),
            max_header_bytes: bound(1024),
        },
    )
    .map_err(|_| JourneyFailure::InvalidInput)?;
    mutate(|state| {
        if state
            .requests
            .iter()
            .any(|entry| entry.request == request && entry.content.manifest() != &manifest)
        {
            return Err(JourneyFailure::Conflict);
        }
        let result = state.catalog.reserve(actor, request).map_err(error)?;
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

pub(crate) fn expose(actor: Principal, request: UploadRequest) -> Result<(), JourneyFailure> {
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
            .mark_exposure_possible(actor, request)
            .map_err(error)?
        {
            LifecycleChange::Changed => Ok(()),
            LifecycleChange::Unchanged => Err(JourneyFailure::InvalidPhase),
        }
    })
}

pub(crate) fn append(
    request: UploadRequest,
    index: u64,
    bytes: &[u8],
) -> Result<(), JourneyFailure> {
    mutate(|state| {
        use ic_blob_storage::model::catalog::admission::UploadPhase;
        if state
            .catalog
            .phase(request.object.first.object().tenant(), request)
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

pub(crate) fn progress(request: UploadRequest) -> Result<JourneyProgress, JourneyFailure> {
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

pub(crate) fn cancel(actor: Principal, request: UploadRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        state
            .catalog
            .cancel(actor, request)
            .map(|_| ())
            .map_err(error)
    })
}

pub(crate) fn complete(request: UploadRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        state
            .catalog
            .confirm_upload(request)
            .map(|_| ())
            .map_err(error)
    })
}

pub(crate) fn release(actor: Principal, request: UploadRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        match state.catalog.apply_reference(
            request.object.root,
            actor,
            ReferenceRequest {
                id: ReferenceRequestId::new(number(1)),
                operation: ReferenceOperation::Release(request.object.first),
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

pub(crate) fn deleted(request: UploadRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        state
            .catalog
            .confirm_provider_deleted(request.object.root, request.object.first.object())
            .map(|_| ())
            .map_err(|_| JourneyFailure::InvalidPhase)
    })
}

pub(crate) fn settled(request: UploadRequest) -> Result<(), JourneyFailure> {
    mutate(|state| {
        state
            .catalog
            .confirm_billing_stopped(request.object.root, request.object.first.object())
            .map(|_| ())
            .map_err(|_| JourneyFailure::InvalidPhase)
    })
}
