//! Passive bounded operator discovery, without a provider call or mutation.
use crate::ops::service::funding::StableFundingJournal;
use crate::ops::service::funding::history::boundary;
use ic_blob_storage_contracts::dto::funding::FundingHistoryFailure;
use ic_blob_storage_contracts::dto::funding::FundingHistoryPage;
use ic_blob_storage_contracts::dto::funding::FundingHistoryRequest;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroUsize;

/// Recover exact local identities and amounts as the installed operator.
/// The host supplies actual caller/service and a trusted result bound.
/// Restored reads remain fenced; history never establishes complete account
/// activity, provider credit, freshness or permission to repeat an attachment.
/// # Errors
/// Rejects actor/scope, malformed or mismatched cursors and inconsistent records.
pub fn inspect<M: Memory>(
    journal: &StableFundingJournal<M>,
    context: UploadContext,
    input: FundingHistoryRequest,
    limit: NonZeroUsize,
) -> Result<FundingHistoryPage, FundingHistoryFailure> {
    boundary::inspect(journal, context, input, limit)
}
