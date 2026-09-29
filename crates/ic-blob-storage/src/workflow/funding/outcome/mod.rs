//! Exact retained outcome inspection without payment, retry or balance refresh.
use crate::{
    dto::funding::outcome::{FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse},
    model::service::upload::UploadContext,
    ops::service::funding::{StableFundingJournal, outcome::boundary},
    policy::billing::reconciliation::assess_funding_reconciliation,
};
use ic_memory::ic_stable_structures::Memory;
/// Inspect as the configured operator with actual caller/service supplied by the host.
/// A missing intent never authorizes a new payment. Transport-only observations
/// preserve missing structured replies; reported balances never establish credit.
/// Restoration keeps observations readable and all mutation fences in force.
/// # Errors
/// Rejects invalid input, scope/actor, changed original intent or inconsistent state.
pub fn inspect<M: Memory>(
    journal: &StableFundingJournal<M>,
    context: UploadContext,
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
    let view = boundary::read(journal, context, input)?;
    Ok(view.map(|view| {
        boundary::present(
            input,
            &view,
            assess_funding_reconciliation(view.transfer),
            journal.is_fenced(),
        )
    }))
}
