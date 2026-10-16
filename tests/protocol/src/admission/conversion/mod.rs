//! Checked fixture projection; it neither allocates identities nor grants authority.

use super::{ContentObservation, ContentState, Request};
use ic_blob_storage::model::service::upload::content::TenantContentView;
use ic_blob_storage_contracts::upload::history::{LifecyclePhase, UploadRootState};
use ic_blob_storage_contracts::{
    binding::{ObjectBinding, ObjectIdentity, ReferenceId, ReferenceKey},
    identity::ProviderRootHash,
    upload::binding::{UploadObject, UploadRequest, UploadRequestId},
};
use std::num::NonZeroU128;

/// Project the fixture's exact upload identity, with incarnation/reference one.
///
/// Returns `None` for a zero operation/namespace or invalid object binding.
/// Callers retain their failure category and check actual service/tenant authority.
#[must_use]
pub fn upload_request(input: Request) -> Option<UploadRequest> {
    let id = NonZeroU128::new(input.id)?;
    let namespace = NonZeroU128::new(input.namespace)?;
    let binding = ObjectBinding::new(
        input.service,
        input.tenant,
        ObjectIdentity {
            namespace,
            object: id,
            incarnation: NonZeroU128::MIN,
        },
    )
    .ok()?;
    Some(UploadRequest {
        id: UploadRequestId::new(id),
        object: UploadObject {
            root: ProviderRootHash::try_from(input.root.as_slice()).ok()?,
            bytes: input.bytes,
            first: ReferenceKey::new(binding, ReferenceId::new(NonZeroU128::MIN)),
        },
    })
}

/// Project the original identity and observed lifecycle without granting authority.
#[must_use]
pub fn observation(view: TenantContentView) -> ContentObservation {
    let request = view.request;
    ContentObservation {
        request: Request {
            service: request.object.first.object().service(),
            tenant: request.object.first.object().tenant(),
            namespace: request.object.first.object().identity().namespace.get(),
            id: request.id.get().get(),
            root: *request.object.root.as_bytes(),
            bytes: request.object.bytes,
        },
        state: content_state(view.state),
    }
}
/// Preserve every retained state, including deletion and continuing billing work.
#[must_use]
pub fn content_state(state: UploadRootState) -> ContentState {
    match state {
        UploadRootState::Reserved => ContentState::Reserved,
        UploadRootState::ExposurePossible => ContentState::ExposurePossible,
        UploadRootState::Cancelled => ContentState::Cancelled,
        UploadRootState::Confirmed(phase) => match phase {
            LifecyclePhase::Live => ContentState::Live,
            LifecyclePhase::DeletionPending => ContentState::DeletionPending,
            LifecyclePhase::ProviderDeleted => ContentState::ProviderDeleted,
            LifecyclePhase::Settled => ContentState::Settled,
        },
    }
}
