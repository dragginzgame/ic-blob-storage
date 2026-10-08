//! Explicit replicated funding inspection; no attached cycles or mutation methods.
use candid::Principal;
use ic_blob_storage_contracts::dto::funding::FundingHistoryPage;
use ic_blob_storage_contracts::dto::funding::FundingHistoryRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeResponse;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::funding::reply;
use ic_blob_storage_contracts::funding::reply::FundingHistoryReplyLimits;
use ic_blob_storage_contracts::funding::reply::FundingReplyError;
use ic_blob_storage_contracts::protocol::FUNDING_HISTORY_METHOD;
use ic_blob_storage_contracts::protocol::FUNDING_OUTCOME_METHOD;
use ic_cdk::call::{Call, CallFailed};
use std::num::{NonZeroU32, NonZeroUsize};
use thiserror::Error;

/// Pinned actual executing canister and complete service/provider/account scope.
/// Construction grants no operator authority. Linking exports no endpoints or hooks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedFundingClient {
    actor: Principal,
    scope: OperatorScope,
    timeout: NonZeroU32,
}
/// Transport or reply refusal, never converted into an empty history/outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum FundingClientError {
    /// Invalid principal, namespace or timeout above 300 seconds.
    #[error("invalid funding client configuration")]
    Configuration,
    /// Actual canister or requested scope differs from configured identities.
    #[error("funding client binding mismatch")]
    Binding,
    /// Requires replicated execution; no ordinary/composite query dispatch.
    #[error("funding client requires replicated execution")]
    Execution,
    /// Platform could not enqueue the call.
    #[error("funding inspection not enqueued")]
    NotEnqueued,
    /// Raw platform rejection code without diagnostic text.
    #[error("funding inspection rejected: {0}")]
    Rejected(u32),
    /// Invalid request, unusable observation or authenticated service refusal.
    #[error(transparent)]
    Reply(#[from] FundingReplyError),
}
impl ReplicatedFundingClient {
    /// Configure a passive client without dispatch, storage or lifecycle ownership.
    /// # Errors
    /// Rejects special principals, zero namespace or timeout above 300 seconds.
    pub fn new(
        actor: Principal,
        scope: OperatorScope,
        timeout: NonZeroU32,
    ) -> Result<Self, FundingClientError> {
        if [Principal::anonymous(), Principal::management_canister()].contains(&actor)
            || timeout.get() > 300
            || reply::check_scope(scope).is_err()
        {
            return Err(FundingClientError::Configuration);
        }
        Ok(Self {
            actor,
            scope,
            timeout,
        })
    }
    fn binding(&self, actual: Principal, scope: OperatorScope) -> Result<(), FundingClientError> {
        if actual != self.actor || scope != self.scope {
            return Err(FundingClientError::Binding);
        }
        Ok(())
    }
    fn check(&self, scope: OperatorScope) -> Result<(), FundingClientError> {
        self.binding(ic_cdk::api::canister_self(), scope)?;
        if !ic_cdk::api::in_replicated_execution() {
            return Err(FundingClientError::Execution);
        }
        Ok(())
    }
    /// Fetch one page in one bounded call; no automatic pagination or retry.
    /// No cycles attach; ordinary IC fees apply. Limits apply after the CDK's
    /// separately platform-bounded reply buffer. Native system API execution traps.
    /// # Errors
    /// Rejects caller/scope/execution, invalid cursor, transport or unusable replies.
    pub async fn history(
        &self,
        input: FundingHistoryRequest,
        limits: FundingHistoryReplyLimits,
    ) -> Result<FundingHistoryPage, FundingClientError> {
        self.check(input.scope)?;
        reply::check_history(input)?;
        let response = Call::bounded_wait(self.scope.service, FUNDING_HISTORY_METHOD)
            .change_timeout(self.timeout.get())
            .with_arg(input)
            .await
            .map_err(failure)?;
        Ok(reply::history(input, response.as_ref(), limits)?)
    }
    /// Inspect one original intent once, preserving absence and restore fencing.
    /// No cycles attach, automatic retry, journal or identity allocation occurs.
    /// Ordinary IC fees apply. A usable observation grants no payment authority.
    /// # Errors
    /// Rejects caller/scope/execution, invalid intent, transport or unusable replies.
    pub async fn outcome(
        &self,
        input: FundingOutcomeRequest,
        max: NonZeroUsize,
    ) -> Result<Option<FundingOutcomeResponse>, FundingClientError> {
        self.check(input.scope)?;
        reply::check_outcome(input)?;
        let response = Call::bounded_wait(self.scope.service, FUNDING_OUTCOME_METHOD)
            .change_timeout(self.timeout.get())
            .with_arg(input)
            .await
            .map_err(failure)?;
        Ok(reply::outcome::inspect(input, response.as_ref(), max)?)
    }
}
fn failure(error: CallFailed) -> FundingClientError {
    match error {
        CallFailed::CallRejected(e) => FundingClientError::Rejected(e.raw_reject_code()),
        CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_) => {
            FundingClientError::NotEnqueued
        }
    }
}
#[cfg(test)]
mod tests;
