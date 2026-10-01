//! One explicit replicated admission or inspection; no retry or provider effects.
use super::{
    UPLOAD_ADMISSION_METHOD, UPLOAD_ADMISSION_STATUS_METHOD, UPLOAD_REVOCATION_METHOD,
    reply::{self, UploadAdmissionReplyError},
};
use crate::dto::upload::admission::{
    UploadAdmissionMutation, UploadAdmissionRequest, UploadAdmissionResponse,
    UploadRevocationResponse,
};
use candid::Principal;
use ic_cdk::call::{Call, CallFailed, Response};
use std::num::{NonZeroU32, NonZeroUsize};
use thiserror::Error;
/// Explicit tenant/service selection. Persist full permission intent before polling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedUploadAdmissionClient {
    tenant: Principal,
    service: Principal,
    timeout: NonZeroU32,
}
/// Transport failure or bounded service reply. Uncertainty preserves the original intent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadAdmissionClientError {
    /// Invalid principal or timeout exceeding 300 seconds.
    #[error("invalid admission client configuration")]
    Configuration,
    /// Configured service/tenant or actual executing tenant differs.
    #[error("admission client binding mismatch")]
    Binding,
    /// Query execution cannot dispatch this client.
    #[error("admission client requires replicated execution")]
    Execution,
    /// Call could not be enqueued.
    #[error("admission call not enqueued")]
    NotEnqueued,
    /// Platform reject code; bounded-wait rejection need not mean non-execution.
    #[error("admission call rejected: {0}")]
    Rejected(u32),
    /// Validation failure or authenticated refusal.
    #[error(transparent)]
    Reply(#[from] UploadAdmissionReplyError),
}
impl ReplicatedUploadAdmissionClient {
    /// Withdraw local issuance permission once, attaching no cycles. Ordinary IC
    /// fees apply. Persist cancellation intent before polling and inspect uncertain
    /// outcomes. Exposed effects remain charged; this neither releases a confirmed
    /// reference nor deletes provider content. The client never retries.
    /// # Errors
    /// Rejects wrong context, transport failure and invalid/non-revoked replies.
    pub async fn revoke(
        &self,
        input: UploadAdmissionRequest,
        max: NonZeroUsize,
    ) -> Result<UploadRevocationResponse, UploadAdmissionClientError> {
        let response = self.call(UPLOAD_REVOCATION_METHOD, input).await?;
        Ok(reply::revocation(input, response.as_ref(), max)?)
    }
    /// Select actual canisters and a bounded positive timeout; sends nothing.
    /// # Errors
    /// Refuses anonymous/management principals and timeout above 300 seconds.
    pub fn new(
        tenant: Principal,
        service: Principal,
        timeout: NonZeroU32,
    ) -> Result<Self, UploadAdmissionClientError> {
        if timeout.get() > 300
            || [tenant, service]
                .into_iter()
                .any(|p| p == Principal::anonymous() || p == Principal::management_canister())
        {
            return Err(UploadAdmissionClientError::Configuration);
        }
        Ok(Self {
            tenant,
            service,
            timeout,
        })
    }
    async fn call(
        &self,
        method: &str,
        input: UploadAdmissionRequest,
    ) -> Result<Response, UploadAdmissionClientError> {
        if input.upload.service != self.service
            || input.upload.tenant != self.tenant
            || ic_cdk::api::canister_self() != self.tenant
        {
            return Err(UploadAdmissionClientError::Binding);
        }
        reply::validate_request(input)?;
        if !ic_cdk::api::in_replicated_execution() {
            return Err(UploadAdmissionClientError::Execution);
        }
        Call::bounded_wait(self.service, method)
            .change_timeout(self.timeout.get())
            .with_arg(input)
            .await
            .map_err(|e| match e {
                CallFailed::CallRejected(e) => {
                    UploadAdmissionClientError::Rejected(e.raw_reject_code())
                }
                CallFailed::InsufficientLiquidCycleBalance(_)
                | CallFailed::CallPerformFailed(_) => UploadAdmissionClientError::NotEnqueued,
            })
    }
    /// Send one admission update with no attached cycles. Ordinary IC fees apply.
    /// Save intent before polling; reconcile unusable replies by exact inspection.
    /// The initial CDK reply buffer is separately platform bounded. Native execution
    /// traps at IC APIs. No automatic retry, certificate or provider call is added.
    /// # Errors
    /// Refuses wrong execution/bindings, transport failures and invalid replies.
    pub async fn admit(
        &self,
        input: UploadAdmissionRequest,
        max: NonZeroUsize,
    ) -> Result<UploadAdmissionMutation, UploadAdmissionClientError> {
        let response = self.call(UPLOAD_ADMISSION_METHOD, input).await?;
        Ok(reply::mutation(input, response.as_ref(), max)?)
    }
    /// Inspect once through replicated execution, attaching no cycles. Expired or
    /// missing observations never authorize repeating an uncertain provider effect.
    /// # Errors
    /// Refuses wrong execution/bindings, transport failures and invalid replies.
    pub async fn inspect(
        &self,
        input: UploadAdmissionRequest,
        max: NonZeroUsize,
    ) -> Result<UploadAdmissionResponse, UploadAdmissionClientError> {
        let response = self.call(UPLOAD_ADMISSION_STATUS_METHOD, input).await?;
        Ok(reply::inspection(input, response.as_ref(), max)?)
    }
}
