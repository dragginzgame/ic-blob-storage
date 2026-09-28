//! Explicit single-call mutation and receipt inspection; no automatic retry.
use super::{
    REFERENCE_APPLY_METHOD, REFERENCE_RECEIPT_METHOD,
    reply::{self, ReferenceReplyError},
};
use crate::dto::reference::{ReferenceCommand, ReferenceMutationResponse, ReferenceReceiptLookup};
use candid::Principal;
use ic_cdk::call::{Call, CallFailed, Response};
use std::num::{NonZeroU32, NonZeroUsize};
use thiserror::Error;
/// Explicit tenant/service configuration. Construction installs and sends nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedReferenceClient {
    tenant: Principal,
    service: Principal,
    timeout: NonZeroU32,
}
/// Local transport failure or bounded service response rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ReferenceClientError {
    /// Invalid principal or timeout outside 1..=300 seconds.
    #[error("invalid reference client configuration")]
    Configuration,
    /// Request or running tenant differs from configured identities.
    #[error("reference client binding mismatch")]
    Binding,
    /// Ordinary/composite query execution is not supported.
    #[error("reference client requires replicated execution")]
    Execution,
    /// Call could not be enqueued.
    #[error("reference call not enqueued")]
    NotEnqueued,
    /// Platform rejection, including bounded-wait uncertainty. Does not prove non-execution.
    #[error("reference call rejected: {0}")]
    Rejected(u32),
    /// Bounded reply validation or remote lookup refusal.
    #[error(transparent)]
    Reply(#[from] ReferenceReplyError),
}
impl ReplicatedReferenceClient {
    /// Select the actual tenant and storage service with a bounded timeout.
    /// # Errors
    /// Rejects anonymous/management identities and timeouts above 300 seconds.
    pub fn new(
        tenant: Principal,
        service: Principal,
        timeout: NonZeroU32,
    ) -> Result<Self, ReferenceClientError> {
        if timeout.get() > 300
            || [tenant, service]
                .into_iter()
                .any(|p| p == Principal::anonymous() || p == Principal::management_canister())
        {
            return Err(ReferenceClientError::Configuration);
        }
        Ok(Self {
            tenant,
            service,
            timeout,
        })
    }
    /// Call the canonical query once through replicated execution with no attached
    /// cycles. Ordinary IC fees apply. The selected IC target authenticates the
    /// reply. Application bounds apply before decoding, after the CDK's separately
    /// platform-bounded buffer. No retry, update fallback or reference change occurs.
    /// Native invocation traps at system APIs; invoke from an IC update.
    /// # Errors
    /// Rejects wrong context, transport failure and invalid/foreign/oversized replies.
    pub async fn receipt(
        &self,
        request: ReferenceCommand,
        max_reply_bytes: NonZeroUsize,
    ) -> Result<ReferenceReceiptLookup, ReferenceClientError> {
        let response = self.send(request, REFERENCE_RECEIPT_METHOD).await?;
        Ok(reply::decode(request, response.as_ref(), max_reply_bytes)?)
    }

    /// Dispatch one exact retain/release update with no attached cycles or automatic
    /// retry. The host must durably retain intent before polling this future and
    /// preserve it across cancellation, uncertain rejection or malformed replies.
    /// A response failure may follow a committed mutation; reconcile the same
    /// receipt. No identity allocation, outbox, publication or provider call is implied.
    /// Ordinary IC fees apply. Invoke from replicated IC execution only.
    /// # Errors
    /// Rejects wrong context, transport errors and invalid/foreign/oversized responses.
    /// Stored transition failures remain inside the successful response envelope.
    pub async fn apply(
        &self,
        request: ReferenceCommand,
        max_reply_bytes: NonZeroUsize,
    ) -> Result<ReferenceMutationResponse, ReferenceClientError> {
        let response = self.send(request, REFERENCE_APPLY_METHOD).await?;
        Ok(reply::decode_mutation(
            request,
            response.as_ref(),
            max_reply_bytes,
        )?)
    }

    async fn send(
        &self,
        request: ReferenceCommand,
        method: &str,
    ) -> Result<Response, ReferenceClientError> {
        if request.upload.service != self.service
            || request.upload.tenant != self.tenant
            || ic_cdk::api::canister_self() != self.tenant
        {
            return Err(ReferenceClientError::Binding);
        }
        reply::check_request(request)?;
        if !ic_cdk::api::in_replicated_execution() {
            return Err(ReferenceClientError::Execution);
        }
        Call::bounded_wait(self.service, method)
            .change_timeout(self.timeout.get())
            .with_arg(request)
            .await
            .map_err(|error| match error {
                CallFailed::CallRejected(error) => {
                    ReferenceClientError::Rejected(error.raw_reject_code())
                }
                CallFailed::InsufficientLiquidCycleBalance(_)
                | CallFailed::CallPerformFailed(_) => ReferenceClientError::NotEnqueued,
            })
    }
}
