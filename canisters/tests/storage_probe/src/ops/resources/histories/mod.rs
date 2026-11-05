//! Operator-only synthetic history via existing durable owners and funding workflow.
use crate::ops::{STATE, funding};
use blob_test_protocol::storage::{Failure, funding::Phase, resources::HistoryPopulationBatch};
use ic_blob_storage::model::billing::journal::FundingJournalScope;
use ic_blob_storage::model::billing::journal::FundingTransportContext;
use ic_blob_storage::model::billing::journal::FundingTransportOutcome;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use std::num::NonZeroU128;

pub(crate) fn populate(
    context: UploadContext,
    batch: HistoryPopulationBatch,
) -> Result<(), Failure> {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().unwrap();
        if context.actor != state.operator {
            return Err(Failure::Denied);
        }
        if state.funding.is_fenced() {
            return Err(Failure::Fenced);
        }
        if batch.funding.is_empty() || batch.funding.len() > 16 {
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
        if funding.retained_intents != batch.funding_start {
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

        Ok(())
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
