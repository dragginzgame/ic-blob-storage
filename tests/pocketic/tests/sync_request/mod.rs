//! Explicit fixture request construction from a passive current-owner observation.
use crate::support::REPLY_LIMITS;
use blob_test_protocol::{GatewaySyncRequest, status::OperatorStatusView};
use candid::Principal;
use ic_testkit::pic::query_candid_args;
use ic_testkit::pocket_ic::PocketIc;

pub(super) fn read(pic: &PocketIc, service: Principal, operator: Principal) -> GatewaySyncRequest {
    let status: Option<OperatorStatusView> =
        query_candid_args(pic, service, operator, "operator_status", (), REPLY_LIMITS)
            .map(|(reply,)| reply)
            .unwrap();
    let status = status.expect("fixture operator");
    GatewaySyncRequest {
        service,
        namespace: status.namespace,
        source: status.sync_source,
        revision: status.sync_revision.expect("fixture edit capacity"),
        sequence: status
            .last_sync
            .checked_add(1)
            .expect("fixture sync capacity"),
    }
}
