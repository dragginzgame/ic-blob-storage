use super::Failure;
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::dto::operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope};
use serde_json::{Value, json};

pub(super) fn decode(bytes: &[u8], scope: OperatorScope) -> Result<LocalServiceStatus, Failure> {
    if bytes.len() > 64 * 1024 {
        return Err(Failure::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(1_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(128)
        .set_full_error_message(false);
    let response: Result<LocalServiceStatus, LocalStatusFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)?;
    let status = response.map_err(|error| match error {
        LocalStatusFailure::Binding => Failure::Binding,
        LocalStatusFailure::Denied => Failure::Denied,
        LocalStatusFailure::Internal => Failure::ServiceInternal,
    })?;
    if status.scope != scope {
        return Err(Failure::Binding);
    }
    if status.gateways.members.len() > 1024 {
        return Err(Failure::InvalidReply);
    }
    Ok(status)
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
        "scope":{"service":s.scope.service.to_text(),"namespace":s.scope.namespace.to_string(),
            "cashier":s.scope.cashier.to_text(),"payment_account":s.scope.payment_account.to_text()},
        "uploads":{
            "operations":s.uploads.operations.to_string(),"active_reservations":s.uploads.active_reservations.to_string(),
            "reserved_bytes":s.uploads.reserved_bytes.to_string(),"logical_bytes":s.uploads.logical_bytes.to_string(),
            "physical_bytes":s.uploads.physical_bytes.to_string(),"liability_bytes":s.uploads.liability_bytes.to_string(),"fenced":s.uploads.fenced},
        "funding":{
            "available_allocation":s.funding.available_allocation.to_string(),"attachment_allowance":s.funding.attachment_allowance.to_string(),
            "transport_accepted":s.funding.transport_accepted.to_string(),"refunded":s.funding.refunded.to_string(),
            "not_enqueued":s.funding.not_enqueued.to_string(),"reserved_or_uncertain":s.funding.reserved_or_uncertain.to_string(),
            "retained_intents":s.funding.retained_intents.to_string(),"intent_capacity":s.funding.intent_capacity.to_string(),
            "last_operation":s.funding.last_operation.map(|n|n.to_string()),"fenced":s.funding.fenced},
        "gateways":{"members":s.gateways.members.iter().map(Principal::to_text).collect::<Vec<_>>(),
            "last_sequence":s.gateways.last_sequence.to_string(),"pending_sequence":s.gateways.pending_sequence.map(|n|n.to_string()),"fenced":s.gateways.fenced},
        "reads":{"last_sequence":s.reads.last_sequence.to_string(),"sessions":s.reads.sessions.to_string(),
            "reserved_bytes":s.reads.reserved_bytes.to_string(),"fenced":s.reads.fenced}
    })
}
