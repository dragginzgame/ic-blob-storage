use super::model::Selection;
use crate::operator::{
    Failure,
    ops::{Report, query_method},
};
use blob_test_protocol::funding::lookup::{
    FundingLookupFailure, FundingLookupState, FundingLookupView,
};
use candid::{de::DecoderConfig, decode_one_with_config};
use serde_json::json;

pub(super) fn query(selection: &Selection) -> Result<Report, Failure> {
    let bytes = query_method(
        &selection.target,
        "lookup_funding",
        candid::encode_one(selection.request).expect("bounded exact lookup"),
    )?;
    report(selection, &bytes)
}

fn report(selection: &Selection, bytes: &[u8]) -> Result<Report, Failure> {
    if bytes.len() > 4096 {
        return Err(Failure::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(1000)
        .set_max_type_len(32)
        .set_full_error_message(false);
    let result: Result<FundingLookupView, FundingLookupFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)?;
    let view = result.map_err(|failure| match failure {
        FundingLookupFailure::Denied => Failure::Denied,
        FundingLookupFailure::Binding => Failure::Binding,
        FundingLookupFailure::InvalidRequest => Failure::InvalidRequest,
        FundingLookupFailure::Conflict => Failure::Conflict,
    })?;
    if view.request != selection.request {
        return Err(Failure::Binding);
    }
    let (state, observation) = match view.state {
        FundingLookupState::Absent => ("Absent", None),
        FundingLookupState::Pending => ("Pending", None),
        FundingLookupState::Observed(value) => (
            "Observed",
            Some(json!({
                "refunded":value.refunded.map(|v| v.to_string()),
                "transport_accepted":value.transport_accepted.map(|v| v.to_string()),
                "outcome":value.outcome,
                    "reconciliation":crate::operator::ops::reconciliation_json(value.reconciliation),
            })),
        ),
    };
    let attempt = view.request.attempt;
    Ok(Report {
        blocked: observation.is_none(),
        value: json!({
            "request":{"id":attempt.id.to_string(), "offered":attempt.offered.to_string(), "accept":attempt.accept.to_string(), "reply":attempt.reply, "trap_callback":attempt.trap_callback},
            "fenced":view.fenced, "state":state, "observation":observation,
        }),
    })
}

#[cfg(test)]
mod tests;
