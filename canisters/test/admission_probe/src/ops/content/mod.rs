//! Convert a passive shared-owner observation; no provider or upload effects.

use super::{STATE, conversion::failure};
use blob_test_protocol::admission::{
    ContentDescriptor, ContentLookup, ContentObservation, ContentState, Failure, Request,
    input::{RetainedDescriptor, RetainedDescriptorInput},
};
use ic_blob_storage::model::service::upload::content::ContentLookup as ModelLookup;
use ic_blob_storage::model::service::upload::content::TenantContentView;
use ic_blob_storage::model::service::upload::download::ContentDescriptorView;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_blob_storage_contracts::upload::history::UploadRootState;
use std::num::NonZeroU128;

pub(crate) fn discover(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest,
) -> Result<
    ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse,
    ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure,
> {
    STATE.with_borrow(|state| {
        ic_blob_storage::workflow::uploads::discovery::inspect(
            &state.as_ref().expect("initialized probe").owner,
            context,
            input,
        )
    })
}

pub(crate) fn descriptor(
    context: UploadContext,
    input: ContentLookup,
) -> Result<Option<ContentDescriptor>, Failure> {
    let query = model_lookup(context, input)?;
    STATE.with_borrow(|state| {
        let owner = &state.as_ref().expect("initialized probe").owner;
        Ok(owner
            .content_descriptor(context, query)
            .map_err(failure)?
            .map(descriptor_view))
    })
}

pub(crate) fn retained_descriptor(
    context: UploadContext,
    input: RetainedDescriptorInput,
) -> Result<Option<RetainedDescriptor>, Failure> {
    let query = model_lookup(context, input.content)?;
    let positive = |n| NonZeroU128::new(n).ok_or(Failure::InvalidInput);
    let object = ObjectBinding::new(
        context.service,
        query.tenant,
        ObjectIdentity {
            namespace: query.namespace,
            object: positive(input.object)?,
            incarnation: positive(input.incarnation)?,
        },
    )
    .map_err(|_| Failure::InvalidInput)?;
    let reference = ReferenceKey::new(object, ReferenceId::new(positive(input.reference)?));
    STATE.with_borrow(|state| {
        let owner = &state.as_ref().expect("initialized probe").owner;
        Ok(owner
            .retained_content_descriptor(context, query.root, reference)
            .map_err(failure)?
            .map(|view| RetainedDescriptor {
                reference: input,
                descriptor: descriptor_view(view.descriptor),
            }))
    })
}

fn descriptor_view(view: ContentDescriptorView<'_>) -> ContentDescriptor {
    ContentDescriptor {
        content: observation(view.content),
        headers: view
            .headers
            .iter()
            .map(|header| (header.name.clone(), header.value.clone()))
            .collect(),
    }
}

fn model_lookup(context: UploadContext, input: ContentLookup) -> Result<ModelLookup, Failure> {
    if input.service != context.service {
        return Err(Failure::WrongService);
    }
    Ok(ModelLookup {
        tenant: input.tenant,
        namespace: NonZeroU128::new(input.namespace).ok_or(Failure::InvalidInput)?,
        root: ProviderRootHash::try_from(input.root.as_slice()).expect("fixed hash width"),
    })
}

fn observation(view: TenantContentView) -> ContentObservation {
    let object = view.request.object.first.object();
    ContentObservation {
        request: Request {
            service: object.service(),
            tenant: object.tenant(),
            namespace: object.identity().namespace.get(),
            id: view.request.id.get().get(),
            root: *view.request.object.root.as_bytes(),
            bytes: view.request.object.bytes,
        },
        state: match view.state {
            UploadRootState::Reserved => ContentState::Reserved,
            UploadRootState::ExposurePossible => ContentState::ExposurePossible,
            UploadRootState::Cancelled => ContentState::Cancelled,
            UploadRootState::Confirmed(phase) => match phase {
                LifecyclePhase::Live => ContentState::Live,
                LifecyclePhase::DeletionPending => ContentState::DeletionPending,
                LifecyclePhase::ProviderDeleted => ContentState::ProviderDeleted,
                LifecyclePhase::Settled => ContentState::Settled,
            },
        },
    }
}
