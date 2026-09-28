//! Explicit canister uploader/observer transport; never an automatic retry or provider upload.
use super::{
    UPLOAD_MANIFEST_INSPECT_METHOD, UPLOAD_MANIFEST_PREPARE_METHOD,
    reply::{self, UploadManifestReplyError, UploadManifestReplyLimits},
};
use crate::{
    dto::upload::{
        admission::UploadAdmissionRequest,
        manifest::{UploadManifestMutation, UploadManifestRequest, UploadManifestResponse},
    },
    ops::service::uploads::admission,
};
use candid::Principal;
use ic_cdk::call::{Call, CallFailed};
use std::num::NonZeroU32;
use thiserror::Error;
#[cfg(test)]
mod tests;

/// Actual executing observer, tenant and selected service. Linking exports no endpoints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedUploadManifestClient {
    actor: Principal,
    tenant: Principal,
    service: Principal,
    timeout: NonZeroU32,
}
/// Failed transport/validation. Preserve exact saved intent through uncertain outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadManifestClientError {
    /// Invalid principal or timeout above 300 seconds.
    #[error("invalid manifest client configuration")]
    Configuration,
    /// Actual executing identity, required role or selected tenant/service differs.
    #[error("manifest client binding mismatch")]
    Binding,
    /// Query execution cannot dispatch the client.
    #[error("manifest client requires replicated execution")]
    Execution,
    /// Call could not be enqueued.
    #[error("manifest call not enqueued")]
    NotEnqueued,
    /// Platform reject code; bounded-wait rejection may leave an uncertain effect.
    #[error("manifest call rejected: {0}")]
    Rejected(u32),
    /// Invalid request/reply, exceeded budget or authenticated service refusal.
    #[error(transparent)]
    Reply(#[from] UploadManifestReplyError),
}
impl ReplicatedUploadManifestClient {
    /// Configure explicit actual actor, tenant and service without sending anything.
    /// The actor must be the admitted uploader to prepare, or tenant/uploader to inspect.
    /// # Errors
    /// Refuses anonymous/management principals and timeout above 300 seconds.
    pub fn new(
        actor: Principal,
        tenant: Principal,
        service: Principal,
        timeout: NonZeroU32,
    ) -> Result<Self, UploadManifestClientError> {
        if timeout.get() > 300
            || [actor, tenant, service]
                .into_iter()
                .any(|p| [Principal::anonymous(), Principal::management_canister()].contains(&p))
        {
            return Err(UploadManifestClientError::Configuration);
        }
        Ok(Self {
            actor,
            tenant,
            service,
            timeout,
        })
    }
    fn check(
        &self,
        input: UploadAdmissionRequest,
        preparing: bool,
    ) -> Result<(), UploadManifestClientError> {
        self.binding(ic_cdk::api::canister_self(), input, preparing)?;
        admission::parse_binding(self.service, input)
            .map_err(|_| UploadManifestReplyError::Invalid)?;
        if !ic_cdk::api::in_replicated_execution() {
            return Err(UploadManifestClientError::Execution);
        }
        Ok(())
    }
    fn binding(
        &self,
        actual: Principal,
        input: UploadAdmissionRequest,
        preparing: bool,
    ) -> Result<(), UploadManifestClientError> {
        if input.upload.service != self.service
            || input.upload.tenant != self.tenant
            || actual != self.actor
        {
            return Err(UploadManifestClientError::Binding);
        }
        let uploader = self.actor == input.uploader;
        let tenant_observer = !preparing && self.actor == self.tenant;
        if !uploader && !tenant_observer {
            return Err(UploadManifestClientError::Binding);
        }
        Ok(())
    }
    /// Send one preparation with no attached cycles; ordinary IC fees apply.
    /// Persist the full intent before polling this future. Raw declaration bounds and
    /// root consistency are checked before encoding/dispatch. Unusable acknowledgments
    /// require exact inspection; this client never retries or issues certificates.
    /// Native execution traps at IC APIs. The initial CDK reply buffer has separate
    /// platform bounds; these limits apply before decoding that buffer.
    /// # Errors
    /// Refuses wrong execution/authority, invalid declarations, transport and reply failures.
    pub async fn prepare(
        &self,
        input: &UploadManifestRequest,
        limits: UploadManifestReplyLimits,
    ) -> Result<UploadManifestMutation, UploadManifestClientError> {
        self.check(input.permission, true)?;
        reply::validate(input.permission, &input.declaration, limits.declaration)?;
        let response = Call::bounded_wait(self.service, UPLOAD_MANIFEST_PREPARE_METHOD)
            .change_timeout(self.timeout.get())
            .with_arg(input)
            .await
            .map_err(failure)?;
        Ok(reply::mutation(input, response.as_ref(), limits)?)
    }
    /// Inspect once through replicated execution, with no attached cycles or mutation
    /// fallback. Historical Prepared/Unprepared grants no uncertain-retry authority.
    /// # Errors
    /// Refuses wrong execution/authority, transport failure and invalid/foreign replies.
    pub async fn inspect(
        &self,
        input: UploadAdmissionRequest,
        limits: UploadManifestReplyLimits,
    ) -> Result<UploadManifestResponse, UploadManifestClientError> {
        self.check(input, false)?;
        let response = Call::bounded_wait(self.service, UPLOAD_MANIFEST_INSPECT_METHOD)
            .change_timeout(self.timeout.get())
            .with_arg(input)
            .await
            .map_err(failure)?;
        Ok(reply::inspection(input, response.as_ref(), limits)?)
    }
}
fn failure(error: CallFailed) -> UploadManifestClientError {
    match error {
        CallFailed::CallRejected(e) => UploadManifestClientError::Rejected(e.raw_reject_code()),
        CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_) => {
            UploadManifestClientError::NotEnqueued
        }
    }
}
