mod render;

use std::panic::{AssertUnwindSafe, catch_unwind};

use blob_test_protocol::{funding::FundingOperatorStatusView, status::OperatorStatusView};
use candid::Principal;
use candid::{de::DecoderConfig, decode_one_with_config};
use ic_testkit::pocket_ic::PocketIc;
use serde_json::Value;
use std::net::SocketAddr;

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
    query_target(
        &QueryTarget {
            server: target.server,
            instance: target.instance,
            canister: target.canister,
            caller: target.caller,
        },
        method,
        args,
    )
}

/// Explicit transport selection, independent of each fixture's semantic binding.
pub(super) struct QueryTarget {
    pub server: SocketAddr,
    pub instance: usize,
    pub canister: Principal,
    pub caller: Principal,
}

pub(super) fn query_target(
    target: &QueryTarget,
    method: &str,
    args: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    // PocketIC's public SDK panics on HTTP/API failures. Keep that boundary distinct
    // from canister rejection; never turn either outcome into an update or retry.
    catch_unwind(AssertUnwindSafe(|| {
        let url = format!("http://{}/", target.server)
            .parse()
            .expect("validated socket URL");
        let pic = PocketIc::new_from_existing_instance(url, target.instance, Some(10_000));
        pic.query_call(target.canister, target.caller, method, args)
            .map_err(|_| Failure::QueryRejected)
        // The attached SDK owns neither this instance nor an HTTP gateway.
    }))
    .map_err(|_| Failure::Transport)?
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
