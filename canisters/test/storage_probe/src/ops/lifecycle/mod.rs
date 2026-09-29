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

// Explicit test substitute: installation operator is the configured verifier in this fixture.
fn completion_authority(
    operator: candid::Principal,
) -> ic_blob_storage::model::service::upload::completion::CompletionAuthority {
    ic_blob_storage::model::service::upload::completion::CompletionAuthority::new(
        ic_cdk::api::canister_self(),
        std::num::NonZeroU128::MIN,
        operator,
    )
    .unwrap()
}
pub(crate) fn verification_plan(
    context: UploadContext,
    input: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadVerificationPlan,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    STATE.with_borrow(|state| {
        let state = state.as_ref().unwrap();
        ic_blob_storage::workflow::uploads::completion::verification_plan(
            &state.uploads,
            completion_authority(state.operator),
            &super::read::download::scope(context),
            context,
            input,
        )
    })
}
pub(crate) fn attest(
    context: UploadContext,
    input: &ic_blob_storage::dto::upload::completion::UploadAttestationRequest,
    fault: Option<blob_test_protocol::storage::WriteFault>,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadAttestationMutation,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().unwrap();
        let authority = completion_authority(state.operator);
        TRAP_WRITE.set(fault);
        let result = ic_blob_storage::workflow::uploads::completion::attest(
            &mut state.uploads,
            authority,
            context,
            input,
            ic_cdk::api::time(),
        );
        TRAP_WRITE.set(None);
        result
    })
}
pub(crate) fn attestation(
    context: UploadContext,
    input: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
) -> Result<
    ic_blob_storage::dto::upload::completion::UploadAttestationResponse,
    ic_blob_storage::dto::upload::completion::UploadAttestationFailure,
> {
    STATE.with_borrow(|state| {
        let state = state.as_ref().unwrap();
        ic_blob_storage::workflow::uploads::completion::inspect(
            &state.uploads,
            completion_authority(state.operator),
            context,
            input,
        )
    })
}
