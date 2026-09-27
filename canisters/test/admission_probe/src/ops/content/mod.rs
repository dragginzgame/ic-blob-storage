//! Convert a passive shared-owner observation; no provider or upload effects.

use super::{STATE, conversion::failure};
use blob_test_protocol::admission::{
    ContentLookup, ContentObservation, ContentState, Failure, Request,
};
use ic_blob_storage::model::{
    catalog::admission::read::UploadRootState,
    identity::ProviderRootHash,
    lifecycle::LifecyclePhase,
    service::upload::{UploadContext, content::ContentLookup as ModelLookup},
};
use std::num::NonZeroU128;

pub(crate) fn lookup(
    context: UploadContext,
    input: ContentLookup,
) -> Result<Option<ContentObservation>, Failure> {
    if input.service != context.service {
        return Err(Failure::WrongService);
    }
    let query = ModelLookup {
        tenant: input.tenant,
        namespace: NonZeroU128::new(input.namespace).ok_or(Failure::InvalidInput)?,
        root: ProviderRootHash::try_from(input.root.as_slice()).expect("fixed hash width"),
    };
    STATE.with_borrow(|state| {
        let owner = &state.as_ref().expect("initialized probe").owner;
        Ok(owner
            .lookup_content(context, query)
            .map_err(failure)?
            .map(|view| {
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
            }))
    })
}
