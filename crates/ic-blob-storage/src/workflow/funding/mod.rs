//! Synchronous preparation gates, without provider dispatch or automatic retry.
pub mod attempt;
pub mod dispatch;
pub mod history;
pub mod outcome;
use crate::{
    model::{
        billing::journal::{FundingIntent, FundingIntentAdmission},
        service::upload::UploadContext,
    },
    ops::service::funding::{
        FundingJournalError, StableFundingJournal, summary::FundingJournalSummary,
    },
    policy::billing::admission::journal::{
        FundingHostEvidence, FundingJournalAdmissionObservation, FundingPreparationAssessment,
        assess_journal_preparation,
    },
};
use ic_memory::ic_stable_structures::Memory;

/// Exact passive report. It cannot be passed back to the preparation handler.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FundingPreparationView {
    /// Full proposed local identity, amount and target balance.
    pub intent: FundingIntent,
    /// Actual local facts from the journal at inspection time.
    pub journal: FundingJournalSummary,
    /// Independent blockers from local storage and host evidence.
    pub assessment: FundingPreparationAssessment,
}
/// Synchronous preparation result, never authority to dispatch or repeat a call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FundingPreparationResult {
    /// No mutation took place; all blockers remain inspectable.
    Blocked(FundingPreparationAssessment),
    /// The current policy allowed a local reservation; effect gates remain separate.
    Prepared(FundingIntentAdmission),
}
/// Inspect exact identity, local constraints and separately scoped host evidence.
/// Reads no whole-history snapshot and performs no writes or provider effects.
/// # Errors
/// Rejects caller/scope mismatch, invalid request or changed retained identity.
pub fn inspect_preparation<M: Memory>(
    journal: &StableFundingJournal<M>,
    execution: UploadContext,
    input: FundingIntent,
    evidence: FundingHostEvidence,
) -> Result<FundingPreparationView, FundingJournalError> {
    let context = journal.admission_context(execution, input)?;
    if evidence.scope != context.summary.scope {
        return Err(FundingJournalError::Binding);
    }
    let view = context.summary;
    let assessment = assess_journal_preparation(
        Some(context.limits),
        input.operation,
        input.offered,
        FundingJournalAdmissionObservation {
            allocation: view.allocation,
            fenced: view.fenced,
            retained_intents: view.retained_intents,
            intent_capacity: view.intent_capacity,
            last_operation: view.last_operation,
            already_retained: context.retained.is_some(),
        },
        evidence.provider_qualified,
        evidence.admission,
    );
    Ok(FundingPreparationView {
        intent: input,
        journal: view,
        assessment,
    })
}
/// Re-read the journal and assess fresh host observations before a new reservation.
/// All checks and writes run synchronously in one update. The host must establish
/// evidence completeness/freshness in that same execution, including linked payers
/// and other service installations. This neither establishes that evidence nor
/// releases a restored fence. A later dispatch still needs its own current gates.
/// # Errors
/// Rejects authority/binding/identity or underlying storage/allocation errors.
/// # Panics
/// Stable traps must propagate to preserve whole-message IC rollback.
pub fn prepare_new<M: Memory>(
    journal: &mut StableFundingJournal<M>,
    execution: UploadContext,
    input: FundingIntent,
    evidence: FundingHostEvidence,
) -> Result<FundingPreparationResult, FundingJournalError> {
    let report = inspect_preparation(journal, execution, input, evidence)?;
    if !report.assessment.can_prepare_intent() {
        return Ok(FundingPreparationResult::Blocked(report.assessment));
    }
    Ok(FundingPreparationResult::Prepared(
        journal.prepare(execution, input)?,
    ))
}
