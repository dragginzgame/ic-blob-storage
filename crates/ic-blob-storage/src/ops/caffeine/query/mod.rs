//! Explicit read-only Cashier request encoding, without issuing a call.
//!
//! Method and arguments have one owner here. Targets and accounts are supplied
//! explicitly; a request proves neither account authority nor provider identity.
//! Installation, tenant, namespace, revision, attempt and response-source binding
//! remain the workflow's responsibility. Reconstructing a request is not retry
//! permission. Schema: `docs/evidence/caffeine-cashier.did`.

use candid::Principal;
use thiserror::Error;

use super::relationship::PaymentRelationshipBinding;

pub mod audit;
pub mod reply;
pub mod transport;

/// Maintained queries only; no arbitrary method, funding or account-link operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CashierQuery {
    /// Read this exact account's reported balances; never infer self-payment.
    Balance {
        /// Explicit requested account, independent of the caller and Cashier.
        account: Principal,
    },
    /// Query the paid canister and retain the expected payer for response checks.
    PaymentRelationship(PaymentRelationshipBinding),
    /// Read the provider's gateway list; the result alone grants no membership.
    StorageGateways,
    /// Inspect one bounded page for an explicit account; no automatic pagination.
    AuditLog(audit::AuditLogQuery),
}

/// Encoded query with its exact target and response expectations retained together.
///
/// There is no transport or attached-cycle amount. Callers must use query transport
/// when inspecting off-chain; this value never authorizes falling back to an update.
/// A canister workflow may separately qualify replicated read-only calls.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashierQueryRequest {
    cashier: Principal,
    query: CashierQuery,
    arguments: Vec<u8>,
}

impl CashierQueryRequest {
    /// Encode the selected query after validating every supplied principal.
    ///
    /// Argument size is bounded by fixed records and IC principal sizes. The payer
    /// in a relationship query is a local expectation: the wire request contains
    /// only the paid canister, as specified by the maintained Cashier interface.
    /// # Errors
    /// Rejects anonymous/management principals, a cursor naming a different audit
    /// account, or a Candid encoding failure.
    pub fn new(cashier: Principal, query: CashierQuery) -> Result<Self, CashierQueryError> {
        validate(cashier, CashierQueryPrincipal::Cashier)?;
        let arguments = match query {
            CashierQuery::Balance { account } => {
                validate(account, CashierQueryPrincipal::BalanceAccount)?;
                candid::encode_args((wire::AccountGetRequest { account },))
            }
            CashierQuery::PaymentRelationship(binding) => {
                validate(binding.paid_canister, CashierQueryPrincipal::PaidCanister)?;
                validate(
                    binding.payment_account,
                    CashierQueryPrincipal::PaymentAccount,
                )?;
                candid::encode_args((wire::PaymentAccountCanisterGetRequest {
                    canister: binding.paid_canister,
                },))
            }
            CashierQuery::StorageGateways => candid::encode_args(()),
            CashierQuery::AuditLog(input) => Ok(audit::encode(input)?),
        }
        .map_err(|_| CashierQueryError::EncodingFailed)?;
        Ok(Self {
            cashier,
            query,
            arguments,
        })
    }

    /// Explicit target; not authenticated or contacted by this value.
    #[must_use]
    pub const fn cashier(&self) -> Principal {
        self.cashier
    }

    /// Original selection, including the expected account or owner/payer pair.
    #[must_use]
    pub const fn query(&self) -> CashierQuery {
        self.query
    }

    /// Exact maintained method corresponding to these arguments.
    #[must_use]
    pub const fn method_name(&self) -> &'static str {
        match self.query {
            CashierQuery::Balance { .. } => "account_balance_get_v1",
            CashierQuery::PaymentRelationship(_) => "payment_account_canister_get_v1",
            CashierQuery::StorageGateways => "storage_gateway_list_v1",
            CashierQuery::AuditLog(_) => "payment_account_audit_log_get_v1",
        }
    }

    /// Complete Candid argument sequence; pass as raw arguments, not as a blob argument.
    #[must_use]
    pub fn arguments(&self) -> &[u8] {
        &self.arguments
    }
}

/// Principal field rejected before any query bytes are exposed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CashierQueryPrincipal {
    /// Configured provider target.
    Cashier,
    /// Account whose balance was requested.
    BalanceAccount,
    /// Usage owner named by a relationship request.
    PaidCanister,
    /// Independently expected relationship payer.
    PaymentAccount,
    /// Explicit audit account; unscoped provider-wide requests are unavailable.
    AuditAccount,
}

/// An invalid request is not replaced with defaults or another payment route.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CashierQueryError {
    /// Anonymous and management principals cannot fill the selected role.
    #[error("invalid Cashier query principal: {field:?}")]
    InvalidPrincipal {
        /// Rejected role.
        field: CashierQueryPrincipal,
    },
    /// Candid failed to encode the fixed argument sequence.
    #[error("Cashier query encoding failed")]
    EncodingFailed,
    /// A supplied cursor names an account other than the explicit audit account.
    #[error("audit cursor account mismatch")]
    AuditCursorAccountMismatch,
}

fn validate(value: Principal, field: CashierQueryPrincipal) -> Result<(), CashierQueryError> {
    if value == Principal::anonymous() || value == Principal::management_canister() {
        return Err(CashierQueryError::InvalidPrincipal { field });
    }
    Ok(())
}

mod wire {
    use candid::{CandidType, Principal};

    #[derive(CandidType)]
    pub(super) struct AccountGetRequest {
        pub account: Principal,
    }

    #[derive(CandidType)]
    pub(super) struct PaymentAccountCanisterGetRequest {
        pub canister: Principal,
    }
}

#[cfg(test)]
mod tests;
