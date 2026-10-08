//! Host-internal local budget authority, independent of provider effects.
use crate::model::billing::journal::renewal::FundingBudgetRenewal;
use crate::ops::service::funding::FundingJournalError;
use crate::ops::service::funding::StableFundingJournal;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

/// Commit one host-authorized budget increase after exact credit reconciliation.
/// Linking creates no endpoint. Caller authentication and the installed ceiling
/// remain authoritative; no grant permits dispatch or restores lifetime slots.
/// # Errors
/// Returns the journal's typed authority, reconciliation and bound refusals.
pub fn increase<M: Memory>(
    journal: &mut StableFundingJournal<M>,
    context: UploadContext,
    input: FundingBudgetRenewal,
) -> Result<bool, FundingJournalError> {
    journal.renew_budget(context, input)
}
