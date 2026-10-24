//! One authenticated passive service assessment, with no payment or balance inference.
use super::{Failure, arguments::Options, query, reply};
use ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationBlocker as B;
use ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationFailure as E;
use ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationRequest as Request;
use ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationResponse as Response;
use ic_blob_storage_contracts::funding::reply::assessment::{self, FundingAssessmentReplyError};
use ic_blob_storage_contracts::protocol;
use ic_blob_storage_contracts::protocol::FUNDING_PREPARATION_METHOD;
use serde_json::{Value, json};

fn decode(request: Request, bytes: &[u8]) -> Result<Response, Failure> {
    assessment::inspect(
        request,
        bytes,
        4096.try_into().expect("positive reply bound"),
    )
    .map_err(|error| match error {
        FundingAssessmentReplyError::Limit => Failure::ReplyTooLarge,
        FundingAssessmentReplyError::Invalid => Failure::InvalidReply,
        FundingAssessmentReplyError::Binding => Failure::Binding,
        FundingAssessmentReplyError::Remote(error) => Failure::FundingAssessmentRefused(error),
    })
}
fn blocker(b: B) -> Value {
    match b {
        B::ProviderUnqualified => json!({"kind":"provider_unqualified"}),
        B::JournalFenced => json!({"kind":"journal_fenced"}),
        B::JournalUncredited => json!({"kind":"journal_uncredited"}),
        B::IdentityRetained => json!({"kind":"identity_retained"}),
        B::IdentityStale => json!({"kind":"identity_stale"}),
        B::JournalFull => json!({"kind":"journal_full"}),
        B::AllocationReserve {
            transferable_cycles,
        } => {
            json!({"kind":"allocation_reserve","attachment_allowance":transferable_cycles.to_string()})
        }
        B::RecoveryUnknown => json!({"kind":"recovery_unknown"}),
        B::FundingUnknown => json!({"kind":"funding_unknown"}),
        B::SpendabilityUnknown => json!({"kind":"spendability_unknown"}),
    }
}
pub(super) async fn run(options: &Options, request: Request) -> Result<Value, Failure> {
    let bytes = query(
        options,
        request.scope.service,
        FUNDING_PREPARATION_METHOD,
        ic_blob_storage_contracts::wire::encode_request(
            protocol::FUNDING_PREPARATION_METHOD,
            request,
        )
        .map_err(|_| Failure::Arguments)?,
    )
    .await?;
    let response = decode(request, &bytes)?;
    Ok(output(options, &response))
}
fn output(options: &Options, response: &Response) -> Value {
    let r = response.request;
    json!({"schema":1,"observation":"funding_preparation_assessment",
        "operator":options.actor.to_text(),"network":options.network,"url":options.url.as_str(),
        "verification":"query_signatures",
        "request":{"scope":reply::scope_json(r.scope),
            "operation":r.operation.to_string(),"offered":r.offered.to_string(),"target_balance":r.target_balance.map(|n|n.to_string())},
        "journal":reply::funding(&response.journal),
        "blockers":response.blockers.iter().copied().map(blocker).collect::<Vec<_>>(),
        "preparation_authorized":false,"dispatch_authorized":false,"retry_authorized":false,
        "provider_credit":"not_established","spendability":"not_established"})
}
pub(super) const fn refusal_code(error: E) -> &'static str {
    match error {
        E::Invalid => "funding_invalid",
        E::Denied => "funding_denied",
        E::Binding => "funding_binding",
        E::Conflict => "funding_conflict",
        E::Internal => "funding_internal",
    }
}

#[cfg(test)]
mod tests;
