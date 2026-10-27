use super::model::{Request, Selection};
use crate::operator::{
    Failure,
    ops::{decode_small, query_method},
};
use blob_test_protocol::balance::BalanceFailure;
use serde_json::{Value, json};

pub(super) fn method(selection: &Selection) -> &'static str {
    match (&selection.request, selection.dry_run) {
        (Request::Balance(_), true) => "preview_balance_refresh",
        (Request::Balance(_), false) => "refresh_balance",
        (Request::Gateway(_), true) => "preview_gateway_sync",
        (Request::Gateway(_), false) => "sync_gateway",
    }
}

pub(super) fn request_json(selection: &Selection) -> Value {
    match &selection.request {
        Request::Balance(r) => {
            json!({"source":r.scope.source.to_text(), "account":r.scope.account.to_text(), "revision":r.revision.to_string(), "sequence":r.sequence.to_string()})
        }
        Request::Gateway(r) => {
            json!({"source":r.source.to_text(), "revision":r.revision.to_string(), "sequence":r.sequence.to_string()})
        }
    }
}

pub(super) fn execute(selection: &Selection) -> Result<Result<(), Value>, Failure> {
    let args = match &selection.request {
        Request::Balance(request) => {
            blob_test_protocol::fixture_wire::encode_one(method(selection), request)
        }
        Request::Gateway(request) => {
            blob_test_protocol::fixture_wire::encode_one(method(selection), request)
        }
    }
    .map_err(|_| Failure::Transport)?;
    let bytes = if selection.dry_run {
        query_method(&selection.target, method(selection), args)
    } else {
        crate::operator::transport::update_target(
            &selection.target.transport,
            method(selection),
            args,
        )
    }?;
    match selection.request {
        Request::Balance(_) => {
            decode_small::<Result<(), BalanceFailure>>(method(selection), &bytes)
                .map(|r| r.map_err(|e| json!(e)))
        }
        Request::Gateway(_) => {
            decode_small::<Result<(), blob_test_protocol::SyncFailure>>(method(selection), &bytes)
                .map(|r| r.map_err(|e| json!(e)))
        }
    }
}
