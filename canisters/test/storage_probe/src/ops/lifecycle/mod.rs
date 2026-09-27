//! Labelled provider substitutes and reference boundary conversion for IC evidence.
use super::{STATE, TRAP_WRITE, conversion};
use blob_test_protocol::{
    admission::input::ReferenceInput,
    storage::{
        FactInput, Failure, ProviderFact, ReferenceMutationInput, ReferenceOutcome, ReferenceResult,
    },
};
use ic_blob_storage::model::{
    lifecycle::{
        LifecycleChange, LifecycleError, ReferenceId,
        binding::ReferenceKey,
        requests::{
            ReferenceOperation, ReferenceRequest, ReferenceRequestId, ReferenceRequestOutcome,
        },
    },
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
fn reference(input: ReferenceInput) -> Result<ReferenceRequest, Failure> {
    let object = conversion::request(input.object)?.object.first.object();
    let key = ReferenceKey::new(
        object,
        ReferenceId::new(NonZeroU128::new(input.reference).ok_or(Failure::Invalid)?),
    );
    Ok(ReferenceRequest {
        id: ReferenceRequestId::new(NonZeroU128::new(input.operation).ok_or(Failure::Invalid)?),
        operation: if input.retain {
            ReferenceOperation::Retain(key)
        } else {
            ReferenceOperation::Release(key)
        },
    })
}
fn result(value: Result<LifecycleChange, LifecycleError>) -> ReferenceResult {
    match value {
        Ok(LifecycleChange::Changed) => ReferenceResult::Changed,
        Ok(LifecycleChange::Unchanged) => ReferenceResult::Unchanged,
        Err(LifecycleError::UnknownReference) => ReferenceResult::UnknownReference,
        Err(LifecycleError::ReferenceReleased) => ReferenceResult::Released,
        Err(LifecycleError::ReferenceLimitReached) => ReferenceResult::Limit,
        Err(LifecycleError::DeletionAlreadyQueued) => ReferenceResult::DeletionQueued,
        Err(_) => unreachable!("reference binding is checked before recording"),
    }
}
pub(crate) fn apply(
    context: UploadContext,
    input: ReferenceMutationInput,
) -> Result<ReferenceOutcome, Failure> {
    let upload = conversion::request(input.request.object)?;
    let request = reference(input.request)?;
    TRAP_WRITE.set(input.fault);
    let value = STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .uploads
            .apply_reference(context, upload, request)
    });
    TRAP_WRITE.set(None);
    value
        .map(|value| match value {
            ReferenceRequestOutcome::Recorded { result: r } => ReferenceOutcome {
                replayed: false,
                result: result(r),
            },
            ReferenceRequestOutcome::Replayed { result: r } => ReferenceOutcome {
                replayed: true,
                result: result(r),
            },
        })
        .map_err(conversion::failure)
}
pub(crate) fn receipt(
    context: UploadContext,
    input: ReferenceInput,
) -> Result<Option<ReferenceResult>, Failure> {
    let upload = conversion::request(input.object)?;
    let request = reference(input)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .reference_receipt(context, upload, request)
        })
        .map(|r| r.map(|v| result(v.result)))
        .map_err(conversion::failure)
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
