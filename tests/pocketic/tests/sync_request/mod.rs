//! Explicit fixture request construction from a passive current-owner observation.
use blob_test_protocol::{GatewaySyncRequest, status::OperatorStatusView};
use ic_platform::pocket_ic::PocketIc;
use ic_platform::principal::ExternalPrincipal as Principal;

pub(super) fn read(pic: &PocketIc, service: Principal, operator: Principal) -> GatewaySyncRequest {
    let status: Option<OperatorStatusView> =
        crate::support::query_fixture(pic, service, operator, "operator_status", ())
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
