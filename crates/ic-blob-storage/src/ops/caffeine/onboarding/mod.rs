//! Offline account-link encoding, never account creation, funding or dispatch.
//! The schema is the maintained Cashier Candid. Explicit raw limits/expiry do not
//! establish their units, enforcement, caller authority or a financial guarantee.
use super::wire;
use candid::{Int, Principal};
use std::num::{NonZeroU64, NonZeroU128};
use thiserror::Error;

/// Exact planned target and principals; no identity or authority is allocated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountLinkBinding {
    /// Selected provider target, not discovered or contacted here.
    pub cashier: Principal,
    /// Intended actual signer, retained locally rather than encoded as authority.
    pub caller: Principal,
    /// Canister whose provider usage would be paid.
    pub paid_canister: Principal,
    /// Explicit payer, never inferred from the caller or paid canister.
    pub payment_account: Principal,
}

/// Restricted proposed provider inputs, with no unlimited or indefinite form.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountLinkTerms {
    /// Positive raw signed-provider limit, bounded to a local u128 input.
    pub daily_limit: NonZeroU128,
    /// Positive raw provider timestamp; units/validity are not inferred.
    pub expiration_timestamp: NonZeroU64,
}

/// Canonical arguments and planned context for separate human review.
/// This is not an effect journal, receipt, permit or safe retry token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountLinkRequest {
    binding: AccountLinkBinding,
    terms: AccountLinkTerms,
    arguments: Vec<u8>,
}
impl AccountLinkRequest {
    /// Validate explicit principals and encode the maintained provider request.
    /// # Errors
    /// Refuses anonymous/management roles and any Candid encoding failure.
    pub fn new(
        binding: AccountLinkBinding,
        terms: AccountLinkTerms,
    ) -> Result<Self, AccountLinkError> {
        for (value, role) in [
            (binding.cashier, AccountLinkRole::Cashier),
            (binding.caller, AccountLinkRole::Caller),
            (binding.paid_canister, AccountLinkRole::PaidCanister),
            (binding.payment_account, AccountLinkRole::PaymentAccount),
        ] {
            if value == Principal::anonymous() || value == Principal::management_canister() {
                return Err(AccountLinkError::InvalidPrincipal(role));
            }
        }
        let arguments = candid::encode_args((wire::PaymentAccountCanisterAddRequest {
            spending_limit_per_day: Int::from(terms.daily_limit.get()),
            paid_canister: binding.paid_canister,
            expiration_timestamp: Some(terms.expiration_timestamp.get()),
            payment_account: Some(binding.payment_account),
        },))
        .map_err(|_| AccountLinkError::Encoding)?;
        Ok(Self {
            binding,
            terms,
            arguments,
        })
    }
    /// Exact maintained mutation method; this value never submits it.
    #[must_use]
    pub const fn method_name(&self) -> &'static str {
        "payment_account_canister_add_v1"
    }
    /// Original planned identities, not authenticated by encoding.
    #[must_use]
    pub const fn binding(&self) -> AccountLinkBinding {
        self.binding
    }
    /// Original explicit raw terms; no allowance arithmetic or unit conversion.
    #[must_use]
    pub const fn terms(&self) -> AccountLinkTerms {
        self.terms
    }
    /// Complete Candid arguments, not a blob argument containing encoded Candid.
    #[must_use]
    pub fn arguments(&self) -> &[u8] {
        &self.arguments
    }
}

/// Principal role refused before any usable arguments are produced.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountLinkRole {
    /// Selected provider target.
    Cashier,
    /// Planned actual signer.
    Caller,
    /// Canister whose usage would be paid.
    PaidCanister,
    /// Explicit payment account.
    PaymentAccount,
}
/// No default payer, sentinel or fallback request is substituted on failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum AccountLinkError {
    /// Anonymous/management principal supplied for this role.
    #[error("invalid account-link principal: {0:?}")]
    InvalidPrincipal(AccountLinkRole),
    /// Fixed provider arguments could not be encoded.
    #[error("account-link encoding failed")]
    Encoding,
}

#[cfg(test)]
mod tests;
