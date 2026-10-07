//! Local funding bookkeeping and shared transport against a labelled substitute.
pub(crate) mod admission;
pub(crate) mod credit;
pub(crate) mod dispatch;
pub(crate) mod summary;
pub(crate) mod transport;
use super::{ProbeMemory, STATE, TRAP_WRITE};
use blob_test_protocol::storage::{
    Failure,
    funding::{Action, Allocation, Command, Intent, Phase},
};
use ic_blob_storage::{
    model::{
        billing::journal::{
            FundingIntent, FundingIntentAdmission, FundingIntentState, FundingTransportOutcome,
        },
        service::upload::UploadContext,
    },
    ops::service::funding::FundingJournalError,
};
use std::num::NonZeroU128;
pub(crate) fn intent(input: Intent) -> Result<FundingIntent, Failure> {
    Ok(FundingIntent {
        service: input.service,
        cashier: input.cashier,
        account: input.account,
        namespace: NonZeroU128::new(input.namespace).ok_or(Failure::Invalid)?,
        operation: NonZeroU128::new(input.operation).ok_or(Failure::Invalid)?,
        offered: NonZeroU128::new(input.offered).ok_or(Failure::Invalid)?,
        target_balance: input
            .target_balance
            .map(|value| NonZeroU128::new(value).ok_or(Failure::Invalid))
            .transpose()?,
    })
}
pub(crate) fn failure(error: FundingJournalError) -> Failure {
    use ic_blob_storage::model::billing::{
        allocation::FundingAllocationError, journal::FundingIntentError,
    };
    match error {
        FundingJournalError::Credit(error) => match error {
            ic_blob_storage::model::billing::journal::credit::FundingCreditError::Conflict
            | ic_blob_storage::model::billing::journal::credit::FundingCreditError::ReceiptReused => Failure::Conflict,
            ic_blob_storage::model::billing::journal::credit::FundingCreditError::AcceptanceRequired => Failure::Phase,
            _ => Failure::Invalid,
        },
        FundingJournalError::Renewal(error) => match error {
            ic_blob_storage::model::billing::journal::renewal::FundingRenewalError::Conflict => Failure::Conflict,
            ic_blob_storage::model::billing::journal::renewal::FundingRenewalError::Amount => Failure::Invalid,
            _ => Failure::Phase,
        },
        FundingJournalError::NotOperator => Failure::Denied,
        FundingJournalError::Binding | FundingJournalError::TransportBinding => Failure::Binding,
        FundingJournalError::Fenced => Failure::Fenced,
        FundingJournalError::Unknown => Failure::Unknown,
        FundingJournalError::StaleIdentity
        | FundingJournalError::Intent(
            FundingIntentError::Conflict | FundingIntentError::OutcomeConflict,
        ) => Failure::Conflict,
        FundingJournalError::Intent(
            FundingIntentError::AlreadyAttempted | FundingIntentError::NotAttempted,
        )
        | FundingJournalError::Allocation(FundingAllocationError::PendingNotLast) => Failure::Phase,
        FundingJournalError::Allocation(
            FundingAllocationError::Capacity
            | FundingAllocationError::RenewalCeiling
            | FundingAllocationError::ReserveWouldBeViolated { .. },
        ) => Failure::Capacity,
        _ => Failure::Invalid,
    }
}
fn phase(state: FundingIntentState) -> Phase {
    match state {
        FundingIntentState::Prepared => Phase::Prepared,
        FundingIntentState::Uncertain => Phase::Uncertain,
        FundingIntentState::NotEnqueued => Phase::NotEnqueued,
        FundingIntentState::Callback { refunded } => Phase::Callback(refunded),
    }
}
pub(crate) fn apply(execution: UploadContext, input: Command) -> Result<bool, Failure> {
    let intent = intent(input.intent)?;
    let source = ic_blob_storage::model::billing::journal::FundingTransportContext {
        service: execution.service,
        cashier: input.source.unwrap_or(intent.cashier),
    };
    TRAP_WRITE.set(input.fault);
    let result = STATE.with_borrow_mut(|state| {
        let journal = &mut state.as_mut().unwrap().funding;
        match input.action {
            Action::Prepare => journal
                .prepare(execution, intent)
                .map(|v| matches!(v, FundingIntentAdmission::Created)),
            Action::Attempt => journal.mark_attempted(execution, intent).map(|_| true),
            Action::NotEnqueued => journal.record_transport(
                execution,
                intent,
                source,
                FundingTransportOutcome::NotEnqueued,
            ),
            Action::Callback(refunded) => journal.record_transport(
                execution,
                intent,
                source,
                FundingTransportOutcome::Callback { refunded },
            ),
        }
    });
    TRAP_WRITE.set(None);
    result.map_err(failure)
}
pub(crate) fn lookup(execution: UploadContext, input: Intent) -> Result<Option<Phase>, Failure> {
    let input = intent(input)?;
    STATE
        .with_borrow(|state| state.as_ref().unwrap().funding.lookup(execution, input))
        .map(|v| v.map(|v| phase(v.state)))
        .map_err(failure)
}
pub(crate) fn allocation(execution: UploadContext) -> Result<Allocation, Failure> {
    STATE.with_borrow(|state| {
        let journal = &state.as_ref().unwrap().funding;
        let view = journal.allocation(execution).map_err(failure)?;
        Ok(Allocation {
            available: view.available(),
            accepted: view.accepted(),
            refunded: view.refunded(),
            not_enqueued: view.not_enqueued(),
            uncertain: view.reserved_or_uncertain(),
            fenced: journal.is_fenced(),
        })
    })
}

pub(crate) fn request(
    execution: UploadContext,
    input: Intent,
) -> Result<blob_test_protocol::storage::funding::Request, Failure> {
    let input = intent(input)?;
    STATE
        .with_borrow(|state| state.as_ref().unwrap().funding.request(execution, input))
        .map(|view| blob_test_protocol::storage::funding::Request {
            cashier: view.cashier(),
            method: view.method_name().into(),
            arguments: view.arguments().to_vec(),
            offered: view.offered().get(),
        })
        .map_err(failure)
}
