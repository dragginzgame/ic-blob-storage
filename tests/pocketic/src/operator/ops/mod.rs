mod render;

use blob_test_protocol::{funding::FundingOperatorStatusView, status::OperatorStatusView};
use ic_platform::candid::{DecodeError, DecodeLimits};
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
    T: ic_platform::candid::contract::CandidType + for<'de> serde::Deserialize<'de>,
{
    decode(
        bytes,
        DecodeLimits {
            reply_bytes: 4096,
            decoding_quota: 100_000,
            skipping_quota: 1000,
            type_entries: 32,
            header_bytes: 4096,
        },
    )
}

/// Codec mechanics belong to Platform; fixtures keep their exact limits and errors.
pub(super) fn decode<T>(bytes: &[u8], limits: DecodeLimits) -> Result<T, Failure>
where
    T: ic_platform::candid::contract::CandidType + for<'de> serde::Deserialize<'de>,
{
    ic_platform::candid::decode_one(bytes, limits).map_err(|error| match error {
        DecodeError::ReplyTooLarge { .. } => Failure::ReplyTooLarge,
        DecodeError::Candid(_) => Failure::InvalidReply,
    })
}

pub(super) fn query(target: &Target) -> Result<Vec<u8>, Failure> {
    query_method(
        target,
        "operator_status",
        ic_platform::candid::contract::encode_args(()).expect("empty arguments"),
    )
}

pub(super) fn query_method(
    target: &Target,
    method: &str,
    args: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    super::transport::query_target(&target.transport, method, args)
}

pub(super) fn report(target: &Target, bytes: &[u8]) -> Result<Report, Failure> {
    let limits = DecodeLimits {
        reply_bytes: 65_536,
        decoding_quota: 1_000_000,
        skipping_quota: 10_000,
        type_entries: 64,
        header_bytes: 65_536,
    };
    match target.binding {
        Binding::Authority(namespace) => {
            let status: Option<OperatorStatusView> = decode(bytes, limits)?;
            let status = status.ok_or(Failure::Denied)?;
            if status.service != target.transport.canister.into() || status.namespace != namespace {
                return Err(Failure::Binding);
            }
            Ok(Report {
                blocked: !status.blockers.is_empty(),
                value: render::authority(&status),
            })
        }
        Binding::Funding(peer) => {
            let status: Option<FundingOperatorStatusView> = decode(bytes, limits)?;
            let status = status.ok_or(Failure::Denied)?;
            if status.service != target.transport.canister.into() || status.peer != peer.into() {
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
