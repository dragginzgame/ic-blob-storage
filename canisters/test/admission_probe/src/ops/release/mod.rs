//! Explicit provider substitutes for retained-state sizing; no outbound calls.
use super::{
    STATE, changed,
    conversion::{failure, request},
    mutate,
};
use blob_test_protocol::admission::{
    ContentLookup, Failure, Outcome,
    input::{ReferenceInput, ReferenceReceipt},
    release::{LifecycleCommand, ReferenceCapacity, ReferenceFailure},
};
use ic_blob_storage::model::{
    catalog::{CatalogError, admission::UploadRequest},
    identity::ProviderRootHash,
    lifecycle::{
        LifecycleChange, LifecycleError, ReferenceId,
        binding::ReferenceKey,
        requests::{
            ReferenceOperation, ReferenceRequest, ReferenceRequestError, ReferenceRequestId,
            ReferenceRequestOutcome,
        },
    },
    service::upload::{UploadAdmissionError, UploadContext, content::ContentLookup as ModelLookup},
};
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

pub(crate) fn receipt(
    context: UploadContext,
    input: ReferenceInput,
) -> Result<Option<ReferenceReceipt>, Failure> {
    let (object, operation) = reference_request(input)?;
    STATE.with_borrow(|state| {
        let owner = &state.as_ref().expect("initialized probe").owner;
        // Keep every field of the original upload bound, including its declared
        // length; the fixture's reference input contains that full request.
        let receipt = owner
            .reference_receipt(context, object.object.root, operation)
            .map_err(|error| match error {
                UploadAdmissionError::Reference(CatalogError::Request(
                    ReferenceRequestError::RequestConflict,
                )) => Failure::Conflict,
                other => reference_error(other),
            })?;
        owner.lookup(context, object).map_err(failure)?;
        Ok(receipt.map(|receipt| ReferenceReceipt {
            request: input,
            result: receipt
                .result
                .map(|change| change == LifecycleChange::Changed)
                .map_err(lifecycle_error),
        }))
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
    input: ContentLookup,
) -> Result<Option<ReferenceCapacity>, Failure> {
    if input.service != context.service {
        return Err(Failure::WrongService);
    }
    let input = ModelLookup {
        tenant: input.tenant,
        namespace: number(input.namespace)?,
        root: ProviderRootHash::try_from(input.root.as_slice()).expect("fixed hash"),
    };
    STATE.with_borrow(|state| {
        state
            .as_ref()
            .expect("initialized probe")
            .owner
            .reference_capacity(context, input)
            .map(|view| {
                view.map(|view| ReferenceCapacity {
                    reference_slots: view.reference_slots as u64,
                    unreserved_receipts: view.unreserved_receipts as u64,
                    release_reserved_receipts: view.release_reserved_receipts as u64,
                    fresh_retains: view.fresh_retains as u64,
                })
            })
            .map_err(failure)
    })
}
