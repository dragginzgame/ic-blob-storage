//! Host-internal reconciliation of exact, independently authenticated credit.
use crate::model::billing::journal::FundingIntent;
use crate::model::billing::journal::credit::FundingCreditConfirmation;
use crate::ops::service::funding::FundingJournalError;
use crate::ops::service::funding::StableFundingJournal;
use crate::ops::service::funding::outcome::FundingOutcomeView;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

/// One synchronous reconciliation result, never permission to repeat a payment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingCreditResult {
    /// Host could not establish exact independent credit; no journal writes.
    NotEstablished,
    /// The immutable receipt and outstanding totals committed together.
    Confirmed,
    /// Identical already retained receipt; accounting did not change again.
    AlreadyConfirmed,
}

/// Authenticate the exact live owner before asking the trusted host to establish
/// credit. The closure must validate provider authority and unique attribution to
/// the original account/intent, independently of a balance or decoded top-up reply.
/// It must not accept caller flags or ingress facts. Preserve original evidence
/// outside the bounded receipt fingerprint. Historical immutable evidence need
/// not be reacquired after a lost confirmation reply; exact replay is harmless.
/// This creates no endpoint, timer, provider contract, retry or restore authority.
/// # Errors
/// Rejects scope, operator, fence, unknown/unaccepted intent or receipt conflicts.
/// # Panics
/// Stable traps must propagate to preserve whole-message IC rollback.
pub fn confirm<M: Memory>(
    journal: &mut StableFundingJournal<M>,
    execution: UploadContext,
    input: FundingIntent,
    establish: impl FnOnce(&FundingOutcomeView) -> Option<FundingCreditConfirmation>,
) -> Result<FundingCreditResult, FundingJournalError> {
    let view = journal.credit_candidate(execution, input)?;
    let Some(confirmation) = establish(&view) else {
        return Ok(FundingCreditResult::NotEstablished);
    };
    if confirmation.intent != input {
        return Err(FundingJournalError::Binding);
    }
    Ok(if journal.record_credit(execution, confirmation)? {
        FundingCreditResult::Confirmed
    } else {
        FundingCreditResult::AlreadyConfirmed
    })
}
