//! Passive operator observations; no durable intent needed for reads.
use crate::ops;
use blob_test_protocol::consumer::Failure;
use candid::Principal;
use ic_blob_storage::dto::funding::{
    FundingHistoryPage, FundingHistoryRequest,
    outcome::{FundingOutcomeRequest, FundingOutcomeResponse},
};
pub(crate) async fn history(
    actor: Principal,
    input: FundingHistoryRequest,
    max: u32,
) -> Result<FundingHistoryPage, Failure> {
    ops::funding::history(actor, input, max).await
}
pub(crate) async fn outcome(
    actor: Principal,
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, Failure> {
    ops::funding::outcome(actor, input).await
}
