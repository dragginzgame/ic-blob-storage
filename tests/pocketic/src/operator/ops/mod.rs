mod render;

use blob_test_protocol::{funding::FundingOperatorStatusView, status::OperatorStatusView};
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

/// Decode exactly one method-bound fixture reply; no trailing result is ignored.
pub(super) fn decode_small<T: serde::de::DeserializeOwned>(
    method: &str,
    bytes: &[u8],
) -> Result<T, Failure> {
    decode(method, bytes, 4096)
}
fn decode<T: serde::de::DeserializeOwned>(
    method: &str,
    bytes: &[u8],
    maximum: usize,
) -> Result<T, Failure> {
    if bytes.len() > maximum {
        return Err(Failure::ReplyTooLarge);
    }
    blob_test_protocol::fixture_wire::decode_one(method, bytes, maximum)
        .map_err(|_| Failure::InvalidReply)
}

pub(super) fn query(target: &Target) -> Result<Vec<u8>, Failure> {
    query_method(
        target,
        "operator_status",
        blob_test_protocol::fixture_wire::encode_args("operator_status", ())
            .expect("empty arguments"),
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
    let maximum = 65_536;
    match target.binding {
        Binding::Authority(namespace) => {
            let status: Option<OperatorStatusView> = decode("operator_status", bytes, maximum)?;
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
            let status: Option<FundingOperatorStatusView> =
                decode("operator_status", bytes, maximum)?;
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
