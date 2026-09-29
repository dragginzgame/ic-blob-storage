//! Canonical query correlation and transactional bounded reply application.
use super::{
    GatewayScope, GatewayStoreError, GatewaySyncToken, StableGatewayRegistry, UploadContext,
};
use crate::ops::caffeine::{
    gateway::GatewayReplyLimits,
    query::{CashierQueryError, CashierQueryRequest, reply::BoundGatewayReplyError},
};
use ic_memory::ic_stable_structures::Memory;
use thiserror::Error;

/// Host-retained correlation for one durably pending read-only query.
/// Not an ingress DTO or source-authentication proof. Retain this value and the
/// original authenticated operator context across transport; never reconstruct
/// either from reply payloads. Restored owners reject even a retained request.
#[derive(Clone, Debug)]
pub struct GatewaySyncRequest {
    pub(crate) scope: GatewayScope,
    pub(crate) token: GatewaySyncToken,
    pub(crate) request: CashierQueryRequest,
}
impl GatewaySyncRequest {
    /// Original durable local sequence, including after cancellation or completion.
    /// It is a correlation value, not a reusable request or restore authority.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.token.sequence()
    }
    /// Canonical target, query method and encoded arguments for trusted transport.
    /// The provider advertises a query; replicated-call support is not implied.
    #[must_use]
    pub const fn request(&self) -> &CashierQueryRequest {
        &self.request
    }
    /// Original installation scope, independent of the reply payload.
    #[must_use]
    pub const fn scope(&self) -> GatewayScope {
        self.scope
    }
}
/// Rejected sync operation. Ordinary errors leave durable state unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum GatewaySyncReplyError {
    /// Durable authority, fence or transition rejection.
    #[error(transparent)]
    Store(#[from] GatewayStoreError),
    /// Canonical request could not be constructed before reserving a sync.
    #[error(transparent)]
    Request(#[from] CashierQueryError),
    /// Request binding, correlation, bounded decoding or membership rejection.
    #[error(transparent)]
    Reply(#[from] BoundGatewayReplyError),
}
impl<M: Memory> StableGatewayRegistry<M> {
    /// Check the exact pending attempt before host transport, without mutation.
    /// This is a current observation, not a permit; completion rechecks everything.
    /// # Errors
    /// Rejects wrong authority, restored state or an invalidated attempt.
    pub fn check_request(
        &self,
        context: UploadContext,
        attempt: &GatewaySyncRequest,
    ) -> Result<(), GatewayStoreError> {
        self.authorize(context, attempt.scope)?;
        if self.fenced {
            return Err(GatewayStoreError::Fenced);
        }
        Ok(self.registry()?.check_sync(attempt.token, attempt.scope)?)
    }
    /// Decode a reply against the original request and commit one complete record.
    /// Authority and restore fencing precede request/source/token checks, which
    /// precede parsing. Failed decoding leaves the exact pending attempt intact.
    /// Source scope must come from authenticated transport/host context, never
    /// from the payload. These comparisons cannot establish source or freshness.
    /// # Errors
    /// Rejects authority/fence, stale correlation and unusable bounded replies.
    /// # Panics
    /// Stable write failures must propagate for IC transaction rollback.
    pub fn apply_reply(
        &mut self,
        context: UploadContext,
        attempt: &GatewaySyncRequest,
        response_scope: GatewayScope,
        bytes: &[u8],
        limits: GatewayReplyLimits,
    ) -> Result<(), GatewaySyncReplyError> {
        self.mutate(context, attempt.scope, |registry| {
            Ok(attempt.request.apply_gateway_sync_reply(
                registry,
                attempt.token,
                response_scope,
                bytes,
                limits,
            )?)
        })
    }
}
