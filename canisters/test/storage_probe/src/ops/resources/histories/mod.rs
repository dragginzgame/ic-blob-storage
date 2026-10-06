//! Operator-only synthetic history via existing durable owners and read workflow.
use crate::ops::{STATE, funding, read};
use blob_test_protocol::storage::{
    Failure,
    funding::Phase,
    resources::{HistoryPopulationBatch, RestorationRead, RestorationReadPage},
};
use ic_blob_storage::{
    model::{
        billing::journal::{FundingJournalScope, FundingTransportContext, FundingTransportOutcome},
        service::{read::session::ReadChunkTarget, upload::UploadContext},
    },
    workflow::reads::sessions::begin,
};
use std::num::{NonZeroU32, NonZeroU128};

pub(crate) fn populate(
    context: UploadContext,
    batch: HistoryPopulationBatch,
) -> Result<(), Failure> {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().unwrap();
        if context.actor != state.operator {
            return Err(Failure::Denied);
        }
        if state.funding.is_fenced() || state.read_sessions.inspect(context).unwrap().fenced {
            return Err(Failure::Fenced);
        }
        let count = batch
            .funding
            .len()
            .checked_add(batch.reads.len())
            .ok_or(Failure::Invalid)?;
        if count == 0
            || count > 16
            || batch
                .reads
                .iter()
                .any(|r| r.admit_fault.is_some() || r.callback_fault.is_some())
        {
            return Err(Failure::Invalid);
        }
        let funding = state
            .funding
            .summary(
                context,
                FundingJournalScope {
                    service: context.service,
                    cashier: state.operator,
                    account: context.service,
                    namespace: NonZeroU128::MIN,
                },
            )
            .map_err(funding::failure)?;
        let reads = state
            .read_sessions
            .inspect(context)
            .map_err(read::sessions::failure)?;
        if funding.retained_intents != batch.funding_start
            || reads.last_sequence != batch.read_start
        {
            return Err(Failure::Conflict);
        }
        for input in batch.funding {
            // Domain failures trap, preserving all previous writes in this IC message.
            let intent = funding::intent(input.intent).unwrap();
            assert!(matches!(
                state.funding.prepare(context, intent).unwrap(),
                ic_blob_storage::model::billing::journal::FundingIntentAdmission::Created
            ));
            if input.phase != Phase::Prepared {
                state.funding.mark_attempted(context, intent).unwrap();
            }
            let outcome = match input.phase {
                Phase::Callback(refunded) => Some(FundingTransportOutcome::Callback { refunded }),
                Phase::NotEnqueued => Some(FundingTransportOutcome::NotEnqueued),
                Phase::Prepared | Phase::Uncertain => None,
            };
            if let Some(outcome) = outcome {
                state
                    .funding
                    .record_transport(
                        context,
                        intent,
                        FundingTransportContext {
                            service: context.service,
                            cashier: state.operator,
                        },
                        outcome,
                    )
                    .unwrap();
            }
            record_synthetic_credit(state, context, input);
        }
        for input in batch.reads {
            let tenant = UploadContext {
                actor: input.target.content.tenant,
                ..context
            };
            let (scope, target) = read::authority::parse(tenant, input).unwrap();
            // Dropping a workflow ticket deliberately retains occupancy. No call is sent.
            let _ticket = begin(
                &state.gateways,
                &state.uploads,
                &mut state.read_sessions,
                tenant,
                scope,
                ReadChunkTarget {
                    target,
                    index: input.index,
                },
            )
            .unwrap();
        }
        Ok(())
    })
}

pub(crate) fn inspect(
    context: UploadContext,
    page: RestorationReadPage,
) -> Result<Vec<RestorationRead>, Failure> {
    STATE.with_borrow(|state| {
        let state = state.as_ref().unwrap();
        let limit = NonZeroU32::new(page.limit).ok_or(Failure::Invalid)?;
        state
            .read_sessions
            .inspect_active(context, page.after, limit)
            .map_err(read::sessions::failure)?
            .into_iter()
            .map(|view| {
                let target = view.chunk.target;
                let object = target.reference.object();
                let identity = object.identity();
                Ok(RestorationRead {
                    sequence: view.sequence,
                    target: blob_test_protocol::storage::gateways::ReadSessionInput {
                        cashier: state.operator,
                        gateway: target.gateway,
                        index: view.chunk.index,
                        admit_fault: None,
                        callback_fault: None,
                        target: blob_test_protocol::admission::input::RetainedDescriptorInput {
                            content: blob_test_protocol::admission::ContentLookup {
                                service: object.service(),
                                tenant: object.tenant(),
                                namespace: identity.namespace.get(),
                                root: *target.root.as_bytes(),
                            },
                            object: identity.object.get(),
                            incarnation: identity.incarnation.get(),
                            reference: target.reference.reference().get().get(),
                        },
                    },
                })
            })
            .collect()
    })
}

fn record_synthetic_credit(
    state: &mut super::super::State,
    context: UploadContext,
    input: blob_test_protocol::storage::resources::FundingPopulationIntent,
) {
    let Some(receipt_digest) = input.credit_digest else {
        return;
    };
    let Phase::Callback(refunded) = input.phase else {
        panic!("synthetic credit requires known callback");
    };
    state
        .funding
        .record_credit(
            context,
            ic_blob_storage::model::billing::journal::credit::FundingCreditConfirmation {
                intent: funding::intent(input.intent).unwrap(),
                accepted_cycles: NonZeroU128::new(input.intent.offered - refunded).unwrap(),
                receipt_digest,
            },
        )
        .unwrap();
}
