//! Bounded authenticated funding observations; no retry, credit or restore authority.
pub mod assessment;
pub mod outcome;
use crate::dto::funding::FundingHistoryFailure;
use crate::dto::funding::FundingHistoryPage;
use crate::dto::funding::FundingHistoryRequest;
use crate::dto::funding::FundingPhase;
use crate::dto::funding::outcome::FundingOutcomeFailure;
use crate::dto::funding::outcome::FundingOutcomeRequest;
use crate::dto::operator::{LocalFundingStatus, OperatorScope};
use crate::funding::transfer::FundingTransfer;
use ic_platform::Principal;
use serde::Deserialize;
use std::num::NonZeroU128;
use std::num::NonZeroUsize;
use thiserror::Error;

/// Reply refusal, preserved separately from a valid empty observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum FundingReplyError {
    /// Encoded bytes or decoded page exceeds the caller's limit.
    #[error("funding reply exceeds limit")]
    Limit,
    /// Malformed encoding, invalid amounts, ordering or inconsistent observations.
    #[error("invalid funding request or reply")]
    Invalid,
    /// Reply or cursor differs from the exact requested scope/intent.
    #[error("funding reply binding mismatch")]
    Binding,
    /// Authenticated service refused history inspection.
    #[error("funding history refused: {0:?}")]
    History(FundingHistoryFailure),
    /// Authenticated service refused exact inspection.
    #[error("funding outcome refused: {0:?}")]
    Outcome(FundingOutcomeFailure),
}
/// Independent byte and entry limits. These do not change the service's page size.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingHistoryReplyLimits {
    /// Applied before Wire decoding; transport must separately bound buffering.
    pub bytes: NonZeroUsize,
    /// Maximum entries accepted after bounded decoding.
    pub entries: NonZeroUsize,
}
/// Validate passive input before an independently authenticated adapter call.
/// # Errors
/// Rejects malformed or differently bound input; grants no dispatch authority.
pub fn check_scope(scope: OperatorScope) -> Result<(), FundingReplyError> {
    if scope.namespace == 0
        || [scope.service, scope.cashier, scope.payment_account]
            .into_iter()
            .any(|p| p == Principal::anonymous() || p == Principal::management_canister())
    {
        return Err(FundingReplyError::Invalid);
    }
    Ok(())
}
/// Validate passive input before an independently authenticated adapter call.
/// # Errors
/// Rejects malformed or differently bound input; grants no dispatch authority.
pub fn check_history(input: FundingHistoryRequest) -> Result<(), FundingReplyError> {
    check_scope(input.scope)?;
    if let Some(cursor) = input.cursor {
        if cursor.scope != input.scope {
            return Err(FundingReplyError::Binding);
        }
        if cursor.before_operation == 0 {
            return Err(FundingReplyError::Invalid);
        }
    }
    Ok(())
}
/// Validate passive input before an independently authenticated adapter call.
/// # Errors
/// Rejects malformed or differently bound input; grants no dispatch authority.
pub fn check_outcome(input: FundingOutcomeRequest) -> Result<(), FundingReplyError> {
    check_scope(input.scope)?;
    if input.operation == 0 || input.offered == 0 || input.target_balance == Some(0) {
        return Err(FundingReplyError::Invalid);
    }
    Ok(())
}
fn decode<T: for<'de> Deserialize<'de>>(
    method: &str,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<T, FundingReplyError> {
    if bytes.len() > max.get() {
        return Err(FundingReplyError::Limit);
    }
    crate::wire::decode_response(method, bytes, max.get()).map_err(|_| FundingReplyError::Invalid)
}
fn transfer(offered: u128, phase: FundingPhase) -> Result<FundingTransfer, FundingReplyError> {
    let offered = NonZeroU128::new(offered).ok_or(FundingReplyError::Invalid)?;
    match phase {
        FundingPhase::Prepared | FundingPhase::Uncertain => Ok(FundingTransfer::unknown(offered)),
        FundingPhase::NotEnqueued => Ok(FundingTransfer::not_enqueued(offered)),
        FundingPhase::Callback { refunded } => {
            FundingTransfer::unbounded_callback(offered, refunded)
                .map_err(|_| FundingReplyError::Invalid)
        }
    }
}
/// Decode one descending page from an independently authenticated service.
/// Checks exact request, scopes, positive original amounts, refunds, strict ordering
/// and continuation at the last returned operation. A page is not a snapshot;
/// changes behind its cursor require a fresh sweep. No call or state change occurs.
/// # Errors
/// Rejects malformed/over-budget replies, mismatched requests and service refusal.
pub fn history(
    input: FundingHistoryRequest,
    bytes: &[u8],
    limits: FundingHistoryReplyLimits,
) -> Result<FundingHistoryPage, FundingReplyError> {
    check_history(input)?;
    let page = decode::<Result<FundingHistoryPage, FundingHistoryFailure>>(
        "blob_funding_history",
        bytes,
        limits.bytes,
    )?
    .map_err(FundingReplyError::History)?;
    if page.request != input {
        return Err(FundingReplyError::Binding);
    }
    if page.entries.len() > limits.entries.get() {
        return Err(FundingReplyError::Limit);
    }
    let mut before = input.cursor.map(|c| c.before_operation);
    for entry in &page.entries {
        if entry.scope != input.scope {
            return Err(FundingReplyError::Binding);
        }
        check_outcome(FundingOutcomeRequest {
            scope: entry.scope,
            operation: entry.operation,
            offered: entry.offered,
            target_balance: entry.target_balance,
        })?;
        if before.is_some_and(|previous| entry.operation >= previous) {
            return Err(FundingReplyError::Invalid);
        }
        transfer(entry.offered, entry.phase)?;
        before = Some(entry.operation);
    }
    if let Some(next) = page.next {
        if next.scope != input.scope {
            return Err(FundingReplyError::Binding);
        }
        if next.before_operation == 0
            || page.entries.last().map(|e| e.operation) != Some(next.before_operation)
        {
            return Err(FundingReplyError::Invalid);
        }
    }
    Ok(page)
}
#[cfg(test)]
mod tests;

/// Check local allocation arithmetic only; this establishes no liquidity or credit.
#[must_use]
pub fn valid_budget(status: &LocalFundingStatus) -> bool {
    status.available_allocation <= status.cumulative_allocation
        && status.cumulative_allocation <= status.renewal_ceiling
        && status
            .transport_accepted
            .checked_add(status.reserved_or_uncertain)
            == status
                .cumulative_allocation
                .checked_sub(status.available_allocation)
}

/// Check a local journal report's accounting and retained-identity consistency.
/// This establishes no provider credit, liquidity, readiness or dispatch authority.
#[must_use]
pub fn valid_journal(status: &LocalFundingStatus) -> bool {
    valid_budget(status)
        && status.intent_capacity > 0
        && status.retained_intents <= status.intent_capacity
        && (status.retained_intents == 0) == status.last_operation.is_none()
        && status.last_operation != Some(0)
        && status.attachment_allowance <= status.available_allocation
        && status.uncredited_accepted <= status.transport_accepted
        && status.provider_acknowledged <= status.uncredited_accepted
}
