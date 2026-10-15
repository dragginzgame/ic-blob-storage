mod render;

use blob_test_protocol::{funding::FundingOperatorStatusView, status::OperatorStatusView};
use candid::{de::DecoderConfig, decode_one_with_config};
use serde_json::Value;

use super::{
    Failure,
    model::{Binding, Target},
};

#[derive(Debug, PartialEq)]
pub(super) struct Report {
    pub blocked: bool,
    pub value: Value,
}

pub(super) use render::budget as budget_json;
pub(super) use render::reconciliation as reconciliation_json;
pub(super) use render::target as target_json;

/// Small fixture replies share decoding limits; callers own binding and errors.
pub(super) fn decode_small<T>(bytes: &[u8]) -> Result<T, Failure>
where
    T: candid::CandidType + for<'de> serde::Deserialize<'de>,
{
    if bytes.len() > 4096 {
        return Err(Failure::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(1000)
        .set_max_type_len(32)
        .set_full_error_message(false);
    decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)
}

pub(super) fn query(target: &Target) -> Result<Vec<u8>, Failure> {
    query_method(
        target,
        "operator_status",
        candid::encode_args(()).expect("empty arguments"),
    )
}

pub(super) fn query_method(
    target: &Target,
    method: &str,
    args: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    super::transport::query_target(&target.into(), method, args)
}

pub(super) fn report(target: &Target, bytes: &[u8]) -> Result<Report, Failure> {
    if bytes.len() > 65_536 {
        return Err(Failure::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(1_000_000)
        .set_skipping_quota(10_000)
        .set_max_type_len(64)
        .set_full_error_message(false);
    match target.binding {
        Binding::Authority(namespace) => {
            let status: Option<OperatorStatusView> =
                decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)?;
            let status = status.ok_or(Failure::Denied)?;
            if status.service != target.canister || status.namespace != namespace {
                return Err(Failure::Binding);
            }
            Ok(Report {
                blocked: !status.blockers.is_empty(),
                value: render::authority(&status),
            })
        }
        Binding::Funding(peer) => {
            let status: Option<FundingOperatorStatusView> =
                decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)?;
            let status = status.ok_or(Failure::Denied)?;
            if status.service != target.canister || status.peer != peer {
                return Err(Failure::Binding);
            }
            Ok(Report {
                blocked: !status.blockers.is_empty(),
                value: render::funding(&status),
            })
        }
    }
}

#[cfg(test)]
mod tests;
