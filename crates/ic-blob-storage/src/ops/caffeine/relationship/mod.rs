//! Bounded inspection of `payment_account_canister_get_v1` responses.
//!
//! A linked payer is separate from the canister owning blobs and callbacks.
//! This decoder issues no request, selects no account and grants no spending,
//! retry, migration or retirement authority. Transport/source/freshness binding
//! remains the caller's responsibility. Schema: `docs/evidence/caffeine-cashier.did`.

use std::num::NonZeroUsize;

use candid::{Int, Nat, Principal, de::DecoderConfig, decode_one_with_config};
use thiserror::Error;

/// Expected relationship from trusted installation/request context, never defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaymentRelationshipBinding {
    /// Canister whose provider usage is paid; not the tenant or project hub.
    pub paid_canister: Principal,
    /// Independently selected payer; may equal the paid canister.
    pub payment_account: Principal,
}

/// Explicit decoder bounds; transport buffering needs a separate bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaymentRelationshipReplyLimits {
    /// Maximum supplied bytes, checked before decoding.
    pub max_bytes: NonZeroUsize,
    /// Maximum Candid decoding work.
    pub decoding_quota: NonZeroUsize,
    /// Maximum work skipping additional fields or arguments.
    pub skipping_quota: NonZeroUsize,
    /// Maximum Candid type-table entries.
    pub max_type_entries: NonZeroUsize,
}

/// Uninterpreted provider figures, bounded by reply size and decoder work.
///
/// Signed values are retained exactly: negative limits are not assumed unlimited,
/// and period spend is not subtracted to manufacture a spendable balance. Timestamps
/// are not compared to a local clock or interpreted as proof of current validity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentRelationshipView {
    binding: PaymentRelationshipBinding,
    spending_limit_per_day: Int,
    current_period_spent: Int,
    current_period_start: u64,
    added_timestamp: u64,
    expiration_timestamp: Option<u64>,
    bandwidth_baseline_uploaded: Nat,
    bandwidth_baseline_downloaded: Nat,
    bandwidth_baseline_ts_ns: u64,
}

impl PaymentRelationshipView {
    /// Both principals matched against the expected request context.
    #[must_use]
    pub const fn binding(&self) -> PaymentRelationshipBinding {
        self.binding
    }

    /// Exact reported signed daily limit, with no assumed sentinel semantics.
    #[must_use]
    pub const fn spending_limit_per_day(&self) -> &Int {
        &self.spending_limit_per_day
    }

    /// Exact reported signed spend, independent of the limit and account balance.
    #[must_use]
    pub const fn current_period_spent(&self) -> &Int {
        &self.current_period_spent
    }

    /// Provider-reported period start, without a local freshness verdict.
    #[must_use]
    pub const fn current_period_start(&self) -> u64 {
        self.current_period_start
    }

    /// Provider-reported creation timestamp.
    #[must_use]
    pub const fn added_timestamp(&self) -> u64 {
        self.added_timestamp
    }

    /// Provider-reported optional expiry; absence does not establish perpetual authority.
    #[must_use]
    pub const fn expiration_timestamp(&self) -> Option<u64> {
        self.expiration_timestamp
    }

    /// Exact arbitrary-width uploaded baseline; not an object inventory.
    #[must_use]
    pub const fn bandwidth_baseline_uploaded(&self) -> &Nat {
        &self.bandwidth_baseline_uploaded
    }

    /// Exact arbitrary-width downloaded baseline; not outstanding billing liability.
    #[must_use]
    pub const fn bandwidth_baseline_downloaded(&self) -> &Nat {
        &self.bandwidth_baseline_downloaded
    }

    /// Provider-reported bandwidth baseline timestamp in nanoseconds.
    #[must_use]
    pub const fn bandwidth_baseline_ts_ns(&self) -> u64 {
        self.bandwidth_baseline_ts_ns
    }
}

/// Query observations, never installation qualification or funding authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentRelationshipReply {
    /// A present relationship names both expected principals.
    ReportedRelationship(PaymentRelationshipView),
    /// No compatible relationship was returned. Candid optional-field subtyping
    /// can also produce this result; it does not prove self-payment or no liability.
    NoRelationshipReported,
    /// An advertised failure, distinct from a successful optional response.
    ProviderFailure(PaymentRelationshipProviderError),
}

/// Advertised errors; diagnostic text is discarded rather than exposed to clients.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaymentRelationshipProviderError {
    /// Reported missing relationship; the principal is diagnostic, not authority.
    RelationshipNotFound(Principal),
    /// Reported authorization failure; the principal is diagnostic, not authority.
    NotAuthorized(Principal),
    /// Provider rejected the request.
    InvalidRequest,
    /// Provider reported an internal failure.
    InternalError,
}

