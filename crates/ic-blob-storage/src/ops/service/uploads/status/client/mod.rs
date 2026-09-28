//! Single replicated authenticated upload observation, with no automatic retry.
use super::{
    UPLOAD_STATUS_METHOD,
    reply::{self, UploadStatusReplyError},
};
use crate::dto::{reference::ReferenceUpload, upload::UploadStatusResponse};
use candid::Principal;
use ic_cdk::call::{Call, CallFailed};
use std::num::{NonZeroU32, NonZeroUsize};
use thiserror::Error;
/// Explicit actual tenant/service selection; construction sends nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedUploadStatusClient {
    tenant: Principal,
    service: Principal,
    timeout: NonZeroU32,
}
/// Transport/validation failure, distinct from historical completion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadStatusClientError {
    /// Invalid principal or timeout above 300 seconds.
    #[error("invalid upload status client")]
    Configuration,
    /// Actual executing tenant or original upload differs from configuration.
    #[error("upload status client binding mismatch")]
    Binding,
    /// Ordinary/composite query execution is refused.
    #[error("upload status requires replicated execution")]
    Execution,
    /// Platform call could not be enqueued.
    #[error("upload status not enqueued")]
    NotEnqueued,
    /// Platform reject code, including bounded-wait uncertainty.
    #[error("upload status rejected: {0}")]
    Rejected(u32),
    /// Bounded decoding or service refusal.
    #[error(transparent)]
    Reply(#[from] UploadStatusReplyError),
}
impl ReplicatedUploadStatusClient {
    /// Bind actual canisters and a positive timeout of at most 300 seconds.
    /// # Errors
    /// Rejects anonymous/management principals and excessive timeout.
    pub fn new(
        tenant: Principal,
        service: Principal,
        timeout: NonZeroU32,
    ) -> Result<Self, UploadStatusClientError> {
        if timeout.get() > 300
            || [tenant, service]
                .into_iter()
                .any(|p| p == Principal::anonymous() || p == Principal::management_canister())
        {
            return Err(UploadStatusClientError::Configuration);
        }
        Ok(Self {
            tenant,
            service,
            timeout,
        })
    }
    /// Call the query once through replicated execution, attaching no cycles.
    /// Ordinary IC fees apply. The IC target authenticates the reply; application
    /// limits apply before decoding, after the separately platform-bounded CDK
    /// buffer. No certificate, upload, retain, retry or provider effect is performed.
    /// Native execution traps at IC APIs; invoke from an IC update.
    /// # Errors
    /// Rejects wrong context, transport failure and invalid or foreign responses.
    pub async fn inspect(
        &self,
        upload: ReferenceUpload,
        max: NonZeroUsize,
    ) -> Result<UploadStatusResponse, UploadStatusClientError> {
        if upload.service != self.service
            || upload.tenant != self.tenant
            || ic_cdk::api::canister_self() != self.tenant
        {
            return Err(UploadStatusClientError::Binding);
        }
        reply::check(upload)?;
        if !ic_cdk::api::in_replicated_execution() {
            return Err(UploadStatusClientError::Execution);
        }
        let response = Call::bounded_wait(self.service, UPLOAD_STATUS_METHOD)
            .change_timeout(self.timeout.get())
            .with_arg(upload)
            .await
            .map_err(|e| match e {
                CallFailed::CallRejected(e) => {
                    UploadStatusClientError::Rejected(e.raw_reject_code())
                }
                CallFailed::InsufficientLiquidCycleBalance(_)
                | CallFailed::CallPerformFailed(_) => UploadStatusClientError::NotEnqueued,
            })?;
        Ok(reply::decode(upload, response.as_ref(), max)?)
    }
}
