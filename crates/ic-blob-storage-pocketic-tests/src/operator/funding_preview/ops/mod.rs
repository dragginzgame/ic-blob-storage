use super::model::Selection;
use crate::operator::{
    Failure,
    ops::{Report, query_method},
};
use blob_test_protocol::funding::preview::{
    FundingPreviewBlocker, FundingPreviewFailure, FundingPreviewView,
};
use candid::{de::DecoderConfig, decode_one_with_config};
use serde_json::{Value, json};

pub(super) fn query(selection: &Selection) -> Result<Report, Failure> {
    let bytes = query_method(
        &selection.target,
        "preview_funding",
        candid::encode_one(selection.request).expect("bounded preview request"),
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
    let result: Result<FundingPreviewView, FundingPreviewFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)?;
    let view = result.map_err(|e| match e {
        FundingPreviewFailure::Denied => Failure::Denied,
        FundingPreviewFailure::Binding => Failure::Binding,
        FundingPreviewFailure::InvalidAmount => Failure::InvalidReply,
    })?;
    if view.request != selection.request {
        return Err(Failure::Binding);
    }
    Ok(Report {
        blocked: !view.blockers.is_empty(),
        value: json!({
            "id":view.request.id.to_string(), "requested_cycles":view.request.requested_cycles.to_string(),
            "revision":view.request.revision.to_string(),
            "available_cycles":view.available_cycles.map(|v| v.to_string()),
            "budget":crate::operator::ops::budget_json(&view.budget),
            "liquidity":{"liquid_cycles":view.liquidity.liquid_cycles.to_string(), "call_cost":view.liquidity.call_cost.to_string()},
            "blockers":view.blockers.into_iter().map(blocker).collect::<Vec<_>>(),
        }),
    })
}

#[cfg(test)]
mod tests;

fn blocker(value: FundingPreviewBlocker) -> Value {
    match value {
        FundingPreviewBlocker::AmountLimitExceeded { maximum_cycles } => {
            json!({"AmountLimitExceeded":{"maximum_cycles":maximum_cycles.to_string()}})
        }
        FundingPreviewBlocker::LiquidityWouldBeViolated {
            transferable_cycles,
        } => {
            json!({"LiquidityWouldBeViolated":{"transferable_cycles":transferable_cycles.to_string()}})
        }
        FundingPreviewBlocker::BudgetReserveWouldBeViolated {
            transferable_cycles,
        } => {
            json!({"BudgetReserveWouldBeViolated":{"transferable_cycles":transferable_cycles.to_string()}})
        }
        FundingPreviewBlocker::ReserveWouldBeViolated {
            requested_cycles,
            transferable_cycles,
        } => {
            json!({"ReserveWouldBeViolated":{"requested_cycles":requested_cycles.to_string(), "transferable_cycles":transferable_cycles.to_string()}})
        }
        other => json!(other),
    }
}
