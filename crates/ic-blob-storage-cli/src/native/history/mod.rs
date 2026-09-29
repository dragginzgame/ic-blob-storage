use super::{Failure, read};
use candid::Principal;
use ic_blob_storage::{
    dto::{
        funding::{
            FundingHistoryCursor, FundingHistoryFailure, FundingHistoryRequest, FundingPhase,
        },
        operator::OperatorScope,
    },
    ops::service::funding::reply::{self, FundingHistoryReplyLimits, FundingReplyError},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CursorInput {
    scope: ScopeInput,
    before_operation: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScopeInput {
    service: String,
    namespace: String,
    cashier: String,
    payment_account: String,
}
fn decimal(text: &str) -> Result<u128, Failure> {
    let n: u128 = text.parse().map_err(|_| Failure::Arguments)?;
    if n == 0 || n.to_string() != text {
        return Err(Failure::Arguments);
    }
    Ok(n)
}

pub(super) fn request(
    scope: OperatorScope,
    cursor: Option<&Path>,
) -> Result<FundingHistoryRequest, Failure> {
    let cursor = cursor
        .map(|path| {
            let input: CursorInput =
                serde_json::from_slice(&read(path, 2048)?).map_err(|_| Failure::Arguments)?;
            let matches = input.scope.service == scope.service.to_text()
                && decimal(&input.scope.namespace)? == scope.namespace
                && input.scope.cashier == scope.cashier.to_text()
                && input.scope.payment_account == scope.payment_account.to_text();
            if !matches {
                return Err(Failure::CursorScope);
            }
            Ok(FundingHistoryCursor {
                scope,
                before_operation: decimal(&input.before_operation)?,
            })
        })
        .transpose()?;
    Ok(FundingHistoryRequest { scope, cursor })
}
fn scope_json(scope: OperatorScope) -> Value {
    json!({"service":scope.service.to_text(),"namespace":scope.namespace.to_string(),"cashier":scope.cashier.to_text(),"payment_account":scope.payment_account.to_text()})
}
fn cursor_json(cursor: FundingHistoryCursor) -> Value {
    json!({"scope":scope_json(cursor.scope),"before_operation":cursor.before_operation.to_string()})
}
fn failure(error: FundingReplyError) -> Failure {
    match error {
        FundingReplyError::Limit => Failure::ReplyLimit,
        FundingReplyError::Invalid | FundingReplyError::Outcome(_) => Failure::InvalidReply,
        FundingReplyError::Binding => Failure::Binding,
        FundingReplyError::History(error) => match error {
            FundingHistoryFailure::Invalid => Failure::ServiceInvalid,
            FundingHistoryFailure::Denied => Failure::Denied,
            FundingHistoryFailure::Binding => Failure::Binding,
            FundingHistoryFailure::CursorScope => Failure::CursorScope,
            FundingHistoryFailure::Internal => Failure::ServiceInternal,
        },
    }
}
pub(super) fn output(
    request: FundingHistoryRequest,
    bytes: &[u8],
    operator: Principal,
    network: &str,
    url: &str,
) -> Result<Value, Failure> {
    let page = reply::history(
        request,
        bytes,
        FundingHistoryReplyLimits {
            bytes: (64 * 1024).try_into().expect("positive byte limit"),
            entries: 32.try_into().expect("positive page limit"),
        },
    )
    .map_err(failure)?;
    let entries: Vec<_> = page.entries.iter().map(|entry| {
        let phase = match entry.phase {
            FundingPhase::Prepared => json!({"state":"prepared"}),
            FundingPhase::Uncertain => json!({"state":"uncertain"}),
            FundingPhase::NotEnqueued => json!({"state":"not_enqueued"}),
            FundingPhase::Callback { refunded } => json!({"state":"callback","refunded":refunded.to_string()}),
        };
        json!({"operation":entry.operation.to_string(),"offered":entry.offered.to_string(),"target_balance":entry.target_balance.map(|n|n.to_string()),"phase":phase})
    }).collect();
    Ok(
        json!({"schema":1,"observation":"funding_history","network":network,"url":url,
        "operator":operator.to_text(),"verification":"query_signatures","scope":scope_json(request.scope),
        "cursor":request.cursor.map(cursor_json),"entries":entries,"next":page.next.map(cursor_json),"fenced":page.fenced}),
    )
}

#[cfg(test)]
mod tests;
