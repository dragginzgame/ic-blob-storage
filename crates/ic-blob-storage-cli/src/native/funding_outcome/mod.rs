//! Inspect retained exact funding intent without dispatch, retry or credit authority.
use super::{Failure, arguments::Options, history, query};
use ic_blob_storage_contracts::dto::funding::outcome::FundingBalanceField;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeFailure;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReconciliation;
use ic_blob_storage_contracts::dto::funding::outcome::FundingResponse;
use ic_blob_storage_contracts::funding::reply::FundingReplyError;
use ic_blob_storage_contracts::funding::reply::outcome;
use ic_blob_storage_contracts::protocol::FUNDING_OUTCOME_METHOD;
use serde_json::{Value, json};

pub(super) async fn run(options: &Options, input: FundingOutcomeRequest) -> Result<Value, Failure> {
    let argument = outcome::inspection_request(input).map_err(|_| Failure::Arguments)?;
    let bytes = query(
        options,
        input.scope.service,
        FUNDING_OUTCOME_METHOD,
        argument,
    )
    .await?;
    output(input, &bytes, options)
}

fn failure(error: FundingReplyError) -> Failure {
    match error {
        FundingReplyError::Limit => Failure::ReplyLimit,
        FundingReplyError::Invalid | FundingReplyError::History(_) => Failure::InvalidReply,
        FundingReplyError::Binding => Failure::Binding,
        FundingReplyError::Outcome(error) => match error {
            FundingOutcomeFailure::Invalid => Failure::ServiceInvalid,
            FundingOutcomeFailure::Denied => Failure::Denied,
            FundingOutcomeFailure::Binding => Failure::Binding,
            FundingOutcomeFailure::Conflict => Failure::FundingConflict,
            FundingOutcomeFailure::Internal => Failure::ServiceInternal,
        },
    }
}

fn response_json(response: FundingResponse) -> Value {
    match response {
        FundingResponse::NotDispatched => json!({"state":"not_dispatched"}),
        FundingResponse::NotEnqueued => json!({"state":"not_enqueued"}),
        FundingResponse::Rejected(code) => json!({"state":"rejected","code":code}),
        FundingResponse::ReportedSuccess(balance) => json!({"state":"reported_success",
            "balance":{"total":balance.total.to_string(),"prepaid":balance.prepaid.to_string(),
                "promotional":balance.promotional.to_string(),"ledger":balance.ledger.to_string()}}),
        FundingResponse::NotAuthorized(principal) => {
            json!({"state":"not_authorized","principal":principal.to_text()})
        }
        FundingResponse::AccountBalanceOverflow => json!({"state":"account_balance_overflow"}),
        FundingResponse::InternalError => json!({"state":"internal_error"}),
        FundingResponse::TopUpWithoutCycles => json!({"state":"top_up_without_cycles"}),
        FundingResponse::ReplyTooLarge => json!({"state":"reply_too_large"}),
        FundingResponse::InvalidReply => json!({"state":"invalid_reply"}),
        FundingResponse::InvalidBalance(field) => {
            json!({"state":"invalid_balance","field":match field {
                FundingBalanceField::Total => "total",
                FundingBalanceField::Prepaid => "prepaid",
                FundingBalanceField::Promotional => "promotional",
                FundingBalanceField::Ledger => "ledger",
            }})
        }
    }
}

fn output(input: FundingOutcomeRequest, bytes: &[u8], options: &Options) -> Result<Value, Failure> {
    let observed = outcome::inspect(input, bytes, 4096.try_into().expect("positive reply bound"))
        .map_err(failure)?;
    let credit_confirmed = observed.is_some_and(|view| {
        matches!(
            view.reconciliation,
            FundingReconciliation::CreditConfirmed { .. }
        )
    });
    let record = observed.map(|view| {
        let reconciliation = match view.reconciliation {
            FundingReconciliation::CreditConfirmed { accepted_cycles, receipt_digest } => {
                json!({"state":"credit_confirmed","accepted":accepted_cycles.to_string(),
                    "receipt_sha256":ic_blob_storage_contracts::identity::ContentDigest::try_from(receipt_digest.as_slice()).expect("fixed receipt digest").to_string()})
            }
            FundingReconciliation::NoTransfer => json!({"state":"no_transfer"}),
            FundingReconciliation::CreditRequired(amount) => {
                json!({"state":"credit_required","accepted":amount.to_string()})
            }
            FundingReconciliation::TransferUnknown(amount) => {
                json!({"state":"transfer_unknown","offered":amount.to_string()})
            }
        };
        json!({"phase":history::phase_json(view.phase),"response":view.response.map(response_json),
            "reconciliation":reconciliation,"renewed_allocation":view.renewed_allocation.to_string(),"fenced":view.fenced})
    });
    Ok(
        json!({"schema":1,"observation":"funding_outcome","operator":options.actor.to_text(),
        "network":options.network,"url":options.url.as_str(),"verification":"query_signatures",
        "scope":history::scope_json(input.scope),"operation":input.operation.to_string(),
        "offered":input.offered.to_string(),"target_balance":input.target_balance.map(|n|n.to_string()),
        "outcome":if record.is_some() {"found"} else {"absent"},"record":record,
        "retry_authorized":false,"provider_credit":if credit_confirmed { "host_confirmed" } else { "not_established" }}),
    )
}

#[cfg(test)]
mod tests;
