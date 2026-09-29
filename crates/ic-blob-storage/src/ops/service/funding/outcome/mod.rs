//! Durable structured observations and conservative exact-intent reconciliation.
pub mod boundary;
mod conversion;
use super::{
    FundingIntent, FundingIntentError, FundingJournalError, FundingTransportContext,
    FundingTransportOutcome, Memory, StableFundingJournal, UploadContext,
};
use crate::{
    model::billing::{
        journal::{FundingIntentState, record::response::FundingResponseRecord},
        transfer::FundingTransfer,
    },
    ops::caffeine::funding::transport::{CashierTopUpObservation, CashierTopUpStatus},
};

/// Retained local evidence, independently readable after lost output or restore.
/// A balance report is not credit evidence, and no result permits a retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingOutcomeView {
    /// Original exact scope, identity and attachment.
    pub intent: FundingIntent,
    /// Retained transport phase.
    pub state: FundingIntentState,
    /// Bounded structured observation, absent if only transport was retained.
    pub response: Option<CashierTopUpStatus>,
    /// Validated attachment facts for the workflow's independent reconciliation policy.
    pub transfer: FundingTransfer,
}

impl<M: Memory> StableFundingJournal<M> {
    /// Commit shared transport evidence, structured response and accounting together.
    /// The host must retain the exact original intent across the call. Namespace
    /// and operation IDs are local: the provider has no corresponding wire fields.
    /// Checks the observation's actual service/target, account, offer and target
    /// balance before writing. Exact replay is unchanged; conflicts never overwrite.
    /// # Errors
    /// Rejects unauthorized/fenced callers, identity or response conflicts and
    /// accounting failures. A lost callback remains uncertain and cannot be retried.
    /// # Panics
    /// Stable failures must propagate for whole-message IC rollback.
    pub fn record_observation(
        &mut self,
        execution: UploadContext,
        input: FundingIntent,
        observation: CashierTopUpObservation,
    ) -> Result<bool, FundingJournalError> {
        self.authorize(execution)?;
        self.mutable()?;
        if observation.account() != input.account
            || observation.target_balance() != input.target_balance
            || observation.transfer().offered() != input.offered
        {
            return Err(FundingIntentError::Conflict.into());
        }
        self.complete_transport(
            execution,
            input,
            observation.source(),
            observation.outcome(),
            Some(conversion::record(observation.status())),
        )
    }

    /// Read an exact outcome without changing reservations, credit or restore fences.
    /// Unknown intent is absence, never permission to send another payment.
    /// # Errors
    /// Rejects caller/scope/identity mismatch or corrupt retained records.
    pub fn outcome(
        &self,
        execution: UploadContext,
        input: FundingIntent,
    ) -> Result<Option<FundingOutcomeView>, FundingJournalError> {
        self.authorize(execution)?;
        Self::scope(&self.totals()?, input)?;
        self.exact(input)?
            .map(|record| {
                let view = record.view().ok_or(FundingJournalError::InvalidRecord)?;
                let transfer = record
                    .transfer()
                    .ok_or(FundingJournalError::InvalidRecord)?;
                Ok(FundingOutcomeView {
                    intent: view.intent,
                    state: view.state,
                    response: conversion::view(record.response()),
                    transfer,
                })
            })
            .transpose()
    }

    pub(super) fn complete_transport(
        &mut self,
        execution: UploadContext,
        input: FundingIntent,
        source: FundingTransportContext,
        outcome: FundingTransportOutcome,
        response: Option<FundingResponseRecord>,
    ) -> Result<bool, FundingJournalError> {
        self.authorize(execution)?;
        self.mutable()?;
        if source.service != execution.service || source.cashier != input.cashier {
            return Err(FundingJournalError::TransportBinding);
        }
        let totals = self.totals()?;
        Self::scope(&totals, input)?;
        let prior = self.required(input)?;
        let transport = prior.complete(outcome)?;
        let next = match response {
            Some(response) => transport.with_response(response)?,
            None => transport,
        };
        if prior == next {
            return Ok(false);
        }
        // Enriching already retained transport must not release its refund twice.
        let totals = if prior == transport {
            totals
        } else {
            totals.resolve(next.transfer().ok_or(FundingJournalError::InvalidRecord)?)?
        };
        self.intents.insert(input.operation.get(), next);
        self.accounting.insert(0, totals);
        Ok(true)
    }
}
