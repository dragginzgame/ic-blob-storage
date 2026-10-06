//! Fixed synthetic credit acquisition for a local qualification fixture only.
use super::{Failure, TRAP_WRITE, admission::with_journal_mut, intent};
use blob_test_protocol::storage::funding::CreditCommand;
use ic_blob_storage::{
    model::{billing::journal::credit::FundingCreditConfirmation, service::upload::UploadContext},
    workflow::funding::credit::{FundingCreditResult, confirm},
};
use std::num::NonZeroU128;

pub(crate) fn run(execution: UploadContext, input: CreditCommand) -> Result<Option<bool>, Failure> {
    let original = intent(input.intent)?;
    let accepted = NonZeroU128::new(input.accepted).ok_or(Failure::Invalid)?;
    TRAP_WRITE.set(input.fault);
    let result = with_journal_mut(|journal| {
        confirm(journal, execution, original, |_| {
            input.established.then_some(FundingCreditConfirmation {
                intent: original,
                accepted_cycles: accepted,
                receipt_digest: input.receipt_digest,
            })
        })
    });
    if input.fault == Some(blob_test_protocol::storage::WriteFault::FundingCommit) && result.is_ok()
    {
        ic_cdk::trap("fixture funding post-commit failure");
    }
    TRAP_WRITE.set(None);
    result
        .map(|result| match result {
            FundingCreditResult::NotEstablished => None,
            FundingCreditResult::Confirmed => Some(true),
            FundingCreditResult::AlreadyConfirmed => Some(false),
        })
        .map_err(super::failure)
}

pub(crate) fn renew(
    execution: UploadContext,
    input: blob_test_protocol::storage::funding::RenewalCommand,
) -> Result<bool, Failure> {
    let original = intent(input.intent)?;
    let additional = NonZeroU128::new(input.additional).ok_or(Failure::Invalid)?;
    TRAP_WRITE.set(input.fault);
    let result = with_journal_mut(|journal| {
        ic_blob_storage::workflow::funding::renewal::increase(
            journal,
            execution,
            ic_blob_storage::model::billing::journal::renewal::FundingBudgetRenewal {
                intent: original,
                additional,
            },
        )
    });
    if input.fault == Some(blob_test_protocol::storage::WriteFault::FundingCommit) && result.is_ok()
    {
        ic_cdk::trap("fixture funding post-commit failure");
    }
    TRAP_WRITE.set(None);
    result.map_err(super::failure)
}
