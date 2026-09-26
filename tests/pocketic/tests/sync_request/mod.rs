//! Explicit fixture request construction from a passive current-owner observation.
use blob_test_protocol::{GatewaySyncRequest, status::OperatorStatusView};
use candid::Principal;
use ic_testkit::{pic::CandidCallExt, pocket_ic::PocketIc};

pub(super) fn read(pic: &PocketIc, service: Principal, operator: Principal) -> GatewaySyncRequest {
    let status: Option<OperatorStatusView> = pic
        .query_candid_as(service, operator, "operator_status", ())
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
