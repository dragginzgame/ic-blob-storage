//! Labelled provider substitutes and reference boundary conversion for IC evidence.
use super::{STATE, TRAP_WRITE, conversion};
use blob_test_protocol::{
    admission::input::ReferenceInput,
    storage::{FactInput, Failure, ProviderFact},
};
use ic_blob_storage::model::{
    lifecycle::{LifecycleChange, ReferenceId, binding::ReferenceKey},
    service::upload::UploadContext,
};
use std::num::NonZeroU128;
pub(crate) fn fact(context: UploadContext, input: FactInput) -> Result<bool, Failure> {
    let request = conversion::request(input.request)?;
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().unwrap();
        if context.actor != state.operator {
            return Err(Failure::Denied);
        }
        TRAP_WRITE.set(input.fault);
        let result = match input.fact {
            ProviderFact::Uploaded => state.uploads.confirm_upload(request),
            ProviderFact::Deleted => state.uploads.confirm_provider_deleted(request),
            ProviderFact::Settled => state.uploads.confirm_billing_stopped(request),
        };
        TRAP_WRITE.set(None);
        result
            .map(|v| v == LifecycleChange::Changed)
            .map_err(conversion::failure)
    })
}
pub(crate) fn live(context: UploadContext, input: ReferenceInput) -> Result<bool, Failure> {
    let upload = conversion::request(input.object)?;
    let key = ReferenceKey::new(
        upload.object.first.object(),
        ReferenceId::new(NonZeroU128::new(input.reference).ok_or(Failure::Invalid)?),
    );
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .reference_is_live(context, upload, key)
        })
        .map_err(conversion::failure)
}
