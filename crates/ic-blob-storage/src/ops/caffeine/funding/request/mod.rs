//! Explicit top-up request encoding for the retained Cashier Candid contract.
//! No dispatch, retry or credit authority; the service owns operation correlation.
use candid::{CandidType, Nat, Principal};
use std::num::NonZeroU128;
use thiserror::Error;

/// Exact `account_top_up_v1` request, never an instruction to issue a paid call.
///
/// Both optional wire envelopes are explicit: the request and account are always
/// present; target balance retains the supplied option. This never selects an
/// account from the eventual caller. Local operation IDs and namespaces are not
/// supported by this provider method and are not invented as wire fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashierTopUpRequest {
    cashier: Principal,
    account: Principal,
    offered: NonZeroU128,
    target_balance: Option<NonZeroU128>,
    arguments: Vec<u8>,
}
impl CashierTopUpRequest {
    /// Encode a bounded fixed-shape request without contacting the provider.
    /// # Errors
    /// Rejects anonymous/management target or account and Candid encoding failure.
    pub fn new(
        cashier: Principal,
        account: Principal,
        offered: NonZeroU128,
        target_balance: Option<NonZeroU128>,
    ) -> Result<Self, TopUpRequestError> {
        for principal in [cashier, account] {
            if principal == Principal::anonymous() || principal == Principal::management_canister()
            {
                return Err(TopUpRequestError::InvalidPrincipal);
            }
        }
        let arguments = candid::encode_args((Some(wire::AccountTopUpRequest {
            account: Some(account),
            target_balance: target_balance.map(|v| Nat::from(v.get())),
        }),))
        .map_err(|_| TopUpRequestError::Encoding)?;
        Ok(Self {
            cashier,
            account,
            offered,
            target_balance,
            arguments,
        })
    }
    /// Configured target, not authenticated deployment evidence.
    #[must_use]
    pub const fn cashier(&self) -> Principal {
        self.cashier
    }
    /// Explicit payment account in the encoded request.
    #[must_use]
    pub const fn account(&self) -> Principal {
        self.account
    }
    /// Exact attachment, separate from requested balance and eventual credit.
    #[must_use]
    pub const fn offered(&self) -> NonZeroU128 {
        self.offered
    }
    /// Original optional target; never inferred from the offered attachment.
    #[must_use]
    pub const fn target_balance(&self) -> Option<NonZeroU128> {
        self.target_balance
    }
    /// Maintained method; callers cannot substitute an arbitrary effect.
    #[must_use]
    pub const fn method_name(&self) -> &'static str {
        "account_top_up_v1"
    }
    /// Canonical Candid arguments for this exact request.
    #[must_use]
    pub fn arguments(&self) -> &[u8] {
        &self.arguments
    }
}
/// Invalid fixed-shape request, before any effect or reservation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum TopUpRequestError {
    /// Anonymous/management principals cannot identify the target or account.
    #[error("invalid top-up principal")]
    InvalidPrincipal,
    /// Candid request encoding failed.
    #[error("top-up request encoding failed")]
    Encoding,
}
mod wire {
    use super::{CandidType, Nat, Principal};
    #[derive(CandidType)]
    pub(super) struct AccountTopUpRequest {
        pub account: Option<Principal>,
        pub target_balance: Option<Nat>,
    }
}
#[cfg(test)]
mod tests;
