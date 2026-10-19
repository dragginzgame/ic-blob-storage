use super::Failure;
use ic_blob_storage_contracts::dto::operator::LocalFundingStatus;
use ic_blob_storage_contracts::dto::operator::LocalServiceStatus;
use ic_blob_storage_contracts::dto::operator::LocalStatusFailure;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_platform::Principal;
use serde_json::{Value, json};

/// Shared bounded small replies; command owners retain error and binding policy.
pub(super) fn decode_small<T: serde::Serialize + for<'de> serde::Deserialize<'de>>(
    method: &str,
    bytes: &[u8],
    too_large: Failure,
) -> Result<T, Failure> {
    if bytes.len() > 4096 {
        return Err(too_large);
    }
    ic_blob_storage_contracts::wire::decode_response(method, bytes, 4096)
        .map_err(|_| Failure::InvalidReply)
}

pub(super) fn decode(bytes: &[u8], scope: OperatorScope) -> Result<LocalServiceStatus, Failure> {
    if bytes.len() > 64 * 1024 {
        return Err(Failure::ReplyTooLarge);
    }
    let response: Result<LocalServiceStatus, LocalStatusFailure> =
        ic_blob_storage_contracts::wire::decode_response("blob_local_status", bytes, 64 * 1024)
            .map_err(|_| Failure::InvalidReply)?;
    let status = response.map_err(|error| match error {
        LocalStatusFailure::Binding => Failure::Binding,
        LocalStatusFailure::Denied => Failure::Denied,
        LocalStatusFailure::Internal => Failure::ServiceInternal,
    })?;
    if status.scope != scope {
        return Err(Failure::Binding);
    }
    if status.gateways.members.len() > 1024
        || !ic_blob_storage_contracts::funding::reply::valid_journal(&status.funding)
    {
        return Err(Failure::InvalidReply);
    }
    Ok(status)
}

pub(super) fn scope_json(scope: OperatorScope) -> Value {
    json!({"service":scope.service.to_text(),"namespace":scope.namespace.to_string(),"cashier":scope.cashier.to_text(),"payment_account":scope.payment_account.to_text()})
}

pub(super) fn output(
    s: &LocalServiceStatus,
    operator: Principal,
    network: &str,
    url: &str,
) -> Value {
    json!({
        "schema":1,"observation":"local_status","network":network,"url":url,
        "operator":operator.to_text(),"verification":"query_signatures",
        "scope":scope_json(s.scope),
        "uploads":{
            "operations":s.uploads.operations.to_string(),"active_reservations":s.uploads.active_reservations.to_string(),
            "reserved_bytes":s.uploads.reserved_bytes.to_string(),"logical_bytes":s.uploads.logical_bytes.to_string(),
            "physical_bytes":s.uploads.physical_bytes.to_string(),"liability_bytes":s.uploads.liability_bytes.to_string(),"fenced":s.uploads.fenced},
        "funding":funding(&s.funding),
        "gateways":{"members":s.gateways.members.iter().map(Principal::to_text).collect::<Vec<_>>(),
            "last_sequence":s.gateways.last_sequence.to_string(),"pending_sequence":s.gateways.pending_sequence.map(|n|n.to_string()),"fenced":s.gateways.fenced},
        "reads":{"last_sequence":s.reads.last_sequence.to_string(),"sessions":s.reads.sessions.to_string(),
            "reserved_bytes":s.reads.reserved_bytes.to_string(),"fenced":s.reads.fenced}
    })
}
pub(super) fn funding(s: &LocalFundingStatus) -> Value {
    json!({"cumulative_allocation":s.cumulative_allocation.to_string(),"renewal_ceiling":s.renewal_ceiling.to_string(),
        "available_allocation":s.available_allocation.to_string(),"attachment_allowance":s.attachment_allowance.to_string(),
        "transport_accepted":s.transport_accepted.to_string(),"uncredited_accepted":s.uncredited_accepted.to_string(),"refunded":s.refunded.to_string(),
        "not_enqueued":s.not_enqueued.to_string(),"reserved_or_uncertain":s.reserved_or_uncertain.to_string(),
        "retained_intents":s.retained_intents.to_string(),"intent_capacity":s.intent_capacity.to_string(),
        "last_operation":s.last_operation.map(|n|n.to_string()),"fenced":s.fenced})
}
