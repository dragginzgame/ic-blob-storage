//! Explicit single-call tenant management from a canister operator or observer.
use super::{
    TENANT_INSPECTION_METHOD, TENANT_UPDATE_METHOD,
    reply::{self, TenantReplyError},
};
use crate::dto::tenant::{TenantEnrollmentResponse, TenantScope, TenantUpdateRequest};
use candid::Principal;
use ic_cdk::call::{Call, CallFailed};
use std::num::{NonZeroU32, NonZeroUsize};
use thiserror::Error;

/// Explicit actual executing canister and one pinned service/namespace/tenant scope.
/// The service still authorizes the actor; construction does not assume operator status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedTenantClient {
    actor: Principal,
    scope: TenantScope,
    timeout: NonZeroU32,
}
/// Transport/validation rejection; retained command intent survives uncertain outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum TenantClientError {
    /// Invalid principal, namespace or timeout above 300 seconds.
    #[error("invalid tenant client configuration")]
    Configuration,
    /// Actual canister or input scope differs from configured identities.
    #[error("tenant client binding mismatch")]
    Binding,
    /// Calls require replicated execution, not ordinary/composite queries.
    #[error("tenant client requires replicated execution")]
    Execution,
    /// Platform could not enqueue the call.
    #[error("tenant call not enqueued")]
    NotEnqueued,
    /// Platform reject code; bounded-wait rejection need not mean non-execution.
    #[error("tenant call rejected: {0}")]
    Rejected(u32),
    /// Request/reply validation or authenticated service refusal.
    #[error(transparent)]
    Reply(#[from] TenantReplyError),
}
impl ReplicatedTenantClient {
    /// Select exact identities and positive timeout without installing or dispatching.
    /// # Errors
    /// Rejects special principals, zero namespace or timeout above 300 seconds.
    pub fn new(
        actor: Principal,
        scope: TenantScope,
        timeout: NonZeroU32,
    ) -> Result<Self, TenantClientError> {
        if actor == Principal::anonymous()
            || actor == Principal::management_canister()
            || timeout.get() > 300
            || reply::check_scope(scope).is_err()
        {
            return Err(TenantClientError::Configuration);
        }
        Ok(Self {
            actor,
            scope,
            timeout,
        })
    }
    fn check(&self, scope: TenantScope) -> Result<(), TenantClientError> {
        self.binding(ic_cdk::api::canister_self(), scope)?;
        if !ic_cdk::api::in_replicated_execution() {
            return Err(TenantClientError::Execution);
        }
        Ok(())
    }
    fn binding(&self, actual: Principal, scope: TenantScope) -> Result<(), TenantClientError> {
        if actual != self.actor || scope != self.scope {
            return Err(TenantClientError::Binding);
        }
        Ok(())
    }
    /// Inspect once using replicated execution with no attached cycles or update fallback.
    /// Ordinary IC fees apply. Reply limits apply before decoding, after the CDK's
    /// separately platform-bounded buffer. Native execution traps at IC system APIs.
    /// # Errors
    /// Rejects wrong caller/execution, transport errors or unusable scoped observations.
    pub async fn inspect(
        &self,
        max: NonZeroUsize,
    ) -> Result<TenantEnrollmentResponse, TenantClientError> {
        self.check(self.scope)?;
        let response = Call::bounded_wait(self.scope.service, TENANT_INSPECTION_METHOD)
            .change_timeout(self.timeout.get())
            .with_arg(self.scope)
            .await
            .map_err(failure)?;
        Ok(reply::inspection(self.scope, response.as_ref(), max)?)
    }
    /// Apply one exact compare-and-set with no attached cycles or automatic retries.
    /// The host must persist the command before polling and retain it after an
    /// uncertain/invalid reply. Inspect current state before choosing another command;
    /// inspection is not proof of this command's historical execution. Ordinary IC
    /// fees apply. This client owns no journal, identity allocator or lifecycle hooks.
    /// # Errors
    /// Rejects wrong caller/scope/execution, invalid intent, transport or unusable reply.
    pub async fn update(
        &self,
        input: TenantUpdateRequest,
        max: NonZeroUsize,
    ) -> Result<TenantEnrollmentResponse, TenantClientError> {
        self.check(input.scope)?;
        reply::validate_update(input)?;
        let response = Call::bounded_wait(self.scope.service, TENANT_UPDATE_METHOD)
            .change_timeout(self.timeout.get())
            .with_arg(input)
            .await
            .map_err(failure)?;
        Ok(reply::mutation(input, response.as_ref(), max)?)
    }
}
fn failure(error: CallFailed) -> TenantClientError {
    match error {
        CallFailed::CallRejected(e) => TenantClientError::Rejected(e.raw_reject_code()),
        CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_) => {
            TenantClientError::NotEnqueued
        }
    }
}

#[cfg(test)]
mod tests;
