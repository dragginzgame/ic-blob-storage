//! Bounded passive assessment replies after independent service authentication.
use crate::dto::funding::assessment::{
    FundingPreparationBlocker as B, FundingPreparationFailure as E,
    FundingPreparationRequest as Request, FundingPreparationResponse as Response,
};
use std::num::NonZeroUsize;
use thiserror::Error;

/// Unusable assessment or authenticated refusal, never funding authorization.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FundingAssessmentReplyError {
    /// Encoded reply exceeds the caller's buffering limit.
    #[error("funding assessment reply exceeds limit")]
    Limit,
    /// Malformed encoding, inconsistent journal or missing/contradictory blockers.
    #[error("invalid funding assessment reply")]
    Invalid,
    /// The echoed scope, operation, amount or optional target differs.
    #[error("funding assessment reply binding mismatch")]
    Binding,
    /// Refusal from the independently authenticated service.
    #[error("funding assessment refused: {0:?}")]
    Remote(E),
}

/// Inspect the current passive diagnostic profile for the caller's original request.
/// The caller owns request admission, authentication and ingress buffering. This
/// checks exact correspondence and local consistency; it grants no preparation,
/// dispatch, retry, provider-credit or spendability authority.
/// # Errors
/// Refuses oversized/malformed replies, changed requests and inconsistent blockers.
pub fn inspect(
    request: Request,
    bytes: &[u8],
    max_bytes: NonZeroUsize,
) -> Result<Response, FundingAssessmentReplyError> {
    if bytes.len() > max_bytes.get() {
        return Err(FundingAssessmentReplyError::Limit);
    }
    let response: Result<Response, E> = crate::wire::decode(bytes, max_bytes.get())
        .map_err(|_| FundingAssessmentReplyError::Invalid)?;
    let response = response.map_err(FundingAssessmentReplyError::Remote)?;
    if response.request != request {
        return Err(FundingAssessmentReplyError::Binding);
    }
    validate(&response)?;
    Ok(response)
}
fn validate(response: &Response) -> Result<(), FundingAssessmentReplyError> {
    let journal = response.journal;
    let blockers = &response.blockers;
    if blockers.len() > 10
        || blockers
            .iter()
            .enumerate()
            .any(|(i, b)| blockers[..i].contains(b))
        || !super::valid_journal(&journal)
    {
        return Err(FundingAssessmentReplyError::Invalid);
    }
    for b in [
        B::ProviderUnqualified,
        B::RecoveryUnknown,
        B::FundingUnknown,
        B::SpendabilityUnknown,
    ] {
        if !blockers.contains(&b) {
            return Err(FundingAssessmentReplyError::Invalid);
        }
    }
    for (b, needed) in [
        (B::JournalFenced, journal.fenced),
        (
            B::JournalUncredited,
            journal.uncredited_accepted > 0 || journal.reserved_or_uncertain > 0,
        ),
        (
            B::JournalFull,
            journal.retained_intents == journal.intent_capacity,
        ),
    ] {
        if blockers.contains(&b) != needed {
            return Err(FundingAssessmentReplyError::Invalid);
        }
    }
    let retained = blockers.contains(&B::IdentityRetained);
    let stale = blockers.contains(&B::IdentityStale);
    if retained && stale
        || (retained || stale)
            != journal
                .last_operation
                .is_some_and(|last| response.request.operation <= last)
        || journal.last_operation == Some(response.request.operation) && !retained
    {
        return Err(FundingAssessmentReplyError::Invalid);
    }
    let mut reserve = None;
    for b in blockers {
        if let B::AllocationReserve {
            transferable_cycles,
        } = b
            && reserve.replace(*transferable_cycles).is_some()
        {
            return Err(FundingAssessmentReplyError::Invalid);
        }
    }
    if (response.request.offered > journal.attachment_allowance) != reserve.is_some()
        || reserve.is_some_and(|n| n != journal.attachment_allowance)
    {
        return Err(FundingAssessmentReplyError::Invalid);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
