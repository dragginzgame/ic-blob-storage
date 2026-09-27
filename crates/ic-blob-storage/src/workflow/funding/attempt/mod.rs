//! Exact first-attempt admission; transport and post-write liquidity stay explicit.
use crate::{
    model::{billing::journal::FundingIntent, service::upload::UploadContext},
    ops::{
        caffeine::funding::request::CashierTopUpRequest,
        service::funding::{FundingJournalError, StableFundingJournal},
    },
    policy::billing::admission::attempt::{
        FundingAttemptAssessment, FundingAttemptEvidence, FundingAttemptObservation,
        assess_first_attempt,
    },
};
use ic_memory::ic_stable_structures::Memory;

/// Synchronous result. No call is sent and no retry authority is created.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FundingAttemptResult {
    /// No journal mutation; the original reservation remains charged.
    Blocked(FundingAttemptAssessment),
    /// The exact intent is now uncertain. Use the canonical request only after
    /// same-message post-write liquidity checks. A dropped result does not permit
    /// another attempt or prove that no effect occurred.
    Marked(CashierTopUpRequest),
}
/// Diagnose an exact retained intent with current, separately bound host evidence.
/// No write, history scan, provider effect or reusable authorization is produced.
/// # Errors
/// Rejects caller, scope, changed request or evidence identity mismatches.
pub fn inspect_attempt<M: Memory>(
    journal: &StableFundingJournal<M>,
    execution: UploadContext,
    input: FundingIntent,
    evidence: FundingAttemptEvidence,
) -> Result<FundingAttemptAssessment, FundingJournalError> {
    let context = journal.admission_context(execution, input)?;
    if evidence.intent != input {
        return Err(FundingJournalError::Binding);
    }
    Ok(assess_first_attempt(
        context.limits,
        FundingAttemptObservation {
            fenced: context.summary.fenced,
            allocation: context.summary.allocation,
            last_operation: context.summary.last_operation,
            state: context.retained.map(|view| view.state),
        },
        evidence,
    ))
}
/// Re-read local state, bind fresh host observations and persist the first marker.
/// Must be composed with transport in the same IC update without an intervening
/// await. After these writes, construct the call and check actual liquid cycles,
/// call cost and complete holds before polling. This function neither acquires
/// host evidence nor guarantees dispatch/credit; it cannot release a restore fence.
/// # Errors
/// Rejects authority, exact identity or underlying journal errors before mutation.
/// # Panics
/// Stable traps must propagate for whole-message rollback.
pub fn mark_first_attempt<M: Memory>(
    journal: &mut StableFundingJournal<M>,
    execution: UploadContext,
    input: FundingIntent,
    evidence: FundingAttemptEvidence,
) -> Result<FundingAttemptResult, FundingJournalError> {
    let assessment = inspect_attempt(journal, execution, input, evidence)?;
    if !assessment.can_mark_attempt() {
        return Ok(FundingAttemptResult::Blocked(assessment));
    }
    Ok(FundingAttemptResult::Marked(
        journal.mark_attempted(execution, input)?,
    ))
}
