//! Explicit replicated gateway-list calls; never an off-chain query fallback.
//!
//! ICP executes a query entry point through consensus when called from an update.
//! This authenticates the called canister's response, not its provider semantics,
//! deployment ownership, list freshness beyond that execution or callback authority.
pub mod account;
use super::{CashierQueryRequest, CashierQueryResponse, CashierQueryTransport};
use crate::ops::caffeine::query::CashierQuery;
use candid::Principal;
use ic_cdk::call::{Call, CallFailed};
use std::num::{NonZeroU32, NonZeroUsize};
use thiserror::Error;

/// Explicit service/Cashier-bound transport for the maintained gateway-list query.
/// Construction sends nothing and installs no endpoints or lifecycle hooks.
/// A host must select and qualify the Cashier, authenticate its operator and
/// persist the exact sync before invoking this transport through the shared handler.
/// Uses one bounded-wait call, no attached cycles and no retry or method fallback.
/// Ordinary platform call/execution costs still apply.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedGatewayQuery {
    service: Principal,
    cashier: Principal,
    timeout: NonZeroU32,
}
impl ReplicatedGatewayQuery {
    /// Select explicit identities and a positive timeout of at most 300 seconds.
    /// The bound matches the locked CDK's platform maximum; never silently clamp.
    /// # Errors
    /// Rejects anonymous/management identities or an unsupported timeout.
    pub fn new(
        service: Principal,
        cashier: Principal,
        timeout: NonZeroU32,
    ) -> Result<Self, ReplicatedQueryError> {
        if [service, cashier]
            .into_iter()
            .any(|p| p == Principal::anonymous() || p == Principal::management_canister())
        {
            return Err(ReplicatedQueryError::InvalidPrincipal);
        }
        if timeout.get() > 300 {
            return Err(ReplicatedQueryError::TimeoutOutOfRange);
        }
        Ok(Self {
            service,
            cashier,
            timeout,
        })
    }
}
/// Transport failure never supplies a gateway list or clears the pending sync.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ReplicatedQueryError {
    /// Anonymous or management identity cannot bind this transport.
    #[error("invalid replicated query principal")]
    InvalidPrincipal,
    /// A positive timeout exceeds the supported maximum.
    #[error("replicated query timeout exceeds 300 seconds")]
    TimeoutOutOfRange,
    /// Actual service or original request Cashier differs from configuration.
    #[error("replicated query binding mismatch")]
    Binding,
    /// The method is outside the selected transport's read-only scope.
    #[error("unsupported replicated query method")]
    Method,
    /// An ordinary query/composite query cannot initiate this transport.
    #[error("replicated execution required")]
    Execution,
    /// Platform refused to enqueue; no response buffer exists.
    #[error("replicated query not enqueued")]
    NotEnqueued,
    /// Platform reject code, including bounded-wait uncertainty. Never retry automatically.
    #[error("replicated query rejected: {0}")]
    Rejected(u32),
    /// Reply exceeded the host limit before handing bytes to the decoder.
    #[error("replicated query reply exceeds byte limit")]
    ReplyTooLarge,
}
impl CashierQueryTransport for ReplicatedGatewayQuery {
    type Error = ReplicatedQueryError;
    /// Must run on the IC. Native system API calls trap. The CDK initially buffers
    /// the platform-bounded reply; this application limit is checked before handing
    /// that owned buffer to the decoder, with no second byte copy. It does not impose
    /// a smaller limit on the CDK's initial allocation or prevent platform call fees.
    async fn query(
        &self,
        request: &CashierQueryRequest,
        max_bytes: NonZeroUsize,
    ) -> Result<CashierQueryResponse, Self::Error> {
        if request.cashier() != self.cashier || ic_cdk::api::canister_self() != self.service {
            return Err(ReplicatedQueryError::Binding);
        }
        if request.query() != CashierQuery::StorageGateways {
            return Err(ReplicatedQueryError::Method);
        }
        self.send(request, max_bytes).await
    }
}
impl ReplicatedGatewayQuery {
    // Both public transports validate their allowed request before this shared IC effect.
    async fn send(
        &self,
        request: &CashierQueryRequest,
        max_bytes: NonZeroUsize,
    ) -> Result<CashierQueryResponse, ReplicatedQueryError> {
        if request.cashier() != self.cashier || ic_cdk::api::canister_self() != self.service {
            return Err(ReplicatedQueryError::Binding);
        }
        if !ic_cdk::api::in_replicated_execution() {
            return Err(ReplicatedQueryError::Execution);
        }
        let response = Call::bounded_wait(self.cashier, request.method_name())
            .change_timeout(self.timeout.get())
            .with_raw_args(request.arguments())
            .await
            .map_err(|error| match error {
                CallFailed::CallRejected(error) => {
                    ReplicatedQueryError::Rejected(error.raw_reject_code())
                }
                CallFailed::InsufficientLiquidCycleBalance(_)
                | CallFailed::CallPerformFailed(_) => ReplicatedQueryError::NotEnqueued,
            })?;
        if response.as_ref().len() > max_bytes.get() {
            return Err(ReplicatedQueryError::ReplyTooLarge);
        }
        Ok(CashierQueryResponse {
            cashier: self.cashier,
            bytes: response.into_bytes(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replicated_gateway_configuration_requires_explicit_valid_principals_and_timeout() {
        let service = Principal::from_slice(&[1, 1]);
        let cashier = Principal::from_slice(&[2, 1]);
        for invalid in [Principal::anonymous(), Principal::management_canister()] {
            assert_eq!(
                ReplicatedGatewayQuery::new(invalid, cashier, NonZeroU32::MIN),
                Err(ReplicatedQueryError::InvalidPrincipal)
            );
            assert_eq!(
                ReplicatedGatewayQuery::new(service, invalid, NonZeroU32::MIN),
                Err(ReplicatedQueryError::InvalidPrincipal)
            );
        }
        for timeout in [301, u32::MAX] {
            assert_eq!(
                ReplicatedGatewayQuery::new(service, cashier, timeout.try_into().unwrap()),
                Err(ReplicatedQueryError::TimeoutOutOfRange)
            );
        }
        for timeout in [1, 300] {
            assert!(
                ReplicatedGatewayQuery::new(service, cashier, timeout.try_into().unwrap()).is_ok()
            );
        }
    }
}
