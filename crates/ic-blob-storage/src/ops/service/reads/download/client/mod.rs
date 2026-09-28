//! Authenticated replicated descriptor call; no automatic retries or body transport.
use super::{
    DOWNLOAD_METHOD,
    reply::{self, DownloadReplyError, DownloadReplyLimits},
};
use crate::{
    dto::download::{DownloadRequest, DownloadResponse},
    model::service::read::download::CaffeineDownloadScope,
};
use candid::Principal;
use ic_cdk::call::{Call, CallFailed};
use std::num::NonZeroU32;
use thiserror::Error;
/// Explicit tenant/service selection. Construction installs nothing and sends nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedDownloadClient {
    tenant: Principal,
    service: Principal,
    timeout: NonZeroU32,
}
/// Transport or bound response failure; never authorizes publication or a paid retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum DownloadClientError {
    /// Anonymous/management identity or timeout outside 1..=300 seconds.
    #[error("invalid descriptor client configuration")]
    Configuration,
    /// Original request or actual executing tenant differs from configuration.
    #[error("descriptor client binding mismatch")]
    Binding,
    /// Ordinary/composite query execution is not allowed.
    #[error("descriptor client requires replicated execution")]
    Execution,
    /// Call could not be enqueued.
    #[error("descriptor call not enqueued")]
    NotEnqueued,
    /// Actual platform rejection code, including bounded-wait uncertainty.
    #[error("descriptor call rejected: {0}")]
    Rejected(u32),
    /// Bounded response or service refusal.
    #[error(transparent)]
    Reply(#[from] DownloadReplyError),
}
impl ReplicatedDownloadClient {
    /// Select actual tenant and storage canisters plus an explicit positive timeout.
    /// # Errors
    /// Rejects unusable principals or timeout greater than 300 seconds.
    pub fn new(
        tenant: Principal,
        service: Principal,
        timeout: NonZeroU32,
    ) -> Result<Self, DownloadClientError> {
        if timeout.get() > 300
            || [tenant, service]
                .into_iter()
                .any(|p| p == Principal::anonymous() || p == Principal::management_canister())
        {
            return Err(DownloadClientError::Configuration);
        }
        Ok(Self {
            tenant,
            service,
            timeout,
        })
    }
    /// Send the canonical update once from the actual tenant canister, attaching
    /// no cycles. Ordinary platform execution/call fees apply. The selected IC
    /// target authenticates the source; response fields cannot choose a peer.
    /// The CDK's initial buffer is platform-bounded; application limits apply before
    /// decoding. No HTTP origin, body call, publication lease or retry is implied.
    /// Native execution traps at system APIs; hosts must invoke from an IC update.
    /// # Errors
    /// Rejects pre-call bindings/execution, transport, decoding or returned identity.
    pub async fn fetch(
        &self,
        request: DownloadRequest,
        scope: &CaffeineDownloadScope,
        limits: DownloadReplyLimits,
    ) -> Result<DownloadResponse, DownloadClientError> {
        if request.service != self.service
            || request.tenant != self.tenant
            || ic_cdk::api::canister_self() != self.tenant
        {
            return Err(DownloadClientError::Binding);
        }
        reply::check_request(request, scope)?;
        if !ic_cdk::api::in_replicated_execution() {
            return Err(DownloadClientError::Execution);
        }
        let response = Call::bounded_wait(self.service, DOWNLOAD_METHOD)
            .change_timeout(self.timeout.get())
            .with_arg(request)
            .await
            .map_err(|error| match error {
                CallFailed::CallRejected(error) => {
                    DownloadClientError::Rejected(error.raw_reject_code())
                }
                CallFailed::InsufficientLiquidCycleBalance(_)
                | CallFailed::CallPerformFailed(_) => DownloadClientError::NotEnqueued,
            })?;
        Ok(reply::decode(request, scope, response.as_ref(), limits)?)
    }
}
