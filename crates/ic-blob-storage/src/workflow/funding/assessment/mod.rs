//! Current unqualified-host preparation diagnosis; no reservation or provider query.
use crate::ops::service::funding::StableFundingJournal;
use crate::ops::service::funding::assessment;
use ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationFailure;
use ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationRequest;
use ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

/// Inspect one proposed new intent synchronously with authenticated local facts.
/// Independent provider, recovery, spendability and account-activity evidence is
/// explicitly missing. The request cannot supply it, clear a fence or reserve funds.
/// # Errors
/// Rejects invalid amounts, operator/scope, changed retained intent or inconsistent state.
pub fn inspect<M: Memory>(
    journal: &StableFundingJournal<M>,
    context: UploadContext,
    request: FundingPreparationRequest,
) -> Result<FundingPreparationResponse, FundingPreparationFailure> {
    let (intent, evidence) = assessment::input(request)?;
    let view = super::inspect_preparation(journal, context, intent, evidence)
        .map_err(assessment::failure)?;
    assessment::present(request, &view.journal, view.assessment.blockers())
}
