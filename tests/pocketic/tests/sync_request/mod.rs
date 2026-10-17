//! Explicit fixture request construction from a passive current-owner observation.
use crate::support::REPLY_LIMITS;
use blob_test_protocol::{GatewaySyncRequest, status::OperatorStatusView};
use candid::Principal;
use ic_testkit::{pic::CandidCallExt, pocket_ic::PocketIc};

pub(super) fn read(pic: &PocketIc, service: Principal, operator: Principal) -> GatewaySyncRequest {
    let status: Option<OperatorStatusView> = pic
        .query_candid(service, operator, "operator_status", (), REPLY_LIMITS)
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
