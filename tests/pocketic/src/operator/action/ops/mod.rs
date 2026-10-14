use super::model::{Request, Selection};
use crate::operator::{
    Failure,
    ops::{decode_small, query_method},
};
use blob_test_protocol::balance::BalanceFailure;
use ic_testkit::pocket_ic::PocketIc;
use serde_json::{Value, json};
use std::panic::{AssertUnwindSafe, catch_unwind};

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
    let bytes = catch_unwind(AssertUnwindSafe(|| {
        let args = match &selection.request {
            Request::Balance(r) => candid::encode_one(r),
            Request::Gateway(r) => candid::encode_one(r),
        }
        .expect("bounded action request");
        if selection.dry_run {
            query_method(&selection.target, method(selection), args)
        } else {
            let target = &selection.target;
            let url = format!("http://{}/", target.server)
                .parse()
                .expect("validated socket URL");
            let pic = PocketIc::new_from_existing_instance(url, target.instance, Some(10_000));
            // Submit exactly this identity once at the application layer. SDK
            // polling/busy handling does not authorize a new sequence or request.
            pic.update_call(target.canister, target.caller, method(selection), args)
                .map_err(|_| Failure::Transport)
        }
    }))
    .map_err(|_| Failure::Transport)??;
    match selection.request {
        Request::Balance(_) => {
            decode_small::<Result<(), BalanceFailure>>(&bytes).map(|r| r.map_err(|e| json!(e)))
        }
        Request::Gateway(_) => decode_small::<Result<(), blob_test_protocol::SyncFailure>>(&bytes)
            .map(|r| r.map_err(|e| json!(e))),
    }
}
