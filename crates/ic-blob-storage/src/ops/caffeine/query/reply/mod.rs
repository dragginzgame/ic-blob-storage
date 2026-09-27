//! Decode account replies and apply gateway sync against the original request.
//!
//! The source principal must come from trusted transport context, never payload
//! assertions. Matching it does not authenticate transport or prove freshness.
//! Workflows still check service, namespace, revision and exact pending attempt.

use candid::Principal;
use thiserror::Error;

use super::{CashierQuery, CashierQueryRequest};
use crate::model::gateway::registry::{GatewayRegistry, GatewayScope, GatewaySyncToken};
use crate::ops::caffeine::{
    audit::{AuditLogReply, AuditLogReplyError, AuditLogReplyLimits, decode_audit_log_reply},
    balance::{BalanceReply, BalanceReplyError, BalanceReplyLimits, decode_balance_reply},
    gateway::{GatewayReplyError, GatewayReplyLimits, apply_gateway_sync_reply},
    relationship::{
        PaymentRelationshipReply, PaymentRelationshipReplyError, PaymentRelationshipReplyLimits,
        decode_payment_relationship_reply,
    },
};

/// Wrong request kind or response source, checked before inspecting payload bytes.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum QueryReplyBindingError {
    /// The selected decoder does not match the original request's method.
    #[error("Cashier reply method mismatch")]
    MethodMismatch,
    /// Trusted transport context identifies a different Cashier.
    #[error("Cashier reply source mismatch")]
    SourceMismatch,
}

/// Failed balance inspection, without a substitute zero balance.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BoundBalanceReplyError {
    /// Original request or response-source mismatch.
    #[error(transparent)]
    Binding(#[from] QueryReplyBindingError),
    /// Bounded payload decoding or account/amount validation failed.
    #[error(transparent)]
    Reply(#[from] BalanceReplyError),
}

/// Failed relationship inspection, never an absent relationship or fallback payer.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BoundRelationshipReplyError {
    /// Original request or response-source mismatch.
    #[error(transparent)]
    Binding(#[from] QueryReplyBindingError),
    /// Bounded payload decoding or owner/payer validation failed.
    #[error(transparent)]
    Reply(#[from] PaymentRelationshipReplyError),
}

/// Rejected gateway observation; membership and pending state remain unchanged.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BoundGatewayReplyError {
    /// Original request method or response-source mismatch.
    #[error(transparent)]
    Binding(#[from] QueryReplyBindingError),
    /// Registry correlation, bounded decoding or list validation failed.
    #[error(transparent)]
    Reply(#[from] GatewayReplyError),
}

/// Unusable audit inspection; never an empty history or a settled payment.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum BoundAuditLogReplyError {
    /// Original method or trusted response-source context differs.
    #[error(transparent)]
    Binding(#[from] QueryReplyBindingError),
    /// Bounded decoding or reported page size failed validation.
    #[error(transparent)]
    Reply(#[from] AuditLogReplyError),
}

impl CashierQueryRequest {
    /// Inspect an audit reply using the original target and requested page bound.
    ///
    /// The stricter of requested and supplied reported-count limits applies. The
    /// payload has no independently verifiable account/filter binding; CSV and
    /// cursor fields remain opaque observations. The host must correlate the
    /// transport to this exact request and must not treat the result as payment
    /// evidence or permission to fetch another page.
    /// # Errors
    /// Rejects wrong method/source and unusable or over-limit audit reports.
    pub fn decode_audit_log_reply(
        &self,
        response_cashier: Principal,
        bytes: &[u8],
        mut limits: AuditLogReplyLimits,
    ) -> Result<AuditLogReply, BoundAuditLogReplyError> {
        let CashierQuery::AuditLog(input) = self.query() else {
            return Err(QueryReplyBindingError::MethodMismatch.into());
        };
        self.check_source(response_cashier)?;
        limits.max_reported_entries = limits.max_reported_entries.min(input.max_entries);
        Ok(decode_audit_log_reply(bytes, limits)?)
    }

    /// Apply a gateway reply through the exact pending registry sync.
    ///
    /// `response_scope` is trusted transport/installation context, not a payload
    /// assertion. Its Cashier must match the original encoded request. Registry
    /// scope and token checks then precede parsing. Success consumes the pending
    /// attempt; every failure preserves membership and the pending attempt.
    /// This cannot authenticate transport, resume a restored instance or undo
    /// revocation with a stale token. The host still owns lifecycle/recovery fences.
    /// # Errors
    /// Rejects wrong method/source, registry scope or token, and unusable payloads.
    pub fn apply_gateway_sync_reply(
        &self,
        registry: &mut GatewayRegistry,
        token: GatewaySyncToken,
        response_scope: GatewayScope,
        bytes: &[u8],
        limits: GatewayReplyLimits,
    ) -> Result<(), BoundGatewayReplyError> {
        if self.query() != CashierQuery::StorageGateways {
            return Err(QueryReplyBindingError::MethodMismatch.into());
        }
        self.check_source(response_scope.cashier())?;
        Ok(apply_gateway_sync_reply(
            registry,
            token,
            response_scope,
            bytes,
            limits,
        )?)
    }

    /// Decode a balance reply using the account retained when encoding the request.
    ///
    /// Method/source checks precede all byte processing. The caller must establish
    /// the source using its transport and preserve this request across the await.
    /// # Errors
    /// Rejects a different query kind or source, then the underlying decoder's errors.
    pub fn decode_balance_reply(
        &self,
        response_cashier: Principal,
        bytes: &[u8],
        limits: BalanceReplyLimits,
    ) -> Result<BalanceReply, BoundBalanceReplyError> {
        let CashierQuery::Balance { account } = self.query() else {
            return Err(QueryReplyBindingError::MethodMismatch.into());
        };
        self.check_source(response_cashier)?;
        Ok(decode_balance_reply(bytes, account, limits)?)
    }

    /// Decode a relationship reply using both original owner and expected payer.
    ///
    /// An absent compatible relationship remains only an observation. This cannot
    /// select self-payment, activate configuration or authorize account linking.
    /// # Errors
    /// Rejects a different query kind or source, then the underlying decoder's errors.
    pub fn decode_relationship_reply(
        &self,
        response_cashier: Principal,
        bytes: &[u8],
        limits: PaymentRelationshipReplyLimits,
    ) -> Result<PaymentRelationshipReply, BoundRelationshipReplyError> {
        let CashierQuery::PaymentRelationship(binding) = self.query() else {
            return Err(QueryReplyBindingError::MethodMismatch.into());
        };
        self.check_source(response_cashier)?;
        Ok(decode_payment_relationship_reply(bytes, binding, limits)?)
    }

    fn check_source(&self, source: Principal) -> Result<(), QueryReplyBindingError> {
        if source != self.cashier() {
            return Err(QueryReplyBindingError::SourceMismatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
