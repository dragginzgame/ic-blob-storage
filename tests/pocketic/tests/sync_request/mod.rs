//! Explicit fixture request construction from a passive current-owner observation.
use blob_test_protocol::{GatewaySyncRequest, status::OperatorStatusView};
use ic_blob_storage_pocketic_tests::fixture_calls;
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::pocket_ic::PocketIc;

pub(super) async fn read(
    pic: &PocketIc,
    service: Principal,
    operator: Principal,
) -> GatewaySyncRequest {
    let status: Option<OperatorStatusView> =
        fixture_calls::query_fixture(pic, service, operator, "operator_status", ())
            .await
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
