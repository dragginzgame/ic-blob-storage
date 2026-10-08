//! Authenticated fixture observer using the shared passive client; no extra journal.
use blob_test_protocol::consumer::Failure;
use candid::Principal;
use ic_blob_storage::ops::service::funding::client::ReplicatedFundingClient;
use ic_blob_storage_contracts::dto::funding::FundingHistoryPage;
use ic_blob_storage_contracts::dto::funding::FundingHistoryRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeResponse;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::funding::reply::FundingHistoryReplyLimits;
fn client(actor: Principal, scope: OperatorScope) -> Result<ReplicatedFundingClient, Failure> {
    super::read(actor, |r| {
        if r.service != scope.service {
            return Err(Failure::Invalid);
        }
        Ok(())
    })?;
    ReplicatedFundingClient::new(ic_cdk::api::canister_self(), scope, 30.try_into().unwrap())
        .map_err(|_| Failure::Invalid)
}
pub(crate) async fn history(
    actor: Principal,
    input: FundingHistoryRequest,
    max: u32,
) -> Result<FundingHistoryPage, Failure> {
    let client = client(actor, input.scope)?;
    let limits = FundingHistoryReplyLimits {
        bytes: (max as usize).try_into().map_err(|_| Failure::Invalid)?,
        entries: 32.try_into().unwrap(),
    };
    client
        .history(input, limits)
        .await
        .map_err(|_| Failure::Transport)
}
pub(crate) async fn outcome(
    actor: Principal,
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, Failure> {
    client(actor, input.scope)?
        .outcome(input, 4096.try_into().unwrap())
        .await
        .map_err(|_| Failure::Transport)
}