/// An unusable response never selects a fallback payer or grants spending.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PaymentRelationshipReplyError {
    /// Anonymous/management cannot identify either expected party.
    #[error("invalid payment relationship binding")]
    InvalidBinding,
    /// Byte limit exceeded before decoding.
    #[error("payment relationship reply exceeds byte limit")]
    ReplyTooLarge,
    /// Malformed, incompatible or above configured decoding/work/type bounds.
    #[error("invalid or over-budget payment relationship reply")]
    InvalidReply,
    /// The returned usage owner differs from the requested canister.
    #[error("payment relationship paid canister mismatch")]
    PaidCanisterMismatch,
    /// The returned payer differs from the independently expected account.
    #[error("payment relationship account mismatch")]
    PaymentAccountMismatch,
}

/// Decode a read-only relationship report without calculating available funding.
///
/// `expected` must come from trusted request context, not the reply. An absent or
/// failed relationship must never trigger a self-account fallback. The caller must
/// separately bind the Cashier, service, configuration revision and query response.
/// # Errors
/// Rejects invalid expected principals, oversized/malformed/over-budget replies,
/// and present relationships naming a different paid canister or payer.
pub fn decode_payment_relationship_reply(
    bytes: &[u8],
    expected: PaymentRelationshipBinding,
    limits: PaymentRelationshipReplyLimits,
) -> Result<PaymentRelationshipReply, PaymentRelationshipReplyError> {
    if !valid_principal(expected.paid_canister) || !valid_principal(expected.payment_account) {
        return Err(PaymentRelationshipReplyError::InvalidBinding);
    }
    if bytes.len() > limits.max_bytes.get() {
        return Err(PaymentRelationshipReplyError::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(limits.decoding_quota.get())
        .set_skipping_quota(limits.skipping_quota.get())
        .set_max_type_len(limits.max_type_entries.get())
        .set_full_error_message(false);
    let reply: wire::GetResult = decode_one_with_config(bytes, &config)
        .map_err(|_| PaymentRelationshipReplyError::InvalidReply)?;
    match reply {
        Ok(wire::GetResponse { relationship: None }) => {
            Ok(PaymentRelationshipReply::NoRelationshipReported)
        }
        Ok(wire::GetResponse {
            relationship: Some(value),
        }) => {
            if value.paid_canister != expected.paid_canister {
                return Err(PaymentRelationshipReplyError::PaidCanisterMismatch);
            }
            if value.payment_account != expected.payment_account {
                return Err(PaymentRelationshipReplyError::PaymentAccountMismatch);
            }
            Ok(PaymentRelationshipReply::ReportedRelationship(
                PaymentRelationshipView {
                    binding: expected,
                    spending_limit_per_day: value.spending_limit_per_day,
                    current_period_spent: value.current_period_spent,
                    current_period_start: value.current_period_start,
                    added_timestamp: value.added_timestamp,
                    expiration_timestamp: value.expiration_timestamp,
                    bandwidth_baseline_uploaded: value.bandwidth_baseline_uploaded,
                    bandwidth_baseline_downloaded: value.bandwidth_baseline_downloaded,
                    bandwidth_baseline_ts_ns: value.bandwidth_baseline_ts_ns,
                },
            ))
        }
        Err(value) => Ok(PaymentRelationshipReply::ProviderFailure(match value {
            wire::GetError::RelationshipNotFound(value) => {
                PaymentRelationshipProviderError::RelationshipNotFound(value)
            }
            wire::GetError::NotAuthorized(value) => {
                PaymentRelationshipProviderError::NotAuthorized(value)
            }
            wire::GetError::InvalidRequest(_) => PaymentRelationshipProviderError::InvalidRequest,
            wire::GetError::InternalError(_) => PaymentRelationshipProviderError::InternalError,
        })),
    }
}

fn valid_principal(value: Principal) -> bool {
    value != Principal::anonymous() && value != Principal::management_canister()
}

mod wire {
    use candid::CandidType;
    use candid::Int;
    use candid::Nat;
    use candid::Principal;
    use serde::Deserialize;

    pub(super) type GetResult = Result<GetResponse, GetError>;

    #[derive(CandidType, Deserialize)]
    pub(super) struct GetResponse {
        pub relationship: Option<Relationship>,
    }

    #[derive(CandidType, Deserialize)]
    pub(super) struct Relationship {
        pub added_timestamp: u64,
        pub spending_limit_per_day: Int,
        pub paid_canister: Principal,
        pub bandwidth_baseline_downloaded: Nat,
        pub expiration_timestamp: Option<u64>,
        pub current_period_spent: Int,
        pub current_period_start: u64,
        pub bandwidth_baseline_ts_ns: u64,
        pub payment_account: Principal,
        pub bandwidth_baseline_uploaded: Nat,
    }

    #[derive(CandidType, Deserialize)]
    pub(super) enum GetError {
        RelationshipNotFound(Principal),
        NotAuthorized(Principal),
        InvalidRequest(String),
        InternalError(String),
    }
}

#[cfg(test)]
mod tests;
