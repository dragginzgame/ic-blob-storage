//! Correlation and consistency checks for one passive exact observation.
use crate::dto::funding::FundingPhase;
use crate::dto::funding::outcome::FundingOutcomeFailure;
use crate::dto::funding::outcome::FundingOutcomeRequest;
use crate::dto::funding::outcome::FundingOutcomeResponse;
use crate::dto::funding::outcome::FundingReconciliation;
use crate::dto::funding::outcome::FundingResponse;
use crate::funding::reply::FundingReplyError;
use crate::funding::reply::check_outcome;
use crate::funding::reply::decode;
use crate::funding::reply::transfer;
use std::num::NonZeroUsize;

/// Validate and encode the exact original intent for passive outcome inspection.
/// This creates no funding operation and grants no retry authority.
/// # Errors
/// Rejects invalid scope, zero identities/amounts or encoding failure.
pub fn inspection_request(input: FundingOutcomeRequest) -> Result<Vec<u8>, FundingReplyError> {
    check_outcome(input)?;
    candid::encode_one(input).map_err(|_| FundingReplyError::Invalid)
}

/// Decode a response from the independently authenticated selected service.
/// Absence, missing structured response and restore fences remain distinct.
/// Checks exact original intent and reported reconciliation against transport
/// amounts; reported provider balances still prove neither credit nor freshness.
/// # Errors
/// Rejects invalid input, oversized/malformed/inconsistent replies or service refusal.
pub fn inspect(
    input: FundingOutcomeRequest,
    bytes: &[u8],
    max: NonZeroUsize,
) -> Result<Option<FundingOutcomeResponse>, FundingReplyError> {
    check_outcome(input)?;
    let observed =
        decode::<Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure>>(bytes, max)?
            .map_err(FundingReplyError::Outcome)?;
    let Some(view) = observed else {
        return Ok(None);
    };
    if view.request != input {
        return Err(FundingReplyError::Binding);
    }
    let transfer = transfer(input.offered, view.phase)?;
    let consistent = match view.reconciliation {
        FundingReconciliation::CreditConfirmed {
            accepted_cycles,
            receipt_digest,
        } => {
            accepted_cycles > 0
                && transfer.accepted() == Some(accepted_cycles)
                && receipt_digest != [0; 32]
                && view.renewed_allocation <= accepted_cycles
        }
        FundingReconciliation::NoTransfer => transfer.accepted() == Some(0),
        FundingReconciliation::CreditRequired(amount) => {
            amount > 0 && transfer.accepted() == Some(amount)
        }
        FundingReconciliation::TransferUnknown(amount) => {
            amount == input.offered && transfer.accepted().is_none()
        }
    };
    let response_matches = match view.response {
        None => true,
        Some(FundingResponse::NotDispatched | FundingResponse::NotEnqueued) => {
            view.phase == FundingPhase::NotEnqueued
        }
        Some(_) => matches!(view.phase, FundingPhase::Callback { .. }),
    };
    let renewal_matches = view.renewed_allocation == 0
        || matches!(
            view.reconciliation,
            FundingReconciliation::CreditConfirmed { .. }
        );
    if !consistent || !response_matches || !renewal_matches {
        return Err(FundingReplyError::Invalid);
    }
    Ok(Some(view))
}
