//! Bounded fixture read conversion and delegation to the durable owner.
pub(crate) mod authority;
pub(crate) mod download;
pub(crate) mod sessions;
pub(crate) mod transport;
use super::{STATE, conversion};
use blob_test_protocol::{
    admission::{
        ContentDescriptor, ContentLookup, ContentObservation, ContentState, Request,
        input::{RetainedDescriptor, RetainedDescriptorInput},
    },
    storage::Failure,
};
use ic_blob_storage::model::service::upload::content::ContentLookup as Lookup;
use ic_blob_storage::model::service::upload::content::TenantContentView;
use ic_blob_storage::ops::service::uploads::read::UploadDescriptorView;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_blob_storage_contracts::upload::history::UploadRootState;
use std::num::NonZeroU128;
fn number(value: u128) -> Result<NonZeroU128, Failure> {
    NonZeroU128::new(value).ok_or(Failure::Invalid)
}
pub(super) fn lookup(execution: UploadContext, input: ContentLookup) -> Result<Lookup, Failure> {
    if input.service != execution.service {
        return Err(Failure::Binding);
    }
    Ok(Lookup {
        tenant: input.tenant,
        namespace: number(input.namespace)?,
        root: ProviderRootHash::try_from(input.root.as_slice()).expect("fixed root"),
    })
}
pub(super) fn observation(view: TenantContentView) -> ContentObservation {
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
pub(crate) fn content_state(state: UploadRootState) -> ContentState {
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
pub(super) fn descriptor(view: UploadDescriptorView) -> ContentDescriptor {
    ContentDescriptor {
        content: observation(view.content),
        headers: view
            .headers
            .into_iter()
            .map(|h| (h.name, h.value))
            .collect(),
    }
}
pub(crate) fn declaration(
    execution: UploadContext,
    input: ContentLookup,
) -> Result<Option<ContentDescriptor>, Failure> {
    let input = lookup(execution, input)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .content_descriptor(execution, input)
        })
        .map(|v| v.map(descriptor))
        .map_err(conversion::failure)
}
pub(crate) fn retained(
    execution: UploadContext,
    input: RetainedDescriptorInput,
) -> Result<Option<RetainedDescriptor>, Failure> {
    let (root, reference) = retained_target(execution, input)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .retained_content_descriptor(execution, root, reference)
        })
        .map(|v| {
            v.map(|v| RetainedDescriptor {
                reference: input,
                descriptor: descriptor(v.descriptor),
            })
        })
        .map_err(conversion::failure)
}
pub(super) fn retained_target(
    execution: UploadContext,
    input: RetainedDescriptorInput,
) -> Result<(ProviderRootHash, ReferenceKey), Failure> {
    let content = lookup(execution, input.content)?;
    let object = ObjectBinding::new(
        execution.service,
        content.tenant,
        ObjectIdentity {
            namespace: content.namespace,
            object: number(input.object)?,
            incarnation: number(input.incarnation)?,
        },
    )
    .map_err(|_| Failure::Invalid)?;
    let reference = ReferenceKey::new(object, ReferenceId::new(number(input.reference)?));
    Ok((content.root, reference))
}
