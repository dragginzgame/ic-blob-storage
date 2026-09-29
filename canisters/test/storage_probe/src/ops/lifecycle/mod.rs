//! Labelled provider substitutes and reference boundary conversion for IC evidence.
use super::{STATE, TRAP_WRITE, conversion};
use blob_test_protocol::storage::{FactInput, Failure, ProviderFact};
use ic_blob_storage::model::{lifecycle::LifecycleChange, service::upload::UploadContext};
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
