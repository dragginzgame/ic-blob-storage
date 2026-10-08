//! Explicit provider substitutes for retained-state sizing; no outbound calls.
use super::{
    STATE, changed,
    conversion::{failure, request},
    mutate,
};
use blob_test_protocol::admission::{
    Failure, Outcome,
    input::ReferenceInput,
    release::{LifecycleCommand, ReferenceFailure},
};
use ic_blob_storage::model::catalog::CatalogError;
use ic_blob_storage::model::lifecycle::LifecycleChange;
use ic_blob_storage::model::lifecycle::LifecycleError;
use ic_blob_storage::model::lifecycle::requests::ReferenceRequestError;
use ic_blob_storage::model::lifecycle::requests::ReferenceRequestOutcome;
use ic_blob_storage::model::service::upload::UploadAdmissionError;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use std::num::NonZeroU128;

pub(crate) fn execute(
    context: UploadContext,
    command: LifecycleCommand,
) -> Result<Outcome, Failure> {
    if let LifecycleCommand::Reference {
        object,
        reference,
        operation,
        retain,
    } = command
    {
        let (object, operation) = reference_request(ReferenceInput {
            object,
            reference,
            operation,
            retain,
        })?;
        let result = mutate(|owner| owner.apply_reference(context, object.object.root, operation))
            .map_err(reference_error)?;
        let (replayed, result) = match result {
            ReferenceRequestOutcome::Recorded { result } => (false, result),
            ReferenceRequestOutcome::Replayed { result } => (true, result),
        };
        return Ok(Outcome::Reference {
            replayed,
            result: result
                .map(|change| change == LifecycleChange::Changed)
                .map_err(lifecycle_error),
        });
    }
    // Only the explicitly installed observer may inject local host facts.
    STATE.with_borrow(|state| {
        let config = state.as_ref().expect("initialized probe").config;
        if context.service != config.bindings().service {
            return Err(Failure::WrongService);
        }
        if context.actor != config.bindings().operator {
            return Err(Failure::NotOperator);
        }
        Ok(())
    })?;
    let input = match command {
        LifecycleCommand::SubstituteCompletion(input)
        | LifecycleCommand::SubstituteDeletion(input)
        | LifecycleCommand::SubstituteSettlement(input) => input,
        LifecycleCommand::Reference { .. } => unreachable!("reference handled above"),
    };
    let object = request(input)?;
    if input.service != context.service {
        return Err(Failure::WrongService);
    }
    // Resolve exact retained operation before applying a host fact; malformed fixture
    // payloads must not be converted into a fact about another stored operation.
    mutate(|owner| {
        owner
            .lookup(
                UploadContext {
                    actor: input.tenant,
                    ..context
                },
                object,
            )
            .map_err(failure)?;
        let result = match command {
            LifecycleCommand::SubstituteCompletion(_) => {
                owner.confirm_upload(object).map_err(|_| Failure::Catalog)
            }
            LifecycleCommand::SubstituteDeletion(_) => owner
                .confirm_provider_deleted(object.object.root, object.object.first.object())
                .map_err(|_| Failure::Catalog),
            LifecycleCommand::SubstituteSettlement(_) => owner
                .confirm_billing_stopped(object.object.root, object.object.first.object())
                .map_err(|_| Failure::Catalog),
            LifecycleCommand::Reference { .. } => unreachable!("reference handled above"),
        };
        result.map(changed)
    })
}

fn reference_request(input: ReferenceInput) -> Result<(UploadRequest, ReferenceRequest), Failure> {
    let object = request(input.object)?;
    let reference = ReferenceKey::new(
        object.object.first.object(),
        ReferenceId::new(number(input.reference)?),
    );
    Ok((
        object,
        ReferenceRequest {
            id: ReferenceRequestId::new(number(input.operation)?),
            operation: if input.retain {
                ReferenceOperation::Retain(reference)
            } else {
                ReferenceOperation::Release(reference)
            },
        },
    ))
}

fn number(n: u128) -> Result<NonZeroU128, Failure> {
    NonZeroU128::new(n).ok_or(Failure::InvalidInput)
}

fn reference_error(error: UploadAdmissionError) -> Failure {
    match error {
        UploadAdmissionError::Reference(CatalogError::Request(
            ReferenceRequestError::ReceiptLimitReached,
        )) => Failure::Capacity,
        other => failure(other),
    }
}

const fn lifecycle_error(error: LifecycleError) -> ReferenceFailure {
    match error {
        LifecycleError::BindingMismatch(_) => ReferenceFailure::Binding,
        LifecycleError::UnknownReference => ReferenceFailure::Unknown,
        LifecycleError::ReferenceReleased => ReferenceFailure::Released,
        LifecycleError::ReferenceLimitReached => ReferenceFailure::Capacity,
        LifecycleError::DeletionAlreadyQueued
        | LifecycleError::LiveReferencesRemain
        | LifecycleError::DeletionNotConfirmed => ReferenceFailure::Phase,
    }
}

pub(crate) fn capacity(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest,
) -> Result<
    ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse,
    ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure,
> {
    STATE.with_borrow(|state| {
        ic_blob_storage::workflow::references::capacity::inspect(
            &state.as_ref().expect("initialized probe").owner,
            context,
            input,
        )
    })
}
