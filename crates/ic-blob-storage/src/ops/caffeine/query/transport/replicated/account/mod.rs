//! Explicit replicated account observations, sharing the bounded IC query effect.
use super::{
    CashierQueryRequest, CashierQueryResponse, CashierQueryTransport, ReplicatedGatewayQuery,
    ReplicatedQueryError,
};
use crate::ops::caffeine::{query::CashierQuery, relationship::PaymentRelationshipBinding};
use ic_platform::Principal;
use std::num::{NonZeroU32, NonZeroUsize};

/// One configured owner and payer, with no arbitrary account, audit or mutation route.
/// Responses authenticate IC execution only; they do not qualify provider economics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplicatedAccountQuery {
    transport: ReplicatedGatewayQuery,
    binding: PaymentRelationshipBinding,
}
impl ReplicatedAccountQuery {
    /// Construct without effects; the owner must be the actual sending service.
    /// # Errors
    /// Rejects special principals and timeouts above 300 seconds.
    pub fn new(
        service: Principal,
        cashier: Principal,
        payer: Principal,
        timeout: NonZeroU32,
    ) -> Result<Self, ReplicatedQueryError> {
        let transport = ReplicatedGatewayQuery::new(service, cashier, timeout)?;
        if payer == Principal::anonymous() || payer == Principal::management_canister() {
            return Err(ReplicatedQueryError::InvalidPrincipal);
        }
        Ok(Self {
            transport,
            binding: PaymentRelationshipBinding {
                paid_canister: service,
                payment_account: payer,
            },
        })
    }
    fn check(&self, request: &CashierQueryRequest) -> Result<(), ReplicatedQueryError> {
        if request.cashier() != self.transport.cashier {
            return Err(ReplicatedQueryError::Binding);
        }
        match request.query() {
            CashierQuery::Balance { account } if account == self.binding.payment_account => Ok(()),
            CashierQuery::PaymentRelationship(binding) if binding == self.binding => Ok(()),
            CashierQuery::Balance { .. } | CashierQuery::PaymentRelationship(_) => {
                Err(ReplicatedQueryError::Binding)
            }
            CashierQuery::StorageGateways | CashierQuery::AuditLog(_) => {
                Err(ReplicatedQueryError::Method)
            }
        }
    }
}
impl CashierQueryTransport for ReplicatedAccountQuery {
    type Error = ReplicatedQueryError;
    /// One bounded call, zero attached cycles, no retry or fallback. The CDK initially
    /// buffers the platform-bounded reply before enforcing the application byte bound.
    async fn query(
        &self,
        request: &CashierQueryRequest,
        max_bytes: NonZeroUsize,
    ) -> Result<CashierQueryResponse, Self::Error> {
        self.check(request)?;
        self.transport.send(request, max_bytes).await
    }
}

#[cfg(test)]
mod tests;
